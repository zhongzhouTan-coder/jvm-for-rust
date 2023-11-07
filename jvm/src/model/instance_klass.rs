use crate::utilities::definition::u2;

use super::{constant_pool::ConstantPool, field::Field, method::Method};

pub struct InstanceKlass {
    constants: ConstantPool,
    this_class_index: u2,
    methods: Vec<Method>,
    fields: Vec<Field>,
}
