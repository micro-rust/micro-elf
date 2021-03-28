//! Section Header Flags.

// This Source Code Form is subject to the terms of the
// Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one
// at https://mozilla.org/MPL/2.0/.

use core::convert::From;

use core::ops::BitAnd;
use core::ops::BitOr;

pub type SectionFlags32 = SectionFlags<u32>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SectionFlags<T>(T);

impl<T : BitAnd<T, Output=T> + Copy + PartialEq> SectionFlags<T> {
	/// Returns `true` if the given flag is present.
	pub fn contains(&self, flag: T) -> bool {
		(self.0 & flag) == flag
	}
}

impl<T : BitAnd<T> + BitOr<T> + PartialEq> From<T> for SectionFlags<T> {
	fn from(x: T) -> SectionFlags<T> {
		SectionFlags(x)
	}
}


impl<T : BitAnd<T, Output=T> + Copy + PartialEq> core::fmt::Display for SectionFlags<T>
	where SectionFlags<T> : ConstFlags<T> {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		let mut args = String::new();

		if self.contains( Self::WRITE     ) { args += " WRITE";     }

		if self.contains( Self::ALLOCATED ) { args += " ALLOC";     }

		if self.contains( Self::EXECUTABLE) { args += " EXEC";      }

		if self.contains( Self::MERGE     ) { args += " MERGE";     }

		if self.contains( Self::STRINGS   ) { args += " STRINGS";   }

		if self.contains( Self::INFOLINK  ) { args += " INFOLINK";  }

		if self.contains( Self::LINKORDER ) { args += " LINKORDER"; }

		if self.contains( Self::GROUP     ) { args += " GROUP";     }

		if self.contains( Self::ORDERED   ) { args += " ORDERED";   }

		if self.contains( Self::EXCLUDE   ) { args += " EXCLUDE";   }

		if self.contains( Self::OSONLY    ) { args += " OS-SPECIFIC"; }

		if self.contains( Self::PROCONLY  ) { args += " PROCESSOR-SPECIFIC"; }

		if self.contains( Self::TLS       ) { args += " THREAD LOCAL STORAGE"; }

		if self.contains( Self::NONCONFORMING) { args += " OS NON-CONFORMING"; }

		write!(f, "{}", args)
	}
}


pub trait ConstFlags<T : BitAnd<T, Output=T> + Copy + PartialEq> {
	/// Section is Writable.
	const WRITE : T;

	/// Section occupies memory during execution.
	const ALLOCATED : T;

	/// Section is executable.
	const EXECUTABLE : T;

	/// Section might be merged.
	const MERGE : T;

	/// Section contains Null terminated strings.
	const STRINGS : T;

	/// Section contains SHT index.
	const INFOLINK : T;

	/// Preserve order after combining.
	const LINKORDER : T;

	/// Indicates non-standard OS specific ahndling.
	const NONCONFORMING : T;

	/// Section is a member of a group.
	const GROUP : T;

	/// Section holds thread-local data.
	const TLS : T;

	/// OS Specific.
	const OSONLY : T;

	/// Processor specific.
	const PROCONLY : T;

	/// Special ordering required (Solaris).
	const ORDERED : T;

	/// Section is excluded unless referenced or allocated.
	const EXCLUDE : T;
}


impl ConstFlags<u64> for SectionFlags<u64> {
	/// Section is Writable.
	const WRITE : u64 = 0x00000001;

	/// Section occupies memory during execution.
	const ALLOCATED : u64 = 0x00000002;

	/// Section is executable.
	const EXECUTABLE : u64 = 0x00000004;

	/// Section might be merged.
	const MERGE : u64 = 0x00000010;

	/// Section contains Null terminated strings.
	const STRINGS : u64 = 0x00000020;

	/// Section contains SHT index.
	const INFOLINK : u64 = 0x00000040;

	/// Preserve order after combining.
	const LINKORDER : u64 = 0x00000040;

	/// Indicates non-standard OS specific ahndling.
	const NONCONFORMING : u64 = 0x00000100;

	/// Section is a member of a group.
	const GROUP : u64 = 0x00000200;

	/// Section holds thread-local data.
	const TLS : u64 = 0x00000400;

	/// OS Specific.
	const OSONLY : u64 = 0x0FF00000;

	/// Processor specific.
	const PROCONLY : u64 = 0xF0000000;

	/// Special ordering required (Solaris).
	const ORDERED : u64 = 0x04000000;

	/// Section is excluded unless referenced or allocated.
	const EXCLUDE : u64 = 0x08000000;
}


