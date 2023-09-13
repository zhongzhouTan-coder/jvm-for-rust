use std::sync::Arc;

use super::{constant_tag::ConstantTag, klass::Klass, symbol::Symbol};

#[derive(Debug, Clone, Default)]
pub struct ConstantPool {
    tags: Vec<ConstantTag>,
    length: u16,
    values: Vec<ConstantValue>,
    flags: u16,
    resolved_klasses: Vec<Klass>,
}

#[derive(Debug, Clone)]
enum ConstantValue {
    Invalid,
    JInt(u32),
    JFloat(f32),
    JDouble(f64),
    JLong(i64),
    Symbol(Arc<Symbol>),
}

#[repr(u16)]
enum Flag {
    HasPreresolution = 1,
    OnStack = 2,
    IsShared = 4,
    HasDynamicConstant = 8,
}

impl ConstantPool {
    pub fn new(length: u16) -> ConstantPool {
        let tags = vec![ConstantTag::JVM_CONSTANT_Invalid; length as usize];
        let values = vec![ConstantValue::Invalid; length as usize];
        let flags = 0x0000u16;
        let resolved_klasses = Vec::new();
        ConstantPool {
            tags,
            length,
            values,
            flags,
            resolved_klasses,
        }
    }

    pub fn length(&self) -> u16 {
        self.length
    }

    pub fn is_within_bounds(&self, index: u16) -> bool {
        index < self.length
    }

    pub fn set_has_dynamic_constant(&mut self) {
        self.flags |= Flag::HasDynamicConstant as u16
    }

    pub fn klass_index_at_put(&mut self, which: u16, name_index: u16) {
        self.tag_at_put(which, ConstantTag::JVM_CONSTANT_ClassIndex);
        self.value_at_put(which, ConstantValue::JInt(name_index as u32));
    }

    pub fn field_at_put(&mut self, which: u16, class_index: u16, name_and_type_index: u16) {
        self.tag_at_put(which, ConstantTag::JVM_CONSTANT_Fieldref);
        self.value_at_put(
            which,
            ConstantValue::JInt((name_and_type_index as u32) << 16 | class_index as u32),
        )
    }

    pub fn method_at_put(&mut self, which: u16, class_index: u16, name_and_type_index: u16) {
        self.tag_at_put(which, ConstantTag::JVM_CONSTANT_Methodref);
        self.value_at_put(
            which,
            ConstantValue::JInt((name_and_type_index as u32) << 16 | class_index as u32),
        )
    }

    pub fn interface_method_at_put(
        &mut self,
        which: u16,
        class_index: u16,
        name_and_type_index: u16,
    ) {
        self.tag_at_put(which, ConstantTag::JVM_CONSTANT_InterfaceMethodref);
        self.value_at_put(
            which,
            ConstantValue::JInt((name_and_type_index as u32) << 16 | class_index as u32),
        )
    }

    pub fn string_index_at_put(&mut self, which: u16, string_index: u16) {
        self.tag_at_put(which, ConstantTag::JVM_CONSTANT_StringIndex);
        self.value_at_put(which, ConstantValue::JInt(string_index as u32))
    }

    pub fn method_handle_index_at_put(&mut self, which: u16, ref_kind: u8, ref_index: u16) {
        self.tag_at_put(which, ConstantTag::JVM_CONSTANT_MethodHandle);
        self.value_at_put(
            which,
            ConstantValue::JInt((ref_index as u32) << 16 | ref_kind as u32),
        )
    }

    pub fn method_type_index_at_put(&mut self, which: u16, ref_index: u16) {
        self.tag_at_put(which, ConstantTag::JVM_CONSTANT_MethodType);
        self.value_at_put(which, ConstantValue::JInt(ref_index as u32))
    }

    pub fn dynamic_constant_at_put(
        &mut self,
        which: u16,
        bsms_attribute_index: u16,
        name_and_type_index: u16,
    ) {
        self.tag_at_put(which, ConstantTag::JVM_CONSTANT_Dynamic);
        self.value_at_put(
            which,
            ConstantValue::JInt((name_and_type_index as u32) << 16 | bsms_attribute_index as u32),
        )
    }

