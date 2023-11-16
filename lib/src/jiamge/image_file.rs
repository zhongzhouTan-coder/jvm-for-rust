use super::{image_decompressor::Decompressors, jimage_error::JImageError};
use once_cell::sync::OnceCell;

use std::io::Read;
use std::{
    collections::HashMap,
    fs::OpenOptions,
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
    attribute_offsets: Option<Arc<Vec<u32>>>,
    attribute_data: Option<Arc<Vec<u8>>>,
    strings: Option<Arc<Vec<u8>>>,
    resources: Option<Arc<Vec<u8>>>,
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
            attribute_offsets: None,
            attribute_data: None,
            strings: None,
            resources: None,
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
            .map_err(|_| JImageError::FileOpenError)?;
        self.file_size = file.metadata().unwrap().len();
        let mut buf: Vec<u8> = Vec::new();
        assert_eq!(
            file.read_to_end(&mut buf).unwrap(),
            self.file_size as usize,
            "fail to read data from file.",
        );
        let header_size = self.header_size();
        if self.file_size < header_size as u64 {
            return Err(JImageError::FileOpenError);
        }
        self.header = ImageHeader::read_from_bytes(&buf[0..header_size]);
        self.index_size = self.index_size();
        if self.file_size < self.index_size as u64
            || self.header.magic() != 0xCAFEDADA
            || self.header.major_version() != 1u32
            || self.header.minor_version() != 0u32
        {
            return Err(JImageError::FileOpenError);
        }
        let tlen = self.table_length() as usize;
        let mut index = self.header_size();
        self.redirect_table = Some(Arc::new(
            buf[index..index + tlen * mem::size_of::<u32>()]
                .chunks_exact(4)
                .into_iter()
                .map(|bytes| i32::from_ne_bytes(bytes.try_into().unwrap()))
                .collect(),
        ));
        index += tlen * mem::size_of::<u32>();
        self.attribute_offsets = Some(Arc::new(
            buf[index..index + tlen * mem::size_of::<u32>()]
                .chunks_exact(mem::size_of::<u32>())
                .into_iter()
                .map(|bytes| u32::from_ne_bytes(bytes.try_into().unwrap()))
                .collect(),
        ));
        index += tlen * mem::size_of::<u32>();
        self.attribute_data = Some(Arc::new(buf[index..index + self.locations_size()].to_vec()));
        index += self.locations_size();
        self.strings = Some(Arc::new(buf[index..index + self.string_size()].to_vec()));
        index += self.string_size();
        self.resources = Some(Arc::new(buf[index..].to_vec()));
        Ok(())
    }

    fn package_to_module(&self, package_name: String) -> Option<String> {
        let mut path = "/packages/".to_string();
        path.push_str(&package_name.replace("/", "."));

        if let Some(location) = ImageFileReader::find_location(&self, path) {
            if let Some(content) = self.get_resource(&location) {
                let mut offset: u32 = 0;
                let mut iter = content.chunks_exact(8);
                while let Some(chunk) = iter.next() {
                    let is_empty: u32 = u32::from_ne_bytes(chunk[0..4].try_into().unwrap());
                    if is_empty != 0 {
                        offset = u32::from_ne_bytes(chunk[4..8].try_into().unwrap());
                        break;
                    }
                }
                return self.get_strings().get(offset);
            }
        }
        None
    }

    fn find_location(&self, path: String) -> Option<ImageLocation> {
        let redirect_ref = Arc::clone(self.redirect_table.as_ref().unwrap());
        let offsets_ref = Arc::clone(self.attribute_offsets.as_ref().unwrap());
        let location_ref = Arc::clone(self.attribute_data.as_ref().unwrap());
        if let Some(index) =
            ImageStrings::find(path.clone(), redirect_ref, self.table_length() as u32)
        {
            assert!(
                (index as usize) < offsets_ref.len(),
                "invalid offsets index."
            );
            let location_offset = offsets_ref[index as usize];
            assert!(
                (location_offset as usize) < location_ref.len(),
                "invalid location offset."
            );
            let mut attributes: [u64; 8] = [0; ImageLocation::ATTRIBUTE_COUNT as usize];
            let mut index: usize = location_offset as usize;
            while let Some(&byte) = location_ref.get(index) {
                if byte == ImageLocation::ATTRIBUTE_END {
                    break;
                }
                let kind = ImageLocation::attribute_kind(byte);
                assert!(
                    kind < ImageLocation::ATTRIBUTE_COUNT,
                    "invalid attribute kind"
                );
                let len = ImageLocation::attribute_length(byte);
                index += 1;
                if let Some(values) = location_ref.get(index..index + len as usize) {
                    attributes[kind as usize] = ImageLocation::attribute_value(values, len);
                    index += len as usize;
                } else {
                    println!("invalid attribute value bytes.");
                    return None;
                }
            }
            let location = ImageLocation::new(attributes);
            if self.verify_location(&location, path) {
                return Some(location);
            }
        }
        None
    }

    fn get_resource(&self, location: &ImageLocation) -> Option<Vec<u8>> {
        let start = location.get_attribute(ImageLocation::ATTRIBUTE_OFFSET) as usize;
        let uncompressed_size =
            location.get_attribute(ImageLocation::ATTRIBUTE_UNCOMPRESSED) as usize;
        let compressed_size = location.get_attribute(ImageLocation::ATTRIBUTE_COMPRESSED) as usize;
        assert!(self.resources.is_some(), "no resource data loaded.");
        let resource_bytes = Arc::clone(self.resources.as_ref().unwrap());
        let end = if compressed_size == 0 {
            uncompressed_size + start
        } else {
            compressed_size + start
        };
        assert!(
            end <= resource_bytes.len(),
            "invalid index access to resource bytes."
        );
        let _data = resource_bytes[start - 64..end + 64].to_vec();
        let data = resource_bytes[start..end].to_vec();
        if compressed_size != 0 {
            return Decompressors::decompress_resource(
                data,
                uncompressed_size as u64,
                self.get_strings(),
            );
        }
        Some(data)
    }

    fn verify_location(&self, location: &ImageLocation, path: String) -> bool {
        let mut path = path;
        let strings = self.get_strings();
        let module = strings
            .get(location.get_attribute(ImageLocation::ATTRIBUTE_MODULE) as u32)
            .expect("expect module string.");
        if !module.is_empty() {
            let prefix = &format!("/{}/", module);
            if let Some(result) = path.strip_prefix(prefix) {
                path = result.to_owned();
            } else {
                return false;
            }
        }
        let parent = strings
            .get(location.get_attribute(ImageLocation::ATTRIBUTE_PARENT) as u32)
            .expect("expect parent string.");
        if !parent.is_empty() {
            let prefix = &format!("{}/", parent);
            if let Some(result) = path.strip_prefix(prefix) {
                path = result.to_owned();
            } else {
                return false;
            }
        }
        let base = strings
            .get(location.get_attribute(ImageLocation::ATTRIBUTE_BASE) as u32)
            .expect("expect base name string.");
        if !base.is_empty() {
            if let Some(result) = path.strip_prefix(&base) {
                path = result.to_owned();
            } else {
                return false;
            }
        }

        let extension = strings
            .get(location.get_attribute(ImageLocation::ATTRIBUTE_EXTENSION) as u32)
            .expect("expect extension string.");
        if !extension.is_empty() {
            let prefix = &format!(".{}", extension);
            if let Some(result) = path.strip_prefix(prefix) {
                path = result.to_owned();
            } else {
                return false;
            }
        }
        path.is_empty()
    }

    fn index_size(&self) -> usize {
        self.table_length() as usize * mem::size_of::<u32>() * 2
            + self.locations_size() as usize
            + self.string_size() as usize
    }

    fn table_length(&self) -> usize {
        self.header.table_length() as usize
    }

    fn locations_size(&self) -> usize {
        self.header.locations_size() as usize
    }

    fn string_size(&self) -> usize {
        self.header.string_size() as usize
    }

    fn header_size(&self) -> usize {
        std::mem::size_of::<ImageHeader>()
    }

    fn get_strings(&self) -> ImageStrings {
        assert!(self.strings.is_some(), "No content of string bytes.");
        ImageStrings {
            data: Some(Arc::clone(self.strings.as_ref().unwrap())),
            size: self.header.string_size(),
        }
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

    #[inline]
    fn attribute_length(data: u8) -> u8 {
        (data & 0x7) + 1
    }

    #[inline]
    fn attribute_kind(data: u8) -> u8 {
        let kind: u8 = data >> 3;
        assert!(
            kind < ImageLocation::ATTRIBUTE_COUNT,
            "invalid attribute kind."
        );
        kind
    }

    #[inline]
    fn attribute_value(data: &[u8], n: u8) -> u64 {
        assert!(
            0 < n && n <= 8 && data.len() >= n as usize,
            "invalid attribute value length."
        );
        let mut value: u64 = 0;
        for i in 0..n {
            value <<= 8;
            value |= unsafe { *data.get_unchecked(i as usize) as u64 };
        }
        value
    }
}

