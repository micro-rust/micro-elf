//! File Header descriptor.
//! Parses the File Header from an ELF file and builds an abstraction over it.


// This Source Code Form is subject to the terms of the
// Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one
// at https://mozilla.org/MPL/2.0/.


mod desc;
mod flags;
mod target;


use crate::common::TableIterator;

use self::desc::HTDescriptor;

pub use self::flags::ARMFlags;
pub use self::target::ELFTarget;

#[derive(Clone, Copy)]
pub struct FileHeader {
	/// Target Descriptor.
	/// Contains all information about the target Architecture, available OS ABI and other information.
	target: ELFTarget,

	/// Entry point of the code.
	entry: u32,

	/// Architecture Flags of the file.
	flags: ARMFlags,

	/// Program Header Table Descriptor.
	phtdesc: HTDescriptor,

	/// Section Header Table Descriptor.
	shtdesc: HTDescriptor,

	/// .shstrtab index.
	shstrtab: u16,

	/// Raw data of the file header.
	raw: [u8; 0x34],
}

impl FileHeader {
	/// Parses the given data into an ELF `FileHeader`.
	pub fn parse(data: &[u8]) -> Result<FileHeader, ()> {
		if data.len() < 0x34 { return Err(()); }

		let mut raw = [0u8; 0x34];

		for i in 0..0x34 {
			raw[i] = data[i];
		}

		let target = ELFTarget::parse(&data[0x00..0x14]);

		let entry = byteread!(32, data[0x18], data[0x19], data[0x1A], data[0x1B]);

		let flags = ARMFlags::create(
			byteread!(32, data[0x24], data[0x25], data[0x26], data[0x27])
		);

		let phtdesc = HTDescriptor::parse(data, 0);

		let shtdesc = HTDescriptor::parse(data, 4);

		let shstrtab = byteread!(16, data[0x32], data[0x33]);

		Ok( FileHeader { target, entry, flags, phtdesc, shtdesc, shstrtab, raw } )
	}

	/// Returns the iterator over the Program Header Table.
	pub fn phtable<'a>(&self, data: &'a [u8]) -> TableIterator<'a> {
		self.phtdesc.iterate(data)
	}

	/// Returns the iterator over the Program Header Table.
	pub fn shtable<'a>(&self, data: &'a [u8]) -> TableIterator<'a> {
		self.shtdesc.iterate(data)
	}

	/// Returns the index of the shstrtab section.
	pub fn shstrtab(&self) -> u16 {
		self.shstrtab
	}
}

impl core::fmt::Debug for FileHeader {
	/// Displays the raw information of the ELF File Header along with its decoded information.
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		let mut args = String::from("ELF File Header [raw]\n");

		args += &format!("{}\n", self.flags);

		// Display ELF Magic
		args += &format!("0x00 : 0x{:02X} 0x{:02X} 0x{:02X} 0x{:02X} | 0x7F 'E' 'L' 'F' - Magic number\n", self.raw[0], self.raw[1], self.raw[2], self.raw[3]);

		// Display architecture.
		args += &format!("0x04 : 0x{:02X}                | {}\n", self.raw[4], if self.target.bit32() { "x32 architecture" } else if self.target.bit64() { "x64 architecture" } else { "Unknown architecture size" });

		// Display endianness.
		args += &format!("0x05 : 0x{:02X}                | {}\n", self.raw[5], if self.target.big() { "BIG-endian" } else if self.target.little() { "little-endian" } else { "Unknown endianness" });

		// Display ELF version.
		args += &format!("0x06 : 0x{:02X}                | ELF Version 1\n", self.raw[6]);

		// Display OS-ABI.
		args += &format!("0x07 : 0x{:02X}                | {}\n", self.raw[7], self.target.osabi());

		// Display OS-ABI version.
		args += &format!("0x08 : 0x{:02X} 0x{:02X} 0x{:02X} 0x{:02X} | OS-ABI version {}\n", self.raw[8], self.raw[9], self.raw[10], self.raw[11], self.raw[8]);

		// Display rest of padding.
		args += &format!("0x0C : 0x{:02X} 0x{:02X} 0x{:02X} 0x{:02X} |\n", self.raw[12], self.raw[13], self.raw[14], self.raw[15]);

		// Display ELF file type.
		args += &format!("0x10 : 0x{:04X}              | {}\n", byteread!(16, self.raw[16], self.raw[17]) ,self.target.filetype());

		// Display Target Instruction Architecture.
		args += &format!("0x12 : 0x{:04X}              | {}\n", byteread!(16, self.raw[18], self.raw[19]), self.target.arch() );

		// Display ELF version.
		args += &format!("0x14 : 0x{:02X} 0x{:02X} 0x{:02X} 0x{:02X} | ELF Version 1\n", self.raw[20], self.raw[21], self.raw[22], self.raw[23]);

		// Display entry address.
		args += &format!("0x18 : 0x{:08X}          | Entry point address\n", byteread!(32, self.raw[24], self.raw[25], self.raw[26], self.raw[27]));

		f.write_str(&args)
	}
}

impl core::fmt::Display for FileHeader {
	/// Displays the raw information of the ELF File Header along with its decoded information.
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		let mut args = String::from("ELF File Header\n");


		args += &format!("{}\n", self.flags);

		args += &format!("{}  ", if self.target.bit32() { "x32 architecture" } else if self.target.bit64() { "x64 architecture" } else { "Unknown architecture size" });

		args += &format!("{}\n", if self.target.big() { "BIG-endian" } else if self.target.little() { "little-endian" } else { "Unknown endianness" });

		args += &format!("{}\n", self.target.arch() );

		args += &format!("{}\n", self.target.osabi());

		args += &format!("{}\n", self.target.filetype());

		args += &format!("ENTRY: 0x{:08X}", self.entry);

		f.write_str(&args)
	}
}
