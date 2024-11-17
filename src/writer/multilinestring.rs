use crate::common::WKBType;
use crate::error::WKBResult;
use crate::writer::linestring::{line_string_wkb_size, write_line_string};
use crate::Endianness;
use byteorder::{BigEndian, ByteOrder, LittleEndian, WriteBytesExt};
use geo_traits::MultiLineStringTrait;
use std::io::Write;

/// The byte length of a MultiLineString
pub fn multi_line_string_wkb_size(geom: &impl MultiLineStringTrait) -> usize {
    let mut sum = 1 + 4 + 4;
    for line_string in geom.line_strings() {
        sum += line_string_wkb_size(&line_string);
    }

    sum
}

/// Write a MultiLineString geometry to a Writer encoded as WKB
pub fn write_multi_line_string<W: Write>(
    mut writer: W,
    geom: &impl MultiLineStringTrait<T = f64>,
    endianness: Endianness,
) -> WKBResult<()> {
    // Byte order
    writer.write_u8(endianness.into())?;

    // Content
    match endianness {
        Endianness::LittleEndian => {
            write_multi_line_string_content::<W, LittleEndian>(writer, geom, endianness)
        }
        Endianness::BigEndian => {
            write_multi_line_string_content::<W, BigEndian>(writer, geom, endianness)
        }
    }
}

fn write_multi_line_string_content<W: Write, B: ByteOrder>(
    mut writer: W,
    geom: &impl MultiLineStringTrait<T = f64>,
    endianness: Endianness,
) -> WKBResult<()> {
    use geo_traits::Dimensions;

    match geom.dim() {
        Dimensions::Xy | Dimensions::Unknown(2) => {
            writer.write_u32::<B>(WKBType::MultiLineString.into())?;
        }
        Dimensions::Xyz | Dimensions::Unknown(3) => {
            writer.write_u32::<B>(WKBType::MultiLineStringZ.into())?;
        }
        _ => panic!(),
    }

    // numPoints
    writer.write_u32::<B>(geom.num_line_strings().try_into().unwrap())?;

    for line_string in geom.line_strings() {
        write_line_string(&mut writer, &line_string, endianness)?;
    }

    Ok(())
}
