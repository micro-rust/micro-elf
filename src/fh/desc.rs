//! Header Table Descriptor.
//! Describes the table in which all the Header of a given type are indexed.

// This Source Code Form is subject to the terms of the
// Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one
// at https://mozilla.org/MPL/2.0/.

use crate::common::TableIterator;

#[derive(Clone, Copy)]
pub struct HTDescriptor {
	/// Offset in file to the Header Table.
	offset: u32,

	/// Size of a Header Table entry.
	size: u16,

	/// Number of entries in the Header Table.
	num: u16,

	/// Raw data.
	raw: [u8; 8],
}


impl HTDescriptor {
	/// Parses the file header and generates a Header Table descriptor.
	pub fn parse(data: &[u8], o: usize) -> HTDescriptor {
		// Get table offset.
		let offset = byteread!(32, data[0x1C + o], data[0x1D + o], data[0x1E + o], data[0x1F + o]);

		// Get the size of each entry.
		let size = byteread!(16, data[0x2A + o], data[0x2B + o]);

		// Get number of entries in the table.
		let num = byteread!(16, data[0x2C + o], data[0x2D + o]);

		// Read the raw data for debugging purposes.
		let raw = [
			// e_#hoff
			data[0x1C + o], data[0x1D + o], data[0x1E + o], data[0x1F + o],
			// e_#hentsize
			data[0x2A + o], data[0x2B + o],
			// e_#hnum
			data[0x2C + o], data[0x2D + o],
		];

		HTDescriptor { offset, size, num, raw }
	}

	/// Iterates over the given byte array.
	pub fn iterate<'a>(&self, data: &'a [u8]) -> TableIterator<'a> {
		TableIterator::create(data, self.offset as usize, self.size as usize, self.num as usize)
	}
}


impl core::fmt::Debug for HTDescriptor {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		let mut args = String::from("Header Table Descriptor { ");

		args += &format!("offset: {} [0x{:02X} 0x{:02X} 0x{:02X} 0x{:02X}], ", self.offset, self.raw[0], self.raw[1], self.raw[2], self.raw[3]);
		args += &format!("entry size: {} [0x{:02X} 0x{:02X}], ", self.size, self.raw[4], self.raw[5]);
		args += &format!("number of entries: {} [0x{:02X} 0x{:02X}] }}", self.num, self.raw[6], self.raw[7]);

		f.write_str(&args)
	}
}