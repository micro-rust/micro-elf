//! ELF metadata.



pub mod header;
pub mod program;
pub mod section;



pub use header::FileHeader;
pub use program::ProgramHeader;
pub use section::SectionHeader;



/// ELF metadata trait. Used to implement dynamic dispatch when analyzing ELFs.
pub trait Metadata: core::fmt::Display {
    
}


pub struct ELF32 {
    /// File header of an ELF object.
    header: FileHeader<u32>,

    /// List of program headers.
    programs: Vec<ProgramHeader<u32>>,

    /// List of section headers.
    sections: Vec<SectionHeader<u32>>,
}

impl ELF32 {
    pub fn parse(raw: &[u8]) -> Result<Self, ()> {
        // Get the read function.
        let read: fn(&[u8]) -> u32 = match raw[0x05] {
            1 => crate::common::bytes::read32::<byteorder::LittleEndian>,
            2 => crate::common::bytes::read32::<byteorder::BigEndian>,

            _ => return Err( () ),
        };

        // Parse the header.
        let header = FileHeader::parse(raw, read)?;

        // Create the program list.
        let mut programs = Vec::new();

        for chunk in Self::chunks(raw, header.phtoffset, header.phnum, header.phtesize) {
            programs.push( ProgramHeader::parse(chunk, read, read)? );
        }
    
        // Create the section list.
        let mut sections = Vec::new();

        for chunk in Self::chunks(raw, header.shtoffset, header.shnum, header.shtesize) {
            sections.push( SectionHeader::parse(chunk, read, read)? );
        }

        // Check if the .shstrtab section is present.
        if sections.len() < header.shstrndx as usize {
            return Err(());
        }

        // Load the names of all the sections.
        {
            // Get the offset and size of the .shstrtab section.
            let offset   = sections[header.shstrndx as usize].offset as usize;
            let filesize = sections[header.shstrndx as usize].filesize as usize;

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
    fn chunks(raw: &[u8], offset: u32, num: u16, size: u16) -> core::slice::ChunksExact<u8> {
        // Calculate the start and end.
        let start = offset as usize;
        let end = offset as usize + (num as usize * size as usize);

        raw[start..end].chunks_exact(size as usize)
    }
}

impl core::fmt::Display for ELF32 {
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

impl Metadata for ELF32 {
    
}