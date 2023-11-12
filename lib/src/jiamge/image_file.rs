use super::{
    image_decompressor::{Decompressors, ImageDecompressor},
    jimage_error::JImageError,
};
use once_cell::sync::OnceCell;
use std::io::{Read, Seek, SeekFrom};
use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    mem,
    sync::{Arc, Mutex},
};

#[derive(Default)]
pub struct ImageFileReader {
    name: String,
    file_size: u64,
    header: ImageHeader,
    index_size: usize,
    redirect_table: Option<Arc<Vec<i32>>>,
    offsets_table: Option<Arc<Vec<u32>>>,
    location_bytes: Option<Arc<Vec<u8>>>,
    string_bytes: Option<Arc<Vec<u8>>>,
    resource_bytes: Option<Arc<Vec<u8>>>,
}

static INSTANCE: OnceCell<Mutex<HashMap<String, Arc<ImageFileReader>>>> = OnceCell::new();

impl ImageFileReader {
    pub fn new(name: String) -> Self {
        ImageFileReader {
            name,
            file_size: 0,
            header: ImageHeader::default(),
            index_size: 0,
            redirect_table: None,
            offsets_table: None,
            location_bytes: None,
            string_bytes: None,
            resource_bytes: None,
        }
    }

    pub fn open(name: String) -> Result<Arc<ImageFileReader>, JImageError> {
        let mut readers = INSTANCE
            .get_or_init(|| {
                let reader_map = HashMap::new();
                Mutex::new(reader_map)
            })
            .lock()
            .unwrap();
        if let Some(reader) = readers.get(&name) {
            Ok(Arc::clone(reader))
        } else {
            let mut reader = ImageFileReader::new(name.clone());
            reader.open_image()?;
            let reader = Arc::new(reader);
            readers.insert(name, Arc::clone(&reader));
            Ok(reader)
        }
    }

    fn open_image(&mut self) -> Result<(), JImageError> {
        let mut file = OpenOptions::new()
            .read(true)
            .write(false)
            .open(self.name.clone())
            .map_err(|err| JImageError::ImageFileOpenError)?;
        self.file_size = file.metadata().unwrap().len();
        let mut buf: Vec<u8> = Vec::new();
        assert!(
            file.read_to_end(&mut buf).unwrap() == self.file_size as usize,
            "fail to read data from file.",
        );
        let header_size = self.header_size();
        if self.file_size < header_size as u64 {
            return Err(JImageError::ImageFileOpenError);
        }
        self.header = ImageHeader::read_from_bytes(buf[0..header_size].try_into().unwrap());
        self.index_size = self.index_size();
        if self.file_size < self.index_size as u64
            || self.header.magic() != 0xCAFEDADA
            || self.header.major_version() != 1u32
            || self.header.minor_version() != 0u32
        {
            return Err(JImageError::ImageFileOpenError);
        }
        let length = self.table_length() as usize;
        let redirect_table_offset = self.header_size();
        let offsets_table_offset = redirect_table_offset + length * mem::size_of::<u32>();
        let location_bytes_offset = offsets_table_offset + length * mem::size_of::<u32>();
        let string_bytes_offset = location_bytes_offset + self.locations_size() as usize;
        let resource_bytes_offset = string_bytes_offset + self.string_size() as usize;
        self.redirect_table = Some(Arc::new(
            buf[redirect_table_offset..offsets_table_offset]
                .chunks_exact(4)
                .into_iter()
                .map(|bytes| i32::from_ne_bytes(bytes.try_into().unwrap()))
                .collect(),
        ));
        self.offsets_table = Some(Arc::new(
            buf[offsets_table_offset..location_bytes_offset]
                .chunks_exact(4)
                .into_iter()
                .map(|bytes| u32::from_ne_bytes(bytes.try_into().unwrap()))
                .collect(),
        ));
        self.location_bytes = Some(Arc::new(
            buf[location_bytes_offset..string_bytes_offset].to_vec(),
        ));
        self.string_bytes = Some(Arc::new(
            buf[string_bytes_offset..resource_bytes_offset].to_vec(),
        ));
        self.resource_bytes = Some(Arc::new(buf[resource_bytes_offset..].to_vec()));
        Ok(())
    }

