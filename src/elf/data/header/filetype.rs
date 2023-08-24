//! All possible ELF file types.



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType {
    /// Relocatable file.
    Relocatable,

    /// Executable file.
    Executable,

    /// Dynamically linked file.
    Dynamic,

    /// Core file.
    Core,

    /// OS specific file.
    OS(u16),

    /// Process specific file.
    Process(u16),

    /// Unknown or undefined file type.
    None,
}

impl core::convert::From<u16> for FileType {
    fn from(d: u16) -> FileType {
        match d {
            0x01 => FileType::Relocatable,
            0x02 => FileType::Executable,
            0x03 => FileType::Dynamic,
            0x04 => FileType::Core,

            0xFE00..=0xFEFF => FileType::OS(d),

            0xFF00..=0xFFFF => FileType::Process(d),

            _ => FileType::None,
        }
    }
}

impl core::fmt::Display for FileType {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        use FileType::*;

        let arg = match *self {
            None => String::from("Unknown"),
            Relocatable => String::from("Relocatable"),
            Executable => String::from("Executable"),
            Dynamic => String::from("Dynamic linked"),
            Core => String::from("Core"),

            OS(d) => format!("OS Specific 0x{:08X}", d),

            Process(d) => format!("Process Specific 0x{:08X}", d),
        };

        write!(f, "{}", arg)
    }
}