#[derive(Default, Debug)]
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
    fn read_from_bytes(bytes: &[u8]) -> Self {
        ImageHeader {
            magic: u32::from_ne_bytes(bytes[0..4].try_into().unwrap()),
            version: u32::from_ne_bytes(bytes[4..8].try_into().unwrap()),
            flags: u32::from_ne_bytes(bytes[8..12].try_into().unwrap()),
            resource_count: u32::from_ne_bytes(bytes[12..16].try_into().unwrap()),
            table_length: u32::from_ne_bytes(bytes[16..20].try_into().unwrap()),
            locations_size: u32::from_ne_bytes(bytes[20..24].try_into().unwrap()),
            string_size: u32::from_ne_bytes(bytes[24..28].try_into().unwrap()),
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

    fn find(name: String, redirect_table: Arc<Vec<i32>>, length: u32) -> Option<u32> {
        let mut hash_code = ImageStrings::hash_code(name.clone(), ImageStrings::HASH_MULTIPLIER);
        let index = hash_code % length;
        if let Some(&value) = redirect_table.get(index as usize) {
            if value > 0 {
                hash_code = ImageStrings::hash_code(name, value as u32);
                return Some(hash_code % length);
            } else if value < 0 {
                return Some((-1 - value) as u32);
            }
        }
        None
    }

    fn hash_code(name: String, seed: u32) -> u32 {
        let bytes = name.bytes();
        let mut value = seed as u32;
        for byte in bytes {
            value = value.wrapping_mul(ImageStrings::HASH_MULTIPLIER) ^ (byte as u32);
        }
        value & 0x7FFF_FFFFu32
    }

    pub fn get(&self, offset: u32) -> Option<String> {
        assert!(offset < self.size, "offset exceeds string table size");
        if let Some(data) = self.data.as_ref() {
            let mut bytes: Vec<u8> = Vec::new();
            for byte in data.iter().skip(offset as usize) {
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
            if let Ok(reader) = ImageFileReader::open(name) {
                assert_eq!(
                    reader.package_to_module("java/lang".to_string()),
                    Some("java.base".to_string())
                );
            }
        }
    }
}
