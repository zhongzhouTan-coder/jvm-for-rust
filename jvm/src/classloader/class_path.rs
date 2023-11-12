use std::{cell::RefCell, fs::File, io::BufReader};

use super::class_file_stream::ClassFileStream;

pub struct ClassPathDirEntry {
    dir: String,
}

pub struct ClassPathZipEntry {
    zip_name: String,
    zip: RefCell<BufReader<File>>,
}

pub struct ClassPathImageEntry {
    name: String,
    jimage: String,
}

pub trait ClassPathEntry {
    fn open_stream(name: String) -> ClassFileStream;
    fn is_jar_file() -> bool;
}

impl ClassPathDirEntry {
    fn new(dir: String) -> Self {
        ClassPathDirEntry { dir }
    }
}

impl ClassPathEntry for ClassPathDirEntry {
    fn open_stream(name: String) -> ClassFileStream {
        todo!()
    }

    fn is_jar_file() -> bool {
        false
    }
}

impl ClassPathEntry for ClassPathZipEntry {
    fn open_stream(name: String) -> ClassFileStream {
        todo!()
    }

    fn is_jar_file() -> bool {
        true
    }
}

impl ClassPathZipEntry {
    fn new(zip_name: String, zip: BufReader<File>) -> Self {
        ClassPathZipEntry {
            zip_name,
            zip: RefCell::new(zip),
        }
    }
}
