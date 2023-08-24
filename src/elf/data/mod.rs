//! ELF metadata.



pub mod header;



pub use header::FileHeader;



/// ELF metadata trait. Used to implement dynamic dispatch when analyzing ELFs.
pub trait Metadata {
    
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

impl Metadata for ELF32 {
    
}