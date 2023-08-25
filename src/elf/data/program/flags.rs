//! Program flags module.
//! Contains all information for an ELF program's flags.



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Flags(pub(self) u32);

impl core::convert::From<u32> for Flags {
    #[inline(always)]
    fn from(x: u32) -> Flags {
        Flags(x)
    }
}

impl core::fmt::Display for Flags {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "0x{:08X}", self.0)
    }
}