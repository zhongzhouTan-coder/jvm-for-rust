use crate::reader::buffer::Buffer;

use super::{attribute_error::AttributeError, attribute_trait::AttributeTrait};

pub struct SignatureAttribute {
    signature_index: u16,
}

impl AttributeTrait for SignatureAttribute {
    fn decode_attribute(buffer: &mut Buffer) -> Result<Self, AttributeError>
    where
        Self: Sized,
    {
        let signature_index = buffer.read_u16()?;
        Ok(SignatureAttribute { signature_index })
    }
}
