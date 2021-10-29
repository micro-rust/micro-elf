//! ELF Section Header.

// This Source Code Form is subject to the terms of the
// Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one
// at https://mozilla.org/MPL/2.0/.


use byteorder::{ NativeEndian, ReadBytesExt };


use crate::{ ElfArch, X32 };
use crate::common::SectionType;


pub struct SectionHeader<ARCH: ElfArch> {
	/// Name of the section.
	name: String,

	/// Offset in SHSTRNDX for its own name.
	nameidx: u32,

	/// Section type.
	shtype: SectionType,

	/// Section attributes.
	sflags: ARCH::Address,

	/// Virtual address of the section in memory.
	vaddr: ARCH::Address,

	/// Offset to the section in the file.
	pub offset: ARCH::Address,

	/// Size in bytes of the section in the file.
	pub filesize: ARCH::Address,

	/// Section index of an associated section.
	link: u32,

	/// Extra information about the section.
	info: u32,

	/// Required alignment of the section.
	align: ARCH::Address,

	/// Size in bytes of each entry for fixed size entry sections.
	entsize: ARCH::Address,
}


impl<ARCH: ElfArch> SectionHeader<ARCH> {
	/// Parses the file header.
	pub fn parse(data: &[u8]) -> Self {
		// Get the index to the name.
		let nameidx = (&data[0..0x4]).read_u32::<NativeEndian>().unwrap();

		// Get section type.
		let shtype = SectionType::from( (&data[0x4..0x8]).read_u32::<NativeEndian>().unwrap() );

		// Create dynamic index.
		let mut index = 0x08;

		// Get flags.
		let sflags = ARCH::read( ARCH::slice( data, index ) );
		index += ARCH::ADDRSIZE;

		// Get Virtual address.
		let vaddr = ARCH::read( ARCH::slice( data, index ) );
		index += ARCH::ADDRSIZE;

		// Get file offset.
		let offset = ARCH::read( ARCH::slice( data, index ) );
		index += ARCH::ADDRSIZE;

		// Get File size.
		let filesize = ARCH::read( ARCH::slice( data, index ) );
		index += ARCH::ADDRSIZE;

		let link = ARCH::slice( data, index ).read_u32::<NativeEndian>().unwrap();
		index += 4;

		// Get extra information.
		let info = ARCH::slice( data, index ).read_u32::<NativeEndian>().unwrap();
		index += 4;

		// Get alignment of the section.
		let align = ARCH::read( ARCH::slice( data, index ) );
		index += ARCH::ADDRSIZE;

		// Get entry size for fixed-sized entries.
		let entsize = ARCH::read( ARCH::slice( data, index ) );


		Self {
			name: String::new(),
			nameidx,
			shtype,
			sflags,
			vaddr,
			offset,
			filesize,
			link,
			info,
			align,
			entsize,
		}
	}

	/// Create a redux display string.
	pub fn redux(&self) -> String {
		let mut args = String::new();

		// Section type - Flags.
		args += &format!("{} - {}\n", self.shtype, self.sflags);

		// File address - Virtual address.
		args += &format!("{} - {}\n", self.offset, self.vaddr);

		// Section size.
		args += &format!("{} Bytes", self.filesize);

		args
	}

	/// Read the sections name from the given SHSTRNDX.
	pub fn naming(&mut self, data: &[u8]) {
		// End of name.
		let mut e = (self.nameidx + 1) as usize;

		// Read until a 0x00.
		while data[e] != 0x0 { e += 1 }

		// Create name.
		self.name = String::from_utf8( data[self.nameidx as usize..e].to_vec() ).unwrap();
	}

	/// Returns the name of the section.
	pub fn name(&self) -> String {
		self.name.clone()
	}
}



impl<ARCH: ElfArch> std::fmt::Display for SectionHeader<ARCH> {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		let mut args = String::new();

		// Section name.
		args += &format!("  Section: {}\n", self.name);

		// Section type
		args += &format!("    Type: {}\n", self.shtype);

		// Program flags.
		args += &format!("    Flags: 0x{:X}\n", self.sflags);

		// Program offset in file.
		args += &format!("    Section offset in file: 0x{:08X}\n",   self.offset);

		// Virtual and physical address.
		args += &format!("    Section Virtual address:  0x{:08X}\n",   self.vaddr);

		// Program size in file and memory.
		args += &format!("    Section File size:   {}\n",   self.filesize);

		// Index of associated section.
		args += &format!("    Associated section index:   {}\n",   self.link);

		// Extra information.
		args += &format!("    Section extra information:   {:b}\n",   self.info);

		// Alignment of the section.
		args += &format!("    Alignment: 2 << {}\n", self.align);

		// Entry size.
		args += &format!("    Section entry size:   {}\n\n",   self.entsize);

		// Index of the 

		write!(f, "{}", args)
	}
}



pub struct SectionFlags<ARCH: ElfArch>(ARCH::Address);

impl<ARCH: ElfArch> SectionFlags<ARCH> {
	/// Creates a new Section flags from the given input.
	pub fn new(x: ARCH::Address) -> Self {
		Self(x)
	}
}


impl core::fmt::Display for SectionFlags<X32> {
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
