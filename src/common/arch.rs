//! Architecture information.

// This Source Code Form is subject to the terms of the
// Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one
// at https://mozilla.org/MPL/2.0/.


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum Architecture {
	None       = 0x000,
	ATT        = 0x001,
	SPARC      = 0x002,
	X86        = 0x003,
	M68k       = 0x004,
	M88k       = 0x005,
	IntelMCU   = 0x006,
	Intel80860 = 0x007,
	MIPS       = 0x008,
	IBM370     = 0x009,
	MIPSRS3000 = 0x00A,
	PARISC     = 0x00E,

	VPP500     = 0x011,
	SPARCPlus  = 0x012,
	Intel80960 = 0x013,
	PowerPC    = 0x014,
	PowerPCX64 = 0x015,
	IBM390     = 0x016,

	V800       = 0x024,
	FR20       = 0x025,
	TRWRH32    = 0x026,
	MRCE       = 0x027,
	Aarch32    = 0x028,
	DigAlpha   = 0x029,
	SuperH     = 0x02A,
	SPARCV9    = 0x02B,
	SiemensTri = 0x02C,
	Argonaut   = 0x02D,
	H8300      = 0x02E,
	H8300H     = 0x02F,

	H8S        = 0x030,
	H8500      = 0x031,
	IA64       = 0x032,
	AMD64      = 0x03E,

	TMS320C    = 0x08C,

	Aarch64    = 0x0B7,

	RISCV      = 0x0F3,
	BPF        = 0x0F7,

	WDC65C816  = 0x101,
}


impl core::convert::From<u16> for Architecture {
	fn from(d: u16) -> Architecture {
		use Architecture::*;

		match d {
			0x001 => ATT,
			0x002 => SPARC,
			0x003 => X86,
			0x004 => M68k,
			0x005 => M88k,
			0x006 => IntelMCU,
			0x007 => Intel80860,
			0x008 => MIPS,
			0x009 => IBM370,
			0x00A => MIPSRS3000,
			0x00E => PARISC,

			0x011 => VPP500,
			0x012 => SPARCPlus,
			0x013 => Intel80960,
			0x014 => PowerPC,
			0x015 => PowerPCX64,
			0x016 => IBM390,

			0x024 => V800,
			0x025 => FR20,
			0x026 => TRWRH32,
			0x027 => MRCE,
			0x028 => Aarch32,
			0x029 => DigAlpha,
			0x02A => SuperH,
			0x02B => SPARCV9,
			0x02C => SiemensTri,
			0x02D => Argonaut,
			0x02E => H8300,
			0x02F => H8300H,

			0x032 => IA64,
			0x03E => AMD64,

			0x08C => TMS320C,

			0x0B7 => Aarch64,

			0x0F3 => RISCV,
			0x0F7 => BPF,

			0x101 => WDC65C816,

			_ => Architecture::None,
		}
	}
}

impl core::fmt::Display for Architecture {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		use Architecture::*;

		let arg = match *self {
			None       => "No arch defined",
			ATT        => "AT&T WE 32100",
			SPARC      => "SPARC",
			X86        => "x86",
			M68k       => "Motorola M68000 (M68k)",
			M88k       => "Motorola M88000 (M88k)",
			IntelMCU   => "Intel MCU",
			Intel80860 => "Intel 80860",
			MIPS       => "MIPS",
			IBM370     => "IBM System / 370",
			MIPSRS3000 => "MIPS RS3000 Little-endian",
			PARISC     => "Hewlett-Packard PA-RISC",

			VPP500     => "Fujitsu VPP500",
			SPARCPlus  => "Enhanced Instruction Set SPARC32Plus",
			Intel80960 => "Intel 80960",
			PowerPC    => "PowerPC",
			PowerPCX64 => "Power PC (64-bit)",
			IBM390     => "IBM System / 390",


			V800       => "NEC V800",
			FR20       => "Fujitsu FR20",
			TRWRH32    => "TRW RH-32",
			MRCE       => "Motorola RCE",
			Aarch32    => "Aarch32 / ARMv7",
			DigAlpha   => "Digital Alpha",
			SuperH     => "Hitachi SuperH",
			SPARCV9    => "SPARC V9",
			SiemensTri => "Siemens TriCore",
			Argonaut   => "Argonaut RISC Core",
			H8300      => "Hitachi H8/300",
			H8300H     => "Hitachi H8/300H",

			H8S        => "Hitachi H8S",
			H8500      => "Hitachi H8/500",
			IA64       => "IA-64",
			AMD64      => "amd64",

			TMS320C    => "TMS320C6000",

			Aarch64    => "ARM 64-bits (ARMv8 / Aarch64)",

			RISCV      => "RISC-V",
			BPF        => "Berkeley Packet Filter",

			WDC65C816  => "WDC 65C816",
		};

		write!(f, "{}", arg)
	}
}
