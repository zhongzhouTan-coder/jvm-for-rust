use crate::reader::buffer::Buffer;

use super::attribute_error::AttributeError;

pub trait AttributeTrait {
    fn decode_attribute(buffer: &mut Buffer) -> Result<Self, AttributeError>
    where
        Self: Sized;
}
