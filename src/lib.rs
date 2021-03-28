//! This library is part of the `micro` framework ecosystem.
//! It deals with ELF parsing and other utilities.
//! This library **ONLY** abstracts over the 32 bit ELF files.

// This Source Code Form is subject to the terms of the
// Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one
// at https://mozilla.org/MPL/2.0/.



pub mod prelude;

#[macro_use]
pub mod common;


mod fh;
mod ph;
mod sh;
mod sym;

use prelude::*;

#[derive(Clone)]
pub struct ELFData<'a> {
	/// Raw data of the ELF file.
	data: &'a [u8],

	/// File Header of the ELF.
	fh: FileHeader,

	/// Program Headers of the ELF.
	ph: Vec<ProgHeader>,

	/// Section Headers of the ELF.
	sh: Vec<SectHeader>,
	/// All symbols present in the ELF file.
	symbols: Vec<Symbol>,
}


impl<'a> ELFData<'a> {
	/// Parses a file and converts it into a struct.
	pub fn parse(data: &'a [u8]) -> ELFData<'a> {
		match (data[0], data[1], data[2], data[3]) {
			(0x7F, 0x45, 0x4C, 0x46) => (),
			_ => panic!("Are you stupid?? This is not ELF data"),
		}

		match data[4] {
			1 => (),
			2 => panic!("This ain't a 32 bit arch ELF"),
			_ => panic!("Unknown data width size"),
		}

		// Get File Header.
		// **********************************************************
		let fh = match FileHeader::parse(data) {
			Ok(x) => x,
			_ => panic!("Failed to build File Header from file"),
		};


		// Get all Program Headers.
		// **********************************************************
		let mut ph: Vec<ProgHeader> = Vec::new();

		for header in fh.phtable(data) {
			ph.push( ProgHeader::parse(header) );
		}



		// Get all Section Headers.
		// **********************************************************
		let mut sh: Vec<SectHeader> = Vec::new();

		for header in fh.shtable(data) {
			sh.push( SectHeader::parse(header) )
		}

		let shstrtab = sh.iter()
			.find( |x| x.is_shstrtab() )
			.expect("Could not extract .shstrtab section")
			.content(data);

		for header in &mut sh {
			header.rename( &shstrtab );
		}


		// Get all symbols
		// **********************************************************
		let mut symbols: Vec<Symbol> = Vec::new();

		let strtab = sh.iter()
			.find( |x| x.is_strtab() )
			.expect("Could not extract .strtab section")
			.content(data);

		let symboliter = sh.iter()
			.find( |x| x.is_symtab() )
			.expect("Could not extract .symtab section")
			.iterate(data)
			.expect(".symtab section is not iterable");

		for symheader in symboliter {
			symbols.push( Symbol::parse(symheader, &strtab) );
		}

		// Link all symbols to their corresponding data.
		for symbol in &mut symbols {
			symbol.link(&sh)
		}

		ELFData { data, fh, ph, sh, symbols }
	}

	/// Returns a list of its symbols.
	pub fn symbols(&self) -> Vec<Symbol> {
		self.symbols.clone()
	}

	/// Attempts to load all Functions from their symbols
	/// and their contents.
	pub fn fndata(&self) -> Vec<(String, &'a[u8])> {

		self.symbols.iter()
			.filter( |sym| sym.is_function() )
			.map( |sym| (sym.name(), sym.content(&self.data)) )
			.filter( |(_, content)| content.is_some() )
			.map( |(name, content)| (name, content.unwrap()) )
			.collect()
	}
}

impl<'a> core::fmt::Debug for ELFData<'a> {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		write!(f, "{:?}", self.fh).unwrap();

		f.write_str("\n***************************************************\n").unwrap();

		for ph in &self.ph {
			write!(f, "{:?}\n", ph).unwrap();
		}

		f.write_str("\n***************************************************\n").unwrap();

		for sh in &self.sh {
			write!(f, "{:?}\n", sh).unwrap();
		}

		f.write_str("\n***************************************************\n")
	}
}

impl<'a> core::fmt::Display for ELFData<'a> {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		write!(f, "{}", self.fh).unwrap();

		f.write_str("\n***************************************************\n").unwrap();

		for ph in &self.ph {
			write!(f, "{}\n", ph).unwrap();
		}

		f.write_str("\n***************************************************\n").unwrap();

		for sh in &self.sh {
			write!(f, "{}\n", sh).unwrap();
		}

		f.write_str("\n***************************************************\n").unwrap();

		for sym in &self.symbols {
			write!(f, "{}\n", sym).unwrap();
		}

		f.write_str("\n***************************************************\n")
	}
}

