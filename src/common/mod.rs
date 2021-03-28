//! Common abstractions of ELF files.


// This Source Code Form is subject to the terms of the
// Mozilla Public License, v. 2.0. If a copy of the MPL
// was not distributed with this file, You can obtain one
// at https://mozilla.org/MPL/2.0/.

#[macro_use]
mod macros;


mod abi;
mod arch;
mod filetype;
mod iterator;


pub use self::abi::TargetOS;
pub use self::arch::Architecture;
pub use self::filetype::ELFType;

pub use self::iterator::TableIterator;

