use super::attribute_info::AttributeInfo;

#[derive(Debug)]
pub struct MethodInfo {
    access_flags: u16,
    name_index: u16,
    descriptor_index: u16,
    attributes_count: u16,
    attributes: Vec<AttributeInfo>,
}

impl MethodInfo {
    pub fn new(
        access_flags: u16,
        name_index: u16,
        descriptor_index: u16,
        attributes_count: u16,
        attributes: Vec<AttributeInfo>,
    ) -> MethodInfo {
        MethodInfo {
            access_flags,
            name_index,
            descriptor_index,
            attributes_count,
            attributes,
        }
    }
}
