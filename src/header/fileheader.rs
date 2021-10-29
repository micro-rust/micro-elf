//! ELF File.

// This Source Code Form is subject to the terms of the
// Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one
// at https://mozilla.org/MPL/2.0/.

use crate::ElfArch;
use crate::common::*;


use byteorder::{ NativeEndian, ReadBytesExt };


use std::io::Cursor;


pub struct FileHeader<ARCH: ElfArch> {
	/// Endianness of the target hardware.
	endianness: Endianness,

	/// Target OS ABI.
	ostarget: OsTarget,

	/// Object file type.
	elftype: ELFType,

	/// Target ISA.
	isa: Architecture,

	/// Entry point.
	entry: ARCH::Address,

	/// Pointer to the Program Header table.
	pub(crate) phoff: ARCH::Address,

	/// Pointer to the Section Header table.
	pub(crate) shoff: ARCH::Address,

	/// Architecture flags.
	flags: u32,

	/// Size of a Program Header table entry.
	pub(crate) phentsize: u16,

	/// Number of Program Header table entries.
	pub(crate) phnum: u16,

	/// Size of a Section Header table entry.
	pub(crate) shentsize: u16,

	/// Number of Section Header table entries.
	pub(crate) shnum: u16,

	/// Index of the Section Header table entry containing section names.
	pub(crate) shstrndx: u16,
}



impl<ARCH: ElfArch> FileHeader<ARCH> {
	/// Parses the file header.
	pub fn parse(data: &[u8]) -> Self {
		// Check MAGIC.
		match &data[0..4] {
			&[0x7F, 0x45, 0x4C, 0x46] => (),
			magic => panic!("Expected [0x7F, 0x45, 0x4C, 0x46], found: [{}, {}, {}, {}]", magic[0], magic[1], magic[2], magic[3]),
		}

		// Check that the class is correct.
		if data[4] != ARCH::CLASSBIT {
			panic!("Expected Class byte {}, found {}", ARCH::CLASSBIT, data[4]);
		}

		// Get endianness.
		let endianness = match data[5] {
			1 => Endianness::Little,
			2 => Endianness::Big,
			e => panic!("Unknown endianness {}", e),
		};

		// Check ELF version.
		if data[6] != 1 {
			panic!("Unknown ELF version: {}", data[6]);
		}

		// Get OS ABI.
		let ostarget = OsTarget::from((data[7], data[8]));

		// Get object file type.
		let elftype = ELFType::from( Cursor::new(&data[0x10..0x12]).read_u16::<NativeEndian>().unwrap() );

		// Get target ISA.
		let isa = Architecture::from( Cursor::new(&data[0x12..0x14]).read_u16::<NativeEndian>().unwrap() );

		// Check ELF version.
		match Cursor::new(&data[0x14..0x18]).read_u32::<NativeEndian>().unwrap() {
			1 => (),
			v => panic!("Unknown ELF version: {}", v),
		}

		// Start dynamic index.
		let mut index = 0x18;

		// Get entry.
		let entry = ARCH::read( ARCH::slice( data, index ) );
		index += ARCH::ADDRSIZE;

		// Get Program Header table.
		let phoff = ARCH::read( ARCH::slice( data, index ) );
		index += ARCH::ADDRSIZE;

		// Get Section Header table.
		let shoff = ARCH::read( ARCH::slice( data, index ) );
		index += ARCH::ADDRSIZE;

		// Get flags.
		let flags = ARCH::slice( data, index ).read_u32::<NativeEndian>().unwrap();
		index += 4;

		// Read the size of this header.
		let selfsize = ARCH::slice( data, index ).read_u16::<NativeEndian>().unwrap();
		index += 2;

		// Get phentsize.
		let phentsize: u16 = ARCH::slice( data, index ).read_u16::<NativeEndian>().unwrap();
		index += 2;

		// Get phnum.
		let phnum = ARCH::slice( data, index ).read_u16::<NativeEndian>().unwrap();
		index += 2;

		// Get shentsize.
		let shentsize = ARCH::slice( data, index ).read_u16::<NativeEndian>().unwrap();
		index += 2;

		// Get shnum.
		let shnum = ARCH::slice( data, index ).read_u16::<NativeEndian>().unwrap();
		index += 2;

		// Get shstrndx.
		let shstrndx = ARCH::slice( data, index ).read_u16::<NativeEndian>().unwrap();


		Self {
			endianness,
			ostarget,
			elftype,
			isa,
			entry,
			phoff,
			shoff,
			flags,
			phentsize,
			phnum,
			shentsize,
			shnum,
			shstrndx,
		}
	}
}

impl<ARCH: ElfArch> std::fmt::Display for FileHeader<ARCH> {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		let mut args = String::new();

		// ELF File type.
		args += &format!("{}\n", self.elftype);

		// Endianness.
		args += &format!("  {}\n", self.endianness);

		// OS ABI.
		args += &format!("  {}\n", self.ostarget);

		// Target ISA.
		args += &format!("  Architecture: {}\n\n", self.isa);

		// Entry point.
		args += &format!("  Entry (VAddr): {}\n\n", self.entry);

		// Program and section header table file offset.
		args += &format!("  Program Header Table offset: 0x{:08X}\n",   self.phoff);
		args += &format!("  Section Header Table offset: 0x{:08X}\n\n", self.shoff);

		// Program and section header size.
		args += &format!("  Program Header entry size: {}\n",   self.phentsize);
		args += &format!("  Section Header entry size: {}\n\n", self.shentsize);

		// Program and section header count.
		args += &format!("  Program Header count: {}\n",   self.phnum);
		args += &format!("  Section Header count: {}\n\n", self.shnum);

		// Index of Section header string section.
		args += &format!("  Index of section name section: {}\n", self.shstrndx);

		// Index of the 

		write!(f, "{}", args)
	}
}
