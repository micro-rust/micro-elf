//! ELF file header metadata.
//! Parsing, analysis and modification of the ELF file header.



mod abi;
mod arch;
mod filetype;



pub use abi::TargetOS;
pub use arch::Architecture;
pub use endianness::Endianness;
pub use filetype::FileType;

use crate::common::address::Address;



/// A common file header structure. Will be instantiated by each implementator.
#[derive(Debug)]
pub struct FileHeader {
    /// Endianness of the target hardware.
    pub(super) endianness: Endianness,

    /// Target OS ABI.
    pub(super) targetos: TargetOS,

    /// Object file type.
    pub(super) filetype: FileType,

    /// Target architecture.
    pub(super) architecture: Architecture,

    /// Entry point of the program.
    pub(super) entry: Address,

    /// Offset of the Program Header Table.
    pub(super) phtoffset: Address,

    /// Offset of the Section Header Table.
    pub(super) shtoffset: Address,

    /// Architecture flags.
    pub(super) flags: u32,

    /// Size of a Program Header Table entry.
    pub(super) phtesize: u16,

    /// Number of Program Header Table entries.
    pub(super) phnum: u16,

    /// Size of a Section Header Table entry.
    pub(super) shtesize: u16,

    /// Number of Section Header Table entries.
    pub(super) shnum: u16,

    /// Index of the Section Header Table entry of the section containing section names.
    pub(super) shstrndx: u16,
}

impl FileHeader {
    /// Parses the given slice of data into an ELF file header.
    pub fn parse<R: AsRef<[u8]>, const INC: usize>(raw: R, read: fn(&[u8]) -> Address) -> Result<Self, ()> {
        // Header size constant.
        let hsize: usize = 40 + (3 * INC);

        // Deref the slice.
        let raw = raw.as_ref();

        // Check there is minimum length.
        if raw.len() < hsize {
            return Err(());
        }

        // Validate the magic number.
        match raw[0x00..0x04] {
            [0x7F, 0x45, 0x4C, 0x46] => (),
            _ => return Err( () ),
        }

        // Get the endianness.
        let endianness = match raw[0x05] {
            1 => Endianness::Little,
            2 => Endianness::Big,

            _ => return Err( () ),
        };

        // Function to read a 16 bit integer.
        let read16: fn(&[u8]) -> u16 = match endianness {
            Endianness::Big    => crate::common::bytes::read16::<byteorder::BigEndian>,
            Endianness::Little => crate::common::bytes::read16::<byteorder::LittleEndian>,
        };

        // Function to read a 32 bit integer.
        let read32: fn(&[u8]) -> u32 = match endianness {
            Endianness::Big    => crate::common::bytes::read32::<byteorder::BigEndian>,
            Endianness::Little => crate::common::bytes::read32::<byteorder::LittleEndian>,
        };

        // Get the target OS ABI.
        let targetos = TargetOS::from( (raw[0x07], raw[0x08]) );

        // Get the ELF type.
        let filetype = FileType::from( read16( &raw[0x10..0x12] ) );

        // Get the target architecture.
        let architecture = Architecture::from( read16( &raw[0x12..0x14] ) );

        // Begin non standard section.
        let mut i = 0x18;

        // Read entry point.
        let entry = read(&raw[i..i+INC]);
        i += INC;

        // Read the program header table offset.
        let phtoffset = read(&raw[i..i+INC]);
        i += INC;

        // Read the section header table offset.
        let shtoffset = read(&raw[i..i+INC]);
        i += INC;

        // Read the flags.
        let flags = read32(&raw[i..i+4]);
        i += 4;

        // Read the size of this header.
        if read16(&raw[i..i+2]) as usize != hsize {
            return Err( () );
        }
        i += 2;

        // Read the size of program header entries.
        let phtesize = read16( &raw[i..i+2] );
        i += 2;

        // Read the number of program headers.
        let phnum = read16( &raw[i..i+2] );
        i += 2;

        // Read the size of section header entries.
        let shtesize = read16( &raw[i..i+2] );
        i += 2;

        // Read the number of section headers.
        let shnum = read16( &raw[i..i+2] );
        i += 2;

        // Read the section name section index.
        let shstrndx = read16(&raw[i..i+2]);
        i += 2;

        assert_eq!(i, hsize);

        Ok(Self {
            endianness,
            targetos,
            filetype,
            architecture,
            entry,
            phtoffset,
            shtoffset,
            flags,
            phtesize,
            phnum,
            shtesize,
            shnum,
            shstrndx,
        })
    }

    /// Returns the target architecture.
    pub const fn architecture(&self)  -> Architecture {
        self.architecture
    }

    /// Returns the target endianness.
    pub const fn endianness(&self)  -> Endianness {
        self.endianness
    }

    /// Returns the target OS.
    pub const fn os(&self)  -> TargetOS {
        self.targetos
    }
}

impl FileHeader {
    /// Creates a pretty print string.
    pub fn prettyprint(&self) -> String {
        // Create the string.
        let mut string = String::new();

        // Add the header.
        string += &format!("ELF {} file header\n", self.entry.bits());

        // Add the target endiannessm, OS and architecture.
        string += &format!("  - Endianness: {:?}\n  - {}\n  - {}\n", self.endianness, self.targetos, self.architecture);

        // Add the file type.
        string += &format!("  - {}\n", self.filetype);

        string += &format!("  - Flags: 0x{:X}\n", self.flags);

        // Add the entry point.
        string += &format!("  - Entry: 0x{:X}\n", self.entry);

        // Add the program and section header table.
        string += &format!("  - Program Header Table\n    · Offset: {}\n    · {} entries\n    · {} bytes per entry\n", self.phtoffset, self.phnum, self.phtesize);
        string += &format!("  - Section Header Table\n    · Offset: {}\n    · {} entries\n    · {} bytes per entry\n", self.shtoffset, self.shnum, self.shtesize);

        // Add the section header index with section names.
        string += &format!("  - Section header string index: {}\n", self.shstrndx);

        string
    }
}
