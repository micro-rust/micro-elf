//! Address multitype enum used to bypass the disgusting state of Rust `dyn`.
//! 



use byteorder::ByteOrder;



#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Address {
    /// `u32` address type.
    U32(u32),

    /// `u64` address type.
    U64(u64),
}

impl Address {
    /// Returns the number of bits in the address.
    pub fn bits(&self) -> usize {
        match self {
            Address::U32(_) => 32,
            Address::U64(_) => 64,
        }
    }

    /// Returns the number of bytes in the address.
    pub fn bytes(&self) -> usize {
        match self {
            Address::U32(_) => 4,
            Address::U64(_) => 8,
        }
    }
}

impl core::convert::From<Address> for usize {
    fn from(a: Address) -> usize {
        match a {
            Address::U32(u) => u as usize,
            Address::U64(u) => u as usize,
        }
    }
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



/// Internal function to read a `u32` as an address.
pub fn read32<B: ByteOrder>(buf: &[u8]) -> Address {
    Address::U32( B::read_u32(buf) )
}



/// Internal function to read a `u64` as an address.
pub fn read64<B: ByteOrder>(buf: &[u8]) -> Address {
    Address::U64( B::read_u64(buf) )
}
