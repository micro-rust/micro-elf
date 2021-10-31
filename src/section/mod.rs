//! Section module.
//! Contains all information for a ELF section.


mod flags;
mod header;
mod stype;


pub use self::flags::{ SectionFlags32, SectionFlags64 };
pub use self::header::{ SectionHeader32, SectionHeader64 };
pub use self::stype::SectionType;

use byteorder::{ ByteOrder, ReadBytesExt };


use crate::{ ElfSection, Error };


use std::convert::TryFrom;



pub struct Section32 {
    /// File header of the section.
    header: SectionHeader32,

    /// Raw contents of the section.
    content: Vec<u8>,
}


impl ElfSection for Section32 {
    type Address = u32;

    fn parse<T: ByteOrder>(header: &[u8], data: &[u8]) -> Result<Box<dyn ElfSection<Address = Self::Address>>, Error> {
        // Get index to the name.
        let nameidx = match (&header[0x00..0x04]).read_u32::<T>() {
            Ok(x) => x,
            _ => return Err( Error::UnexpectedEOF ),
        };

        // Get section type.
        let sectiontype = match (&header[0x04..0x08]).read_u32::<T>() {
            Ok(x) => SectionType::from( x ),
            _ => return Err( Error::UnexpectedEOF ),
        };

        // Get flags.
        let flags = match (&header[0x08..0x0C]).read_u32::<T>() {
            Ok(x) => SectionFlags32::from( x ),
            _ => return Err( Error::UnexpectedEOF ),
        };

        // Get virtual address.
        let vaddr = match (&header[0x0C..0x10]).read_u32::<T>() {
            Ok(x) => x,
            _ => return Err( Error::UnexpectedEOF ),
        };

        // Get file offset.
        let offset = match (&header[0x10..0x14]).read_u32::<T>() {
            Ok(x) => x,
            _ => return Err( Error::UnexpectedEOF ),
        };

        // Get size of the section in the file.
        let filesize = match (&header[0x14..0x18]).read_u32::<T>() {
            Ok(x) => x,
            _ => return Err( Error::UnexpectedEOF ),
        };

        // Get index of associated section.
        let link = match (&header[0x18..0x1C]).read_u32::<T>() {
            Ok(x) => x,
            _ => return Err( Error::UnexpectedEOF ),
        };

        // Get extra information.
        let extrainfo = match (&header[0x1C..0x20]).read_u32::<T>() {
            Ok(x) => x,
            _ => return Err( Error::UnexpectedEOF ),
        };

        // Get address alignment.
        let alignment = match (&header[0x20..0x24]).read_u32::<T>() {
            Ok(x) => x,
            _ => return Err( Error::UnexpectedEOF ),
        };

        // Get entry size if there are entries.
        let entrysize = match (&header[0x24..0x28]).read_u32::<T>() {
            Ok(x) => x,
            _ => return Err( Error::UnexpectedEOF ),
        };


        // Build the header.
        let header = SectionHeader32 {
            name: String::new(),
            nameidx,
            sectiontype,
            flags,
            vaddr,
            offset,
            filesize,
            link,
            extrainfo,
            alignment,
            entrysize,
        };


        // Get contents of section.
        let content = match filesize {
            0 => Vec::new(),
            _ => {
                // Starting offset.
                let s = usize::try_from(offset).unwrap();

                // Section end.
                let e = s + usize::try_from(filesize).unwrap();

                data[s..e].to_vec()
            },
        };

        Ok( Box::new(Section32 {
            header,
            content,
        }))
    }

    fn name(&self) -> String {
        self.header.name.clone()
    }

    fn naming(&mut self, strtab: &[u8]) {
        // Get base offset.
        let s = usize::try_from(self.header.nameidx).unwrap();

        // Get dynamic index.
        let mut e = s + 1;

        // Check end of NULL terminated string.
        while strtab[e] != 0x00 { e += 1 }

        // Read in the name.
        self.header.name = unsafe { String::from_utf8_unchecked( strtab[s..e].to_vec() ) };
    }

    fn info(&self) -> String {
        // Create empty string.
        let mut args = String::new();

        // Section type - Flags.
        args += &format!("{} - {}\n", self.header.sectiontype, self.header.flags);

        // File address - Virtual address.
        args += &format!("Address: File: 0x{:08X} - Virtual: 0x{:08X}\n", self.header.offset, self.header.vaddr);

        // Section size.
        args += &format!("Size: {} Bytes\n", self.header.filesize);

        args
    }

    fn raw(&self) -> Vec<u8> {
        self.content.clone()
    }

