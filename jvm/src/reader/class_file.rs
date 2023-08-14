use super::{
    attribute::Attribute, class_file_version::ClassFileVersion, constant_info::ConstantInfo,
    field_info::FieldInfo, method_info::MethodInfo, type_conversion::ToUsizeSafe,
};

#[derive(Debug, Default)]
pub struct ClassFile {
    pub major_version: ClassFileVersion,
    pub constant_pool_count: u16,
    pub constant_pool: Vec<ConstantInfo>,
    pub access_flags: u16,
    pub this_class: u16,
    pub super_class: u16,
    pub interfaces_count: u16,
    pub interfaces: Vec<u16>,
    pub fields_count: u16,
    pub fields: Vec<FieldInfo>,
    pub methods_count: u16,
    pub methods: Vec<MethodInfo>,
    pub attributes_count: u16,
    pub attributes: Vec<Attribute>,
}

impl ClassFile {
    pub fn get_constant_info(&self, index: u16) -> &ConstantInfo {
        match self.constant_pool.get(index.into_usize_safe() - 1) {
            Some(constant_info) => constant_info,
            None => panic!("Invalid index for constant pool, out of bounds."),
        }
    }
}
