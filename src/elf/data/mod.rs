//! ELF metadata.



pub mod header;



pub use header::FileHeader;



/// ELF metadata trait. Used to implement dynamic dispatch when analyzing ELFs.
pub trait Metadata: core::fmt::Display {
    
}


pub struct ELF32 {
    /// File header of an ELF object.
    header: FileHeader<u32>,
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

        Ok(Self {
            header,
        })
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

        write!(f, "{}", args)
    }
}

impl Metadata for ELF32 {
    
}