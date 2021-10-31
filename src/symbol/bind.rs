//! Symbol binding.


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolBind {
	/// Local bind.
	Local,

	/// Global bind.
	Global,

	/// Weak bind.
	Weak,

	/// Processor specific.
	ProcessorLow,

	/// Processor specific.
	ProcessorMid,

	/// Processor specific.
	ProcessorHigh,

    None,
}

impl core::convert::From<u8> for SymbolBind {
    fn from(u: u8) -> Self {
        match u {
        	0 => SymbolBind::Local,
            1 => SymbolBind::Global,
            2 => SymbolBind::Weak,

            13 => SymbolBind::ProcessorLow,
            14 => SymbolBind::ProcessorMid,
            15 => SymbolBind::ProcessorHigh,

            _ => SymbolBind::None,
        }
    }
}

impl core::fmt::Display for SymbolBind {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let s = match *self {
            SymbolBind::Local   => "Local",
            SymbolBind::Global => "Global",
            SymbolBind::Weak  => "Weak",

            SymbolBind::ProcessorLow  => "Processor 13",
            SymbolBind::ProcessorMid  => "Processor 14",
            SymbolBind::ProcessorHigh => "Processor 15",

            SymbolBind::None => "No binding",
        };

        write!(f, "{}", s)
    }
}
