//! Symbols of an ELF file.

mod bind;
mod stype;


use byteorder::{ ByteOrder, ReadBytesExt };

use core::convert::TryFrom;

pub use self::bind::SymbolBind;
pub use self::stype::SymbolType;


use crate::{ ElfSymbol, Error };


pub struct Symbol32 {
    /// Name of the symbol.
    name: String,

    /// Index of the symbol's name.
    nameidx: u32,

    /// Value of the symbol.
    /// Can be an address, an absolute value or other.
    value: u32,

    /// Size of the symbol.
    size: u32,

    /// Symbol binding.
    bind: SymbolBind,

    /// Symbol type.
    stype: SymbolType,

    /// Section index of the section this symbol relates to.
    shidx: u16,
}


impl ElfSymbol for Symbol32 {
    type Address = u32;

    fn parse<T: ByteOrder>(symbol: &[u8]) -> Result<Box<dyn ElfSymbol<Address = Self::Address>>, Error> {
        // Get index to the name.
        let nameidx = match (&symbol[0x00..0x04]).read_u32::<T>() {
            Ok(x) => x,
            _ => return Err( Error::UnexpectedEOF ),
        };

        // Get value of the symbol.
        let value = match (&symbol[0x04..0x08]).read_u32::<T>() {
            Ok(x) => x,
            _ => return Err( Error::UnexpectedEOF ),
        };

        // Get size of the symbol.
        let size = match (&symbol[0x08..0x0C]).read_u32::<T>() {
            Ok(x) => x,
            _ => return Err( Error::UnexpectedEOF ),
        };

        // Get info of the symbol.
        let info : u8 = symbol[0x0C];

        let bind = SymbolBind::from(info >> 4);
        let stype = SymbolType::from(info & 0xF);

        // Get the related section index.
        let shidx = match (&symbol[0x0E..0x10]).read_u16::<T>() {
            Ok(x) => x,
            _ => return Err( Error::UnexpectedEOF ),
        };

        Ok( Box::new( Symbol32 {
            name: String::new(),
            nameidx,
            value,
            size,
            bind,
            stype,
            shidx,
        }))
    }

    fn value(&self) -> Self::Address {
        self.value
    }

    fn section(&self) -> usize {
        self.shidx as usize
    }

    fn size(&self) -> Self::Address {
        self.size
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn is_function(&self) -> bool {
        self.stype == SymbolType::Function
    }

    fn naming(&mut self, strtab: &[u8]) {
        // Get base offset.
        let s = usize::try_from(self.nameidx).unwrap();

        // Get dynamic index.
        let mut e = s + 1;

        // Check end of NULL terminated string.
        while strtab[e] != 0x00 { e += 1 }

        // Read in the name.
        self.name = unsafe { String::from_utf8_unchecked( strtab[s..e].to_vec() ) };
    }

    fn prettyprint(&self, tab: String) -> String {
        let mut args = String::new();

        // Name of the symbol.
        args += &format!("{}{}\n", tab, self.name);

        // Bind of the symbol.
        args += &format!("{}  Bind: {}\n", tab, self.bind);

        // Size of the symbol.
        args += &format!("{}  Size: {} Bytes\n", tab, self.size);

        // Size of the symbol.
        args += &format!("{}  Type: {}\n", tab, self.stype);

        // Size of the symbol.
        args += &format!("{}  Value: 0x{:08X}\n", tab, self.value);


        args
    }
}

impl core::fmt::Display for Symbol32 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let mut args = String::new();

        // Name of the symbol.
        args += &format!("{}\n", self.name);

        // Bind of the symbol.
        args += &format!("  Bind: {}\n", self.bind);

        // Size of the symbol.
        args += &format!("  Size: {} Bytes\n", self.size);

        // Size of the symbol.
        args += &format!("  Type: {}\n", self.stype);

        // Size of the symbol.
        args += &format!("  Value: 0x{:08X}\n", self.value);


        write!(f, "{}", args)
    }
}