use super::klass::Klass;

#[derive(Debug, Clone, Copy)]
pub struct InstanceKlass {}

impl InstanceKlass {
    pub fn cast(klass: &Klass) -> InstanceKlass {
        InstanceKlass {}
    }
}
