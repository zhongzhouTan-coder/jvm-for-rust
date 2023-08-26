pub struct ModuleAttribute {
    module_name_index: u16,
    module_flags: u16,
    module_version_index: u16,
    requires_count: u16,
    requires: Vec<Require>,
    exports_count: u16,
    exports: Vec<Export>,
    opens_count: u16,
    opens: Vec<Open>,
    uses_count: u16,
    uses_index: Vec<u16>,
    provides_count: u16,
    provides: Vec<Provide>,
}

pub struct Require {
    requires_index: u16,
    requires_flags: u16,
    requires_version_index: u16,
}

pub struct Export {
    exports_index: u16,
    exports_flags: u16,
    exports_to_count: u16,
    exports_to_index: Vec<u16>,
}

pub struct Open {
    opens_index: u16,
    opens_flags: u16,
    opens_to_count: u16,
    opens_to_index: Vec<u16>,
}

pub struct Provide {
    provides_index: u16,
    provides_with_count: u16,
    provides_with_index: Vec<u16>,
}
