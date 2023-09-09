//! ELF program header metadata.
//! Parsing, analysis and modification of the ELF program header.



mod flags;
mod programtype;



pub use flags::Flags;
pub use programtype::ProgramType;

use crate::common::address::Address;



/// A common program header structure. Will be instantiated by each implementator.
#[derive(Debug)]
pub struct ProgramHeader {
    /// Type of program header.
    pub(super) programtype: ProgramType,

    /// Program flags.
    pub(super) flags: Flags,

    /// Offset in the file.
    pub(super) offset: Address,

    /// Virtual load address.
    pub(super) vaddr: Address,

    /// Physical load address.
    pub(super) paddr: Address,

    /// Size in bytes of the segment in the file.
    pub(super) filesize: Address,

    /// Size in bytes of the segment in memory.
    pub(super) memsize: Address,

    /// Alignment of the segment in memory.
    pub(super) alignment: Address,
}

impl ProgramHeader {
    /// Parses the given slice of data into an ELF file header.
    pub fn parse<R: AsRef<[u8]>, const INC: usize>(raw: R, read: fn(&[u8]) -> Address, read32: fn(&[u8]) -> u32) -> Result<Self, ()> {
        // Header size constant.
        let hsize: usize = 8 + (6 * INC);

        // Deref the slice.
        let raw = raw.as_ref();

        // Check there is minimum length.
        if raw.len() < hsize {
            return Err(());
        }

        // Get the program type.
        let programtype = ProgramType::from( read32(raw) );

        // Create the empty flags.
        let mut flags = Flags::from(0);

        // Create the dynamic index.
        let mut i = 0x04;

        if INC == 8 {
            flags = Flags::from( read32( &raw[i..i+4] ) );
            i += 4;
        }

        // Read the offset.
        let offset = read( &raw[i..i+INC] );
        i += INC;

        // Read the virtual address.
        let vaddr = read( &raw[i..i+INC] );
        i += INC;

        // Read the virtual address.
        let paddr = read( &raw[i..i+INC] );
        i += INC;

        // Read the virtual address.
        let filesize = read( &raw[i..i+INC] );
        i += INC;

        // Read the virtual address.
        let memsize = read( &raw[i..i+INC] );
        i += INC;

        if INC == 4 {
            flags = Flags::from( read32( &raw[i..i+4] ) );
            i += 4;
        }

        // Read the alignment.
        let alignment = read( &raw[i..i+INC] );
        i += INC;

        assert_eq!(i, hsize);

        Ok(Self {
            programtype,
            flags,
            offset,
            vaddr,
            paddr,
            filesize,
            memsize,
            alignment,
        })
    }
}

impl ProgramHeader {
    /// Creates a pretty print of the segment's information.
    pub fn prettyprint(&self) -> String {
        // Create output string.
        let mut args = String::new();

        // Section name.
        args += &format!("Program:\n");

        // Section type.
        args += &format!("  Program type: {}\n", self.programtype);

        // Program flags.
        args += &format!("  - Flags: {}\n", self.flags);

        // Program offset in file.
        args += &format!("  - Address: Physical: 0x{:X} | Virtual: 0x{:X}\n", self.paddr, self.vaddr);
        args += &format!("  - Address: File: 0x{:X}\n", self.offset);

        // Program and memory size in file and memory.
        args += &format!("  - File size: {} Bytes\n", self.filesize);
        args += &format!("  - Mem size : {} Bytes\n", self.memsize);

        // Alignment of the section.
        args += &format!("  - Alignment: 2 << {} Bytes\n", self.alignment);

        args
    }
}
