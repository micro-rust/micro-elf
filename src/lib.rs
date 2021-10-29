//! This library is part of the `micro` framework ecosystem.
//! It deals with ELF parsing and other utilities.
//! This library **ONLY** abstracts over the 32 bit ELF files.

// This Source Code Form is subject to the terms of the
// Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one
// at https://mozilla.org/MPL/2.0/.


pub mod common;
pub mod header;

use common::*;
use header::*;


pub struct Elf<ARCH: ElfArch> {
	/// Full ELF file.
	data: Vec<u8>,

	/// File header.
	fh: FileHeader<ARCH>,

	/// All program headers.
	ph: Vec<ProgramHeader<ARCH>>,

	/// All section headers.
	sh: Vec<SectionHeader<ARCH>>,
}


impl<ARCH: ElfArch> Elf<ARCH> {
	/// Parses the ELF file.
	pub fn parse(data: Vec<u8>) -> Self {
		// Parse the file header.
		let fh = FileHeader::parse(&data);

		// Get offset of the Program header table and read them all into a vector.
		let s: usize = ARCH::as_usize(fh.phoff);
		let e: usize = (fh.phnum * fh.phentsize) as usize;

		let ph = data[s..s+e]
			.chunks(fh.phentsize.into())
			.map( |ph| ProgramHeader::parse(ph) )
			.collect();

		// Get offset of the Section header table and read them all into a vector.
		let s: usize = ARCH::as_usize(fh.shoff);
		let e: usize = (fh.shnum * fh.shentsize) as usize;

		let mut sh: Vec<SectionHeader<ARCH>> = data[s..s+e]
			.chunks(fh.shentsize.into())
			.map( |sh| SectionHeader::parse(sh) )
			.collect();

		// Get the data from the SHSTRNDX.
		let (off, size) = {
			let section = &sh[fh.shstrndx as usize];
			( ARCH::as_usize(section.offset), ARCH::as_usize(section.filesize) )
		};

		let names = &data[off..off+size];

		// Let all sections get their name.
		for section in &mut sh {
			section.naming( names );
		}


		Elf { data, fh, ph, sh }
	}

	/// Returns a reference to the file header.
	pub fn fileheader(&self) -> &FileHeader<ARCH> {
		&self.fh
	}

	/// Returns a reference to the program headers.
	pub fn programs(&self) -> &[ProgramHeader<ARCH>] {
		&self.ph
	}

	/// Returns a reference to the section headers.
	pub fn sections(&self) -> &[SectionHeader<ARCH>] {
		&self.sh
	}
}

impl<ARCH: ElfArch> std::fmt::Display for Elf<ARCH> {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		let mut args = String::new();

		// Display File Header.
		args += &format!("{}", self.fh);

		// Display all Program Headers.
		args += "Program headers:\n";

		for ph in &self.ph {
			args += &format!("{}", ph);
		}

		// Display all Section Headers.
		args += "Section headers:\n";

		for sh in &self.sh {
			args += &format!("{}", sh);
		}

		write!(f, "{}", args)
	}
}



pub struct X32;

impl ElfArch for X32 {
	type Address = u32;
	type SectionFlags = SectionFlags32;

	const CLASSBIT: u8 = 1;
	const ADDRSIZE: usize = 4;

	const PFLAGS: usize = 0x18;
	const PALIGN: usize = 0x1C;
	const POFFSET: usize = 0x04;

	fn read(data: &[u8]) -> Self::Address {
		use byteorder::{ NativeEndian, ReadBytesExt };
		use std::io::Cursor;

		Cursor::new(data).read_u32::<NativeEndian>().expect("Buffer to read U32 not big enough.")
	}

	fn slice(data: &[u8], off: usize) -> &[u8] {
		&data[off..off+Self::ADDRSIZE]
	}

	fn as_usize(x: u32) -> usize {
		x as usize
	}
}


pub trait ElfArch {
	type Address: Copy + Clone + std::fmt::Debug + std::fmt::Display + std::fmt::UpperHex + std::fmt::Binary +core::convert::TryInto<usize>;
	type SectionFlags: core::fmt::Display + core::convert::From<Self::Address>;

	const CLASSBIT: u8;
	const ADDRSIZE: usize;

	const PFLAGS: usize;
	const PALIGN: usize;
	const POFFSET: usize;

	fn read(data: &[u8]) -> Self::Address;
	fn slice(data: &[u8], off: usize) -> &[u8];
	fn as_usize(x: Self::Address) -> usize;
}

pub struct SectionFlags32(u32);

impl core::convert::From<u32> for SectionFlags32 {
	fn from(x: u32) -> Self {
		Self(x)
	}
}

impl core::fmt::Display for SectionFlags32 {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		let mut args = String::new();

		if self.0 & 0x001 == 0x001 {
			args += "WRITE ";
		}

		if self.0 & 0x002 == 0x002 {
			args += "ALLOC ";
		}

		if self.0 & 0x004 == 0x004 {
			args += "EXEC ";
		}

		if self.0 & 0x010 == 0x010 {
			args += "MERGE ";
		}

		if self.0 & 0x020 == 0x020 {
			args += "STRINGS ";
		}

		if self.0 & 0x040 == 0x040 {
			args += "LINKINFO ";
		}

		if self.0 & 0x080 == 0x080 {
			args += "LINKORDER ";
		}

		if self.0 & 0x100 == 0x100 {
			args += "NONCONFORMING ";
		}

		if self.0 & 0x200 == 0x200 {
			args += "GROUP ";
		}

		if self.0 & 0x400 == 0x400 {
			args += "TLS ";
		}

		if self.0 & 0x0FF00000 == 0x0FF00000 {
			args += "MASKOS ";
		}

		if self.0 & 0xF0000000 == 0xF0000000 {
			args += "MASKPROC ";
		}

		write!(f, "{}", args)
	}
}
