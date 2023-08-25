//! ELF program header metadata.
//! Parsing, analysis and modification of the ELF program header.



mod flags;
mod programtype;



pub use flags::Flags;
pub use programtype::ProgramType;



/// A common program header structure. Will be instantiated by each implementator.
pub struct ProgramHeader<T: core::convert::TryInto<usize> + Sized> {
    /// Type of program header.
    pub(super) programtype: ProgramType,

    /// Program flags.
    pub(super) flags: Flags,

    /// Offset in the file.
    pub(super) offset: T,

    /// Virtual load address.
    pub(super) vaddr: T,

    /// Physical load address.
    pub(super) paddr: T,

    /// Size in bytes of the segment in the file.
    pub(super) filesize: T,

    /// Size in bytes of the segment in memory.
    pub(super) memsize: T,

    /// Alignment of the segment in memory.
    pub(super) alignment: T,
}

impl<T: core::convert::TryInto<usize> + Sized> ProgramHeader<T> {
    // Byte size of the type for increment.
    const INC: usize = core::mem::size_of::<T>();

    // Size of the header depending on the inner type.
    const HSIZE: usize = 8 + (6 * core::mem::size_of::<T>());

    /// Parses the given slice of data into an ELF file header.
    pub fn parse<R: AsRef<[u8]>>(raw: R, read: fn(&[u8]) -> T, read32: fn(&[u8]) -> u32) -> Result<Self, ()> {
        // Deref the slice.
        let raw = raw.as_ref();

        // Check there is minimum length.
        if raw.len() < Self::HSIZE {
            return Err(());
        }

        // Get the program type.
        let programtype = ProgramType::from( read32(raw) );

        // Create the empty flags.
        let mut flags = Flags::from(0);

        // Create the dynamic index.
        let mut i = 0x04;

        if core::mem::size_of::<T>() == 8 {
            flags = Flags::from( read32( &raw[i..i+4] ) );
            i += 4;
        }

        // Read the offset.
        let offset = read( &raw[i..i+Self::INC] );
        i += Self::INC;

        // Read the virtual address.
        let vaddr = read( &raw[i..i+Self::INC] );
        i += Self::INC;

        // Read the virtual address.
        let paddr = read( &raw[i..i+Self::INC] );
        i += Self::INC;

        // Read the virtual address.
        let filesize = read( &raw[i..i+Self::INC] );
        i += Self::INC;

        // Read the virtual address.
        let memsize = read( &raw[i..i+Self::INC] );
        i += Self::INC;

        if core::mem::size_of::<T>() == 4 {
            flags = Flags::from( read32( &raw[i..i+4] ) );
            i += 4;
        }

        // Read the alignment.
        let alignment = read( &raw[i..i+Self::INC] );
        i += Self::INC;

        assert_eq!(i, Self::HSIZE);

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

impl<T: core::convert::TryInto<usize> + Sized + core::fmt::Display + core::fmt::UpperHex> ProgramHeader<T> {
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
        args += &format!("  - Address: Physical: 0x{:08X} | Virtual: 0x{:08X}\n", self.paddr, self.vaddr);
        args += &format!("  - Address: File: 0x{:08X}\n", self.offset);

        // Program size in file and memory.
        args += &format!("  - File size: {} Bytes\n", self.filesize);

        // Alignment of the section.
        args += &format!("  - Alignment: 2 << {} Bytes\n", self.alignment);

        args
    }
}