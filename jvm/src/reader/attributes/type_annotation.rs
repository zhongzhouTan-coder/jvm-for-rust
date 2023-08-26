use super::element_value_pair::ElementValuePair;

pub struct TypeAnnotation {
    target_type: u8,
    target_info: TargetInfo,
    target_path: TypePath,
    type_index: u16,
    num_element_value_pairs: u16,
    element_value_pairs: Vec<ElementValuePair>,
}

pub enum TargetInfo {
    TypeParameterTarget(u8),
    SuperTypeTarget(u16),
    TypeParameterBoundTarget(u8, u8),
    EmptyTarget,
    FormalParameterTarget(u8),
    ThrowsTarget(u16),
    LocalVarTarget(u16, Table),
    CatchTarget(u16),
    OffsetTarget(u16),
    TypeArgumentTarget(u16, u8),
}

pub struct Table {
    start_pc: u16,
    length: u16,
    index: u16,
}

pub struct TypePath {
    path_length: u8,
    paths: Vec<Path>,
}

pub struct Path {
    type_path_kind: u8,
    type_argument_index: u8,
}
