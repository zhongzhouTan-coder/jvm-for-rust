use super::type_annotation::TypeAnnotation;

pub struct RuntimeInvisibleTypeAnnotationsAttribute {
    num_annotations: u16,
    annotations: Vec<TypeAnnotation>,
}
