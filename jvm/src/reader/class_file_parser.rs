use core::num;

use crate::{
    reader::{
        reference_kind::ReferenceKind, symbol::Symbol, symbol_table::SymbolTable,
        verifier::Verifier,
    },
    switch,
};

use super::{
    buffer::Buffer, class_access_flag::ClassAccessFlag, class_file::ClassFile,
    class_file_error::ClassFileError, class_file_version::ClassFileVersion,
    constant_pool::ConstantPool, constant_tag::ConstantTag, jvm_constants::JAVA_CLASSFILE_MAGIC,
    vm_symbols::VmSymbols,
};

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
        self.parse_access_flags()?;
        self.parse_this_class();
        self.parse_super_class();
        self.parse_interfaces();
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
                    match SymbolTable::lookup_only(&utf8_bytes.to_vec()) {
                        Some(symbol) => self.class_file
                            .constant_pool
                            .symbol_at_put(index as usize, symbol),
                        None => {
                            let result = SymbolTable::new_symbol(utf8_bytes.to_vec(), Symbol::new(utf8_bytes.to_vec()));
                            self.class_file
                                .constant_pool
                                .symbol_at_put(index as usize, result);
                        }
                    }
                }
                ConstantTag::JVM_CONSTANT_Module => {
                    self.buffer.guarantee_more(3);
                    self.buffer.skip_u8_fast(2);
                }
                ConstantTag::JVM_CONSTANT_Package => {
                    self.buffer.guarantee_more(3);
                    self.buffer.skip_u8_fast(2);
                }
                _ => {}
            });
            index += 1;
        }

        let mut index = 1;
        let mut num_klasses = 0;
        while index < cp_size {
            match *self.class_file.constant_pool.tag_at(index as usize) {
                ConstantTag::JVM_CONSTANT_Fieldref
                | ConstantTag::JVM_CONSTANT_Methodref
                | ConstantTag::JVM_CONSTANT_InterfaceMethodref => {
                    let klass_ref_index = self
                        .class_file
                        .constant_pool
                        .klass_ref_index_at(index as usize);
                    let name_and_type_ref_index = self
                        .class_file
                        .constant_pool
                        .name_and_type_ref_index_at(index as usize);
                    self.guarantee_property(
                        self.valid_klass_reference_at(klass_ref_index as usize),
                        &format!("Invalid constant pool index {}.", klass_ref_index),
                    )?;
                    self.guarantee_property(
                        self.valid_name_and_type_reference_at(name_and_type_ref_index as usize),
                        &format!("Invalid constant pool index {}.", name_and_type_ref_index),
                    )?;
                }
                ConstantTag::JVM_CONSTANT_Integer | ConstantTag::JVM_CONSTANT_Float => {}
                ConstantTag::JVM_CONSTANT_Long | ConstantTag::JVM_CONSTANT_Double => {
                    index += 1;
                    self.guarantee_property(
                        index < cp_size
                            && self
                                .class_file
                                .constant_pool
                                .tag_at(index as usize)
                                .is_invalid(),
                        &format!("Improper constant long/double index {}.", index),
                    )?;
                }
                ConstantTag::JVM_CONSTANT_NameAndType => {
                    let name_ref_index = self
                        .class_file
                        .constant_pool
                        .name_ref_index_at(index as usize);
                    let signature_ref_index = self
                        .class_file
                        .constant_pool
                        .signature_ref_index_at(index as usize);
                    self.guarantee_property(
                        self.valid_symbol_at(name_ref_index as usize),
                        &format!("Invalid constant pool index {}.", name_ref_index),
                    )?;
                    self.guarantee_property(
                        self.valid_symbol_at(signature_ref_index as usize),
                        &format!("Invalid constant pool index {}.", signature_ref_index),
                    )?;
                }
                ConstantTag::JVM_CONSTANT_ClassIndex => {
                    let class_index = self.class_file.constant_pool.klass_index_at(index as usize);
                    self.guarantee_property(
                        self.valid_symbol_at(class_index as usize),
                        &format!("Invalid constant index {}.", class_index),
                    )?;
                    self.class_file.constant_pool.unresolved_klass_at_put(
                        index as usize,
                        class_index,
                        num_klasses,
                    );
                    num_klasses += 1;
                }
                ConstantTag::JVM_CONSTANT_StringIndex => {
                    let string_index = self
                        .class_file
                        .constant_pool
                        .string_index_at(index as usize);
                    self.guarantee_property(
                        self.valid_symbol_at(string_index as usize),
                        &format!("Invalid constant pool index {}.", string_index),
                    )?;
                    let symbol = self
                        .class_file
                        .constant_pool
                        .symbol_at(string_index as usize);
                    self.class_file
                        .constant_pool
                        .unresolved_string_at_put(index as usize, symbol);
                }
                ConstantTag::JVM_CONSTANT_MethodHandle => {
                    let ref_index = self
                        .class_file
                        .constant_pool
                        .method_handle_index_at(index as usize);
                    self.guarantee_property(
                        self.valid_cp_range(ref_index as usize),
                        &format!("Invalid constant pool index {}.", ref_index),
                    )?;
                    let tag = self.class_file.constant_pool.tag_at(ref_index as usize);
                    let ref_kind = self
                        .class_file
                        .constant_pool
                        .method_handle_ref_kind_at(index as usize);
                }
                _ => panic!("Invalid constant type."),
            }
        }
        Ok(())
    }

    fn valid_klass_reference_at(&self, index: usize) -> bool {
        self.class_file.constant_pool.is_within_bounds(index)
            && self
                .class_file
                .constant_pool
                .tag_at(index)
                .is_klass_or_klass_reference()
    }

    fn valid_name_and_type_reference_at(&self, index: usize) -> bool {
        self.valid_cp_range(index)
            && self
                .class_file
                .constant_pool
                .tag_at(index)
                .is_name_and_type()
    }

    fn valid_cp_range(&self, index: usize) -> bool {
        index > 0 && index < self.class_file.constant_pool.length()
    }

    fn valid_symbol_at(&self, index: usize) -> bool {
        self.class_file.constant_pool.is_within_bounds(index)
            && self.class_file.constant_pool.tag_at(index).is_utf8()
    }

    fn parse_access_flags(&mut self) -> Result<(), ClassFileError> {
        let access_flags = self.buffer.guarantee_more(8);
        let mut flags;
        if self.class_file.major_version >= ClassFileVersion::Jdk9 as u16 {
            flags = self.buffer.get_u16_fast()
                & (ClassAccessFlag::jvm_recognized_flags() | (ClassAccessFlag::MODULE as u16));
        } else {
            flags = self.buffer.get_u16_fast() & ClassAccessFlag::jvm_recognized_flags();
        }
        if (flags & ClassAccessFlag::INTERFACE as u16) > 0
            && self.class_file.major_version < ClassFileVersion::Jdk6 as u16
        {
            flags |= ClassAccessFlag::ABSTRACT as u16;
        }
        // "we need to verify the access flag is valid."
        self.class_file.access_flags = flags;
        self.class_file.super_class_index = self.buffer.get_u16_fast();
        Ok(())
    }

    fn parse_this_class(&mut self) {
        self.buffer.guarantee_more(2);
        self.class_file.this_class_index = self.buffer.get_u16_fast();
        self.class_file.class_name = self
            .class_file
            .constant_pool
            .klass_name_at(self.class_file.this_class_index as usize);
    }

    fn parse_super_class(&mut self) -> Result<(), ClassFileError> {
        self.buffer.guarantee_more(2);
        let super_class_index = self.buffer.get_u16_fast();
        if super_class_index == 0 {
            self.guarantee_property(
                self.class_file.class_name.data == VmSymbols::java_lang_object(),
                &format!(
                    "Invalid superclass index {} in class file.",
                    super_class_index
                ),
            )?;
        }
        self.class_file.super_class_index = super_class_index;
        Ok(())
    }

    fn parse_interfaces(&mut self) {
        self.buffer.guarantee_more(2);
        let itfs_len = self.buffer.get_u16_fast();
        for index in 0..itfs_len {
            let interface_index = self.buffer.get_u16_fast();
        }
    }
}