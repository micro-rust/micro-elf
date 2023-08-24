//! Section module.
//! Contains all information for a ELF section.


mod flags;
mod header;
mod ftype;


pub use self::flags::{ FileFlags32, FileFlags64 };
pub use self::header::{ FileHeader32, FileHeader64 };
pub use self::ftype::FileType;

use byteorder::{ ByteOrder, ReadBytesExt };


use crate::{ ElfFile, Error };
use crate::common::{ Architecture, Endianness, OsTarget };


use core::convert::TryFrom;



pub struct File32 {
    /// File header of the ELF.
    header: FileHeader32,

    /// Raw contents of the ELF header.
    content: Vec<u8>,
}


impl ElfFile for File32 {
    /// Address size.
    type Address = u32;

    /// Parses the section from its header and the full ELF contents.
    fn parse<T: ByteOrder>(header: &[u8]) -> Result<Box<dyn ElfFile<Address = Self::Address>>, Error> where Self: Sized {
        // Check magic.
        match &header[0x00..0x04] {
            &[0x7F, 0x45, 0x4C, 0x46] => (),
            _ => return Err( Error::BadElfMagic ),
        }

        // Check x86 or x64 format.
        match header[0x04] {
            1 => (),
            _ => return Err( Error::BadElfFormat ),
        }

        // Get endianness.
        let endianness = match header[0x05] {
            1 => Endianness::Little,
            2 => Endianness::Big,

            _ => return Err( Error::UnknownEndianness ),
        };

        // Check ELF Version.
        match header[0x06] {
            1 => (),
            _ => return Err( Error::UnknownElfVersion ),
        }

        // Get OS ABI.
        let os = OsTarget::from((header[0x07], header[0x08]));

        // Get ELF File type.
        let filetype = match (&header[0x10..0x12]).read_u16::<T>() {
            Ok(x) => FileType::from( x ),
            _ => return Err( Error::UnexpectedEOF ),
        };

        // Get ISA.
        let isa = match (&header[0x12..0x14]).read_u16::<T>() {
            Ok(x) => Architecture::from( x ),
            _ => return Err( Error::UnexpectedEOF ),
        };

        // Check ELF Version.
        match (&header[0x14..0x18]).read_u32::<T>() {
            Ok(1) => (),
            Ok(_) => return Err( Error::UnknownElfVersion ),
            _ => return Err( Error::UnexpectedEOF ),
        }

        // Get entry.
        let entry = match (&header[0x18..0x1C]).read_u32::<T>() {
            Ok(x) => x,
            _ => return Err( Error::UnexpectedEOF ),
        };

        // Get Program Header table offset.
        let phoffset = match (&header[0x1C..0x20]).read_u32::<T>() {
            Ok(x) => x,
            _ => return Err( Error::UnexpectedEOF ),
        };

        // Get Section Header table offset.
        let shoffset = match (&header[0x20..0x24]).read_u32::<T>() {
            Ok(x) => x,
            _ => return Err( Error::UnexpectedEOF ),
        };

        // Get flags.
        let flags = match (&header[0x18..0x1C]).read_u32::<T>() {
            Ok(x) => FileFlags32::from( x ),
            _ => return Err( Error::UnexpectedEOF ),
        };

        // Get File Header size.
        match (&header[0x18..0x1C]).read_u16::<T>() {
            Ok(_) => (),
            _ => return Err( Error::UnexpectedEOF ),
        }

        // Get Program Header entry size.
        let phentrysize = match (&header[0x2A..0x2C]).read_u16::<T>() {
            Ok(x) => x,
            _ => return Err( Error::UnexpectedEOF ),
        };

        // Get number of Program Headers.
        let phnum = match (&header[0x2C..0x2E]).read_u16::<T>() {
            Ok(x) => x,
            _ => return Err( Error::UnexpectedEOF ),
        };

        // Get Section Header entry size.
        let shentrysize = match (&header[0x2E..0x30]).read_u16::<T>() {
            Ok(x) => x,
            _ => return Err( Error::UnexpectedEOF ),
        };

        // Get number of Section Headers.
        let shnum = match (&header[0x30..0x32]).read_u16::<T>() {
            Ok(x) => x,
            _ => return Err( Error::UnexpectedEOF ),
        };

        // Get index of .strtab.
        let shstrndx = match (&header[0x32..0x34]).read_u16::<T>() {
            Ok(x) => x,
            _ => return Err( Error::UnexpectedEOF ),
        };

        Ok(Box::new(File32 {
            content: header.to_vec(),
            header: FileHeader32 {
                endianness,
                os,
                filetype,
                isa,
                entry,
                phoffset,
                shoffset,
                flags,
                phentrysize,
                phnum,
                shentrysize,
                shnum,
                shstrndx,
            },
        }))
    }

    fn programs(&self) -> (usize, usize, usize) {
        (
            usize::try_from(self.header.phoffset).unwrap(),
            usize::try_from(self.header.phnum).unwrap(),
            usize::try_from(self.header.phentrysize).unwrap(),
        )
    }

    fn sections(&self) -> (usize, usize, usize) {
        (
            usize::try_from(self.header.shoffset).unwrap(),
            usize::try_from(self.header.shnum).unwrap(),
            usize::try_from(self.header.shentrysize).unwrap(),
        )
    }

    /// Returns the index of the .strtab section.
    fn shstrndx(&self) -> usize {
        usize::try_from(self.header.shstrndx).unwrap()
    }

    /// Returns a nicely formatted String with a reduced information of the section.
    fn info(&self) -> String {
        // Create output string.
        let mut args = String::new();

        // Add file type.
        args += &format!("{}\n", self.header.filetype);

        // Add format and endianness.
        args += &format!("32-bit {}\n", self.header.endianness);

        // Add OS and architecture target.
        args += &format!("{} | {}\n", self.header.isa, self.header.os);

        // Add entry.
        args += &format!("Entry point: 0x{:08X}\n", self.header.entry);

        // Add program header table.
        args += &format!("Program Header table: {} headers of {} bytes at address 0x{:08X}\n", self.header.phnum, self.header.phentrysize, self.header.phoffset);

        // Add program header table.
        args += &format!("Section Header table: {} headers of {} bytes at address 0x{:08X}\n", self.header.shnum, self.header.shentrysize, self.header.shoffset);

        args
    }

    /// Returns the raw contents.
    fn raw(&self) -> Vec<u8> {
        self.content.clone()
    }

    /// Returns a pretty print of the section.
    fn prettyprint(&self, _: String) -> String {
        self.info()
    }
}
