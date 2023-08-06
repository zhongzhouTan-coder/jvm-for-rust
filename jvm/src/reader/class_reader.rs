use core::num;
use std::{default, str::FromStr};

use super::{
    buffer::{Buffer, BufferError},
    class_file::ClassFile,
    constant_pool_info::{ConstantInfo, ConstantPoolInfo},
};

struct ClassReader<'a> {
    buffer: Buffer<'a>,
    class_file: ClassFile,
}

type Result<T> = std::result::Result<T, ClassReaderError>;

#[derive(Debug, PartialEq, Eq)]
pub enum ClassReaderError {
    InvalidClassData,
    UnsupportedVersion(String),
}

impl<'a> ClassReader<'a> {
    pub fn new(data: &[u8]) -> ClassReader {
        ClassReader {
            buffer: Buffer::new(data),
            class_file: Default::default(),
        }
    }

    fn read(mut self) -> Result<ClassFile> {
        self.read_magic_number()?;
        self.read_minor_version()?;
        self.read_major_version()?;
        self.read_const_pool_count()?;
        Ok(self.class_file)
    }

    fn read_magic_number(&mut self) -> Result<()> {
        match self.buffer.read_u32() {
            Ok(0xCAFEBABE) => {
                self.class_file.magic = 0xCAFEBABE;
                Ok(())
            }
            Ok(_) => Err(ClassReaderError::InvalidClassData),
            Err(err) => Err(err.into()),
        }
    }

    fn read_minor_version(&mut self) -> Result<()> {
        match self.buffer.read_u16() {
            Ok(version) => {
                self.class_file.minor_version = version;
                Ok(())
            }
            Err(err) => Err(err.into()),
        }
    }

    fn read_major_version(&mut self) -> Result<()> {
        match self.buffer.read_u16() {
            Ok(version) => {
                if version > 61 {
                    Err(ClassReaderError::UnsupportedVersion(
                        String::from_str("Only support JDK 17 or below.").unwrap(),
                    ))
                } else {
                    self.class_file.major_version = version;
                    Ok(())
                }
            }
            Err(err) => Err(err.into()),
        }
    }

    fn read_const_pool_count(&mut self) -> Result<()> {
        match self.buffer.read_u16() {
            Ok(count) => {
                self.class_file.constant_pool_count = count;
                Ok(())
            }
            Err(err) => Err(err.into()),
        }
    }

    fn read_const_pool(&mut self) -> Result<()> {
        let count = self.class_file.constant_pool_count;
        for _ in 0..count {
            match self.buffer.read_u8()? {
                1 => self.read_utf8()?,
                3 => self.read_integer()?,
                4 => self.read_float()?,
                _ => return Err(ClassReaderError::InvalidClassData),
            }
        }
        Ok(())
    }

    fn read_utf8(&mut self) -> Result<()> {
        let length = self.buffer.read_u16()?;
        match self.buffer.read_utf8(length.try_into().unwrap()) {
            Ok(content) => {
                let utf8_info = ConstantPoolInfo::new(1, ConstantInfo::Utf8(length, content));
                self.class_file.constant_pool.push(utf8_info);
                Ok(())
            }
            Err(err) => Err(err.into()),
        }
    }

    fn read_integer(&mut self) -> Result<()> {
        match self.buffer.read_integer() {
            Ok(number) => {
                let integer_info = ConstantPoolInfo::new(3, ConstantInfo::Integer(number));
                self.class_file.constant_pool.push(integer_info);
                Ok(())
            }
            Err(err) => Err(err.into()),
        }
    }

    fn read_float(&mut self) -> Result<()> {
        match self.buffer.read_float() {
            Ok(float) => {
                let float_info = ConstantPoolInfo::new(4, ConstantInfo::Float(float));
                self.class_file.constant_pool.push(float_info);
                Ok(())
            }
            Err(err) => Err(err.into()),
        }
    }

    fn read_long(&mut self) -> Result<()> {
        match self.buffer.read_long() {
            Ok(long) => {
                let long_info = ConstantPoolInfo::new(5, ConstantInfo::Long(long));
                self.class_file.constant_pool.push(long_info);
                Ok(())
            }
            Err(err) => Err(err.into()),
        }
    }

    fn read_double(&mut self) -> Result<()> {
        match self.buffer.read_double() {
            Ok(double) => {
                let double_info = ConstantPoolInfo::new(6, ConstantInfo::Double(double));
                self.class_file.constant_pool.push(double_info);
                Ok(())
            }
            Err(err) => Err(err.into()),
        }
    }

    fn read_class(&mut self) -> Result<()> {
        match self.buffer.read_u16() {
            Ok(name_index) => {
                let class_info = ConstantPoolInfo::new(7, ConstantInfo::Class(name_index));
                self.class_file.constant_pool.push(class_info);
                Ok(())
            }
            Err(err) => Err(err.into()),
        }
    }

    fn read_string(&mut self) -> Result<()> {
        match self.buffer.read_u16() {
            Ok(string_index) => {
                let string_info = ConstantPoolInfo::new(8, ConstantInfo::String(string_index));
                self.class_file.constant_pool.push(string_info);
                Ok(())
            }
            Err(err) => Err(err.into()),
        }
    }

    fn read_field_ref(&mut self) -> Result<()> {
        let class_index = self.buffer.read_u16()?;
        let name_and_type_index = self.buffer.read_u16()?;
        let field_ref_info =
            ConstantPoolInfo::new(9, ConstantInfo::FieldRef(class_index, name_and_type_index));
        self.class_file.constant_pool.push(field_ref_info);
        Ok(())
    }
}

impl From<BufferError> for ClassReaderError {
    fn from(error: BufferError) -> Self {
        match error {
            BufferError::OutBoundaryOfData => Self::InvalidClassData,
        }
    }
}
