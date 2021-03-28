//! Iterator over Table entries.


// This Source Code Form is subject to the terms of the
// Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one
// at https://mozilla.org/MPL/2.0/.



#[derive(Debug, Clone)]
pub struct TableIterator<'a> {
	/// Data that is iterated over.
	data: &'a [u8],

	/// Current index.
	idx: usize,

	/// Minimum offset.
	offset: usize,

	/// Size of each header.
	size: usize,

	/// Number of headers.
	num: usize,
}

impl<'a> TableIterator<'a> {
	/// Creates the iterator over the given data.
	pub fn create(data: &'a [u8], offset: usize, size: usize, num: usize) -> TableIterator<'a> {
		TableIterator {
			data,
			idx: 0,
			offset,
			size,
			num,
		}
	}
}


impl<'a> core::iter::Iterator for TableIterator<'a> {
	type Item = &'a [u8];

	fn next(&mut self) -> Option<Self::Item> {
		if self.idx >= self.num { return None; }

		let s = self.offset + (self.idx * self.size);

		self.idx += 1;

		let e = self.offset + (self.idx * self.size);

		Some( &self.data[s..e] )
	}
} 