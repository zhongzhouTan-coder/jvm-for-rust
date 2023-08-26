use crate::reader::{attribute::Attribute, buffer::Buffer};

use super::{attribute_error::AttributeError, attribute_trait::AttributeTrait};

pub struct RecordAttribute {
    components_count: u16,
    components: Vec<RecordComponentInfo>,
}

impl AttributeTrait for RecordAttribute {
    fn decode_attribute(buffer: &mut Buffer) -> Result<Self, AttributeError>
    where
        Self: Sized,
    {
        let components_count = buffer.read_u16()?;
        let components = (0..components_count)
            .map(|_| RecordComponentInfo::decode_attribute(buffer))
            .collect::<Result<Vec<RecordComponentInfo>, AttributeError>>()?;
        Ok(RecordAttribute {
            components_count,
            components,
        })
    }
}

pub struct RecordComponentInfo {
    name_index: u16,
    descriptor_index: u16,
    attributes_count: u16,
    attributes: Vec<Attribute>,
}

impl AttributeTrait for RecordComponentInfo {
    fn decode_attribute(buffer: &mut Buffer) -> Result<Self, AttributeError>
    where
        Self: Sized,
    {
        let name_index = buffer.read_u16()?;
        let descriptor_index = buffer.read_u16()?;
        let attributes_count = buffer.read_u16()?;
        let attributes = (0..attributes_count)
            .map(|_| Attribute::decode_attribute(buffer))
            .collect::<Result<Vec<Attribute>, AttributeError>>()?;
        Ok(RecordComponentInfo {
            name_index,
            descriptor_index,
            attributes_count,
            attributes,
        })
    }
}
