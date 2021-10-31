//! This library is part of the `micro` framework ecosystem.
//! It deals with ELF parsing and other utilities.
//! This library **ONLY** abstracts over the 32 bit ELF files.

// This Source Code Form is subject to the terms of the
// Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one
// at https://mozilla.org/MPL/2.0/.


#![allow(incomplete_features)]

#![feature(generic_const_exprs)]


pub mod common;
pub mod file;
pub mod program;
pub mod section;


use core::convert::TryFrom;


use self::file::File32;
use self::program::Program32;
use self::section::Section32;



pub fn parse(data: &Vec<u8>) -> Box<dyn ElfTrait> {
        match data[0x04] {
            1 => Elf32::parse(data),
            2 => unimplemented!(),

            _ => panic!("Bad ELF file"),
        }
}


pub struct Elf32 {
    /// File header.
    fh: Box<dyn ElfFile<Address = u32>>,

    /// All program headers.
    ph: Vec<Box<dyn ElfProgram<Address = u32>>>,

    /// All section headers.
    sh: Vec<Box<dyn ElfSection<Address = u32>>>,
}


impl Elf32 {
    /// Parses the ELF file.
    pub fn parse(data: &Vec<u8>) -> Box<dyn ElfTrait> {
        // Check magic.
        match data[0x05] {
            1 => Self::parselittle(data),
            2 => Self::parsebig(data),
            _ => panic!("Bad ELF file"),
        }
    }

    /// Parses the ELF file in 32 bit little endian mode.
    fn parselittle(data: &Vec<u8>) -> Box<dyn ElfTrait> {
        // Parse the file header.
        let fh = File32::parse::<byteorder::LittleEndian>(&data).unwrap();

        // Get program and section tables.
        let (phoff, phnum, phentsize) = fh.programs();
        let (shoff, shnum, shentsize) = fh.sections();
        let shstrndx = fh.shstrndx();

        // Get offset of the Program header table and read them all into a vector.
        let s: usize = usize::try_from(phoff).unwrap();
        let e: usize = (phnum * phentsize) as usize;

        let ph: Vec<Box<dyn ElfProgram<Address = u32>>> = data[s..s+e]
            .chunks(phentsize.into())
            .map( |sh| Program32::parse::<byteorder::LittleEndian>(sh, &data).unwrap() )
            .collect();

        // Get offset of the Section header table and read them all into a vector.
        let s: usize = usize::try_from(shoff).unwrap();
        let e: usize = (shnum * shentsize) as usize;

        let mut sh: Vec<Box<dyn ElfSection<Address = u32>>> = data[s..s+e]
            .chunks(shentsize.into())
            .map( |sh| Section32::parse::<byteorder::LittleEndian>(sh, &data).unwrap() )
            .collect();

        // Get the .strtab section.
        let strtab = sh[shstrndx as usize].raw();

        // Let all sections get their name.
        for section in &mut sh {
            section.naming( &strtab );
        }


        Box::new( Self { fh, ph, sh } )
    }

    /// Parses the ELF file in 32 bit big endian mode.
    fn parsebig(data: &Vec<u8>) -> Box<dyn ElfTrait> {
        // Parse the file header.
        let fh = File32::parse::<byteorder::BigEndian>(&data).unwrap();

        // Get program and section tables.
        let (phoff, phnum, phentsize) = fh.programs();
        let (shoff, shnum, shentsize) = fh.sections();
        let shstrndx = fh.shstrndx();

        // Get offset of the Program header table and read them all into a vector.
        let s: usize = usize::try_from(phoff).unwrap();
        let e: usize = (phnum * phentsize) as usize;

        let ph: Vec<Box<dyn ElfProgram<Address = u32>>> = data[s..s+e]
            .chunks(phentsize.into())
            .map( |sh| Program32::parse::<byteorder::BigEndian>(sh, &data).unwrap() )
            .collect();

        // Get offset of the Section header table and read them all into a vector.
        let s: usize = usize::try_from(shoff).unwrap();
        let e: usize = (shnum * shentsize) as usize;

        let mut sh: Vec<Box<dyn ElfSection<Address = u32>>> = data[s..s+e]
            .chunks(shentsize.into())
            .map( |sh| Section32::parse::<byteorder::BigEndian>(sh, &data).unwrap() )
            .collect();

        // Get the .strtab section.
        let strtab = sh[shstrndx as usize].raw();

        // Let all sections get their name.
        for section in &mut sh {
            section.naming( &strtab );
        }


        Box::new( Self { fh, ph, sh } )
    }
}

