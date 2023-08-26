use crate::reader::buffer;

use super::{attribute_error::AttributeError, attribute_trait::AttributeTrait};

pub struct BootstrapMethodsAttribute {
    num_bootstrap_methods: u16,
    bootstrap_methods: Vec<BootstrapMethod>,
}

impl AttributeTrait for BootstrapMethodsAttribute {
    fn decode_attribute(buffer: &mut buffer::Buffer) -> Result<Self, AttributeError>
    where
        Self: Sized,
    {
        let num_bootstrap_methods = buffer.read_u16()?;
        let bootstrap_methods = (0..num_bootstrap_methods)
            .map(|_| BootstrapMethod::decode_attribute(buffer))
            .collect::<Result<Vec<BootstrapMethod>, AttributeError>>()?;
        Ok(BootstrapMethodsAttribute {
            num_bootstrap_methods,
            bootstrap_methods,
        })
    }
}

pub struct BootstrapMethod {
    bootstrap_method_ref: u16,
    num_bootstrap_arguments: u16,
    bootstrap_arguments: Vec<u16>,
}

impl AttributeTrait for BootstrapMethod {
    fn decode_attribute(buffer: &mut buffer::Buffer) -> Result<Self, AttributeError>
    where
        Self: Sized,
    {
        let bootstrap_method_ref = buffer.read_u16()?;
        let num_bootstrap_arguments = buffer.read_u16()?;
        let bootstrap_arguments = (0..num_bootstrap_arguments)
            .map(|_| buffer.read_u16())
            .map(|method| method.map_err(|err| AttributeError::from(err)))
            .collect::<Result<Vec<u16>, AttributeError>>()?;
        Ok(BootstrapMethod {
            bootstrap_method_ref,
            num_bootstrap_arguments,
            bootstrap_arguments,
        })
    }
}
