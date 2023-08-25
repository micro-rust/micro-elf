//! ELF section header metadata.
//! Parsing, analysis and modification of the ELF section header.



mod sectiontype;



pub use sectiontype::SectionType;



/// A common section header structure. Will be instantiated by each implementator.
pub struct SectionHeader<T: core::convert::TryInto<usize> + Sized> {
    /// Section name.
    pub(super) name: String,

    /// Section name offset in the .shstrtab section.
    pub(super) nameidx: u32,

    /// Section type.
    pub(super) sectiontype: SectionType,

    /// Section flags.
    /// Dependent on the section type.
    pub(super) flags: T,

    /// Virtual address of the section in memory (for loaded sections).
    pub(super) vaddr: T,

    /// Offset of the section in the file image.
    pub(super) offset: T,

    /// Size in bytes of the section in the file image.
    pub(super) filesize: T,

    /// Section index of an associated section.
    pub(super) link: u32,

    /// Extra information of the section.
    pub(super) info: u32,

    /// Alignment of the section.
    pub(super) alignment: T,

    /// Size in bytes of the entries in the section (for fixed sized entries).
    pub(super) entrysize: T,
}

impl<T: core::convert::TryInto<usize> + Sized> SectionHeader<T> {
    // Byte size of the type for increment.
    const INC: usize = core::mem::size_of::<T>();

    // Size of the header depending on the inner type.
    const HSIZE: usize = 16 + (6 * core::mem::size_of::<T>());

    /// Parses the given slice of data into an ELF file header.
    pub fn parse<R: AsRef<[u8]>>(raw: R, read: fn(&[u8]) -> T, read32: fn(&[u8]) -> u32) -> Result<Self, ()> {
        // Deref the slice.
        let raw = raw.as_ref();

        // Check there is minimum length.
        if raw.len() < Self::HSIZE {
            return Err(());
        }

        // Get the section name index.
        let nameidx = read32(&raw[0x00..0x04]);

        // Get the section type.
        let sectiontype = SectionType::from( read32( &raw[0x04..0x08] ) );

        // Begin dynamic section.
        let mut i = 0x08;

        // Read the flags of the section.
        let flags = read( &raw[i..i+Self::INC] );
        i += Self::INC;

        // Read the virtual address.
        let vaddr = read( &raw[i..i+Self::INC] );
        i += Self::INC;

        // Read the file offset.
        let offset = read( &raw[i..i+Self::INC] );
        i += Self::INC;

        // Read the file size.
        let filesize = read( &raw[i..i+Self::INC] );
        i += Self::INC;

        // Read the link information.
        let link = read32( &raw[i..i+4] );
        i += 4;

        // Read the extra information.
        let info = read32( &raw[i..i+4] );
        i += 4;

        // Read the alignment.
        let alignment = read( &raw[i..i+Self::INC] );
        i += Self::INC;

        // Read the entry size.
        let entrysize = read( &raw[i..i+Self::INC] );
        i += Self::INC;

        assert_eq!(i, Self::HSIZE);

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
}

impl<T: core::convert::TryInto<usize> + Sized + core::fmt::Display + core::fmt::UpperHex> SectionHeader<T> {
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
        args += &format!("  - Address: File: 0x{:08X} | Virtual: 0x{:08X}\n", self.offset, self.vaddr);

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
