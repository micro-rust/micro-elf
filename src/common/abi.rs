//! OS ABI target.

// This Source Code Form is subject to the terms of the
// Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one
// at https://mozilla.org/MPL/2.0/.


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OsTarget {
	SystemV(u8),
	HPUX(u8),
	NetBSD(u8),
	Linux(u8),
	GNUHurd(u8),
	Solaris(u8),
	AIX(u8),
	IRIX(u8),
	FreeBSD(u8),
	Tru64(u8),
	NovellModesto(u8),
	OpenBSD(u8),
	OpenVMS(u8),
	NonStopKernel(u8),
	AROS(u8),
	FenixOS(u8),
	CloudABI(u8),
	OpenVOS(u8),

	None,
}



impl core::convert::From<(u8, u8)> for OsTarget {
	fn from((os, v): (u8, u8)) -> OsTarget {
		use OsTarget::*;

		match os {
			0x00 => SystemV(v),
			0x01 => HPUX(v),
			0x02 => NetBSD(v),
			0x03 => Linux(v),
			0x04 => GNUHurd(v),
			0x06 => Solaris(v),
			0x07 => AIX(v),
			0x08 => IRIX(v),
			0x09 => FreeBSD(v),
			0x0A => Tru64(v),
			0x0B => NovellModesto(v),
			0x0C => OpenBSD(v),
			0x0D => OpenVMS(v),
			0x0E => NonStopKernel(v),
			0x0F => AROS(v),
			0x10 => FenixOS(v),
			0x11 => CloudABI(v),
			0x12 => OpenVOS(v),

			_ => None,
		}
	}
}

impl core::convert::From<u8> for OsTarget {
	fn from(os: u8) -> OsTarget {
		use OsTarget::*;

		match os {
			0x00 => SystemV(0),
			0x01 => HPUX(0),
			0x02 => NetBSD(0),
			0x03 => Linux(0),
			0x04 => GNUHurd(0),
			0x06 => Solaris(0),
			0x07 => AIX(0),
			0x08 => IRIX(0),
			0x09 => FreeBSD(0),
			0x0A => Tru64(0),
			0x0B => NovellModesto(0),
			0x0C => OpenBSD(0),
			0x0D => OpenVMS(0),
			0x0E => NonStopKernel(0),
			0x0F => AROS(0),
			0x10 => FenixOS(0),
			0x11 => CloudABI(0),
			0x12 => OpenVOS(0),

			_ => None,
		}
	}
}


impl core::fmt::Display for OsTarget {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		use OsTarget::*;

		let arg = match *self {
			SystemV(v)       => format!("System V - rev {}", v),
			HPUX(v)          => format!("HP-UX - rev {}", v),
			NetBSD(v)        => format!("NetBSD - rev {}", v),
			Linux(v)         => format!("Linux - rev {}", v),
			GNUHurd(v)       => format!("GNU Hurd - rev {}", v),
			Solaris(v)       => format!("Solaris - rev {}", v),
			AIX(v)           => format!("AIX - rev {}", v),
			IRIX(v)          => format!("IRIX - rev {}", v),
			FreeBSD(v)       => format!("FreeBSD - rev {}", v),
			Tru64(v)         => format!("Tru64 - rev {}", v),
			NovellModesto(v) => format!("Novell Modesto - rev {}", v),
			OpenBSD(v)       => format!("OpenBSD - rev {}", v),
			OpenVMS(v)       => format!("OpenVMS - rev {}", v),
			NonStopKernel(v) => format!("NonStop Kernel - rev {}", v),
			AROS(v)          => format!("AROS - rev {}", v),
			FenixOS(v)       => format!("Fenix OS - rev {}", v),
			CloudABI(v)      => format!("Cload ABI - rev {}", v),
			OpenVOS(v)       => format!("Stratus Technologies OpenVOS - rev {}", v),

			None => String::from("No OS ABI"),
		};

		write!(f, "{}", arg)
	}
}