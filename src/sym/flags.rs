// This Source Code Form is subject to the terms of the
// Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one
// at https://mozilla.org/MPL/2.0/.


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolBind {
	Local,
	Global,
	Weak,
	Number,
	UniqueGNU,
	OSOnly,
	ProcessorOnly,
}


impl core::convert::From<u8> for SymbolBind {
	fn from(x: u8) -> SymbolBind {
		use SymbolBind::*;

		match x {
			0x01 => Global,
			0x02 => Weak,
			0x03 => Number,
			0x10 => UniqueGNU,
			0x11..=0x12 => OSOnly,
			0x13..=0x15 => ProcessorOnly,

			_    => Local,
		}
	}
}

impl core::fmt::Display for SymbolBind {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		use SymbolBind::*;

		let arg = match *self {
			Local         => "LOCAL     ",
			Global        => "GLOBAL    ",
			Weak          => "WEAK      ",
			Number        => "NUMBER    ",
			UniqueGNU     => "GNU UNIQUE",
			OSOnly        => "OS        ",
			ProcessorOnly => "PROCESSOR ",
		};


		write!(f, "{}", arg)
	}
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolType {
	Unspecified,
	Object,
	Function,
	Section,
	FileName,
	Common,
	TLS,
	Number,
	GNUIFunction,
	OSOnly,
	ProcessorOnly,
}


impl core::convert::From<u8> for SymbolType {
	fn from(x: u8) -> SymbolType {
		use SymbolType::*;

		match x {
			0x01 => Object,
			0x02 => Function,
			0x03 => Section,
			0x04 => FileName,
			0x05 => Common,
			0x06 => TLS,
			0x07 => Number,
			0x10 => GNUIFunction,

			0x11..=0x12 => OSOnly,
			0x13..=0x15 => ProcessorOnly,

			_    => Unspecified,
		}
	}
}


impl core::fmt::Display for SymbolType {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		use SymbolType::*;

		let arg = match *self {
			Unspecified   => "UNSPECIFED",
			Object        => "OBJECT    ",
			Function      => "FUNCTION  ",
			Section       => "SECTION   ",
			FileName      => "FILENAME  ",
			Common        => "COMMON    ",
			TLS           => "TLS       ",
			Number        => "NUMBER    ",
			GNUIFunction  => "GNU IFUNC ",
			OSOnly        => "OS        ",
			ProcessorOnly => "PROCESSOR ",
		};


		write!(f, "{}", arg)
	}
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolVisibility {
	Default,
	Internal,
	Hidden,
	Protected,
}

impl core::convert::From<u8> for SymbolVisibility {
	fn from(x: u8) -> SymbolVisibility {
		use SymbolVisibility::*;

		match x & 0x3 {
			0x1 => Internal,
			0x2 => Hidden,
			0x3 => Protected,

			_ => Default,
		}
	}
}

impl core::fmt::Display for SymbolVisibility {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		use SymbolVisibility::*;

		let arg = match *self {
			Default   => "DEFAULT  ",
			Internal  => "INTERNAL ",
			Hidden    => "HIDDEN   ",
			Protected => "PROTECTED",
		};


		write!(f, "{}", arg)
	}
}