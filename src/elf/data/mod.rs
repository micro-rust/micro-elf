//! ELF metadata.



pub mod header;
pub mod program;
pub mod section;



pub use header::FileHeader;
pub use program::ProgramHeader;
pub use section::SectionHeader;



pub struct ELFData {
    /// File header of an ELF object.
    pub(super) header: FileHeader,

    /// List of program headers.
    pub(super) programs: Vec<ProgramHeader>,

    /// List of section headers.
    pub(super) sections: Vec<SectionHeader>,
}

impl ELFData {
    pub fn parse(raw: &[u8]) -> Result<Self, ()> {
        // Get the read function.
        let read: fn(&[u8]) -> crate::common::address::Address = match (raw[0x04], raw[0x05]) {
            (1, 1) => crate::common::address::read32::<byteorder::LittleEndian>,
            (1, 2) => crate::common::address::read32::<byteorder::BigEndian>,

            (2, 1) => crate::common::address::read64::<byteorder::LittleEndian>,
            (2, 2) => crate::common::address::read64::<byteorder::BigEndian>,

            _ => return Err( () ),
        };

        // Get the read 32 bit function.
        let read32: fn(&[u8]) -> u32 = match raw[0x05] {
            1 => crate::common::bytes::read32::<byteorder::LittleEndian>,
            2 => crate::common::bytes::read32::<byteorder::BigEndian>,

            _ => return Err( () ),
        };

        // Get the adequate File Header parse function.
        let fparse: fn(_, _) -> Result<FileHeader, _> = match raw[0x04] {
            1 => FileHeader::parse::<_, 4>,
            2 => FileHeader::parse::<_, 8>,

            _ => return Err(()),
        };

        // Parse the header.
        let header = fparse(raw, read)?;

        // Create the program list.
        let mut programs = Vec::new();

        // Get the adequate Program Header parse function.
        let pparse: fn(_, _, _) -> Result<ProgramHeader, _> = match raw[0x04] {
            1 => ProgramHeader::parse::<_, 4>,
            2 => ProgramHeader::parse::<_, 8>,

            _ => return Err(()),
        };

        for chunk in Self::chunks(raw, header.phtoffset, header.phnum, header.phtesize) {
            programs.push( pparse(chunk, read, read32)? );
        }
    
        // Create the section list.
        let mut sections = Vec::new();

        // Get the adequate Section Header parse function.
        let sparse: fn(_, _, _) -> Result<SectionHeader, _> = match raw[0x04] {
            1 => SectionHeader::parse::<_, 4>,
            2 => SectionHeader::parse::<_, 8>,

            _ => return Err(()),
        };

        for chunk in Self::chunks(raw, header.shtoffset, header.shnum, header.shtesize) {
            sections.push( sparse(chunk, read, read32)? );
        }

        // Check if the .shstrtab section is present.
        if sections.len() < header.shstrndx as usize {
            return Err(());
        }

        // Load the names of all the sections.
        {
            // Get the offset and size of the .shstrtab section.
            let offset   = usize::from( sections[ usize::from(header.shstrndx) ].offset );
            let filesize = usize::from( sections[ usize::from(header.shstrndx) ].filesize );

            // Get the range of raw data of this section.
            let names = &raw[offset..offset+filesize];

            // Rename all the sections.
            for section in &mut sections {
                section.rename( names );
            }
        }

        Ok(Self {
            header,
            programs,
            sections,
        })
    }

    /// Internal function to create chunk iterators over the tables of the file.
    fn chunks(raw: &[u8], offset: crate::common::address::Address, num: u16, size: u16) -> core::slice::ChunksExact<u8> {
        // Calculate the start and end.
        let start = usize::from(offset);
        let end = usize::from(offset) + (num as usize * size as usize);

        raw[start..end].chunks_exact(size as usize)
    }
}

impl core::fmt::Display for ELFData {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        // Create the argument string.
        let mut args = String::new();

        // Set the header.
        args += "ELF Object 32-bit\n";

        // Add the header.
        args += &format!("{}", self.header.prettyprint());

        // Add the sections.
        for program in &self.programs {
            args += &format!("\n{}", program.prettyprint());
        }

        for section in &self.sections {
            args += &format!("\n{}", section.prettyprint());
        }

        write!(f, "{}", args)
    }
}
