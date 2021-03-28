//! Program Header abstraction for 32 bit architectures.

// This Source Code Form is subject to the terms of the
// Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one
// at https://mozilla.org/MPL/2.0/.


#[derive(Debug, Clone, Copy)]
pub struct ProgHeader {
	/// Type of Program.
	ptype: ProgType,

	/// Offset of the segment in the file image.
	offset: u32,

	/// Virtual address of the segment.
	virt: u32,

	/// Physical address of the segment.
	phys: u32,

	/// Size in bytes of the segment in the file image.
	filesize: u32,

	/// Size in bytes of the segment in memory.
	memsize: u32,

	/// Flags of the Program.
	flags: u32,

	/// Alignment of the segment.
	align: u32,

	/// Raw data.
	raw: [u8; 0x20],
}


impl ProgHeader {
	pub fn parse(data: &[u8]) -> ProgHeader {
		// Get the p_type
		let ptype = ProgType::from( byteread!(32, data[0x00], data[0x01], data[0x02], data[0x03]) );

		// Get the offset.
		let offset = byteread!(32, data[0x04], data[0x05], data[0x06], data[0x07]);

		// Get the virtual and physical address.
		let virt = byteread!(32, data[0x08], data[0x09], data[0x0A], data[0x0B]);
		let phys = byteread!(32, data[0x0C], data[0x0D], data[0x0E], data[0x0F]);

		// Get file size and memory size.
		let filesize = byteread!(32, data[0x10], data[0x11], data[0x12], data[0x13]);
		let memsize  = byteread!(32, data[0x14], data[0x15], data[0x16], data[0x17]);

		// Get flags.
		let flags = byteread!(32, data[0x18], data[0x19], data[0x1A], data[0x1B]);

		// Get alignment.
		let align = byteread!(32, data[0x1C], data[0x1D], data[0x1E], data[0x1F]);

		let mut raw = [0; 32];

		for i in 0..32 {
			raw[i] = data[i];
		}

		ProgHeader { ptype, offset, virt, phys, filesize, memsize, flags, align, raw }
	}
}

impl core::fmt::Display for ProgHeader {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		let mut args = String::from("Program Header");

		args += &format!(" : {}\n", self.ptype);

		args += match self.flags & 0xF {
			0x00 => "FLAGS : ---\n",
			0x01 => "FLAGS : --X\n",
			0x02 => "FLAGS : -W-\n",
			0x03 => "FLAGS : -WX\n",
			0x04 => "FLAGS : R--\n",
			0x05 => "FLAGS : R-X\n",
			0x06 => "FLAGS : RW-\n",
			0x07 => "FLAGS : RWX\n",

			_    => "FLAGS : ---\n",
		};

		args += &format!("  Image offset     : 0x{:08X}\n", self.offset);

		args += &format!("  Virtual address  : 0x{:08X}\n", self.virt);

		args += &format!("  Physical address : 0x{:08X}\n", self.phys);

		args += &format!("  File size        : {} bytes\n", self.filesize);

		args += &format!("  Memory size      : {} bytes\n", self.memsize);

		args += &format!("  Alignment        : {} bytes\n", self.align);


		write!(f, "{}\n", args)
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum ProgType {
	Null        = 0x00000000,
	Load        = 0x00000001,
	Dynamic     = 0x00000002,
	Interp      = 0x00000003,
	Note        = 0x00000004,
	SharedLib   = 0x00000005,
	ProgHeader  = 0x00000006,
	TLS         = 0x00000007,
	OSLow       = 0x60000000,
	OSHigh      = 0x6FFFFFFF,
	ProcessLow  = 0x70000000,
	ProcessHigh = 0x7FFFFFFF,
}


impl core::convert::From<u32> for ProgType {
	fn from(t: u32) -> ProgType {
		use ProgType::*;

		match t {
			0x00000001 => Load,
			0x00000002 => Dynamic,
			0x00000003 => Interp,
			0x00000004 => Note,
			0x00000005 => SharedLib,
			0x00000006 => ProgHeader,
			0x00000007 => TLS,
			0x60000000 => OSLow,
			0x6FFFFFFF => OSHigh,
			0x70000000 => ProcessLow,
			0x7FFFFFFF => ProcessHigh,

			_ => Null,
		}
	}
}

impl core::fmt::Display for ProgType {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		use ProgType::*;

		let arg = match *self {
			Null        => "Null",
			Load        => "Loadable",
			Dynamic     => "Dynamic Linking",
			Interp      => "Interpreter",
			Note        => "Note",
			SharedLib   => "Shared Library",
			ProgHeader  => "Program Header Table",
			TLS         => "Thread Local Storage",
			OSLow       => "OS Specific (Low)",
			OSHigh      => "OS Specific (High)",
			ProcessLow  => "Process Specific (Low)",
			ProcessHigh => "Process Specific (High)",
		};

		write!(f, "{}", arg)
	}
}
