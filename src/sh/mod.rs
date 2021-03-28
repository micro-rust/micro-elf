//! Section Header abstraction for 32 bit architectures.

// This Source Code Form is subject to the terms of the
// Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one
// at https://mozilla.org/MPL/2.0/.

use crate::common::TableIterator;

mod flags;

pub use self::flags::{
	SectionFlags32, SectionType
};

#[derive(Debug, Clone)]
pub struct SectHeader {
	/// Name of the section.
	name: String,

	/// Index of the name inside .strtab.
	stridx: u32,

	/// Type of the section header.
	stype: SectionType,

	/// Section attributes.
	flags: SectionFlags32,

	/// Virtual address of the section in memory.
	addr: u32,

	/// Offset of the section in the file image.
	off: u32,

	/// Size in bytes of the section in the file image.
	filesize: u32,

	/// Section index of an associated section.
	link: u32,

	/// Contains extra information about the section.
	info: u32,

	/// Alignment of the section.
	align: u32,

	/// Size in bytes of each entry.
	esize: u32,
}

impl SectHeader {
	pub fn parse(data: &[u8]) -> SectHeader {
		// String index.
		let stridx = byteread!(32, data[0x00], data[0x01], data[0x02], data[0x03]);

		// Section type.
		let stype = SectionType::from( byteread!(32, data[0x04], data[0x05], data[0x06], data[0x07]) );

		// Section flags.
		let flags = SectionFlags32::from( byteread!(32, data[0x08], data[0x09], data[0x0A], data[0x0B]) );

		// Virtual address and image offset.
		let addr = byteread!(32, data[0x0C], data[0x0D], data[0x0E], data[0x0F]);

		// Physical offset.
		let off = byteread!(32, data[0x10], data[0x11], data[0x12], data[0x13]);

		// Image size.
		let filesize = byteread!(32, data[0x14], data[0x15], data[0x16], data[0x17]);

		// Image size.
		let link = byteread!(32, data[0x18], data[0x19], data[0x1A], data[0x1B]);

		// Image size.
		let info = byteread!(32, data[0x1C], data[0x1D], data[0x1E], data[0x1F]);

		// Alignment.
		let align = byteread!(32, data[0x20], data[0x21], data[0x22], data[0x23]);

		// Image size.
		let esize = byteread!(32, data[0x24], data[0x25], data[0x26], data[0x27]);

		SectHeader { name: String::new(), stridx, stype, flags, addr, off, filesize, link, info, align, esize }
	}

	pub fn is_strtab(&self) -> bool {
		&self.name == ".strtab"
	}

	pub fn is_shstrtab(&self) -> bool {
		self.stype == SectionType::StringTable
	}

	pub fn is_symtab(&self) -> bool {
		&self.name == ".symtab"
	}

	pub fn iterate<'a>(&self, data: &'a [u8]) -> Option<TableIterator<'a>> {
		match self.esize {
			0 => None,
			_ => Some(
				TableIterator::create(
					data,
					self.off as usize,
					self.esize as usize,
					(self.filesize / self.esize) as usize
				)
			),
		}
	}

	pub fn rename(&mut self, strtab: &[u8]) {
		let mut idx = self.stridx as usize;

		while strtab[idx] != 0x00 { idx += 1; }

		self.name = unsafe { String::from_utf8_unchecked( strtab[self.stridx as usize..idx].to_vec() ) };
	}

	pub fn content(&self, data: &[u8]) -> Vec<u8> {
		match self.filesize {
			0 => Vec::new(),
			n => {
				let s = self.off as usize;
				let e = (self.off + n) as usize;

				data[s..e].to_vec()
			},
		}
	}

	#[inline]
	pub fn offset(&self) -> usize {
		self.off as usize
	}

	#[inline]
	pub fn virt(&self) -> usize {
		self.addr as usize
	}
}


impl core::fmt::Display for SectHeader {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		let mut args = String::from("Section");

		args += &format!(" {}:\n", self.name);

		args += &format!("  Type : {}\n", self.stype);

		args += &format!("  Flags: {}\n", self.flags);

		args += &format!("  Virt: 0x{:08X}\n", self.addr);

		args += &format!("  Content: {} bytes @ 0x{:08X}\n", self.filesize, self.off);

		args += &format!("  Alignment: {} bytes (2**{})\n", self.align, self.align.trailing_zeros());

		if self.esize != 0 {
			args += &format!("  Entry size: {} bytes\n", self.esize);
		}

		write!(f, "{}", args)
	}
}