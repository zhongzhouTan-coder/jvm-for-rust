use std::sync::Arc;

use super::{
    attribute::Attribute, constant_pool::ConstantPool, field_info::FieldInfo,
    instance_klass::InstanceKlass, method_info::MethodInfo, symbol::Symbol,
};

#[derive(Debug, Default)]
pub struct ClassFile {
    pub major_version: u16,
    pub minor_version: u16,
    pub constant_pool: ConstantPool,
    pub access_flags: u16,
    pub class_name: Arc<Symbol>,
    pub this_class_index: u16,
    pub super_class_index: u16,
    pub super_class: Option<InstanceKlass>,
    pub interfaces: Vec<InstanceKlass>,
    pub fields_count: u16,
    pub fields: Vec<FieldInfo>,
    pub methods_count: u16,
    pub methods: Vec<MethodInfo>,
    pub attributes_count: u16,
    pub attributes: Vec<Attribute>,
}
