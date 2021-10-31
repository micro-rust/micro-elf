//! Program flags module.
//! Contains all information for an ELF program's flags.


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProgramFlags32(pub(self) u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProgramFlags64(pub(self) u64);


impl core::convert::From<u32> for ProgramFlags32 {
    #[inline(always)]
    fn from(x: u32) -> ProgramFlags32 {
        ProgramFlags32(x)
    }
}

impl core::fmt::Display for ProgramFlags32 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "0x{:08X}", self.0)
    }
}