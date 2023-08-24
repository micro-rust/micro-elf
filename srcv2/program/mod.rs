//! Section module.
//! Contains all information for a ELF section.


mod flags;
mod header;
mod ptype;


pub use self::flags::{ ProgramFlags32, ProgramFlags64 };
pub use self::header::{ ProgramHeader32, ProgramHeader64 };
pub use self::ptype::ProgramType;

use byteorder::{ ByteOrder, ReadBytesExt };


use crate::{ ElfProgram, Error };


use std::convert::TryFrom;



pub struct Program32 {
    /// Program header of the section.
    header: ProgramHeader32,

    /// Raw contents of the section.
    content: Vec<u8>,
}


impl ElfProgram for Program32 {
    type Address = u32;

    fn parse<T: ByteOrder>(header: &[u8], data: &[u8]) -> Result<Box<dyn ElfProgram<Address = Self::Address>>, Error> {
        // Get program type.
        let programtype = match (&header[0x00..0x04]).read_u32::<T>() {
            Ok(x) => ProgramType::from( x ),
            _ => return Err( Error::UnexpectedEOF ),
        };

        // Get offset in file.
        let offset = match (&header[0x04..0x08]).read_u32::<T>() {
            Ok(x) => x,
            _ => return Err( Error::UnexpectedEOF ),
        };

        // Get Virtual address of the segment.
        let vaddr = match (&header[0x08..0x0C]).read_u32::<T>() {
            Ok(x) => x,
            _ => return Err( Error::UnexpectedEOF ),
        };

        // Get Physical address of the segment.
        let paddr = match (&header[0x0C..0x10]).read_u32::<T>() {
            Ok(x) => x,
            _ => return Err( Error::UnexpectedEOF ),
        };

        // Get file size of the segment.
        let filesize = match (&header[0x10..0x14]).read_u32::<T>() {
            Ok(x) => x,
            _ => return Err( Error::UnexpectedEOF ),
        };

        // Get size in memory of the segment.
        let memsize = match (&header[0x14..0x18]).read_u32::<T>() {
            Ok(x) => x,
            _ => return Err( Error::UnexpectedEOF ),
        };

        // Get flags of the segment.
        let flags = match (&header[0x08..0x0C]).read_u32::<T>() {
            Ok(x) => ProgramFlags32::from( x ),
            _ => return Err( Error::UnexpectedEOF ),
        };

        // Get Alignment of the segment.
        let alignment = match (&header[0x08..0x0C]).read_u32::<T>() {
            Ok(x) => x,
            _ => return Err( Error::UnexpectedEOF ),
        };

        // Build the header.
        let header = ProgramHeader32 {
            programtype,
            offset,
            vaddr,
            paddr,
            filesize,
            memsize,
            flags,
            alignment,
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

        Ok( Box::new(Program32 {
            header,
            content,
        }))
    }

    fn info(&self) -> String {
        // Create empty string.
        let mut args = String::new();

        // Program type - Flags.
        args += &format!("{} - {}\n", self.header.programtype, self.header.flags);

        // File address - Virtual address.
        args += &format!("Address: Physical: 0x{:08X} - Virtual: 0x{:08X}\nAddress: File: 0x{:08X}", self.header.offset, self.header.vaddr, self.header.offset);

        // Program size.
        args += &format!("Size: {} Bytes\n", self.header.filesize);

        args
    }

    fn raw(&self) -> Vec<u8> {
        self.content.clone()
    }

    fn prettyprint(&self, tab: String) -> String {
        // Create output string.
        let mut args = String::new();

        // Section name.
        args += &format!("{}Program:\n", tab);

        // Section type.
        args += &format!("{}  Program type: {}\n", tab, self.header.programtype);

        // Program flags.
        args += &format!("{}  Flags: {}\n", tab, self.header.flags);

        // Program offset in file.
        args += &format!("{}  Address: Physical: 0x{:08X} | Virtual: 0x{:08X}\n", tab, self.header.paddr, self.header.vaddr);
        args += &format!("{}  Address: File: 0x{:08X}\n", tab, self.header.offset);

        // Program size in file and memory.
        args += &format!("{}  File size: {} Bytes\n", tab, self.header.filesize);

        // Alignment of the section.
        args += &format!("{}  Alignment: 2 << {} Bytes\n", tab, self.header.alignment);

        args
    }
}
