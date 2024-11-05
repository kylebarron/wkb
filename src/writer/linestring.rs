use crate::common::WKBType;
use crate::error::WKBResult;
use crate::Endianness;
use byteorder::{BigEndian, ByteOrder, LittleEndian, WriteBytesExt};
use geo_traits::{CoordTrait, LineStringTrait};
use std::io::Write;

/// The byte length of a LineString
pub fn line_string_wkb_size(geom: &impl LineStringTrait) -> usize {
    let header = 1 + 4 + 4;
    let each_coord = geom.dim().size() * 8;
    let all_coords = geom.num_coords() * each_coord;
    header + all_coords
}

/// Write a LineString geometry to a Writer encoded as WKB
pub fn write_line_string<W: Write>(
    writer: &mut W,
    geom: &impl LineStringTrait<T = f64>,
    endianness: Endianness,
) -> WKBResult<()> {
    // Byte order
    writer.write_u8(endianness.into()).unwrap();

    // Content
    match endianness {
        Endianness::LittleEndian => write_line_string_content::<W, LittleEndian>(writer, geom),
        Endianness::BigEndian => write_line_string_content::<W, BigEndian>(writer, geom),
    }
}

fn write_line_string_content<W: Write, B: ByteOrder>(
    writer: &mut W,
    geom: &impl LineStringTrait<T = f64>,
) -> WKBResult<()> {
    use geo_traits::Dimensions;

    match geom.dim() {
        Dimensions::Xy | Dimensions::Unknown(2) => {
            writer.write_u32::<B>(WKBType::LineString.into()).unwrap();
        }
        Dimensions::Xyz | Dimensions::Unknown(3) => {
            writer.write_u32::<B>(WKBType::LineStringZ.into()).unwrap();
        }
        _ => panic!(),
    }

    // numPoints
    writer
        .write_u32::<B>(geom.num_coords().try_into().unwrap())
        .unwrap();

    for coord in geom.coords() {
        writer.write_f64::<B>(coord.x()).unwrap();
        writer.write_f64::<B>(coord.y()).unwrap();

        if geom.dim().size() == 3 {
            writer.write_f64::<B>(coord.nth_unchecked(2)).unwrap();
        }
    }

    Ok(())
}
