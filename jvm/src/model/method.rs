use crate::utilities::definition::{u1, u2, u4};

pub struct Method {
    flags: u2,
    name_index: u2,
    descriptor_index: u2,
    code: Code,
}

struct Code {
    max_stack: u2,
    max_locals: u2,
    code_size: u4,
    code: Vec<u1>,
    line_number_table: Option<LineNumberTable>,
}

struct LineNumberTable {
    entries: Vec<(u2, u2)>,
}
