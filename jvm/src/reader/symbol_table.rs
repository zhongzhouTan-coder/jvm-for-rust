use std::{
    collections::HashMap,
    sync::{Arc, Mutex, RwLock},
};

use once_cell::sync::OnceCell;

use super::symbol::Symbol;

type SymbolKey = Vec<u8>;

pub struct SymbolTable {
    shared_table: HashMap<SymbolKey, Arc<Symbol>>,
    dynamic_table: HashMap<SymbolKey, Arc<Symbol>>,
    local_table: HashMap<SymbolKey, Arc<Symbol>>,
    look_up_shared_first: bool,
}

static mut INSTANCE: OnceCell<Mutex<SymbolTable>> = OnceCell::new();

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            shared_table: HashMap::new(),
            dynamic_table: HashMap::new(),
            local_table: HashMap::new(),
            look_up_shared_first: true,
        }
    }

    pub fn global() -> &'static Self {
        unsafe {}
    }

    pub fn new_symbol(&mut self, key: SymbolKey, value: Symbol) {
        self.local_table.insert(key, Arc::new(value));
    }

    pub fn lookup_only(&self, name: &SymbolKey) -> Option<Arc<Symbol>> {
        match self.look_up_shared_first {
            true => self
                .lookup_shared(name)
                .map_or_else(|| self.lookup_dynamic(name), |symbol| Some(symbol)),
            false => self
                .lookup_dynamic(name)
                .map_or_else(|| self.lookup_shared(name), |symbol| Some(symbol)),
        }
    }

    pub fn lookup_shared(&self, name: &SymbolKey) -> Option<Arc<Symbol>> {
        self.dynamic_table
            .get(name)
            .map_or_else(|| None, |symbol| Some(symbol.clone()))
    }

    pub fn lookup_dynamic(&self, name: &SymbolKey) -> Option<Arc<Symbol>> {
        self.dynamic_table
            .get(name)
            .map_or_else(|| None, |symbol| Some(symbol.clone()))
    }
}
