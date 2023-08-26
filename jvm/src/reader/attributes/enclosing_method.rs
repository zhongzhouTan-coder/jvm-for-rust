use crate::reader::buffer::Buffer;

use super::{attribute_error::AttributeError, attribute_trait::AttributeTrait};

pub struct EnclosingMethodAttribute {
    class_index: u16,
    method_index: u16,
}

impl AttributeTrait for EnclosingMethodAttribute {
    fn decode_attribute(buffer: &mut Buffer) -> Result<Self, AttributeError>
    where
        Self: Sized,
    {
        let class_index = buffer.read_u16()?;
        let method_index = buffer.read_u16()?;
        Ok(EnclosingMethodAttribute {
            class_index,
            method_index,
        })
    }
}
