use std::sync::{Arc, Once};

use once_cell::sync::OnceCell;

use super::image_file::ImageStrings;

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
#[derive(Default)]
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
        compressed_data: &Vec<u8>,
        uncompressed_size: u64,
        strings: &ImageStrings,
    ) -> Option<Vec<u8>> {
        let mut compressed_resource = compressed_data;
        let mut uncompressed_data: Option<Vec<u8>> = None;
        loop {
            let mut header = ResourceHeader::default();
            let mut offset = 0;
            header.magic = Decompressors::get_u32(compressed_resource, offset).unwrap();
            offset += 4;
            header.size = Decompressors::get_u64(compressed_resource, offset).unwrap();
            offset += 8;
            header.uncompressed_size = Decompressors::get_u64(compressed_resource, offset).unwrap();
            offset += 8;
            header.decompressor_name_offset =
                Decompressors::get_u32(compressed_resource, offset).unwrap();
            offset += 4;
            header.decompressor_config_offset =
                Decompressors::get_u32(compressed_resource, offset).unwrap();
            offset += 4;
            header.is_terminal = Decompressors::get_u8(compressed_resource, offset).unwrap();
            let has_header = header.magic == ResourceHeader::RESOURCE_HEADER_MAGIC;
            if !has_header {
                break;
            }
            let decompressor_name = strings.get(header.decompressor_name_offset).unwrap();
            if let Some(decompressor) = Decompressors::get_decompressor(&decompressor_name) {
                if let Some(data) = decompressor.decompress_resource(
                    compressed_resource,
                    uncompressed_size,
                    &header,
                    strings,
                ) {
                    uncompressed_data = Some(data);
                    compressed_resource = uncompressed_data.as_ref().unwrap();
                };
            }
        }
        uncompressed_data
    }

    fn get_u32(data: &Vec<u8>, index: usize) -> Option<u32> {
        if let Some(bytes) = data.get(index..index + 4) {
            return Some(u32::from_ne_bytes(bytes.try_into().unwrap()));
        }
        None
    }

    fn get_u64(data: &Vec<u8>, index: usize) -> Option<u64> {
        if let Some(bytes) = data.get(index..index + 8) {
            return Some(u64::from_ne_bytes(bytes.try_into().unwrap()));
        }
        None
    }

    fn get_u8(data: &Vec<u8>, index: usize) -> Option<u8> {
        if let Some(bytes) = data.get(index) {
            return Some(u8::from_ne_bytes([bytes.clone()]));
        }
        None
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
        compressed_data: &Vec<u8>,
        uncompressed_size: u64,
        resource_header: &ResourceHeader,
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
        compressed_data: &Vec<u8>,
        uncompressed_size: u64,
        resource_header: &ResourceHeader,
        strings: &ImageStrings,
    ) -> Option<Vec<u8>> {
        todo!()
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
        compressed_data: &Vec<u8>,
        uncompressed_size: u64,
        resource_header: &ResourceHeader,
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
