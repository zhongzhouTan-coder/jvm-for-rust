use crate::reader::buffer::Buffer;

use super::{attribute_error::AttributeError, attribute_trait::AttributeTrait};

pub struct LocalVariableTypeTableAttribute {
    local_variable_type_table_length: u16,
    local_variable_type_table: Vec<LocalVariableTypeTable>,
}

impl AttributeTrait for LocalVariableTypeTableAttribute {
    fn decode_attribute(buffer: &mut Buffer) -> Result<Self, AttributeError>
    where
        Self: Sized,
    {
        let local_variable_type_table_length = buffer.read_u16()?;
        let local_variable_type_table = (0..local_variable_type_table_length)
            .map(|_| LocalVariableTypeTable::decode_attribute(buffer))
            .collect::<Result<Vec<LocalVariableTypeTable>, AttributeError>>()?;
        Ok(LocalVariableTypeTableAttribute {
            local_variable_type_table_length,
            local_variable_type_table,
        })
    }
}

pub struct LocalVariableTypeTable {
    start_pc: u16,
    length: u16,
    name_index: u16,
    signature_index: u16,
    index: u16,
}

impl AttributeTrait for LocalVariableTypeTable {
    fn decode_attribute(buffer: &mut Buffer) -> Result<Self, AttributeError>
    where
        Self: Sized,
    {
        let start_pc = buffer.read_u16()?;
        let length = buffer.read_u16()?;
        let name_index = buffer.read_u16()?;
        let signature_index = buffer.read_u16()?;
        let index = buffer.read_u16()?;
        Ok(LocalVariableTypeTable {
            start_pc,
            length,
            name_index,
            signature_index,
            index,
        })
    }
}
