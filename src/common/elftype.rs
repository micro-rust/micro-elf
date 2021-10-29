//! File Type of an ELF file type.


// This Source Code Form is subject to the terms of the
// Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one
// at https://mozilla.org/MPL/2.0/.


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ELFType {
	None,
	Relocatable,
	Executable,
	Dynamic,
	Core,

	OSLow,
	OSHigh,

	ProcessLow,
	ProcessHigh,
}


impl core::convert::From<u16> for ELFType {
	fn from(d: u16) -> ELFType {
		match d {
			0x01 => ELFType::Relocatable,
			0x02 => ELFType::Executable,
			0x03 => ELFType::Dynamic,
			0x04 => ELFType::Core,

			0xFE00 => ELFType::OSLow,
			0xFEFF => ELFType::OSHigh,

			0xFF00 => ELFType::ProcessLow,
			0xFFFF => ELFType::ProcessHigh,

			_ => ELFType::None,
		}
	}
}


impl core::fmt::Display for ELFType {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		use ELFType::*;

		let arg = match *self {
			None => "Unknown ELF file type",
			Relocatable => "Relocatable file",
			Executable => "Executable file",
			Dynamic => "Dynamic linked file",
			Core => "Core file",

			OSLow => "OS Specific file (low)",
			OSHigh => "OS SPecific file (high)",

			ProcessLow => "Process file (low)",
			ProcessHigh => "Process file (high)",
		};

		write!(f, "{}", arg)
	}
}