    pub fn invoke_dynamic_at_put(
        &mut self,
        which: u16,
        bsms_attribute_index: u16,
        name_and_type_index: u16,
    ) {
        self.tag_at_put(which, ConstantTag::JVM_CONSTANT_InvokeDynamic);
        self.value_at_put(
            which,
            ConstantValue::JInt((name_and_type_index as u32) << 16 | bsms_attribute_index as u32),
        )
    }

    pub fn int_at_put(&mut self, which: u16, bytes: u32) {
        self.tag_at_put(which, ConstantTag::JVM_CONSTANT_Integer);
        self.value_at_put(which, ConstantValue::JInt(bytes))
    }

    pub fn float_at_put(&mut self, which: u16, bytes: u32) {
        self.tag_at_put(which, ConstantTag::JVM_CONSTANT_Float);
        self.value_at_put(which, ConstantValue::JFloat(f32::from_bits(bytes)))
    }

    pub fn long_at_put(&mut self, which: u16, bytes: u64) {
        self.tag_at_put(which, ConstantTag::JVM_CONSTANT_Long);
        self.value_at_put(
            which,
            ConstantValue::JLong(i64::from_be_bytes(bytes.to_be_bytes())),
        )
    }

    pub fn double_at_put(&mut self, which: u16, bytes: u64) {
        self.tag_at_put(which, ConstantTag::JVM_CONSTANT_Double);
        self.value_at_put(which, ConstantValue::JDouble(f64::from_bits(bytes)));
    }

    pub fn name_and_type_at_put(&mut self, which: u16, name_index: u16, signature_index: u16) {
        self.tag_at_put(which, ConstantTag::JVM_CONSTANT_NameAndType);
        self.value_at_put(
            which,
            ConstantValue::JInt((signature_index as u32) << 16 | name_index as u32),
        )
    }

    pub fn symbol_at_put(&mut self, which: u16, symbol: Arc<Symbol>) {
        self.tag_at_put(which, ConstantTag::JVM_CONSTANT_Utf8);
        self.value_at_put(which, ConstantValue::Symbol(symbol))
    }

    pub fn unresolved_klass_at_put(
        &mut self,
        which: u16,
        name_index: u16,
        resolved_klass_index: u16,
    ) {
        self.tag_at_put(which, ConstantTag::JVM_CONSTANT_UnresolvedClass);
        self.value_at_put(
            which,
            ConstantValue::JInt((resolved_klass_index as u32) << 16 | name_index as u32),
        )
    }

    pub fn unresolved_string_at_put(&mut self, which: u16, symbol: Arc<Symbol>) {
        self.tag_at_put(which, ConstantTag::JVM_CONSTANT_String);
        self.value_at_put(which, ConstantValue::Symbol(symbol));
    }

    pub fn tag_at_put(&mut self, which: u16, tag: ConstantTag) {
        if let Some(element) = self.tags.get_mut(which as usize) {
            *element = tag
        }
    }

    pub fn value_at_put(&mut self, which: u16, value: ConstantValue) {
        if let Some(element) = self.values.get_mut(which as usize) {
            *element = value
        }
    }

    pub fn klass_name_at(&self, which: u16) -> Arc<Symbol> {
        self.symbol_at(self.klass_slot_at(which).0)
    }

    pub fn symbol_at(&self, which: u16) -> Arc<Symbol> {
        match self.values.get(which as usize) {
            Some(ConstantValue::Symbol(symbol)) => symbol.clone(),
            _ => panic!("invalid symbol index in constant pool"),
        }
    }

    pub fn klass_slot_at(&self, which: u16) -> (u16, u16) {
        match self.values.get(which as usize) {
            Some(ConstantValue::JInt(value)) => (ConstantPool::extract_low_u16(value),
                                                 ConstantPool::extract_high_u16(value)),
            _ => panic!("invalid data in constant pool"),
        }
    }

    pub fn tag_at(&self, which: u16) -> &ConstantTag {
        unsafe { self.tags.get_unchecked(which as usize) }
    }

    pub fn klass_ref_index_at(&self, which: u16) -> u16 {
        match self.values.get(which as usize) {
            Some(ConstantValue::JInt(value)) => ConstantPool::extract_low_u16(value),
            _ => panic!("Invalid klass index."),
        }
    }

