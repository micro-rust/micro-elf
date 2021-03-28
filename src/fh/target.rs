//! Target Descriptor.
//! Abstraction for ease of use of the File Header data.


// This Source Code Form is subject to the terms of the
// Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one
// at https://mozilla.org/MPL/2.0/.


use crate::common::{ Architecture, TargetOS, ELFType };

/// ELF Target Descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ELFTarget {
	/// Raw data of the OS and ABI information.
	rawabi: u32,

	/// Raw data of the architecture information.
	rawarch: u32,

	/// File type of the ELF.
	filetype: ELFType,

	/// OS ABI target.
	os: TargetOS,

	/// Instruction Set Architecture.
	arch: Architecture,
}

impl ELFTarget {
	/// Parses the given data into an ELF Target Descriptor.
	pub fn parse(d: &[u8]) -> ELFTarget {
		// Check ELF magic.
		match (d[0], d[1], d[2], d[3]) {
			(0x7F, 0x45, 0x4C, 0x46) => (),
			s => panic!("File given is not ELF type. Signature: {:?}", s),
		}

		// Get Instruction Set Architecture (enum and raw) and x32/x64 and endianness.
		let rawarchb = byteread!(32, d[0x12], d[0x13], 0, 0);
		let rawarchx = byteread!(32, d[0x04], d[0x05], 0, 0);

		let arch = Architecture::from(rawarchb as u16);

		let rawarch = (rawarchx << 16) | rawarchb;


		// Get OS ABI, OS ABI version and File Type as raw data.
		let rawabib = byteread!(32, d[0x10], d[0x11], 0, 0);
		let rawabix = byteread!(32, d[0x07], d[0x08], 0, 0);

		let filetype = ELFType::parse(rawabib as u16);

		let rawabi = (rawabix << 16) | rawabib;

		let os = TargetOS::from((d[7], d[8]));

		ELFTarget { rawabi, rawarch, filetype, os, arch }
	}

	/// Returns `true` if the target has a 32-bit architecture.
	#[inline]
	pub fn bit32(&self) -> bool {
		(self.rawarch & 0x00FF0000) == 0x00010000
	}

	/// Returns `true` if the target has a 64-bit architecture.
	#[inline]
	pub fn bit64(&self) -> bool {
		(self.rawarch & 0x00FF0000) == 0x00020000
	}

	/// Returns `true` if the target has a little endian architecture.
	#[inline]
	pub fn little(&self) -> bool {
		(self.rawarch & 0xFF000000) == 0x01000000
	}

	/// Returns `true` if the target has a big endian architecture.
	#[inline]
	pub fn big(&self) -> bool {
		(self.rawarch & 0xFF000000) == 0x02000000
	}

	/// Returns the OS ABI.
	#[inline]
	pub fn osabi(&self) -> TargetOS {
		self.os
	}

	/// Returns the Architecture.
	#[inline]
	pub fn arch(&self) -> Architecture {
		self.arch
	}

	/// Returns the File Type.
	#[inline]
	pub fn filetype(&self) -> ELFType {
		self.filetype
	}
}


impl core::fmt::Display for ELFTarget {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		write!(f,
			"{}-bit {}-endian\n{}\n{}",
			if self.bit32() { "32" } else if self.bit64() { "64" } else { "??" },
			if self.big() { "big" } else if self.little() { "little" } else { "??" },
			self.arch,
			self.os
		)
	}
}