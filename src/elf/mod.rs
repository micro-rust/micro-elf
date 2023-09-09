//! ELF format module.
//! Contains the basic abstractions over the format and ways to analyze and
//! modify its contents.



pub mod data;



use data::{
    ELFData, ProgramHeader, SectionHeader,
};



/// A container of the raw ELF data as well as the parsed metadata necessary
/// for the analysis and modification of the contents.
#[derive(Debug)]
pub struct ELFObject<R: AsRef<[u8]>> {
    /// The metadata of the ELF object.
    /// Indexes all content within the file.
    metadata: ELFData,

    /// The raw ELF data.
    raw: R,
}

impl<R: AsRef<[u8]>> ELFObject<R> {
    /// Parses the given data into an ELF object.
    pub fn parse(raw: R) -> Result<Self, ()> {
        // Parse the data.
        let metadata = ELFData::parse(raw.as_ref())?;

        Ok( Self { metadata, raw } )
    }

    /// Returns a reference to the list of programs.
    pub fn programs(&self) -> &Vec<ProgramHeader> {
        &self.metadata.programs
    }

    /// Returns a reference to the list of sections.
    pub fn sections(&self) -> &Vec<SectionHeader> {
        &self.metadata.sections
    }

    /// Returns the section given an ID (String, &str or usize).
    pub fn section<I: data::section::SectionID>(&self, id: I) -> Option<&SectionHeader> {
        if I::NUMERIC {
            // Get the section at the given index.
            self.sections().get(id.index())
        } else {
            // Get the name.
            let name = id.name();

            // Find the section with the given name.
            self.sections().iter().find(|section| section.name() == &name)
        }
    }

    /// Returns the contents of the given item.
    pub fn content<I: data::HasContent>(&self, item: I) -> Option<&[u8]> {
        if I::SECTION {
            // Get the file size of the section.
            let size = item.size();

            if size > 0 {
                // Get the offset.
                let offset = item.offset();

                Some( &self.raw.as_ref()[offset..offset+size] )
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Returns the endianness of the ELF object.
    pub const fn endianness(&self) -> endianness::Endianness {
        self.metadata.endianness()
    }
}

impl<R: AsRef<[u8]>> core::fmt::Display for ELFObject<R> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.metadata)
    }
}
