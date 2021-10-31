//! Section header module.
//! Contains all information for a ELF section's header.


#![allow(dead_code)]

use super::{ SectionFlags32, SectionFlags64, SectionType };



#[derive(Debug, Clone)]
pub struct SectionHeader32 {
    /// Section name.
    pub(super) name: String,

    /// Section name offset in .shstrtab.
    pub(super) nameidx: u32,

    /// Section type.
    pub(super) sectiontype: SectionType,

    /// Section attributes.
    pub(super) flags: SectionFlags32,

    /// Virtual address of the section in memory.
    pub(super) vaddr: u32,

    /// Offset to the section in the file.
    pub(super) offset: u32,

    /// Size in bytes of the section in the file.
    pub(super) filesize: u32,

    /// Section index of an associated section.
    pub(super) link: u32,

    /// Extra information about the section.
    pub(super) extrainfo: u32,

    /// Required alignment of the section.
    pub(super) alignment: u32,

    /// Size in bytes of each entry for fixed size entry sections.
    pub(super) entrysize: u32,
}


#[derive(Debug, Clone)]
pub struct SectionHeader64 {
    /// Section name.
    pub(super) name: String,

    /// Section name offset in .shstrtab.
    pub(super) nameidx: u32,

    /// Section type.
    pub(super) sectiontype: SectionType,

    /// Section attributes.
    pub(super) flags: SectionFlags64,

    /// Virtual address of the section in memory.
    pub(super) vaddr: u64,

    /// Offset to the section in the file.
    pub(super) offset: u64,

    /// Size in bytes of the section in the file.
    pub(super) filesize: u64,

    /// Section index of an associated section.
    pub(super) link: u32,

    /// Extra information about the section.
    pub(super) extrainfo: u32,

    /// Required alignment of the section.
    pub(super) alignment: u64,

    /// Size in bytes of each entry for fixed size entry sections.
    pub(super) entrysize: u64,
}
