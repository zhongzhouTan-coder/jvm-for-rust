//! HeapObj is basic obj structure for heap object
//!
//! Contains a header has info that can be used for garbage collector
//!
//! # Header structure
//! - First Bit represents whether this obj is a marked in a gc marking phase
//! - Second Bit represents whether this obj is a small or medium obj
//! - Third Bit represents whether this obj is pinned, 0 for unpinned and 1 for pinned
//! - Forth Bit represents whether this obj is a forwarding pointer in a gc sweeping phase
//! - Remaining Bits are not used
//!
//! # forwarding_pointer
//! The address of the forwarding pointer that the object moved to and any reference to this location
//! will be replaced with this forwarding pointer address.
use super::address::Address;
pub struct HeapObj {
    header: u8,
    forwarding_pointer: Address,
}

impl HeapObj {
    fn is_mark(&self) -> bool {
        self.header & 0b0000_0001 == 1
    }

    fn set_mark(&mut self) {
        self.header |= 0b0000_0001;
    }

    fn is_medium(&self) -> bool {
        self.header & 0b0000_0010 == 1
    }

    fn set_medium(&mut self) {
        self.header |= 0b0000_0010;
    }

    fn is_forwarding(&self) -> bool {
        self.header & 0b0000_1000 == 1
    }

    fn set_forwarding(&mut self) {
        self.header |= 0b0000_1000;
    }

    fn get_forwarding_pointer(&self) -> Address {
        self.forwarding_pointer
    }

    fn set_forwarding_pointer(&mut self, addr: Address) {
        self.forwarding_pointer = addr;
    }
}
