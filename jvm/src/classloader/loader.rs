use std::fs;
use std::{collections::HashMap, path::PathBuf};

#[derive(Debug)]
pub struct ClassLoader {
    classpath: PathBuf,
    loaded_classes: HashMap<String, Vec<u8>>,
    parent: Option<Box<ClassLoader>>,
}

impl ClassLoader {
    pub fn new(classpath: PathBuf, parent: Option<Box<ClassLoader>>) -> Self {
        ClassLoader {
            classpath,
            loaded_classes: HashMap::new(),
            parent,
        }
    }

    pub fn load_class(&mut self, classname: &str) -> Option<Vec<u8>> {
        if let Some(bytes) = self.loaded_classes.get(classname) {
            return Some(bytes.clone());
        }

        if let Some(parent) = &mut self.parent {
            if let Some(bytes) = parent.load_class(classname) {
                return Some(bytes);
            }
        }

        let class_file_path = self.get_class_file_path(classname);

        match fs::read(&class_file_path) {
            Ok(bytes) => {
                self.loaded_classes
                    .insert(classname.to_string(), bytes.clone());
                Some(bytes)
            }
            Err(_) => None,
        }
    }

    fn get_class_file_path(&self, classname: &str) -> PathBuf {
        let class_file_name = format!("{}.class", classname.replace(".", "/"));
        let class_file_path = self.classpath.join(&class_file_name);
        if class_file_path.exists() {
            return class_file_path;
        }
        PathBuf::new()
    }
}
