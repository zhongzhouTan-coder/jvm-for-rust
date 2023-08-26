use crate::reader::buffer::Buffer;

use super::{attribute_error::AttributeError, attribute_trait::AttributeTrait};

#[derive(Debug, PartialEq, Eq)]
pub struct ConstantValueAttribute {
    constant_value_index: u16,
}

impl AttributeTrait for ConstantValueAttribute {
    fn decode_attribute(buffer: &mut Buffer) -> Result<Self, AttributeError>
    where
        Self: Sized,
    {
        match buffer.read_u16() {
            Ok(index) => Ok(ConstantValueAttribute {
                constant_value_index: index,
            }),
            Err(_) => Err(AttributeError::DecodeAttributeError),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::reader::{attributes::attribute_trait::AttributeTrait, buffer::Buffer};

    use super::ConstantValueAttribute;

    #[test]
    fn decode_bytes_to_constant_value() {
        let bytes = [0x01u8, 0x01u8];
        let mut buffer = Buffer::new(&bytes);
        let result = ConstantValueAttribute::decode_attribute(&mut buffer);
        let expect = ConstantValueAttribute {
            constant_value_index: 0x0101u16,
        };
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(expect));
    }
}
