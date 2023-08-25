//! All possible types of symbols.


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolType {
    /// No type / Unknown type.
    None,

    /// Object.
    Object,

    /// Function.
    Function,

    /// Section.
    Section,

    /// File.
    File,

    /// Processor specific.
    ProcessorLow,

    /// Processor specific.
    ProcessorMid,

    /// Processor specific.
    ProcessorHigh,
}


impl core::convert::From<u8> for SymbolType {
    fn from(u: u8) -> Self {
        match u {
            1 => SymbolType::Object,
            2 => SymbolType::Function,
            3 => SymbolType::Section,
            4 => SymbolType::File,

            13 => SymbolType::ProcessorLow,
            14 => SymbolType::ProcessorMid,
            15 => SymbolType::ProcessorHigh,

            _ => SymbolType::None,
        }
    }
}

impl core::fmt::Display for SymbolType {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let s = match *self {
            SymbolType::Object   => "Object",
            SymbolType::Function => "Function",
            SymbolType::Section  => "Section",
            SymbolType::File     => "File",

            SymbolType::ProcessorLow  => "Processor 13",
            SymbolType::ProcessorMid  => "Processor 14",
            SymbolType::ProcessorHigh => "Processor 15",

            SymbolType::None => "No type",
        };

        write!(f, "{}", s)
    }
}
