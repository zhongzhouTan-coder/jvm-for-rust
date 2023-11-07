use crate::utilities::definition::{jint, u2};

pub struct Field {
    flags: u2,
    name_index: u2,
    descriptor_index: u2,
    constant_value_index: u2,
}
