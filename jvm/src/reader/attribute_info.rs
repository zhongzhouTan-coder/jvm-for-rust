#[derive(Debug)]
pub struct AttributeInfo {
    attribute_name_index: u16,
    attribute_length: u32,
    info: Vec<u8>,
}

impl AttributeInfo {
    pub fn new(attribute_name_index: u16, attribute_length: u32, info: Vec<u8>) -> AttributeInfo {
        AttributeInfo {
            attribute_name_index,
            attribute_length,
            info,
        }
    }
}
