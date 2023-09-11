use std::sync::Arc;

use super::{class_access_flag::ClassAccessFlag, symbol::Symbol};

#[derive(Debug, Clone, Default)]
pub struct Klass {
    id: ClassId,
    name: Arc<Symbol>,
    source_file: String,
    super_klass: Option<Box<Klass>>,
    sub_klass: Option<Box<Klass>>,
    access_flags: ClassAccessFlag,
}

#[derive(Debug, Clone, Default)]
struct ClassId(u32);
