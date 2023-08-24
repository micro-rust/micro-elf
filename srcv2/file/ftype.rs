//! File Type of an ELF file type.



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType {
    None,
    Relocatable,
    Executable,
    Dynamic,
    Core,

    OSLow,
    OSHigh,

    ProcessLow,
    ProcessHigh,
}


impl core::convert::From<u16> for FileType {
    fn from(d: u16) -> FileType {
        match d {
            0x01 => FileType::Relocatable,
            0x02 => FileType::Executable,
            0x03 => FileType::Dynamic,
            0x04 => FileType::Core,

            0xFE00 => FileType::OSLow,
            0xFEFF => FileType::OSHigh,

            0xFF00 => FileType::ProcessLow,
            0xFFFF => FileType::ProcessHigh,

            _ => FileType::None,
        }
    }
}


impl core::fmt::Display for FileType {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        use FileType::*;

        let arg = match *self {
            None => "Unknown ELF file type",
            Relocatable => "Relocatable file",
            Executable => "Executable file",
            Dynamic => "Dynamic linked file",
            Core => "Core file",

            OSLow => "OS Specific file (low)",
            OSHigh => "OS SPecific file (high)",

            ProcessLow => "Process file (low)",
            ProcessHigh => "Process file (high)",
        };

        write!(f, "{}", arg)
    }
}
