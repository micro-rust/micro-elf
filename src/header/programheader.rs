//! ELF Program Header.

// This Source Code Form is subject to the terms of the
// Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one
// at https://mozilla.org/MPL/2.0/.


use byteorder::{ NativeEndian, ReadBytesExt };


use crate::ElfArch;
use crate::common::ProgramType;



pub struct ProgramHeader<ARCH: ElfArch> {
	/// Program Type.
	ptype: ProgramType,

	/// Program flags.
	pflags: u32,

	/// Offset of the segment in memory.
	offset: ARCH::Address,

	/// Virtual address of the segment in memory.
	vaddr: ARCH::Address,

	/// Physical address of the segment in physical memory.
	paddr: ARCH::Address,

	/// Size in bytes of the segment in the file image.
	filesize: ARCH::Address,

	/// Size in bytes of the segment in memory.
	memsize: ARCH::Address,

	/// Alignment of the section.
	align: ARCH::Address,
}


impl<ARCH: ElfArch> ProgramHeader<ARCH> {
	/// Parses the file header.
	pub fn parse(data: &[u8]) -> Self {
		// Get program type.
		let ptype = ProgramType::from( (&data[0..0x4]).read_u32::<NativeEndian>().unwrap() );

		// Get flags.
		let pflags = ARCH::slice( data, ARCH::PFLAGS ).read_u32::<NativeEndian>().unwrap();

		// Get dynamic index.
		let mut index = ARCH::POFFSET;

		// Get program offset in the file image.
		let offset = ARCH::read( ARCH::slice( data, index ) );
		index += ARCH::ADDRSIZE;

		// Get virtual address.
		let vaddr = ARCH::read( ARCH::slice( data, index ) );
		index += ARCH::ADDRSIZE;

		// Get physical address.
		let paddr = ARCH::read( ARCH::slice( data, index ) );
		index += ARCH::ADDRSIZE;

		// Get size in file.
		let filesize = ARCH::read( ARCH::slice( data, index ) );
		index += ARCH::ADDRSIZE;

		// Get size in memory.
		let memsize = ARCH::read( ARCH::slice( data, index ) );

		// Get alignment.
		let align = ARCH::read( ARCH::slice( data, ARCH::PALIGN ) );

		Self {
			ptype,
			pflags,
			offset,
			vaddr,
			paddr,
			filesize,
			memsize,
			align,
		}
	}
}


impl<ARCH: ElfArch> std::fmt::Display for ProgramHeader<ARCH> {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		let mut args = String::new();

		// Program type.
		args += &format!("  Program Header\n    Type: {}\n", self.ptype);

		// Program flags.
		args += &format!("    {:b}\n", self.pflags);

		// Program offset in file.
		args += &format!("    Program offset in file: 0x{:08X}\n",   self.offset);

		// Virtual and physical address.
		args += &format!("    Program Virtual address:  0x{:08X}\n",   self.vaddr);
		args += &format!("    Program Physical address: 0x{:08X}\n\n", self.paddr);

		// Program size in file and memory.
		args += &format!("    Program File size:   {}\n",   self.filesize);
		args += &format!("    Program Memory size: {}\n\n", self.memsize);

		// Alignment of the section.
		args += &format!("    Alignment: 2 << {}\n\n", self.align);

		// Index of the 

		write!(f, "{}", args)
	}
}
