//! Address multitype enum used to bypass the disgusting state of Rust `dyn`.
//! 




#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Address {
    /// `u32` address type.
    U32(pub u32),

    /// `u64` address type.
    U64(pub u64),
}

impl core::fmt::Display for Address {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Address::U32(u) => write!(f, "{}", u),
            Address::U64(u) => write!(f, "{}", u),
        }
    }
}

impl core::fmt::UpperHex for Address {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Address::U32(u) => write!(f, "0x{:08X}", u),
            Address::U64(u) => write!(f, "0x{:016X}", u),
        }
    }
}