impl ConstFlags<u32> for SectionFlags<u32> {
	/// Section is Writable.
	const WRITE : u32 = 0x00000001;

	/// Section occupies memory during execution.
	const ALLOCATED : u32 = 0x00000002;

	/// Section is executable.
	const EXECUTABLE : u32 = 0x00000004;

	/// Section might be merged.
	const MERGE : u32 = 0x00000010;

	/// Section contains Null terminated strings.
	const STRINGS : u32 = 0x00000020;

	/// Section contains SHT index.
	const INFOLINK : u32 = 0x00000040;

	/// Preserve order after combining.
	const LINKORDER : u32 = 0x00000040;

	/// Indicates non-standard OS specific ahndling.
	const NONCONFORMING : u32 = 0x00000100;

	/// Section is a member of a group.
	const GROUP : u32 = 0x00000200;

	/// Section holds thread-local data.
	const TLS : u32 = 0x00000400;

	/// OS Specific.
	const OSONLY : u32 = 0x0FF00000;

	/// Processor specific.
	const PROCONLY : u32 = 0xF0000000;

	/// Special ordering required (Solaris).
	const ORDERED : u32 = 0x04000000;

	/// Section is excluded unless referenced or allocated.
	const EXCLUDE : u32 = 0x08000000;
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SectionType {
	Null,
	ProgramBits,
	SymbolTable,
	StringTable,
	RelocationA,
	Hash,
	Dynamic,
	Note,
	NoBits,
	Relocation,
	SharedLibrary,
	DynamicSymbol,
	InitArray,
	FiniArray,
	PreinitArray,
	Group,
	XSymbolTable,
	Number,
	OSOnly,
	ProcessOnly,
}


impl core::convert::From<u32> for SectionType {
	fn from(d: u32) -> SectionType {
		use SectionType::*;

		match d {
			0x01 => ProgramBits,
			0x02 => SymbolTable,
			0x03 => StringTable,
			0x04 => RelocationA,
			0x05 => Hash,
			0x06 => Dynamic,
			0x07 => Note,
			0x08 => NoBits,
			0x09 => Relocation,
			0x0A => SharedLibrary,
			0x0B => DynamicSymbol,
			0x0E => InitArray,
			0x0F => FiniArray,
			0x10 => PreinitArray,
			0x11 => Group,
			0x12 => XSymbolTable,
			0x13 => Number,

			0x60000000..=0x6FFFFFFF => OSOnly,
			0x70000000..=0x7FFFFFFF => ProcessOnly,

			_ => Null,
		}
	}
}

impl core::convert::From<u64> for SectionType {
	fn from(d: u64) -> SectionType {
		use SectionType::*;

		match d {
			0x01 => ProgramBits,
			0x02 => SymbolTable,
			0x03 => StringTable,
			0x04 => RelocationA,
			0x05 => Hash,
			0x06 => Dynamic,
			0x07 => Note,
			0x08 => NoBits,
			0x09 => Relocation,
			0x0A => SharedLibrary,
			0x0B => DynamicSymbol,
			0x0E => InitArray,
			0x0F => FiniArray,
			0x10 => PreinitArray,
			0x11 => Group,
			0x12 => XSymbolTable,
			0x13 => Number,

			0x60000000..=0x6FFFFFFF => OSOnly,
			0x70000000..=0x7FFFFFFF => ProcessOnly,

			_ => Null,
		}
	}
}


impl core::fmt::Display for SectionType {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		use SectionType::*;

		let arg = match *self {
			Null          => "NULL",
			ProgramBits   => "Program Data",
			SymbolTable   => "Symbol Table",
			StringTable   => "String Table",
			RelocationA   => "Relocation entries with addends",
			Hash          => "Symbol Hash Table",
			Dynamic       => "Dynamic linking information",
			Note          => "Notes",
			NoBits        => "Program Space with no data (bss)",
			Relocation    => "Relocation entries without addends",
			SharedLibrary => "Shared Library (reserved)",
			DynamicSymbol => "Dynamic Symbol Table",
			InitArray     => "Array of Constructors",
			FiniArray     => "Array of Destructors",
			PreinitArray  => "Array of Pre-Constructors",
			Group         => "Sectio Group",
			XSymbolTable  => "Extended Section Indices",
			Number        => "Number of defined types",
			OSOnly        => "OS Specific",
			ProcessOnly   => "Process Specific",
		};

		write!(f, "{}", arg)
	}
}
