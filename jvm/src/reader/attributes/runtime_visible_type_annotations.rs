use super::type_annotation::TypeAnnotation;

pub struct RuntimeVisibleTypeAnnotationsAttribute {
    num_annotations: u16,
    annotations: Vec<TypeAnnotation>,
}
