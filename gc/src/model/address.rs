#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug)]
pub struct Address(usize);

impl Address {
    pub fn new(address: usize) -> Address {
        Address(address)
    }

    #[inline(always)]
    pub fn plus(&self, offset: usize) -> Address {
        Address(self.0 + offset)
    }

    #[inline(always)]
    pub fn minus(&self, offset: usize) -> Address {
        Address(self.0 - offset)
    }

    #[inline(always)]
    pub fn diff(&self, other: Address) -> usize {
        self.0 - other.0
    }

    #[inline(always)]
    pub fn to_usize(&self) -> usize {
        self.0
    }

    #[inline(always)]
    pub fn store<T>(&self, value: T) {
        unsafe {
            std::ptr::write(self.0 as *mut T, value);
        }
    }

    #[inline(always)]
    pub fn load<T>(&self) -> T {
        unsafe { std::ptr::read(self.0 as *const T) }
    }

    #[inline(always)]
    pub fn zero() -> Address {
        Address(0)
    }

    #[inline(always)]
    pub fn is_null(&self) -> bool {
        self.0 == 0
    }
}

impl std::ops::Add<usize> for Address {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        Address(self.0 + rhs)
    }
}
