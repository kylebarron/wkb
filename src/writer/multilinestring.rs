use crate::common::WKBType;
use crate::error::WKBResult;
use crate::writer::linestring::{line_string_wkb_size, write_line_string};
use crate::Endianness;
use byteorder::{LittleEndian, WriteBytesExt};
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
    writer: &mut W,
    geom: &impl MultiLineStringTrait<T = f64>,
    endianness: Endianness,
) -> WKBResult<()> {
    // Byte order
    writer.write_u8(endianness.into())?;

    let wkb_type = WKBType::MultiLineString(geom.dim().try_into()?);
    writer.write_u32::<LittleEndian>(wkb_type.into())?;

    // numPoints
    writer.write_u32::<LittleEndian>(geom.num_line_strings().try_into().unwrap())?;

    for line_string in geom.line_strings() {
        write_line_string(writer, &line_string, endianness)?;
    }

    Ok(())
}
