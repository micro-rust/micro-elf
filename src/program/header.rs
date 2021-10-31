//! Program header module.
//! Contains all information for a ELF program's header.


#![allow(dead_code)]


use super::{ ProgramType, ProgramFlags32,ProgramFlags64 };


pub struct ProgramHeader32 {
    /// Program Type.
    pub(super) programtype: ProgramType,

    /// Program flags.
    pub(super) flags: ProgramFlags32,

    /// Offset of the segment in memory.
    pub(super) offset: u32,

    /// Virtual address of the segment in memory.
    pub(super) vaddr: u32,

    /// Physical address of the segment in physical memory.
    pub(super) paddr: u32,

    /// Size in bytes of the segment in the file image.
    pub(super) filesize: u32,

    /// Size in bytes of the segment in memory.
    pub(super) memsize: u32,

    /// Alignment of the section.
    pub(super) alignment: u32,
}


pub struct ProgramHeader64 {
    /// Program Type.
    pub(super) programtype: ProgramType,

    /// Program flags.
    pub(super) flags: ProgramFlags64,

    /// Offset of the segment in memory.
    pub(super) offset: u64,

    /// Virtual address of the segment in memory.
    pub(super) vaddr: u64,

    /// Physical address of the segment in physical memory.
    pub(super) paddr: u64,

    /// Size in bytes of the segment in the file image.
    pub(super) filesize: u64,

    /// Size in bytes of the segment in memory.
    pub(super) memsize: u64,

    /// Alignment of the section.
    pub(super) alignment: u64,
}
