use super::annotation::Annotation;

pub struct RuntimeInvisibleAnnotationsAttribute {
    num_annotations: u16,
    annotations: Vec<Annotation>,
}
