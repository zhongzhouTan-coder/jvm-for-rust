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
