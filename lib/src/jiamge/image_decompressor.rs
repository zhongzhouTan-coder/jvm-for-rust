use std::sync::{Arc, Once};

use once_cell::sync::OnceCell;

use super::{image_file::ImageStrings, zip_utils::inflate};

/// Compressed resource located in image have an header
/// The header contains:
/// magic: A magic u32, required to retrieved the header in the compressed content
/// size: The size of the compressed resource
/// uncompressed_size: The uncompressed size of the compressed resource
/// decompressor_name_offset: The ImageDecompressor instance name StringTable offset
/// decompressor_config_offset: StringTable offset of configuration that could be needed by
/// the decompressor in order to decompress
/// is_terminal: 1 the compressed content is terminal. Uncompressing it would create the actual resource
/// 0 the compressed content is not terminal. Uncompressing it will result in a compressed content to be
/// decompressed (This occurs when a stack of compressors have been used to compress the resource)
pub struct ResourceHeader {
    magic: u32,
    size: u64,
    uncompressed_size: u64,
    decompressor_name_offset: u32,
    decompressor_config_offset: u32,
    is_terminal: u8,
}

impl ResourceHeader {
    const RESOURCE_HEADER_MAGIC: u32 = 0xCAFEFAFAu32;

    fn from_bytes(bytes: Vec<u8>) -> ResourceHeader {
        assert!(
            bytes.len() == std::mem::size_of::<ResourceHeader>(),
            "invalid resource header data"
        );
        ResourceHeader {
            magic: u32::from_ne_bytes(bytes[0..4].try_into().unwrap()),
            size: u64::from_ne_bytes(bytes[4..12].try_into().unwrap()),
            uncompressed_size: u64::from_ne_bytes(bytes[12..20].try_into().unwrap()),
            decompressor_name_offset: u32::from_ne_bytes(bytes[20..24].try_into().unwrap()),
            decompressor_config_offset: u32::from_ne_bytes(bytes[24..28].try_into().unwrap()),
            is_terminal: bytes[28],
        }
    }
}

pub struct Decompressors {
    decompressors: Vec<Arc<dyn ImageDecompressor>>,
}

impl Decompressors {
    pub fn get_decompressor(name: &str) -> Option<Arc<dyn ImageDecompressor>> {
        static INSTANCE: OnceCell<Decompressors> = OnceCell::new();
        let decompressor = INSTANCE.get_or_init(|| {
            let mut decompressors: Vec<Arc<dyn ImageDecompressor>> = Vec::new();
            decompressors.push(Arc::new(ZipDecompressor::initialize()));
            decompressors.push(Arc::new(SharedStringDecompressor::initialize()));
            Decompressors { decompressors }
        });
        decompressor
            .decompressors
            .iter()
            .find(|dec| dec.get_name() == name)
            .map(|dec| Arc::clone(dec))
    }

    pub fn decompress_resource(
        compressed_data: Vec<u8>,
        uncompressed_size: u64,
        strings: ImageStrings,
    ) -> Option<Vec<u8>> {
        let mut data = compressed_data;
        loop {
            let header_size = std::mem::size_of::<ResourceHeader>();
            let header = ResourceHeader::from_bytes(data[0..header_size].to_vec());
            let has_header = header.magic == ResourceHeader::RESOURCE_HEADER_MAGIC;
            if !has_header {
                break;
            }
            let decompressor_name = strings.get(header.decompressor_name_offset).unwrap();
            let decompressor = Decompressors::get_decompressor(&decompressor_name)
                .expect("unrecognized decompressor name.");
            if let Some(content) =
                decompressor.decompress_resource(data[header_size..].to_vec(), header, &strings)
            {
                data = content;
            } else {
                return None;
            };
        }
        assert!(
            data.len() == uncompressed_size as usize,
            "fail to decompress compressed data."
        );
        Some(data)
    }
}

/// Resource located in jimage file can be compressed. Compression occurs at
/// jimage file creation time. When compressed a resource is added an header that
/// contains the name of the compressor that compressed it.
/// Various compression strategies can be applied to compress a resource.
/// The same resource can even be compressed multiple time by a stack of compressors.
/// At runtime, a resource is decompressed in a loop until there is no more header
/// meaning that the resource is equivalent to the not compressed resource.
pub trait ImageDecompressor: Sync + Send {
    fn get_name(&self) -> String;
    fn decompress_resource(
        &self,
        compressed_data: Vec<u8>,
        resource_header: ResourceHeader,
        strings: &ImageStrings,
    ) -> Option<Vec<u8>>;
}

pub struct ZipDecompressor {
    name: String,
}

impl ImageDecompressor for ZipDecompressor {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn decompress_resource(
        &self,
        compressed_data: Vec<u8>,
        resource_header: ResourceHeader,
        strings: &ImageStrings,
    ) -> Option<Vec<u8>> {
        let mut in_buf = compressed_data;
        let in_len: u64 = resource_header.size;
        let mut out_buf: Vec<u8> = vec![0u8; resource_header.uncompressed_size as usize];
        let out_len = resource_header.uncompressed_size;
        if let Err(super::jimage_error::JImageError::DecompressError(msg)) =
            inflate(&mut out_buf, out_len, &mut in_buf, in_len)
        {
            println!("{}", msg);
            return None;
        }
        Some(out_buf)
    }
}

impl ZipDecompressor {
    pub fn initialize() -> Self {
        ZipDecompressor {
            name: "zip".to_string(),
        }
    }
}

pub struct SharedStringDecompressor {
    name: String,
}

impl ImageDecompressor for SharedStringDecompressor {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn decompress_resource(
        &self,
        compressed_data: Vec<u8>,
        resource_header: ResourceHeader,
        strings: &ImageStrings,
    ) -> Option<Vec<u8>> {
        todo!()
    }
}

impl SharedStringDecompressor {
    pub fn initialize() -> Self {
        SharedStringDecompressor {
            name: "compact-cp".to_string(),
        }
    }
}