    fn content(&self) -> Vec<(u32, u32)> {
        match self.header.filesize {
            0 => return Vec::new(),
            _ => (),
        }

        // Maximum number of output items range.
        let maxitems = (self.header.filesize / 4) + 1;

        // Create the output.
        self.content
            .chunks(4)
            .map(|data| match data.len() {
                1 => [data[0],       0,       0,       0].as_ref().read_u32::<byteorder::LittleEndian>(),
                2 => [data[0], data[1],       0,       0].as_ref().read_u32::<byteorder::LittleEndian>(),
                3 => [data[0], data[1], data[2],       0].as_ref().read_u32::<byteorder::LittleEndian>(),
                4 => [data[0], data[1], data[2], data[3]].as_ref().read_u32::<byteorder::LittleEndian>(),

                _ => panic!("Unexpected array length while parsing a byte array."),
            })
            .map(|data| data.unwrap())
            .collect::<Vec<u32>>()
            .iter()
            .zip(0..=maxitems)
            .map(|(data, offset)| (self.header.vaddr + offset, *data) )
            .collect()
    }

    /*
    fn content(&self) -> Vec<(u32, [u8; 4])> {
        match self.header.filesize {
            0 => return Vec::new(),
            _ => (),
        }

        // Maximum number of output items range.
        let maxitems = (self.header.filesize / 4) + 1;

        // Create the output.
        self.content
            .chunks(4)
            .zip(0..=maxitems)
            .map(|(data, offset)| (self.header.vaddr + offset, [data[0], data[1], data[2], data[3]]) )
            .collect()
    }
    */

    /*
    fn content16<T: ByteOrder>(&self) -> Vec<(u32, [u16; 2])> {
        match self.header.filesize {
            0 => return Vec::new(),
            _ => (),
        }

        // Maximum number of output items range.
        let maxitems = (self.header.filesize / 4) + 1;

        // Create the output.
        self.content
            .chunks(2)
            .map(|data| match data.len() {
                1 => [data[0],       0].as_ref().read_u16::<T>(),
                2 => [data[0], data[1]].as_ref().read_u16::<T>(),

                _ => panic!("Unexpected array length while parsing a byte array."),
            })
            .map(|data| data.unwrap() )
            .collect::<Vec<u16>>()
            .chunks(2)
            .zip(0..=maxitems)
            .map(|(data, offset)| (self.header.vaddr + offset, [data[0], data[1]]) )
            .collect()
    }

    fn content32<T: ByteOrder>(&self) -> Vec<(u32, [u32; 1])> {
        match self.header.filesize {
            0 => return Vec::new(),
            _ => (),
        }

        // Maximum number of output items range.
        let maxitems = (self.header.filesize / 4) + 1;

        // Create the output.
        self.content
            .chunks(4)
            .map(|data| match data.len() {
                1 => [data[0],       0,       0,       0].as_ref().read_u32::<T>(),
                2 => [data[0], data[1],       0,       0].as_ref().read_u32::<T>(),
                3 => [data[0], data[1], data[2],       0].as_ref().read_u32::<T>(),
                4 => [data[0], data[1], data[2], data[3]].as_ref().read_u32::<T>(),

                _ => panic!("Unexpected array length while parsing a byte array."),
            })
            .map(|data| data.unwrap())
            .collect::<Vec<u32>>()
            .iter()
            .zip(0..=maxitems)
            .map(|(data, offset)| (self.header.vaddr + offset, [*data]) )
            .collect()
    }
    */

    fn prettyprint(&self, tab: String) -> String {
        // Create output string.
        let mut args = String::new();

        // Section name.
        args += &format!("{}Section: {}\n", tab, self.header.name);

        // Section type.
        args += &format!("{}  Section type: {}\n", tab, self.header.sectiontype);

        // Program flags.
        args += &format!("{}  Flags: {}\n", tab, self.header.flags);

        // Program offset in file.
        args += &format!("{}  Address: File: 0x{:08X} | Virtual: 0x{:08X}\n", tab, self.header.offset, self.header.vaddr);

        // Program size in file and memory.
        args += &format!("{}  File size: {} Bytes\n", tab, self.header.filesize);

        // Optional header link.
        match self.header.link {
            0 => (),
            _ => args += &format!("{}  Associated section: {}\n", tab, self.header.link),
        }

        // Extra information.
        args += &format!("{}  Section extra information:   {:b}\n", tab, self.header.extrainfo);

        // Alignment of the section.
        args += &format!("{}  Alignment: 2 << {} Bytes\n", tab, self.header.alignment);

        // Optional entry size.
        match self.header.entrysize {
            0 => (),
            _ => args += &format!("{}  Entry size: {}\n", tab, self.header.entrysize),
        }

        args
    }
}
