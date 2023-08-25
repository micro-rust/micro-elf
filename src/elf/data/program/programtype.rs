//! All possible program types.


pub enum ProgramType {
    Null        ,
    Load        ,
    Dynamic     ,
    Interpreter ,
    Note        ,
    SharedLib   ,
    ProgHeader  ,
    TLS         ,
    OS(u32)     ,
    Process(u32),
}

impl core::convert::From<u32> for ProgramType {
    fn from(t: u32) -> ProgramType {
        use ProgramType::*;

        match t {
            0x00000001 => Load,
            0x00000002 => Dynamic,
            0x00000003 => Interpreter,
            0x00000004 => Note,
            0x00000005 => SharedLib,
            0x00000006 => ProgHeader,
            0x00000007 => TLS,

            0x60000000..=0x6FFFFFFF => OS(t),
            0x70000000..=0x7FFFFFFF => Process(t),

            _ => Null,
        }
    }
}

impl core::fmt::Display for ProgramType {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        use ProgramType::*;

        let arg = match *self {
            Null         => String::from("Null"),
            Load         => String::from("Loadable"),
            Dynamic      => String::from("Dynamic Linking"),
            Interpreter  => String::from("Interpreter"),
            Note         => String::from("Note"),
            SharedLib    => String::from("Shared Library"),
            ProgHeader   => String::from("Program Header Table"),
            TLS          => String::from("Thread Local Storage"),
            OS(t)        => format!("OS Specific ({})", t),
            Process(t)   => format!("Process Specific ({})", t),
        };

        write!(f, "{}", arg)
    }
}
