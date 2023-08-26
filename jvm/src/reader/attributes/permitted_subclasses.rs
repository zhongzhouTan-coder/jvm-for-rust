use crate::reader::buffer::Buffer;

use super::{attribute_error::AttributeError, attribute_trait::AttributeTrait};

pub struct PermittedSubclassesAttribute {
    number_of_classes: u16,
    classes: Vec<u16>,
}

impl AttributeTrait for PermittedSubclassesAttribute {
    fn decode_attribute(buffer: &mut Buffer) -> Result<Self, AttributeError>
    where
        Self: Sized,
    {
        let number_of_classes = buffer.read_u16()?;
        let classes = (0..number_of_classes)
            .map(|_| buffer.read_u16())
            .map(|result| result.map_err(|err| AttributeError::from(err)))
            .collect::<Result<Vec<u16>, AttributeError>>()?;
        Ok(PermittedSubclassesAttribute {
            number_of_classes,
            classes,
        })
    }
}
