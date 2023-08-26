use crate::reader::{attribute::Attribute, buffer::Buffer, type_conversion::ToUsizeSafe};

use super::{
    attribute_error::AttributeError, attribute_trait::AttributeTrait,
    exception_table::ExceptionTable,
};

pub struct CodeAttribute {
    max_stack: u16,
    max_locals: u16,
    code_length: u32,
    code: Vec<u8>,
    exception_table_length: u16,
    exception_table: ExceptionTable,
    attributes_count: u16,
    attributes: Vec<Attribute>,
}

impl AttributeTrait for CodeAttribute {
    fn decode_attribute(buffer: &mut Buffer) -> Result<Self, AttributeError>
    where
        Self: Sized,
    {
        let max_stack = buffer.read_u16()?;
        let max_locals = buffer.read_u16()?;
        let code_length = buffer.read_u32()?;
        let code = buffer.read_vec_u8(code_length.into_usize_safe())?;
        let exception_table_length = buffer.read_u16()?;
        let exception_table = ExceptionTable::decode_attribute(buffer)?;
        let attributes_count = buffer.read_u16()?;
        let attributes = (0..attributes_count)
            .map(|_| Attribute::decode_attribute(buffer))
            .collect::<Result<Vec<Attribute>, AttributeError>>()?;
        Ok(CodeAttribute {
            max_stack,
            max_locals,
            code_length,
            code,
            exception_table_length,
            exception_table,
            attributes_count,
            attributes,
        })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn we_can_decode_a_code_attribute_buffer() {}
}
