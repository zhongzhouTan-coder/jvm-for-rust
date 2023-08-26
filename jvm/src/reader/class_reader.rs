use crate::reader::field_info::FieldInfo;

use super::{
    attribute::Attribute,
    buffer::{Buffer, BufferError},
    class_file::ClassFile,
    class_file_version::ClassFileVersion,
    constant_info::ConstantInfo,
    method_info::MethodInfo,
    type_conversion::ToUsizeSafe,
};

pub struct ClassReader<'a> {
    buffer: Buffer<'a>,
    class_file: ClassFile,
}

type Result<T> = std::result::Result<T, ClassReaderError>;

#[derive(Debug, PartialEq, Eq)]
pub enum ClassReaderError {
    InvalidClassData,
    InvalidClassFormat(String),
}

impl<'a> ClassReader<'a> {
    pub fn new(data: &[u8]) -> ClassReader {
        ClassReader {
            buffer: Buffer::new(data),
            class_file: Default::default(),
        }
    }

    pub fn read(mut self) -> Result<ClassFile> {
        self.check_magic_number()?;
        self.read_and_check_version()?;
        self.read_const_pool_count()?;
        self.read_const_pool()?;
        self.read_access_flags()?;
        self.read_this_class()?;
        self.read_super_class()?;
        self.read_interfaces_count()?;
        self.read_interfaces()?;
        self.read_fields_count()?;
        self.read_fields()?;
        self.read_methods_count()?;
        self.read_methods()?;
        self.read_attributes_count()?;
        self.read_attributes()?;
        self.check_trunked_data()?;
        Ok(self.class_file)
    }

    fn check_magic_number(&mut self) -> Result<()> {
        match self.buffer.read_u32() {
            Ok(0xCAFEBABE) => Ok(()),
            Ok(_) => Err(ClassReaderError::InvalidClassFormat(String::from(
                "Magic number check error!",
            ))),
            Err(err) => Err(err.into()),
        }
    }

