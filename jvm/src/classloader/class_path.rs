use std::{cell::RefCell, fs::File, io::BufReader, sync::Arc};

use lib::jimage::JImageFile;

use super::class_file_stream::ClassFileStream;
pub trait ClassPathEntry {
    fn open_stream(&self, name: String) -> ClassFileStream;
    fn is_jar_file() -> bool {
        false
    }
    fn is_modules_image() -> bool {
        false
    }
    fn from_class_path_attr() -> bool {
        false
    }
    fn jimage() -> Option<Arc<JImageFile>> {
        None
    }
    fn name() -> String;
}
pub struct ClassPathDirEntry {
    dir: String,
}

impl ClassPathDirEntry {
    fn new(dir: String) -> Self {
        ClassPathDirEntry { dir }
    }
}

impl ClassPathEntry for ClassPathDirEntry {
    fn open_stream(&self, name: String) -> ClassFileStream {
        assert!(
            !self.dir.is_empty() && !name.is_empty(),
            "invalid dir and name."
        );
        ClassFileStream {}
    }

    fn name() -> String {
        todo!()
    }
}

pub struct ClassPathZipEntry {
    zip_name: String,
    zip: RefCell<BufReader<File>>,
}

impl ClassPathZipEntry {
    fn new(zip_name: String, zip: BufReader<File>) -> Self {
        ClassPathZipEntry {
            zip_name,
            zip: RefCell::new(zip),
        }
    }
}

impl ClassPathEntry for ClassPathZipEntry {
    fn open_stream(&self, name: String) -> ClassFileStream {
        todo!()
    }

    fn is_jar_file() -> bool {
        true
    }

    fn name() -> String {
        todo!()
    }
}

pub struct ClassPathImageEntry {
    name: String,
}

impl ClassPathImageEntry {
    fn new(name: String) -> Self {
        ClassPathImageEntry { name }
    }
}

impl ClassPathEntry for ClassPathImageEntry {
    fn open_stream(&self, name: String) -> ClassFileStream {
        todo!()
    }

    fn name() -> String {
        todo!()
    }
}
