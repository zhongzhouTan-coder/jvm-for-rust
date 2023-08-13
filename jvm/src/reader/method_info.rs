use super::{attribute_info::AttributeInfo, method_access_flag::MethodAccessFlag};

#[derive(Debug)]
pub struct MethodInfo {
    access_flag: u16,
    name: String,
    descriptor: String,
    attributes_count: u16,
    attributes: Vec<AttributeInfo>,
}

impl MethodInfo {
    pub fn new(
        access_flag: u16,
        name: String,
        descriptor: String,
        attributes_count: u16,
        attributes: Vec<AttributeInfo>,
    ) -> MethodInfo {
        MethodInfo {
            access_flag,
            name,
            descriptor,
            attributes_count,
            attributes,
        }
    }
}
