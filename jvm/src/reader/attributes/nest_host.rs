use crate::reader;

use super::{attribute_error::AttributeError, attribute_trait::AttributeTrait};
pub struct NestHostAttribute {
    host_class_index: u16,
}

impl AttributeTrait for NestHostAttribute {
    fn decode_attribute(buffer: &mut reader::buffer::Buffer) -> Result<Self, AttributeError>
    where
        Self: Sized,
    {
        let host_class_index = buffer.read_u16()?;
        Ok(NestHostAttribute { host_class_index })
    }
}
