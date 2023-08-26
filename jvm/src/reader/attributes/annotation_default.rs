use super::annotation::Annotation;

pub struct AnnotationDefaultAttribute {
    default_value: ElementValue,
}
pub struct ElementValue {
    tag: u8,
    value: Value,
}

pub struct Value {
    const_value_index: u16,
    enum_const_value: EnumConstValue,
    class_info_index: u16,
    annotation_value: Annotation,
    array_value: ArrayValue,
}

pub struct EnumConstValue {
    type_name_index: u16,
    const_name_index: u16,
}

pub struct ArrayValue {
    num_values: u16,
    values: Vec<ElementValue>,
}
