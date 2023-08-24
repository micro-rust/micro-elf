//! Section flags module.
//! Contains all information for an ELF section's flags.


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SectionFlags32(pub(self) u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SectionFlags64(pub(self) u64);


impl core::convert::From<u32> for SectionFlags32 {
    #[inline(always)]
    fn from(x: u32) -> SectionFlags32 {
        SectionFlags32(x)
    }
}

impl core::fmt::Display for SectionFlags32 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let mut args = String::new();

        if self.0 & 0x001 == 0x001 {
            args += "WRITE ";
        }

        if self.0 & 0x002 == 0x002 {
            args += "ALLOC ";
        }

        if self.0 & 0x004 == 0x004 {
            args += "EXEC ";
        }

        if self.0 & 0x010 == 0x010 {
            args += "MERGE ";
        }

        if self.0 & 0x020 == 0x020 {
            args += "STRINGS ";
        }

        if self.0 & 0x040 == 0x040 {
            args += "LINKINFO ";
        }

        if self.0 & 0x080 == 0x080 {
            args += "LINKORDER ";
        }

        if self.0 & 0x100 == 0x100 {
            args += "NONCONFORMING ";
        }

        if self.0 & 0x200 == 0x200 {
            args += "GROUP ";
        }

        if self.0 & 0x400 == 0x400 {
            args += "TLS ";
        }

        if self.0 & 0x0FF00000 == 0x0FF00000 {
            args += "MASKOS ";
        }

        if self.0 & 0xF0000000 == 0xF0000000 {
            args += "MASKPROC ";
        }

        write!(f, "{}", args)
    }
}
