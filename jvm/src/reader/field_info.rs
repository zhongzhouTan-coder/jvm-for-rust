use crate::reader::attribute::Attribute;

#[derive(Debug)]
pub struct FieldInfo {
    access_flags: u16,
    name: String,
    descriptor: String,
    attributes: Vec<Attribute>,
}

impl FieldInfo {
    pub fn new(
        access_flags: u16,
        name: String,
        descriptor: String,
        attributes: Vec<Attribute>,
    ) -> FieldInfo {
        FieldInfo {
            access_flags,
            name,
            descriptor,
            attributes,
        }
    }
}
