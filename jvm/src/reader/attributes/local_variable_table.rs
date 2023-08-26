use crate::reader::buffer::Buffer;

use super::{
    attribute_error::AttributeError, attribute_trait::AttributeTrait, local_variable_type_table,
};

pub struct LocalVariableTableAttribute {
    local_variable_table_length: u16,
    local_variable_table: Vec<LocalVariableTable>,
}

impl AttributeTrait for LocalVariableTableAttribute {
    fn decode_attribute(buffer: &mut Buffer) -> Result<Self, AttributeError>
    where
        Self: Sized,
    {
        let local_variable_table_length = buffer.read_u16()?;
        let local_variable_table = (0..local_variable_table_length)
            .map(|_| LocalVariableTable::decode_attribute(buffer))
            .collect::<Result<Vec<LocalVariableTable>, AttributeError>>()?;
        Ok(LocalVariableTableAttribute {
            local_variable_table_length,
            local_variable_table,
        })
    }
}

pub struct LocalVariableTable {
    start_pc: u16,
    length: u16,
    name_index: u16,
    descriptor_index: u16,
    index: u16,
}

impl AttributeTrait for LocalVariableTable {
    fn decode_attribute(buffer: &mut Buffer) -> Result<Self, AttributeError>
    where
        Self: Sized,
    {
        let start_pc = buffer.read_u16()?;
        let length = buffer.read_u16()?;
        let name_index = buffer.read_u16()?;
        let descriptor_index = buffer.read_u16()?;
        let index = buffer.read_u16()?;
        Ok(LocalVariableTable {
            start_pc,
            length,
            name_index,
            descriptor_index,
            index,
        })
    }
}
