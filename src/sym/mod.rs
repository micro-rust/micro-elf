//! Symbol abstraction.

// This Source Code Form is subject to the terms of the
// Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one
// at https://mozilla.org/MPL/2.0/.


mod flags;

use crate::sh::SectHeader;

pub use self::flags::{
	SymbolBind, SymbolType, SymbolVisibility
};



#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Symbol {
	/// Name of the symbol.
	name: String,

	/// Index of the symbol name inside the strtab section.
	nameidx: u32,

	/// Address of the symbol.
	addr: u32,

	/// Size of the object pointed at.
	size: u32,

	/// Symbol binding.
	bind: SymbolBind,

	/// Symbol Type.
	stype: SymbolType,

	/// Symbol visibility.
	vis: SymbolVisibility,

	/// Section index.
	shidx: u16,

	/// Pointers to the file section that contains the symbol.
	data: Option<(usize, usize)>,
}

impl Symbol {
	/// Parses an entry in the symbol table.
	pub fn parse(entry: &[u8], strtab: &[u8]) -> Symbol {
		// Get name index first.
		let nameidx = byteread!(32, entry[0x0], entry[0x1], entry[0x2], entry[0x3]);

		// Get address.
		let addr = byteread!(32, entry[0x4], entry[0x5], entry[0x6], entry[0x7]);

		// Get size of the symbol.
		let size = byteread!(32, entry[0x8], entry[0x9], entry[0xA], entry[0xB]);

		// Get info.
		let info = entry[0xC];

		let bind = SymbolBind::from( (info >> 4) & 0xF );
		let stype = SymbolType::from( info & 0xF );

		// Get visibility.
		let vis = SymbolVisibility::from( entry[0xD] );

		// Get section index.
		let shidx = byteread!(16, entry[0xE], entry[0xF]);


		// Read name from strtab.
		let mut idx = nameidx as usize;

		while strtab[idx] != 0x00 { idx += 1; }

		let name = unsafe { String::from_utf8_unchecked( strtab[nameidx as usize..idx].to_vec() ) };

		Symbol { name, nameidx, addr, size, bind, stype, vis, shidx, data: None }
	}

	/// Links the symbol to the file offset.
	pub fn link(&mut self, sections: &[SectHeader]) {
		if self.size == 0 { return; }
		if self.addr == 0 { return; }

		let offset = sections[self.shidx as usize].offset();
		let virt = sections[self.shidx as usize].virt();

		if (self.addr as usize) < virt { return; }

		let delta = self.addr as usize - virt;
		let s = offset + delta;

		let e = s + self.size as usize;

		self.data = Some( (s, e) );
	}

	/// Returns the contents of the symbol read from the raw data.
	pub fn content<'a>(&self, data: &'a [u8]) -> Option<&'a [u8]> {
		if let Some((s, e)) = self.data {
			Some( &data[s..e] )
		} else {
			None
		}
	}

	/// Returns the name of the symbol.
	#[inline]
	pub fn name(&self) -> String {
		self.name.clone()
	}

	/// Returns `true` if the symbol is a function.
	#[inline]
	pub fn is_function(&self) -> bool {
		self.stype == SymbolType::Function
	}
}

impl core::fmt::Display for Symbol {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		const KB1 : u32 = 1024              ;
		const MB1 : u32 = 1024 * 1024       ;
		const GB1 : u32 = 1024 * 1024 * 1024;

		const KB1_ : u32 = 1024               - 1;
		const MB1_ : u32 = 1024 * 1024        - 1;
		const GB1_ : u32 = 1024 * 1024 * 1024 - 1;

		const KB1F : f32 = KB1 as f32;
		const MB1F : f32 = MB1 as f32;
		const GB1F : f32 = GB1 as f32;

		let mut args = String::new();

		args += &format!("0x{:08X} ", self.addr);

		match self.size {
			0  ..=KB1_      => args += &format!(" [  {:4}  B]",  self.size               ),
			KB1..=MB1_      => args += &format!(" [{:6.1} KB]" , (self.size as f32) / KB1F),
			MB1..=GB1_      => args += &format!(" [{:6.1} MB]" , (self.size as f32) / MB1F),
			GB1..=u32::MAX  => args += &format!(" [{:6.1} GB]" , (self.size as f32) / GB1F),
		}

		args += &format!(" {} {} {}", self.stype, self.bind, self.vis);

		if let Some((a, _)) = self.data {
			args += &format!(" link @ 0x{:08X}", a);
		} else {
			args += "                  ";
		}

		args += &format!(" <{}>", self.name);

		write!(f, "{}", args)
	}
}