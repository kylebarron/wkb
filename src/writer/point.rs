use crate::common::WKBType;
use crate::error::WKBResult;
use crate::Endianness;
use byteorder::{BigEndian, ByteOrder, LittleEndian, WriteBytesExt};
use core::f64;
use geo_traits::{CoordTrait, PointTrait};
use std::io::Write;

/// The byte length of a Point
pub fn point_wkb_size(dim: geo_traits::Dimensions) -> usize {
    let header = 1 + 4;
    let coords = dim.size() * 8;
    header + coords
}

/// Write a Point geometry to a Writer encoded as WKB
pub fn write_point<W: Write>(
    writer: &mut W,
    geom: &impl PointTrait<T = f64>,
    endianness: Endianness,
) -> WKBResult<()> {
    // Byte order header
    writer.write_u8(endianness.into())?;

    // Content
    match endianness {
        Endianness::LittleEndian => write_point_content::<W, LittleEndian>(writer, geom),
        Endianness::BigEndian => write_point_content::<W, BigEndian>(writer, geom),
    }
}

/// Write a Point geometry to a Writer encoded as WKB
fn write_point_content<W: Write, B: ByteOrder>(
    writer: &mut W,
    geom: &impl PointTrait<T = f64>,
) -> WKBResult<()> {
    use geo_traits::Dimensions;

    match geom.dim() {
        Dimensions::Xy | Dimensions::Unknown(2) => {
            writer.write_u32::<B>(WKBType::Point.into())?;
        }
        Dimensions::Xyz | Dimensions::Unknown(3) => {
            writer.write_u32::<B>(WKBType::PointZ.into())?;
        }
        _ => panic!(),
    }

    if let Some(coord) = geom.coord() {
        writer.write_f64::<B>(coord.x())?;
        writer.write_f64::<B>(coord.y())?;

        if coord.dim().size() == 3 {
            writer.write_f64::<B>(coord.nth_unchecked(2))?;
        }
    } else {
        // Write POINT EMPTY as f64::NAN values
        for _ in 0..geom.dim().size() {
            writer.write_f64::<B>(f64::NAN)?;
        }
    }

    Ok(())
}