impl ElfTrait for Elf32 {
    fn fileheader(&self) -> &Box<dyn ElfFile<Address = u32>> {
        &self.fh
    }

    fn programs(&self) -> &Vec<Box<dyn ElfProgram<Address = u32>>> {
        &self.ph
    }

    fn sections(&self) -> &Vec<Box<dyn ElfSection<Address = u32>>> {
        &self.sh
    }
}

impl std::fmt::Display for Elf32 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut args = String::new();

        // Display File Header.
        args += &format!("{}", self.fh.prettyprint(String::new()));

        // Display all Program Headers.
        args += "\nProgram headers:\n";

        for ph in &self.ph {
            args += &format!("{}\n", ph.prettyprint(String::from("  ")));
        }

        // Display all Section Headers.
        args += "\nSection headers:\n";

        for sh in &self.sh {
            args += &format!("{}\n", sh.prettyprint(String::from("  ")));
        }

        write!(f, "{}", args)
    }
}



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    /// The input stream ended unexpectedly.
    UnexpectedEOF,

    /// The ELF file magic number is not correct.
    BadElfMagic,

    /// A bad format was selected.
    BadElfFormat,

    /// AN unknwon endiannes was found.
    UnknownEndianness,

    /// An ELF version other than 1 was found.
    UnknownElfVersion,
}


pub trait ElfSection {
    /// Address size.
    type Address;

    /// Parses the section from its header and the full ELF contents.
    fn parse<T: byteorder::ByteOrder>(header: &[u8], data: &[u8]) -> Result<Box<dyn ElfSection<Address = Self::Address>>, Error> where Self: Sized;

    /// Returns the name of the section.
    fn name(&self) -> String;

    /// The section reads its name from the .strtab section.
    fn naming(&mut self, strtab: &[u8]);

    /// Returns a nicely formatted String with a reduced information of the section.
    fn info(&self) -> String;

    /// Returns the raw contents.
    fn raw(&self) -> Vec<u8>;

    /// Returns the content of the Section ready to be formatted.
    fn content(&self) -> Vec<(Self::Address, Self::Address)>;

    /// Returns a pretty print of the section.
    fn prettyprint(&self, tab: String) -> String;
}

pub trait ElfProgram {
    /// Address size.
    type Address;

    /// Parses the section from its header and the full ELF contents.
    fn parse<T: byteorder::ByteOrder>(header: &[u8], data: &[u8]) -> Result<Box<dyn ElfProgram<Address = Self::Address>>, Error> where Self: Sized;

    /// Returns a nicely formatted String with a reduced information of the section.
    fn info(&self) -> String;

    /// Returns the raw contents.
    fn raw(&self) -> Vec<u8>;

    /// Returns a pretty print of the section.
    fn prettyprint(&self, tab: String) -> String;
}


pub trait ElfFile {
    /// Address size.
    type Address;

    /// Parses the section from its header and the full ELF contents.
    fn parse<T: byteorder::ByteOrder>(header: &[u8]) -> Result<Box<dyn ElfFile<Address = Self::Address>>, Error> where Self: Sized;

    /// Returns the offset, number and size of the program headers.
    fn programs(&self) -> (usize, usize, usize);

    /// Returns the offset, number and size of the section headers.
    fn sections(&self) -> (usize, usize, usize);

    /// Returns the index of the .strtab section.
    fn shstrndx(&self) -> usize;

    /// Returns a nicely formatted String with a reduced information of the section.
    fn info(&self) -> String;

    /// Returns the raw contents.
    fn raw(&self) -> Vec<u8>;

    /// Returns a pretty print of the section.
    fn prettyprint(&self, tab: String) -> String;
}


pub trait ElfTrait: core::fmt::Display {
    /// Returns a reference to the file header.
    fn fileheader(&self) -> &Box<dyn ElfFile<Address = u32>>;

    /// Returns a reference to the program headers.
    fn programs(&self) -> &Vec<Box<dyn ElfProgram<Address = u32>>>;

    /// Returns a reference to the section headers.
    fn sections(&self) -> &Vec<Box<dyn ElfSection<Address = u32>>>;
}


pub trait ElfFormat: num_integer::Integer {}

impl ElfFormat for u32 {}
impl ElfFormat for u64 {}
