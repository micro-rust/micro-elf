//! Target flags.

// This Source Code Form is subject to the terms of the
// Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one
// at https://mozilla.org/MPL/2.0/.


#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ARMFlags(u32);


impl ARMFlags {
	/// Creates the flags from the given `u32`.
	pub fn create(d: u32) -> Self {
		ARMFlags(d)
	}
}

impl core::fmt::Debug for ARMFlags {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		let mut args = String::new();

		args += &format!("[ABI {}", (self.0 >> 24) & 0xFF);

		match self.0 & 0x00800000 {
			0 => (),
			_ => args += ", BE-8 code",
		}

		match self.0 & 0xFF00 {
			0x0400 => args += ", hard-float",
			0x0200 => args += ", soft-float",
			_ => (),
		}

		write!(f, "{}]", args)
	}
}

impl core::fmt::Display for ARMFlags {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		let mut args = String::new();

		args += &format!("[ABI {}", (self.0 >> 24) & 0xFF);

		match self.0 & 0x00800000 {
			0 => (),
			_ => args += ", BE-8 code",
		}

		match self.0 & 0xFF00 {
			0x0400 => args += ", hard-float",
			0x0200 => args += ", soft-float",
			_ => (),
		}

		write!(f, "{}]", args)
	}
}
