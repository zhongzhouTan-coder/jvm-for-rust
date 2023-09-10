use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use once_cell::sync::Lazy;

use super::symbol::Symbol;

type SymbolKey = Vec<u8>;

pub struct SymbolTable {
    shared_table: HashMap<SymbolKey, Arc<Symbol>>,
    dynamic_table: HashMap<SymbolKey, Arc<Symbol>>,
    local_table: HashMap<SymbolKey, Arc<Symbol>>,
    look_up_shared_first: bool,
}

static INSTANCE: Lazy<Mutex<SymbolTable>> = Lazy::new(|| Mutex::new(SymbolTable::new()));

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            shared_table: HashMap::new(),
            dynamic_table: HashMap::new(),
            local_table: HashMap::new(),
            look_up_shared_first: true,
        }
    }

    pub fn new_symbol(key: SymbolKey, value: Symbol) -> Arc<Symbol> {
        let mut symbol_table = INSTANCE.lock().unwrap();
        let key_ref = &key.clone();
        symbol_table.local_table.insert(key, Arc::new(value));
        symbol_table.local_table.get(key_ref).unwrap().clone()
    }

    pub fn lookup_only(name: &SymbolKey) -> Option<Arc<Symbol>> {
        let symbol = INSTANCE.lock().unwrap();
        match symbol.look_up_shared_first {
            true => SymbolTable::lookup_shared(name)
                .map_or_else(|| SymbolTable::lookup_dynamic(name), |symbol| Some(symbol)),
            false => SymbolTable::lookup_dynamic(name)
                .map_or_else(|| SymbolTable::lookup_shared(name), |symbol| Some(symbol)),
        }
    }

    pub fn lookup_shared(name: &SymbolKey) -> Option<Arc<Symbol>> {
        let symbol_table: std::sync::MutexGuard<'_, SymbolTable> = INSTANCE.lock().unwrap();
        symbol_table
            .dynamic_table
            .get(name)
            .map_or_else(|| None, |symbol| Some(symbol.clone()))
    }

    pub fn lookup_dynamic(name: &SymbolKey) -> Option<Arc<Symbol>> {
        let symbol_table: std::sync::MutexGuard<'_, SymbolTable> = INSTANCE.lock().unwrap();
        symbol_table
            .dynamic_table
            .get(name)
            .map_or_else(|| None, |symbol| Some(symbol.clone()))
    }
}