    fn package_to_module(&self, package_name: String) -> Option<String> {
        let mut path = "/packages/".to_string();
        path.push_str(&package_name.replace("/", "."));

        if let Some(location) = ImageFileReader::find_location(&self, path) {
            let size = location.get_attribute(ImageLocation::ATTRIBUTE_UNCOMPRESSED);
            let mut content: Vec<u8> = vec![0; size as usize];
        }
        None
    }

    fn find_location(&self, path: String) -> Option<ImageLocation> {
        let redirect_ref = Arc::clone(self.redirect_table.as_ref().unwrap());
        let offsets_ref = Arc::clone(self.offsets_table.as_ref().unwrap());
        let location_ref = Arc::clone(self.location_bytes.as_ref().unwrap());
        if let Some(index) = ImageStrings::find(path, redirect_ref, self.table_length()) {
            assert!(
                index < offsets_ref.len() as i32,
                "index exceeds offset count!"
            );
            let location_offset = offsets_ref[index as usize];
            let mut attributes: [u64; 8] = [0; ImageLocation::ATTRIBUTE_COUNT as usize];
            unsafe {
                let mut _data = location_ref.as_ptr().add(location_offset as usize);
                while !_data.is_null() && *_data != 0 {
                    let byte = _data.read();
                    let kind = byte >> 3;
                    assert!(
                        kind < ImageLocation::ATTRIBUTE_COUNT,
                        "invalid attribute kind"
                    );
                    let len = (byte & 0x7) + 1;
                    assert!(len > 0 && len < 8, "invalid attribute value length");
                    let mut value: u64 = 0;
                    for i in 1..=len {
                        value <<= 8;
                        value |= _data.add(i as usize).read() as u64;
                    }
                    attributes[kind as usize] = value;
                    _data = _data.add((len + 1) as usize);
                }
            }
            return Some(ImageLocation::new(attributes));
        }
        None
    }

    fn get_resource(&self, location: &ImageLocation) -> Option<Vec<u8>> {
        let offset = location.get_attribute(ImageLocation::ATTRIBUTE_OFFSET);
        let uncompressed_size = location.get_attribute(ImageLocation::ATTRIBUTE_UNCOMPRESSED);
        let compressed_size = location.get_attribute(ImageLocation::ATTRIBUTE_COMPRESSED);
        assert!(self.resource_bytes.is_some(), "no resource data loaded.");
        let resource_bytes = Arc::clone(self.resource_bytes.as_ref().unwrap());
        match compressed_size {
            0 => {
                let end_offset = offset + compressed_size;
                let compressed_data = resource_bytes[offset as usize..end_offset as usize].to_vec();
                let image_strings = ImageStrings::new(
                    Arc::clone(self.string_bytes.as_ref().unwrap()),
                    self.header.string_size(),
                );
                Decompressors::decompress_resource(
                    &compressed_data,
                    uncompressed_size,
                    &image_strings,
                )
            }
            _ => {
                let end_offset = uncompressed_size + offset;
                Some(resource_bytes[offset as usize..end_offset as usize].to_vec())
            }
        }
    }

    fn index_size(&self) -> usize {
        self.table_length() as usize * mem::size_of::<u32>() * 2
            + self.locations_size() as usize
            + self.string_size() as usize
    }

    fn table_length(&self) -> u32 {
        self.header.table_length()
    }

    fn locations_size(&self) -> u32 {
        self.header.locations_size()
    }

    fn string_size(&self) -> u32 {
        self.header.string_size()
    }

    fn header_size(&self) -> usize {
        std::mem::size_of::<ImageHeader>()
    }
}

pub struct ImageLocation {
    attributes: [u64; ImageLocation::ATTRIBUTE_COUNT as usize],
}

