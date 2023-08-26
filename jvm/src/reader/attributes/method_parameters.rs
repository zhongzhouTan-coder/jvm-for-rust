pub struct MethodParametersAttribute {
    parameters_count: u8,
    parameters: Vec<Parameter>,
}

pub struct Parameter {
    name_index: u16,
    access_flags: u16,
}
