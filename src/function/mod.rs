//! Funcitons defined in the ELF file.


use crate::{ ElfSymbol, ElfSection };
use core::convert::TryFrom;

#[derive(Debug, Clone)]
pub struct Function {
    /// Name of the function.
    name: String,

    /// Base address.
    addr: u32,

    /// Raw data of the function.
    content: Vec<u8>,

    // TODO : Implement disassembler.
    //asm: GenericAsmType,
}

impl Function {
    /// Tries to read the function info and contents from the given symbol.
    pub fn parse32(symbol: &Box<dyn ElfSymbol<Address = u32>>, section: &[u8]) -> Function {
        // Get section, address and size.
        let sidx = symbol.section();
        let addr = symbol.value();
        let size = usize::try_from( symbol.size() ).unwrap();

        // Check that the size is not 0.
        if size == 0 {
            return Function {
                name: String::from("UNDEFINED FUNCTION"),
                addr: 0,
                content: Vec::new(),
            };
        }

        // Get raw data.
        let content = section[sidx..(sidx+size)].to_vec();

        Function {
            name: symbol.name(),
            addr: addr,
            content,
        }
    }
}


impl core::fmt::Display for Function {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let mut args = String::new();

        // Name of the symbol.
        args += &format!("Function: {}\n", self.name);

        // Bind of the symbol.
        args += &format!("Address: 0x{:08X}\n", self.addr);

        // Size of the symbol.
        args += &format!("Size: {} Bytes\n", self.content.len());

        write!(f, "{}", args)
    }
}
