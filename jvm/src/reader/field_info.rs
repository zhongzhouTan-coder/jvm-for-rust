use crate::reader::attribute_info::AttributeInfo;

use super::field_access_flag::FieldAccessFlag;

#[derive(Debug)]
pub struct FieldInfo {
    access_flag: u16,
    name: String,
    descriptor: String,
    attributes_count: u16,
    attributes: Vec<AttributeInfo>,
}

impl FieldInfo {
    pub fn new(
        access_flag: u16,
        name: String,
        descriptor: String,
        attributes_count: u16,
        attributes: Vec<AttributeInfo>,
    ) -> FieldInfo {
        FieldInfo {
            access_flag,
            name,
            descriptor,
            attributes_count,
            attributes,
        }
    }
}
