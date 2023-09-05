use crate::{
    reader::{symbol::Symbol, symbol_table::SymbolTable, verifier::Verifier},
    switch,
};

use super::{
    buffer::Buffer, class_file::ClassFile, class_file_error::ClassFileError,
    constant_pool::ConstantPool, constant_tag::ConstantTag,
};

const JAVA_CLASSFILE_MAGIC: u32 = 0xCAFEBABEu32;

pub struct ClassFileParser<'a> {
    buffer: Buffer<'a>,
    class_file: ClassFile,
}

impl<'a> ClassFileParser<'a> {
    pub fn new(data: &'a [u8], length: usize, source: &'a str) -> ClassFileParser<'a> {
        ClassFileParser {
            buffer: Buffer::new(data, length, source),
            class_file: Default::default(),
        }
    }

    pub fn parse_buffer(mut self) -> Result<(), ClassFileError> {
        self.check_and_read_version()?;
        self.parse_constant_pool()?;
        Ok(())
    }

    fn guarantee_property(&self, b: bool, msg: &str) -> Result<(), ClassFileError> {
        if !b {
            return Err(ClassFileError::ClassFormatError(msg.to_string()));
        }
        Ok(())
    }

    fn check_and_read_version(&mut self) -> Result<(), ClassFileError> {
        self.buffer.guarantee_more(8);
        let magic_number = self.buffer.get_u32_fast();
        self.guarantee_property(
            magic_number == JAVA_CLASSFILE_MAGIC,
            &format!("Incompatible magic value {} in class file.", magic_number),
        )?;
        self.class_file.major_version = self.buffer.get_u16_fast();
        self.class_file.minor_version = self.buffer.get_u16_fast();
        Ok(())
    }

    fn parse_constant_pool(&mut self) -> Result<(), ClassFileError> {
        self.buffer.guarantee_more(3);
        let cp_size = self.buffer.get_u16_fast();
        self.guarantee_property(
            cp_size > 1,
            &format!("Illegal constant pool size {}.", cp_size),
        )?;
        self.class_file.constant_pool = ConstantPool::new(cp_size as usize);
        let mut index = 1;
        while index < cp_size {
            let tag = self.buffer.get_u8_fast();
            switch!(tag, u8, {
                ConstantTag::JVM_CONSTANT_Class => {
                    self.buffer.guarantee_more(3);
                    let name_index = self.buffer.get_u16_fast();
                    self.class_file
                        .constant_pool
                        .klass_index_at_put(index as usize, name_index);
                }
                ConstantTag::JVM_CONSTANT_Fieldref => {
                    self.buffer.guarantee_more(5);
                    let class_index = self.buffer.get_u16_fast();
                    let name_and_type_index = self.buffer.get_u16_fast();
                    self.class_file
                        .constant_pool
                        .field_at_put(index as usize, class_index, name_and_type_index);
                }
                ConstantTag::JVM_CONSTANT_Methodref => {
                    self.buffer.guarantee_more(5);
                    let class_index = self.buffer.get_u16_fast();
                    let name_and_type_index = self.buffer.get_u16_fast();
                    self.class_file
                        .constant_pool
                        .method_at_put(index as usize, class_index, name_and_type_index);
                }
                ConstantTag::JVM_CONSTANT_InterfaceMethodref => {
                    self.buffer.guarantee_more(5);
                    let class_index = self.buffer.get_u16_fast();
                    let name_and_type_index = self.buffer.get_u16_fast();
                    self.class_file
                        .constant_pool
                        .interface_method_at_put(index as usize, class_index, name_and_type_index);
                }
                ConstantTag::JVM_CONSTANT_String => {
                    self.buffer.guarantee_more(3);
                    let string_index = self.buffer.get_u16_fast();
                    self.class_file.constant_pool.string_index_at_put(index as usize, string_index);
                }
                ConstantTag::JVM_CONSTANT_MethodHandle => {
                    let major_version = self.class_file.major_version;
                    assert!(major_version >= Verifier::INVOKED_DYNAMIC_MAJOR_VERSION as u16,
                        "Class file version does not support constant tag {}", tag);
                    self.buffer.guarantee_more(4);
                    let ref_kind = self.buffer.get_u8_fast();
                    let ref_index = self.buffer.get_u16_fast();
                    self.class_file
                        .constant_pool
                        .method_handle_index_at_put(index as usize, ref_kind, ref_index);
                }
                ConstantTag::JVM_CONSTANT_MethodType => {
                    let major_version = self.class_file.major_version;
                    assert!(major_version >= Verifier::INVOKED_DYNAMIC_MAJOR_VERSION as u16,
                        "Class file version does not support constant tag {}", tag);
                    self.buffer.guarantee_more(3);
                    let signature_index = self.buffer.get_u16_fast();
                    self.class_file
                        .constant_pool
                        .method_type_index_at_put(index as usize, signature_index);
                }
                ConstantTag::JVM_CONSTANT_Dynamic => {
                    let major_version = self.class_file.major_version;
                    assert!(major_version >= Verifier::DYNAMIC_CONSTANT_MAJOR_VERSION as u16,
                        "Class file version does not support constant tag {}", tag);
                    self.buffer.guarantee_more(5);
                    let bootstrap_specifier_index = self.buffer.get_u16_fast();
                    let name_and_type_index = self.buffer.get_u16_fast();
                    self.class_file
                        .constant_pool
                        .dynamic_constant_at_put(index as usize, bootstrap_specifier_index, name_and_type_index);
                }
                ConstantTag::JVM_CONSTANT_InvokeDynamic => {
                    let major_version = self.class_file.major_version;
                    assert!(major_version >= Verifier::INVOKED_DYNAMIC_MAJOR_VERSION as u16,
                        "Class file version does not support constant tag {}", tag);
                    self.buffer.guarantee_more(5);
                    let bootstrap_specifier_index = self.buffer.get_u16_fast();
                    let name_and_type_index = self.buffer.get_u16_fast();
                    self.class_file
                        .constant_pool
                        .invoke_dynamic_at_put(index as usize, bootstrap_specifier_index, name_and_type_index);
                }
                ConstantTag::JVM_CONSTANT_Integer => {
                    self.buffer.guarantee_more(5);
                    let bytes = self.buffer.get_u32_fast();
                    self.class_file
                        .constant_pool
                        .int_at_put(index as usize, bytes);
                }
                ConstantTag::JVM_CONSTANT_Float => {
                    self.buffer.guarantee_more(5);
                    let bytes = self.buffer.get_u32_fast();
                    self.class_file
                        .constant_pool
                        .float_at_put(index as usize, bytes);
                }
                ConstantTag::JVM_CONSTANT_Long => {
                    self.guarantee_property(index + 1 < cp_size, &format!("Invalid constant pool entry {}", index))?;
                    self.buffer.guarantee_more(9);
                    let bytes = self.buffer.get_u64_fast();
                    self.class_file
                        .constant_pool
                        .long_at_put(index as usize, bytes);
                    index += 1;
                }
                ConstantTag::JVM_CONSTANT_Double => {
                    self.guarantee_property(index + 1 < cp_size, &format!("Invalid constant pool entry {}", index))?;
                    self.buffer.guarantee_more(9);
                    let bytes = self.buffer.get_u64_fast();
                    self.class_file
                        .constant_pool
                        .double_at_put(index as usize, bytes);
                    index += 1;
                }
                ConstantTag::JVM_CONSTANT_NameAndType => {
                    self.buffer.guarantee_more(5);
                    let name_index = self.buffer.get_u16_fast();
                    let signature_index = self.buffer.get_u16_fast();
                    self.class_file
                        .constant_pool
                        .name_and_type_at_put(index as usize, name_index, signature_index);
                }
                ConstantTag::JVM_CONSTANT_Utf8 => {
                    self.buffer.guarantee_more(2);
                    let utf8_length = self.buffer.get_u16_fast();
                    self.buffer.guarantee_more((utf8_length + 1) as usize);
                    let utf8_bytes = self.buffer.get_u8_array_fast(utf8_length as usize);
                    match SymbolTable::global().lookup_only(&utf8_bytes.to_vec()) {
                        Some(symbol) => self.class_file
                            .constant_pool
                            .symbol_at_put(index as usize, symbol),
                        None => {
                            SymbolTable::global()
                                .new_symbol(utf8_bytes.to_vec(), Symbol::new(utf8_bytes.to_vec()))
                        }
                    }
                }
                _ => {}
            });
            index += 1;
        }
        Ok(())
    }
}
