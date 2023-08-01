use std::{collections::HashMap, path::PathBuf};

pub struct ClassLoader {
    classpath: Vec<PathBuf>,
    loaded_classes: HashMap<String, Vec<u8>>,
    parent: Option<Box<ClassLoader>>,
}
