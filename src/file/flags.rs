//! File flags module.
//! Contains all information for an ELF file's flags.


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FileFlags32(pub(self) u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FileFlags64(pub(self) u64);


impl core::convert::From<u32> for FileFlags32 {
    #[inline(always)]
    fn from(x: u32) -> FileFlags32 {
        FileFlags32(x)
    }
}

impl core::fmt::Display for FileFlags32 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "0x{:08X}", self.0)
    }
}
