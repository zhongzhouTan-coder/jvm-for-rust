use crate::reader::buffer::Buffer;

use super::{attribute_error::AttributeError, attribute_trait::AttributeTrait};

pub struct ExceptionsAttribute {
    number_of_exceptions: u16,
    exception_index_table: Vec<u16>,
}

impl AttributeTrait for ExceptionsAttribute {
    fn decode_attribute(buffer: &mut Buffer) -> Result<Self, AttributeError>
    where
        Self: Sized,
    {
        let number_of_exceptions = buffer.read_u16()?;
        let exception_index_table = (0..number_of_exceptions)
            .map(|_| buffer.read_u16())
            .map(|result| result.map_err(|err| AttributeError::from(err)))
            .collect::<Result<Vec<u16>, AttributeError>>()?;
        Ok(ExceptionsAttribute {
            number_of_exceptions,
            exception_index_table,
        })
    }
}
