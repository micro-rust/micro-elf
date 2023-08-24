


#![allow(dead_code)]


use crate::common::{ Architecture, Endianness, OsTarget };

use super::{ FileType, FileFlags32, FileFlags64 };


pub struct FileHeader32 {
    /// Endianness of the target hardware.
    pub(super) endianness: Endianness,

    /// Target OS ABI.
    pub(super) os: OsTarget,

    /// Object file type.
    pub(super) filetype: FileType,

    /// Target ISA.
    pub(super) isa: Architecture,

    /// Entry point.
    pub(super) entry: u32,

    /// Pointer to the Program Header table.
    pub(super) phoffset: u32,

    /// Pointer to the Section Header table.
    pub(super) shoffset: u32,

    /// Architecture flags.
    pub(super) flags: FileFlags32,

    /// Size of a Program Header table entry.
    pub(super) phentrysize: u16,

    /// Number of Program Header table entries.
    pub(super) phnum: u16,

    /// Size of a Section Header table entry.
    pub(super) shentrysize: u16,

    /// Number of Section Header table entries.
    pub(super) shnum: u16,

    /// Index of the Section Header table entry containing section names.
    pub(super) shstrndx: u16,
}


pub struct FileHeader64 {
    /// Endianness of the target hardware.
    pub(super) endianness: Endianness,

    /// Target OS ABI.
    pub(super) os: OsTarget,

    /// Object file type.
    pub(super) filetype: FileType,

    /// Target ISA.
    pub(super) isa: Architecture,

    /// Entry point.
    pub(super) entry: u64,

    /// Pointer to the Program Header table.
    pub(super) phoffset: u64,

    /// Pointer to the Section Header table.
    pub(super) shoffset: u64,

    /// Architecture flags.
    pub(super) flags: FileFlags64,

    /// Size of a Program Header table entry.
    pub(super) phentrysize: u16,

    /// Number of Program Header table entries.
    pub(super) phnum: u16,

    /// Size of a Section Header table entry.
    pub(super) shentrysize: u16,

    /// Number of Section Header table entries.
    pub(super) shnum: u16,

    /// Index of the Section Header table entry containing section names.
    pub(super) shstrndx: u16,
}
