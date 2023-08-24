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
    /// Parses the given data into an ELF object.
    pub fn parse(raw: R) -> Result<Self, ()> {
        // Get the slice.
        let slice = raw.as_ref();

        // Check the inner size to create the metadata.
        match slice[0x04] {
            1 => {
                // Parse the data.
                let metadata = Box::new( data::ELF32::parse(raw.as_ref())? );

                Ok(Self { metadata, raw })
            },

            2 => {
                // Parse the data.
                let metadata = Box::new( data::ELF32::parse(raw.as_ref())? );

                Ok(Self { metadata, raw })
            },

            _ => return Err( () ),
        }
    }

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

impl<R: AsRef<[u8]>> core::fmt::Display for ELFObject<R> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.metadata)
    }
}
