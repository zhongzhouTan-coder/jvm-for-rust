mod image_decompressor;
mod image_file;
mod jimage_error;
mod zip_utils;
use std::sync::Arc;

use self::{
    image_file::{ImageFileReader, ImageLocation},
    jimage_error::JImageError,
};

pub type JImageFile = ImageFileReader;
pub type JImageLocationRef = u32;
pub type ResourceSize = u64;

const JIMAGE_MAX_PATH: usize = 4096;

pub fn JIMAGE_Open(name: String) -> Result<Arc<JImageFile>, JImageError> {
    ImageFileReader::open(name)
}

pub fn JIMAGE_Close(name: &str) {
    ImageFileReader::close(name)
}

pub fn JIMAGE_PackageToModule(jimage: Arc<JImageFile>, package_name: String) -> Option<String> {
    jimage.package_to_module(package_name)
}

pub fn JIMAGE_FindResource(
    jimage: Arc<JImageFile>,
    module_name: String,
    version: String,
    name: String,
) -> Option<(JImageLocationRef, ResourceSize)> {
    let full_path = format!("/{}/{}", module_name, name);
    assert!(!name.is_empty(), "name should not be empty.");
    assert!(
        full_path.len() <= JIMAGE_MAX_PATH,
        "full path should not exceed max path."
    );
    jimage.find_location_index(full_path)
}

pub fn JIMAGE_GetResource(jimage: Arc<JImageFile>, location: JImageLocationRef) -> Option<Vec<u8>> {
    jimage.get_resource(location)
}

type JImageResourceVisitor = fn(
    jimage: Arc<JImageFile>,
    module_name: String,
    version: String,
    package: String,
    name: String,
    extension: String,
    arg: *const (),
) -> bool;

pub fn JIMAGE_ResourceIterator(
    jimage: Arc<JImageFile>,
    visitor: JImageResourceVisitor,
    arg: *const (),
) {
    let n_entries = jimage.table_length();
    let strings = jimage.get_strings();
    for i in 0..n_entries {
        if let Some(location) = jimage.get_location(i as u32) {
            let module_offset = location.get_attribute(ImageLocation::ATTRIBUTE_MODULE) as u32;
            if module_offset == 0 {
                continue;
            }

            let module = strings.get(module_offset).unwrap();
            if module == "modules".to_owned() || module == "packages".to_owned() {
                continue;
            }

            let parent_offset = location.get_attribute(ImageLocation::ATTRIBUTE_PARENT) as u32;
            let parent = strings.get(parent_offset).unwrap();
            let base_offset = location.get_attribute(ImageLocation::ATTRIBUTE_BASE) as u32;
            let base = strings.get(base_offset).unwrap();
            let ext_offset = location.get_attribute(ImageLocation::ATTRIBUTE_EXTENSION) as u32;
            let extension = strings.get(ext_offset).unwrap();

            if visitor(
                Arc::clone(&jimage),
                module,
                "9".to_owned(),
                parent,
                base,
                extension,
                arg.clone(),
            ) {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::{JIMAGE_FindResource, JIMAGE_Open, JIMAGE_PackageToModule};

    #[test]
    fn we_can_open_an_image() {
        if let Ok(java_home) = std::env::var("JAVA_HOME") {
            let path = format!("{}/lib/modules", java_home);
            let result = JIMAGE_Open(path);
            assert!(result.is_ok(), "JImage failed to open.")
        }
    }

    #[test]
    fn we_can_get_module_of_packages() {
        if let Ok(java_home) = std::env::var("JAVA_HOME") {
            let path = format!("{}/lib/modules", java_home);
            let module = JIMAGE_Open(path)
                .ok()
                .and_then(|image| JIMAGE_PackageToModule(image, "sun/reflect".to_string()));
            assert_eq!(
                module,
                Some("jdk.unsupported".to_owned()),
                "fail to find module of packages."
            );
        }
    }

    #[test]
    fn we_can_find_a_resource() {
        if let Ok(java_home) = std::env::var("JAVA_HOME") {
            let path = format!("{}/lib/modules", java_home);
            let result = JIMAGE_Open(path).ok().and_then(|image| {
                JIMAGE_FindResource(
                    image,
                    "java.base".to_owned(),
                    "9.0".to_owned(),
                    "java/lang/String.class".to_owned(),
                )
            });
            assert!(result.is_some(), "fail to get resource offset");
        }
    }
}