    pub fn name_and_type_ref_index_at(&self, which: u16) -> u16 {
        match self.values.get(which as usize) {
            Some(ConstantValue::JInt(value)) => ConstantPool::extract_high_u16(value),
            _ => panic!("Invalid name and type index."),
        }
    }

    pub fn name_ref_index_at(&self, which: u16) -> u16 {
        match self.values.get(which as usize) {
            Some(ConstantValue::JInt(value)) => ConstantPool::extract_low_u16(value),
            _ => panic!("Invalid name ref index {}.", which),
        }
    }

    pub fn signature_ref_index_at(&self, which: u16) -> u16 {
        match self.values.get(which as usize) {
            Some(ConstantValue::JInt(value)) => ConstantPool::extract_high_u16(value),
            _ => panic!("Invalid signature ref index {}.", which),
        }
    }

    pub fn klass_index_at(&self, which: u16) -> u16 {
        match self.values.get(which as usize) {
            Some(ConstantValue::JInt(value)) => ConstantPool::extract_low_u16(value),
            _ => panic!("Invalid klass index {}.", which),
        }
    }

    pub fn string_index_at(&self, which: u16) -> u16 {
        match self.values.get(which as usize) {
            Some(ConstantValue::JInt(value)) => ConstantPool::extract_low_u16(value),
            _ => panic!("Invalid string index {}.", which),
        }
    }

    pub fn method_handle_index_at(&self, which: u16) -> u16 {
        match self.values.get(which as usize) {
            Some(ConstantValue::JInt(value)) => ConstantPool::extract_high_u16(value),
            _ => panic!("Invalid method handle index {}.", which),
        }
    }

    pub fn method_handle_ref_kind_at(&self, which: u16) -> u16 {
        match self.values.get(which as usize) {
            Some(ConstantValue::JInt(value)) => ConstantPool::extract_low_u16(value),
            _ => panic!("Invalid method handle ref kind index {}.", which),
        }
    }

    pub fn method_type_index_at(&self, which: u16) -> u16 {
        match self.values.get(which as usize) {
            Some(ConstantValue::JInt(value)) => ConstantPool::extract_low_u16(value),
            _ => panic!("Invalid method type index {}.", which),
        }
    }

    pub fn bootstrap_name_and_type_ref_index_at(&self, which: u16) -> u16 {
        match self.values.get(which as usize) {
            Some(ConstantValue::JInt(value)) => ConstantPool::extract_high_u16(value),
            _ => panic!("Invalid bootstrap name and type ref index {}.", which),
        }
    }

    pub fn resolved_klass_at(&self, which: u16) -> &Klass {
        assert!(self.tag_at(which).is_klass(), "Corrupted constant pool.");
        let resolved_klass_index = self.resolved_klass_index_at(which as u16);
        match self.resolved_klasses.get(resolved_klass_index as usize) {
            Some(klass) => klass,
            None => panic!("Invalid klass index {}.", resolved_klass_index),
        }
    }

    pub fn resolved_klass_index_at(&self, which: u16) -> u16 {
        match self.values.get(which as usize) {
            Some(ConstantValue::JInt(value)) => ConstantPool::extract_high_u16(value),
            _ => panic!("Invalid klass index {}.", which),
        }
    }

    pub fn allocate_resolved_klasses(&mut self, num_klasses: u16) {
        self.resolved_klasses = vec![Klass::default(); num_klasses as usize]
    }

    pub fn extract_low_u16(value: &u32) -> u16 {
        (value.clone() & 0x0000ffff) as u16
    }

    pub fn extract_high_u16(&value: &u32) -> u16 {
        (value.clone() >> 16) as u16
    }
}

#[cfg(test)]
mod tests {
    use super::ConstantPool;

    #[test]
    fn we_can_extract_low_u16_from_u32() {
        let value = 0x0101f342u32;
        let result = ConstantPool::extract_low_u16(&value);
        assert_eq!(result, 0xf342u16)
    }

    #[test]
    fn we_can_extract_high_u16_from_u32() {
        let value = 0x0101f342u32;
        let result = ConstantPool::extract_high_u16(&value);
        assert_eq!(result, 0x0101u16)
    }
}
