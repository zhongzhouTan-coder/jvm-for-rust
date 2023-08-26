use crate::reader::buffer::Buffer;

use super::{attribute_error::AttributeError, attribute_trait::AttributeTrait};

pub struct SourceFileAttribute {
    sourcefile_index: u16,
}

impl AttributeTrait for SourceFileAttribute {
    fn decode_attribute(buffer: &mut Buffer) -> Result<Self, AttributeError>
    where
        Self: Sized,
    {
        let sourcefile_index = buffer.read_u16()?;
        Ok(SourceFileAttribute { sourcefile_index })
    }
}
