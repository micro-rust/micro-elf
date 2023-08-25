//! ELF format module.
//! Contains the basic abstractions over the format and ways to analyze and
//! modify its contents.



pub mod data;



use data::ELFData;



/// A container of the raw ELF data as well as the parsed metadata necessary
/// for the analysis and modification of the contents.
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
}

impl<R: AsRef<[u8]>> core::fmt::Display for ELFObject<R> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.metadata)
    }
}
