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
    mut writer: W,
    geom: &impl MultiLineStringTrait<T = f64>,
) -> WKBResult<()> {
    use geo_traits::Dimensions;

    // Byte order
    writer.write_u8(Endianness::LittleEndian.into())?;

    match geom.dim() {
        Dimensions::Xy | Dimensions::Unknown(2) => {
            writer.write_u32::<LittleEndian>(WKBType::MultiLineString.into())?;
        }
        Dimensions::Xyz | Dimensions::Unknown(3) => {
            writer.write_u32::<LittleEndian>(WKBType::MultiLineStringZ.into())?;
        }
        _ => panic!(),
    }

    // numPoints
    writer.write_u32::<LittleEndian>(geom.num_line_strings().try_into().unwrap())?;

    for line_string in geom.line_strings() {
        write_line_string(&mut writer, &line_string)?;
    }

    Ok(())
}
