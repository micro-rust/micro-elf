//! Symbol binding.


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Bind {
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

impl core::convert::From<u8> for Bind {
    fn from(u: u8) -> Self {
        match u {
        	0 => Bind::Local,
            1 => Bind::Global,
            2 => Bind::Weak,

            13 => Bind::ProcessorLow,
            14 => Bind::ProcessorMid,
            15 => Bind::ProcessorHigh,

            _ => Bind::None,
        }
    }
}

impl core::fmt::Display for Bind {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let s = match *self {
            Bind::Local   => "Local",
            Bind::Global => "Global",
            Bind::Weak  => "Weak",

            Bind::ProcessorLow  => "Processor 13",
            Bind::ProcessorMid  => "Processor 14",
            Bind::ProcessorHigh => "Processor 15",

            Bind::None => "No binding",
        };

        write!(f, "{}", s)
    }
}
