use super::element_value_pair::ElementValuePair;

pub struct Annotation {
    type_index: u16,
    num_element_value_pairs: u16,
    element_value_pairs: Vec<ElementValuePair>,
}
