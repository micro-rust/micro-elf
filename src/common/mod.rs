//! Common abstractions of ELF files.


// This Source Code Form is subject to the terms of the
// Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one
// at https://mozilla.org/MPL/2.0/.


mod abi;
mod arch;


pub use self::abi::OsTarget;
pub use self::arch::Architecture;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Endianness {
	Big,
	Little,
}

impl std::fmt::Display for Endianness {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		let s = match *self {
			Endianness::Big => "Big Endian",
			Endianness::Little => "Little Endian",
		};

		write!(f, "{}", s)
	}
}