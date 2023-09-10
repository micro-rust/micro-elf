//! Symbols of an ELF file.



mod bind;
mod symtype;



pub use bind::Bind;
pub use symtype::SymbolType;

use crate::common::address::Address;



/// A common section header structure. Will be instantiated by each implementator.
#[derive(Debug)]
pub struct Symbol {
    /// Section name.
    pub(super) name: String,

    /// Section name offset in the .shstrtab section.
    pub(super) nameidx: u32,

    /// The value of the symbol.
    pub(super) value: Address,

    /// Size of the symbol.
    /// A null size may mean unsized or unknown size.
    pub(super) size: Address,

    /// The bind of the symbol.
    pub(super) bind: Bind,

    /// The type of the symbol.
    pub(super) symboltype: SymbolType,

    /// Index of the related section.
    pub(super) shidx: u16,
}

impl Symbol {
    /// Parses the given slice of data into an ELF file header.
    pub fn parse<R: AsRef<[u8]>, const INC: usize>(raw: R, read: fn(&[u8]) -> Address, read16: fn(&[u8]) -> u16, read32: fn(&[u8]) -> u32) -> Result<Self, ()> {
        // Get the slice.
        let raw = raw.as_ref();

        // Start dynamic index.
        let mut i = 0;

        match INC {
            4 => {
                // Read the name index.
                let nameidx = read32( &raw[i..i+4] );
                i += 4;

                // Read the value.
                let value = read( &raw[i..i+4] );
                i += 4;

                // Read the size.
                let size = read( &raw[i..i+4] );
                i += 4;

                // Read the info.
                let info = raw[i];
                i += 1;

                // Read the other field.
                i += 1;

                // Read the associated section.
                let shidx = read16( &raw[i..i+2] );
                i += 2;

                assert_eq!(i, 16);

                // Process the information.
                let bind = Bind::from( info >> 4 );
                let symboltype = SymbolType::from( info & 0xF );

                Ok(Self {
                    name: String::new(),
                    nameidx,
                    value,
                    size,
                    bind,
                    symboltype,
                    shidx,
                })
            },

            8 => {
                // Read the name index.
                let nameidx = read32( &raw[i..i+4] );
                i += 4;

                // Read the info.
                let info = raw[i];
                i += 1;

                // Read the other field.
                i += 1;

                // Read the associated section.
                let shidx = read16( &raw[i..i+2] );
                i += 2;

                // Read the value.
                let value = read( &raw[i..i+8] );
                i += 8;

                // Read the size.
                let size = read( &raw[i..i+8] );
                i += 8;

                assert_eq!(i, 24);

                // Process the information.
                let bind = Bind::from( info >> 4 );
                let symboltype = SymbolType::from( info & 0xF );


                Ok(Self {
                    name: String::new(),
                    nameidx,
                    value,
                    size,
                    bind,
                    symboltype,
                    shidx,
                })
            },

            _ => Err( () ),
        }
    }

    /// Returns a reference to the name of this symbol.
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Returns `true` if the symbol is an function.
    pub fn is_function(&self) -> bool {
        self.symboltype == SymbolType::Function
    }

    /// Returns `true` if the symbol is an object.
    pub fn is_object(&self) -> bool {
        self.symboltype == SymbolType::Object
    }

    /// Grabs the name of this section from the given raw strings.
    pub(super) fn rename(&mut self, names: &[u8]) {
        use core::ffi::CStr;

        // Get the referenced C string.
        let cstr = match CStr::from_bytes_until_nul( &names[self.nameidx as usize..] ) {
            Err(_) => return,
            Ok(s) => s,
        };

        // Attempt to transform into a Rust str.
        let string = match cstr.to_str() {
            Err(_) => return,
            Ok(s) => s,
        };

        // Create the name string.
        self.name = String::from( string );
    }

    /// Creates a pretty print of the section's information.
    pub fn prettyprint(&self) -> String {
        // Create output string.
        let mut args = String::new();

        // Section name.
        args += &format!("Symbol \"{}\"\n", self.name);

        // Symbol value and size.
        args += &format!("  - Value: {:X}\n", self.value);
        args += &format!("  - Size : {} bytes\n", self.size);

        // Related section.
        args += &format!("  - Related section: {}\n", self.shidx);

        // Information on the symbol.
        args += &format!("  - Bind: {}\n", self.bind);
        args += &format!("  - Type: {}\n", self.symboltype);

        args
    }
}
