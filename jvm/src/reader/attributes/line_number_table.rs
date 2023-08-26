use crate::reader::buffer::Buffer;

use super::{attribute_error::AttributeError, attribute_trait::AttributeTrait};

pub struct LineNumberTableAttribute {
    line_number_table_length: u16,
    line_number_table: Vec<LineNumberTable>,
}

impl AttributeTrait for LineNumberTableAttribute {
    fn decode_attribute(buffer: &mut Buffer) -> Result<Self, AttributeError>
    where
        Self: Sized,
    {
        let line_number_table_length = buffer.read_u16()?;
        let line_number_table = (0..line_number_table_length)
            .map(|_| LineNumberTable::decode_attribute(buffer))
            .collect::<Result<Vec<LineNumberTable>, AttributeError>>()?;
        Ok(LineNumberTableAttribute {
            line_number_table_length,
            line_number_table,
        })
    }
}

pub struct LineNumberTable {
    start_pc: u16,
    line_number: u16,
}

impl AttributeTrait for LineNumberTable {
    fn decode_attribute(buffer: &mut Buffer) -> Result<Self, AttributeError>
    where
        Self: Sized,
    {
        let start_pc = buffer.read_u16()?;
        let line_number = buffer.read_u16()?;
        Ok(LineNumberTable {
            start_pc,
            line_number,
        })
    }
}