impl ImageLocation {
    const ATTRIBUTE_END: u8 = 0;
    const ATTRIBUTE_MODULE: u8 = 1;
    const ATTRIBUTE_PARENT: u8 = 2;
    const ATTRIBUTE_BASE: u8 = 3;
    const ATTRIBUTE_EXTENSION: u8 = 4;
    const ATTRIBUTE_OFFSET: u8 = 5;
    const ATTRIBUTE_COMPRESSED: u8 = 6;
    const ATTRIBUTE_UNCOMPRESSED: u8 = 7;
    const ATTRIBUTE_COUNT: u8 = 8;

    fn new(attributes: [u64; ImageLocation::ATTRIBUTE_COUNT as usize]) -> Self {
        ImageLocation { attributes }
    }

    fn get_attribute(&self, kind: u8) -> u64 {
        assert!(
            ImageLocation::ATTRIBUTE_END < kind && kind < ImageLocation::ATTRIBUTE_COUNT,
            "invalid attribute kind"
        );
        self.attributes[kind as usize]
    }
}

#[derive(Default, Debug)]
#[repr(C)]
pub struct ImageHeader {
    magic: u32,
    version: u32,
    flags: u32,
    resource_count: u32,
    table_length: u32,
    locations_size: u32,
    string_size: u32,
}

impl ImageHeader {
    fn read_from_bytes(bytes: [u8; std::mem::size_of::<ImageHeader>()]) -> Self {
        unsafe {
            std::mem::transmute::<[u8; std::mem::size_of::<ImageHeader>()], ImageHeader>(bytes)
        }
    }

    fn magic(&self) -> u32 {
        self.magic
    }

    fn major_version(&self) -> u32 {
        self.version >> 16
    }

    fn minor_version(&self) -> u32 {
        self.version & 0xFFFFu32
    }

    fn flags(&self) -> u32 {
        self.flags
    }

    fn table_length(&self) -> u32 {
        self.table_length
    }

    fn locations_size(&self) -> u32 {
        self.locations_size
    }

    fn string_size(&self) -> u32 {
        self.string_size
    }
}

pub struct ImageStrings {
    data: Option<Arc<Vec<u8>>>,
    size: u32,
}

impl ImageStrings {
    const HASH_MULTIPLIER: u32 = 0x01000193u32;

    fn new(data: Arc<Vec<u8>>, size: u32) -> Self {
        ImageStrings {
            data: Some(data),
            size,
        }
    }

    fn find(name: String, redirect_table: Arc<Vec<i32>>, length: u32) -> Option<i32> {
        let mut hash_code = ImageStrings::hash_code(name.clone(), ImageStrings::HASH_MULTIPLIER);
        let index = hash_code % length;
        if let Some(&value) = redirect_table.get(index as usize) {
            if value > 0 {
                hash_code = ImageStrings::hash_code(name, value as u32);
                return Some((hash_code % length) as i32);
            } else if value < 0 {
                return Some(-1 - value);
            }
        }
        None
    }

    fn hash_code(name: String, seed: u32) -> u32 {
        let bytes = name.bytes();
        let mut value = seed;
        for byte in bytes {
            value = (value * ImageStrings::HASH_MULTIPLIER) ^ byte as u32;
        }

        value & 0x7FFF_FFFF
    }

    pub fn get(&self, decompressor_name_offset: u32) -> Option<String> {
        assert!(
            decompressor_name_offset < self.size,
            "offset exceeds string table size"
        );
        if let Some(data) = self.data.as_ref() {
            let mut bytes: Vec<u8> = Vec::new();
            for byte in data.iter().skip(decompressor_name_offset as usize) {
                if *byte == 0 {
                    break;
                }
                bytes.push(byte.clone());
            }
            return Some(String::from_utf8(bytes).unwrap());
        }
        None
    }
}

#[cfg(test)]
mod image_file {
    use std::env;

    use super::ImageFileReader;

    #[test]
    fn should_open_image_file() {
        if let Ok(java_home) = env::var("JAVA_HOME") {
            let name = match cfg!(windows) {
                true => format!("{}\\lib\\modules", java_home),
                false => format!("{}/lib/modules", java_home),
            };
            let result = ImageFileReader::open(name);
            assert!(result.is_ok());
        }
    }
}
