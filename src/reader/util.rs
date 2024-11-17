use byteorder::{BigEndian, LittleEndian};

use crate::Endianness;
use std::io::Error;

pub(crate) trait ReadBytesExt: byteorder::ReadBytesExt {
    fn read_u32(&mut self, byte_order: Endianness) -> Result<u32, Error> {
        match byte_order {
            Endianness::BigEndian => byteorder::ReadBytesExt::read_u32::<BigEndian>(self),
            Endianness::LittleEndian => byteorder::ReadBytesExt::read_u32::<LittleEndian>(self),
        }
    }

    fn read_f64(&mut self, byte_order: Endianness) -> Result<f64, Error> {
        match byte_order {
            Endianness::BigEndian => byteorder::ReadBytesExt::read_f64::<BigEndian>(self),
            Endianness::LittleEndian => byteorder::ReadBytesExt::read_f64::<LittleEndian>(self),
        }
    }
}

/// All types that implement `Read` get methods defined in `ReadBytesExt`
/// for free.
impl<R: std::io::Read + ?Sized> ReadBytesExt for R {}
