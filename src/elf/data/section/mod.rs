//! ELF section header metadata.
//! Parsing, analysis and modification of the ELF section header.



mod id;
mod sectiontype;



pub use id::SectionID;
pub use sectiontype::SectionType;

use crate::common::address::Address;



/// A common section header structure. Will be instantiated by each implementator.
#[derive(Debug)]
pub struct SectionHeader {
    /// Section name.
    pub(super) name: String,

    /// Section name offset in the .shstrtab section.
    pub(super) nameidx: u32,

    /// Section type.
    pub(super) sectiontype: SectionType,

    /// Section flags.
    /// Dependent on the section type.
    pub(super) flags: Address,

    /// Virtual address of the section in memory (for loaded sections).
    pub(super) vaddr: Address,

    /// Offset of the section in the file image.
    pub(super) offset: Address,

    /// Size in bytes of the section in the file image.
    pub(super) filesize: Address,

    /// Section index of an associated section.
    pub(super) link: u32,

    /// Extra information of the section.
    pub(super) info: u32,

    /// Alignment of the section.
    pub(super) alignment: Address,

    /// Size in bytes of the entries in the section (for fixed sized entries).
    pub(super) entrysize: Address,
}

impl SectionHeader {
    /// Parses the given slice of data into an ELF file header.
    pub fn parse<R: AsRef<[u8]>, const INC: usize>(raw: R, read: fn(&[u8]) -> Address, read32: fn(&[u8]) -> u32) -> Result<Self, ()> {
        // Full header size constant.
        let hsize: usize = 16 + ( 6 * INC );

        // Deref the slice.
        let raw = raw.as_ref();

        // Check there is minimum length.
        if raw.len() < hsize {
            return Err(());
        }

        // Get the section name index.
        let nameidx = read32(&raw[0x00..0x04]);

        // Get the section type.
        let sectiontype = SectionType::from( read32( &raw[0x04..0x08] ) );

        // Begin dynamic section.
        let mut i = 0x08;

        // Read the flags of the section.
        let flags = read( &raw[i..i+INC] );
        i += INC;

        // Read the virtual address.
        let vaddr = read( &raw[i..i+INC] );
        i += INC;

        // Read the file offset.
        let offset = read( &raw[i..i+INC] );
        i += INC;

        // Read the file size.
        let filesize = read( &raw[i..i+INC] );
        i += INC;

        // Read the link information.
        let link = read32( &raw[i..i+4] );
        i += 4;

        // Read the extra information.
        let info = read32( &raw[i..i+4] );
        i += 4;

        // Read the alignment.
        let alignment = read( &raw[i..i+INC] );
        i += INC;

        // Read the entry size.
        let entrysize = read( &raw[i..i+INC] );
        i += INC;

        assert_eq!(i, hsize);

        Ok(Self {
            name: String::new(),
            nameidx,
            sectiontype,
            flags,
            vaddr,
            offset,
            filesize,
            link,
            info,
            alignment,
            entrysize,
        })
    }

    /// Returns a reference to the name of the section.
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Grabs the name of this section from the given raw strings.
    pub(super) fn rename(&mut self, names: &[u8]) {
        use core::ffi::CStr;

        // Get the referenced C string.
        let cstr = match CStr::from_bytes_until_nul( &names[self.nameidx as usize..] ) {
            Err(_) => return,
            Ok(s) => s,
        };

        // Attempt to transform into a Rust str.
        let string = match cstr.to_str() {
            Err(_) => return,
            Ok(s) => s,
        };

        // Create the name string.
        self.name = String::from( string );
    }

    /// Creates a pretty print of the section's information.
    pub fn prettyprint(&self) -> String {
        // Create output string.
        let mut args = String::new();

        // Section name.
        args += &format!("Section \"{}\"\n", self.name);

        // Section type.
        args += &format!("  - Section type: {}\n", self.sectiontype);

        // Program flags.
        args += &format!("  - Flags: {}\n", self.flags);

        // Program offset in file.
        args += &format!("  - Address:\n    · File: {:X}\n    · Virtual: {:X}\n", self.offset, self.vaddr);

        // Program size in file and memory.
        args += &format!("  - File size: {} Bytes\n", self.filesize);

        // Optional header link.
        args += &format!("  - Associated section: {}\n", self.link);

        // Extra information.
        args += &format!("  - Section extra information:   {:b}\n", self.info);

        // Alignment of the section.
        args += &format!("  - Alignment: 2 << {} Bytes\n", self.alignment);

        // Optional entry size.
        args += &format!("  - Entry size: {}\n", self.entrysize);

        args
    }
}

impl super::HasContent for SectionHeader {
    const PROGRAM: bool = false;
    const SECTION: bool = true;
    const SYMBOL: bool = false;

    fn offset(&self) -> usize {
        usize::from( self.offset )
    }

    fn size(&self) -> usize {
        usize::from( self.filesize )
    }
}

impl super::HasContent for &SectionHeader {
    const PROGRAM: bool = false;
    const SECTION: bool = true;
    const SYMBOL: bool = false;

    fn offset(&self) -> usize {
        usize::from( self.offset )
    }

    fn size(&self) -> usize {
        usize::from( self.filesize )
    }
}
