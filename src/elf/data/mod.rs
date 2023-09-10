//! ELF metadata.



pub mod header;
pub mod program;
pub mod section;
pub mod symbol;



pub use header::FileHeader;
pub use program::ProgramHeader;
pub use section::SectionHeader;
pub use symbol::Symbol;

use std::sync::Arc;



#[derive(Debug)]
pub struct ELFData {
    /// File header of an ELF object.
    pub(super) header: Arc<FileHeader>,

    /// List of program headers.
    pub(super) programs: Vec<Arc<ProgramHeader>>,

    /// List of section headers.
    pub(super) sections: Vec<Arc<SectionHeader>>,

    /// List of symbols.
    pub(super) symbols: Vec<Arc<Symbol>>,
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

        // Get the read 16 bit function.
        let read16: fn(&[u8]) -> u16 = match raw[0x05] {
            1 => crate::common::bytes::read16::<byteorder::LittleEndian>,
            2 => crate::common::bytes::read16::<byteorder::BigEndian>,

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
        let header = Arc::new( fparse(raw, read)? );

        // Create the program list.
        let mut programs = Vec::new();

        // Get the adequate Program Header parse function.
        let pparse: fn(_, _, _) -> Result<ProgramHeader, _> = match raw[0x04] {
            1 => ProgramHeader::parse::<_, 4>,
            2 => ProgramHeader::parse::<_, 8>,

            _ => return Err(()),
        };

        for chunk in Self::chunks(raw, header.phtoffset, header.phnum, header.phtesize) {
            programs.push( Arc::new( pparse(chunk, read, read32)? ) );
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

        // Put all sections in ARC.
        let sections: Vec<Arc<SectionHeader>> = sections.into_iter()
            .map(|x| Arc::new(x))
            .collect();

        // Get the symbol table section contents.
        let symtab = match sections.iter().find(|section| &section.name == ".symtab") {
            Some(section) => section,
            _ => return Err( () ),
        };

        // Create the symbol list.
        let mut symbols = Vec::new();

        // Get the adequate Section Header parse function.
        let sparse: fn(_, _, _, _) -> Result<Symbol, _> = match raw[0x04] {
            1 => Symbol::parse::<_, 4>,
            2 => Symbol::parse::<_, 8>,

            _ => return Err(()),
        };

        for chunk in Self::chunks(raw, symtab.offset, usize::from(symtab.filesize) / usize::from(symtab.entrysize), symtab.entrysize) {
            symbols.push( sparse(chunk, read, read16, read32)? );
        }

        // Load the names of all the symbols.
        {
            // Get the string table section contents.
            let header = match sections.iter().find(|section| &section.name == ".strtab") {
                Some(section) => section,
                _ => return Err( () ),
            };

            // Get the offset and size of the .strtab section.
            let offset   = usize::from( header.offset );
            let filesize = usize::from( header.filesize );

            // Get the range of raw data of this section.
            let names = &raw[offset..offset+filesize];

            // Rename all the sections.
            for symbol in &mut symbols {
                symbol.rename( names );
            }
        }

        // Put all symbols in ARC.
        let symbols = symbols.into_iter()
            .map(|x| Arc::new(x))
            .collect();

        Ok(Self {
            header,
            programs,
            sections,
            symbols,
        })
    }

    /// Internal function to create chunk iterators over the tables of the file.
    fn chunks<O: Copy, N: Copy, S: Copy>(raw: &[u8], offset: O, num: N, size: S) -> core::slice::ChunksExact<u8> where usize: From<O> + From<N> + From<S> {
        // Calculate the start and end.
        let start = usize::from(offset);
        let end = usize::from(offset) + (usize::from(num) * usize::from(size));

        raw[start..end].chunks_exact(usize::from(size))
    }

    /// Returns the endianness of the ELF object.
    pub fn endianness(&self) -> endianness::Endianness {
        self.header.endianness
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

        for symbol in &self.symbols {
            args += &format!("\n{}", symbol.prettyprint());
        }

        write!(f, "{}", args)
    }
}



/// Common trait for all items that can contain data in the file image.
pub trait HasContent {
    /// Is the item a program.
    const PROGRAM: bool;

    /// Is the item a section.
    const SECTION: bool;

    /// Is the item a symbol.
    const SYMBOL: bool;

    /// Returns the size in the file image.
    fn size(&self) -> usize;

    /// Returns the offset into the file image.
    fn offset(&self) -> usize;
}
