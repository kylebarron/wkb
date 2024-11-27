use crate::common::WKBType;
use crate::error::WKBResult;
use crate::writer::point::{point_wkb_size, write_point};
use crate::Endianness;
use byteorder::{BigEndian, ByteOrder, LittleEndian, WriteBytesExt};
use geo_traits::MultiPointTrait;
use std::io::Write;

/// The byte length of a MultiPoint
pub fn multi_point_wkb_size(geom: &impl MultiPointTrait<T = f64>) -> usize {
    1 + 4 + 4 + (geom.num_points() * point_wkb_size(geom.dim()))
}

/// Write a MultiPoint geometry to a Writer encoded as WKB
pub fn write_multi_point<W: Write>(
    writer: &mut W,
    geom: &impl MultiPointTrait<T = f64>,
    endianness: Endianness,
) -> WKBResult<()> {
    // Byte order
    writer.write_u8(endianness.into())?;

    // Content
    match endianness {
        Endianness::LittleEndian => {
            write_multi_point_content::<W, LittleEndian>(writer, geom, endianness)
        }
        Endianness::BigEndian => {
            write_multi_point_content::<W, BigEndian>(writer, geom, endianness)
        }
    }
}

fn write_multi_point_content<W: Write, B: ByteOrder>(
    writer: &mut W,
    geom: &impl MultiPointTrait<T = f64>,
    endianness: Endianness,
) -> WKBResult<()> {
    let wkb_type = WKBType::MultiPoint(geom.dim().try_into()?);
    writer.write_u32::<B>(wkb_type.into())?;

    // numPoints
    writer.write_u32::<B>(geom.num_points().try_into().unwrap())?;

    for point in geom.points() {
        write_point(writer, &point, endianness)?;
    }

    Ok(())
}
