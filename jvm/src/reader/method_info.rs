use super::attribute::Attribute;

#[derive(Debug)]
pub struct MethodInfo {
    pub access_flags: u16,
    pub name: String,
    pub descriptor: String,
    pub attributes_count: u16,
    pub attributes: Vec<Attribute>,
}

impl MethodInfo {
    pub fn new(
        access_flags: u16,
        name: String,
        descriptor: String,
        attributes_count: u16,
        attributes: Vec<Attribute>,
    ) -> MethodInfo {
        MethodInfo {
            access_flags,
            name,
            descriptor,
            attributes_count,
            attributes,
        }
    }
}