    fn read_and_check_version(&mut self) -> Result<()> {
        self.buffer.read_u16()?;
        match self.buffer.read_u16() {
            Ok(version) => {
                self.class_file.major_version = ClassFileVersion::from_u16(version);
                Ok(())
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
        let count = self.class_file.constant_pool_count - 1;
        for _ in 0..count {
            let tag = self.buffer.read_u8()?;
            let constant_info = match tag {
                1 => self.read_utf8()?,
                3 => self.read_integer()?,
                4 => self.read_float()?,
                5 => self.read_long()?,
                6 => self.read_double()?,
                7 => self.read_class()?,
                8 => self.read_string()?,
                9 => self.read_field_ref()?,
                10 => self.read_method_ref()?,
                11 => self.read_interface_method_ref()?,
                12 => self.read_name_and_type()?,
                15 => self.read_method_handle()?,
                16 => self.read_method_type()?,
                17 => self.read_dynamic()?,
                18 => self.read_invoke_dynamic()?,
                19 => self.read_module()?,
                20 => self.read_package()?,
                _ => return Err(ClassReaderError::InvalidClassData),
            };
            self.class_file.constant_pool.push(constant_info);
        }
        Ok(())
    }

    fn read_utf8(&mut self) -> Result<ConstantInfo> {
        let length = self.buffer.read_u16()?;
        match self.buffer.read_utf8(length.try_into().unwrap()) {
            Ok(content) => Ok(ConstantInfo::Utf8(content)),
            Err(err) => Err(err.into()),
        }
    }

    fn read_integer(&mut self) -> Result<ConstantInfo> {
        match self.buffer.read_integer() {
            Ok(number) => Ok(ConstantInfo::Integer(number)),
            Err(err) => Err(err.into()),
        }
    }

    fn read_float(&mut self) -> Result<ConstantInfo> {
        match self.buffer.read_float() {
            Ok(float) => Ok(ConstantInfo::Float(float)),
            Err(err) => Err(err.into()),
        }
    }

    fn read_long(&mut self) -> Result<ConstantInfo> {
        match self.buffer.read_long() {
            Ok(long) => Ok(ConstantInfo::Long(long)),
            Err(err) => Err(err.into()),
        }
    }

    fn read_double(&mut self) -> Result<ConstantInfo> {
        match self.buffer.read_double() {
            Ok(double) => Ok(ConstantInfo::Double(double)),
            Err(err) => Err(err.into()),
        }
    }

    fn read_class(&mut self) -> Result<ConstantInfo> {
        match self.buffer.read_u16() {
            Ok(name_index) => Ok(ConstantInfo::Class(name_index)),
            Err(err) => Err(err.into()),
        }
    }

    fn read_string(&mut self) -> Result<ConstantInfo> {
        match self.buffer.read_u16() {
            Ok(string_index) => Ok(ConstantInfo::String(string_index)),
            Err(err) => Err(err.into()),
        }
    }

    fn read_field_ref(&mut self) -> Result<ConstantInfo> {
        let class_index = self.buffer.read_u16()?;
        let name_and_type_index = self.buffer.read_u16()?;
        Ok(ConstantInfo::FieldRef(class_index, name_and_type_index))
    }

    fn read_method_ref(&mut self) -> Result<ConstantInfo> {
        let class_index = self.buffer.read_u16()?;
        let name_and_type_index = self.buffer.read_u16()?;
        Ok(ConstantInfo::MethodRef(class_index, name_and_type_index))
    }

    fn read_interface_method_ref(&mut self) -> Result<ConstantInfo> {
        let class_index = self.buffer.read_u16()?;
        let name_and_type_index = self.buffer.read_u16()?;
        Ok(ConstantInfo::InterfaceMethodRef(
            class_index,
            name_and_type_index,
        ))
    }

    fn read_name_and_type(&mut self) -> Result<ConstantInfo> {
        let name_index = self.buffer.read_u16()?;
        let descriptor_index = self.buffer.read_u16()?;
        Ok(ConstantInfo::NameAndType(name_index, descriptor_index))
    }

    fn read_method_handle(&mut self) -> Result<ConstantInfo> {
        let reference_kind = self.buffer.read_u8()?;
        let reference_index = self.buffer.read_u16()?;
        Ok(ConstantInfo::MethodHandle(reference_kind, reference_index))
    }

    fn read_method_type(&mut self) -> Result<ConstantInfo> {
        match self.buffer.read_u16() {
            Ok(index) => Ok(ConstantInfo::MethodType(index)),
            Err(err) => Err(err.into()),
        }
    }

    fn read_dynamic(&mut self) -> Result<ConstantInfo> {
        let bootstrap_method_attr_index = self.buffer.read_u16()?;
        let name_and_type_index = self.buffer.read_u16()?;
        Ok(ConstantInfo::Dynamic(
            bootstrap_method_attr_index,
            name_and_type_index,
        ))
    }

    fn read_invoke_dynamic(&mut self) -> Result<ConstantInfo> {
        let bootstrap_method_attr_index = self.buffer.read_u16()?;
        let name_and_type_index = self.buffer.read_u16()?;
        Ok(ConstantInfo::InvokeDynamic(
            bootstrap_method_attr_index,
            name_and_type_index,
        ))
    }

    fn read_module(&mut self) -> Result<ConstantInfo> {
        match self.buffer.read_u16() {
            Ok(name_index) => Ok(ConstantInfo::Module(name_index)),
            Err(err) => Err(err.into()),
        }
    }

    fn read_package(&mut self) -> Result<ConstantInfo> {
        match self.buffer.read_u16() {
            Ok(name_index) => Ok(ConstantInfo::Package(name_index)),
            Err(err) => Err(err.into()),
        }
    }

    fn read_access_flags(&mut self) -> Result<()> {
        match self.buffer.read_u16() {
            Ok(flags) => {
                self.class_file.access_flags = flags;
                Ok(())
            }
            Err(err) => Err(err.into()),
        }
    }

    fn read_this_class(&mut self) -> Result<()> {
        match self.buffer.read_u16() {
            Ok(this_class) => {
                self.class_file.this_class = this_class;
                Ok(())
            }
            Err(err) => Err(err.into()),
        }
    }

    fn read_super_class(&mut self) -> Result<()> {
        match self.buffer.read_u16() {
            Ok(super_class) => {
                self.class_file.super_class = super_class;
                Ok(())
            }
            Err(err) => Err(err.into()),
        }
    }

    fn read_interfaces_count(&mut self) -> Result<()> {
        match self.buffer.read_u16() {
            Ok(interfaces_count) => {
                self.class_file.interfaces_count = interfaces_count;
                Ok(())
            }
            Err(err) => Err(err.into()),
        }
    }

    fn read_interfaces(&mut self) -> Result<()> {
        let interfaces_count = self.class_file.interfaces_count;
        self.class_file.interfaces = (0..interfaces_count)
            .map(|_| self.buffer.read_u16())
            .map(|result| result.map_err(|err| err.into()))
            .collect::<Result<Vec<u16>>>()?;
        Ok(())
    }

    fn read_fields_count(&mut self) -> Result<()> {
        match self.buffer.read_u16() {
            Ok(fields_count) => {
                self.class_file.fields_count = fields_count;
                Ok(())
            }
            Err(err) => Err(err.into()),
        }
    }

    fn read_fields(&mut self) -> Result<()> {
        let fields_count = self.class_file.fields_count;
        for _ in 0..fields_count {
            let access_flags = self.buffer.read_u16()?;
            let name_index = self.buffer.read_u16()?;
            let name = match self.class_file.get_constant_info(name_index) {
                ConstantInfo::Utf8(content) => content.clone(),
                _ => panic!("Invalid name index for field - {}.", name_index),
            };
            let descriptor_index = self.buffer.read_u16()?;
            let descriptor = match self.class_file.get_constant_info(descriptor_index) {
                ConstantInfo::Utf8(content) => content.clone(),
                _ => panic!("Invalid name index for descriptor - {}.", descriptor_index),
            };
            let attributes_count = self.buffer.read_u16()?;
            let attributes = (0..attributes_count)
                .map(|_| self.read_attribute())
                .collect::<Result<Vec<Attribute>>>()?;
            self.class_file
                .fields
                .push(FieldInfo::new(access_flags, name, descriptor, attributes));
        }
        Ok(())
    }

    fn read_attribute(&mut self) -> Result<Attribute> {
        let attribute_name_index = self.buffer.read_u16()?;
        let attribute_length = self.buffer.read_u32()?;
        let info = self
            .buffer
            .read_vec_u8(attribute_length.into_usize_safe())?;
        Ok(Attribute::new(attribute_name_index, attribute_length, info))
    }

    fn read_methods_count(&mut self) -> Result<()> {
        match self.buffer.read_u16() {
            Ok(methods_count) => {
                self.class_file.methods_count = methods_count;
                Ok(())
            }
            Err(err) => Err(err.into()),
        }
    }

    fn read_methods(&mut self) -> Result<()> {
        let methods_count = self.class_file.methods_count;
        for _ in 0..methods_count {
            let access_flags = self.buffer.read_u16()?;
            let name_index = self.buffer.read_u16()?;
            let name = match self.class_file.get_constant_info(name_index) {
                ConstantInfo::Utf8(content) => content.clone(),
                _ => panic!("Invalid method name index - {}.", name_index),
            };
            let descriptor_index = self.buffer.read_u16()?;
            let descriptor = match self.class_file.get_constant_info(descriptor_index) {
                ConstantInfo::Utf8(content) => content.clone(),
                _ => panic!("Invalid method descriptor index - {}.", descriptor_index),
            };
            let attributes_count = self.buffer.read_u16()?;
            let attributes = (0..attributes_count)
                .map(|_| self.read_attribute())
                .collect::<Result<Vec<Attribute>>>()?;
            self.class_file.methods.push(MethodInfo::new(
                access_flags,
                name,
                descriptor,
                attributes_count,
                attributes,
            ));
        }
        Ok(())
    }

    fn read_attributes_count(&mut self) -> Result<()> {
        match self.buffer.read_u16() {
            Ok(attributes_count) => {
                self.class_file.attributes_count = attributes_count;
                Ok(())
            }
            Err(err) => Err(err.into()),
        }
    }

    fn read_attributes(&mut self) -> Result<()> {
        let attributes_count = self.class_file.attributes_count;
        for _ in 0..attributes_count {
            let attribute = self.read_attribute()?;
            self.class_file.attributes.push(attribute);
        }
        Ok(())
    }

    fn check_trunked_data(&self) -> Result<()> {
        if self.buffer.has_unread_data()? {
            Err(ClassReaderError::InvalidClassFormat(String::from(
                "Read trunked class file data!",
            )))
        } else {
            Ok(())
        }
    }
}

impl From<BufferError> for ClassReaderError {
    fn from(error: BufferError) -> Self {
        match error {
            BufferError::OutBoundaryOfData => Self::InvalidClassData,
        }
    }
}
