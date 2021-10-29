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

	/// Returns a reference to the program headers.
	pub fn programs(&self) -> &[ProgramHeader<ARCH>] {
		&self.ph
	}

	/// Returns a reference to the section headers.
	pub fn sections(&self) -> &[ProgramHeader<ARCH>] {
		&self.ph
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
	const CLASSBIT: u8;
	const ADDRSIZE: usize;

	const PFLAGS: usize;
	const PALIGN: usize;
	const POFFSET: usize;

	fn read(data: &[u8]) -> Self::Address;
	fn slice(data: &[u8], off: usize) -> &[u8];
	fn as_usize(x: Self::Address) -> usize;
}
