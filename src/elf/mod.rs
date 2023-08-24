//! ELF format module.
//! Contains the basic abstractions over the format and ways to analyze and
//! modify its contents.



pub mod data;



use self::data::Metadata;



/// A container of the raw ELF data as well as the parsed metadata necessary
/// for the analysis and modification of the contents.
pub struct ELFObject<R: AsRef<[u8]>> {
    /// The metadata of the ELF object.
    /// Indexes all content within the file.
    metadata: Box<dyn Metadata>,

    /// The raw ELF data.
    raw: R,
}

impl<R: AsRef<[u8]>> ELFObject<R> {
    /// Returns an iterator over the programs of the ELF object.
    pub fn programs() {
        
    }

    /// Returns an iterator over the sections of the ELF object.
    pub fn sections() {

    }

    /// Returns an iterator over the symbols of the ELF object.
    pub fn symbols() {
        
    }
}
