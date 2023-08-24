//! Common utilities related to byte reading and writing.



use byteorder::ByteOrder;



/// Internal function to read a `u16`.
pub fn read16<B: ByteOrder>(buf: &[u8]) -> u16 {
    B::read_u16(buf)
}



/// Internal function to read a `u32`.
pub fn read32<B: ByteOrder>(buf: &[u8]) -> u32 {
    B::read_u32(buf)
}



/// Internal function to read a `u64`.
pub fn read64<B: ByteOrder>(buf: &[u8]) -> u64 {
    B::read_u64(buf)
}
