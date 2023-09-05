use std::ops;

use crate::reader::buffer::Buffer;

use super::{attribute_error::AttributeError, attribute_trait::AttributeTrait};

pub struct InnerClassesAttribute {
    number_of_classes: u16,
    classes: Vec<Classes>,
}

impl AttributeTrait for InnerClassesAttribute {
    fn decode_attribute(buffer: &mut Buffer) -> Result<Self, AttributeError>
    where
        Self: Sized,
    {
        let number_of_classes = buffer.read_u16()?;
        let classes = (0..number_of_classes)
            .map(|_| Classes::decode_attribute(buffer))
            .collect::<Result<Vec<Classes>, AttributeError>>()?;
        Ok(InnerClassesAttribute {
            number_of_classes,
            classes,
        })
    }
}

struct Classes {
    inner_class_info_index: u16,
    outer_class_info_index: u16,
    inner_name_index: u16,
    inner_class_access_flags: u16,
}

impl AttributeTrait for Classes {
    fn decode_attribute(buffer: &mut Buffer) -> Result<Self, AttributeError>
    where
        Self: Sized,
    {
        let inner_class_info_index = buffer.read_u16()?;
        let outer_class_info_index = buffer.read_u16()?;
        let inner_name_index = buffer.read_u16()?;
        let inner_class_access_flags = buffer.read_u16()?;
        Ok(Classes {
            inner_class_info_index,
            outer_class_info_index,
            inner_name_index,
            inner_class_access_flags,
        })
    }
}

#[repr(u16)]
#[allow(non_camel_case_types)]
pub enum InnerClassAccessFlags {
    PUBLIC = 0x0001u16,
    PRIVATE = 0x0002u16,
    PROTECTED = 0x0004u16,
    STATIC = 0x0008u16,
    FINAL = 0x0010u16,
    INTERFACE = 0x0200u16,
    ABSTRACT = 0x0400u16,
    SYNTHETIC = 0x1000u16,
    ANNOTATION = 0x2000u16,
    ENUM = 0x4000u16,
}

impl ops::BitOr for InnerClassAccessFlags {
    type Output = u16;

    fn bitor(self, rhs: Self) -> Self::Output {
        self as u16 | rhs as u16
    }
}
