use crate::common::WKBType;
use crate::error::WKBResult;
use crate::writer::point::{point_wkb_size, write_point};
use crate::Endianness;
use byteorder::{LittleEndian, WriteBytesExt};
use geo_traits::MultiPointTrait;
use std::io::Write;

/// The byte length of a MultiPoint
pub fn multi_point_wkb_size(geom: &impl MultiPointTrait) -> usize {
    1 + 4 + 4 + (geom.num_points() * point_wkb_size(geom.dim()))
}

/// Write a MultiPoint geometry to a Writer encoded as WKB
pub fn write_multi_point<W: Write>(
    writer: &mut W,
    geom: &impl MultiPointTrait<T = f64>,
    endianness: Endianness,
) -> WKBResult<()> {
    use geo_traits::Dimensions;

    // Byte order
    writer.write_u8(endianness.into())?;

    match geom.dim() {
        Dimensions::Xy | Dimensions::Unknown(2) => {
            writer.write_u32::<LittleEndian>(WKBType::MultiPoint.into())?;
        }
        Dimensions::Xyz | Dimensions::Unknown(3) => {
            writer.write_u32::<LittleEndian>(WKBType::MultiPointZ.into())?;
        }
        _ => panic!(),
    }

    // numPoints
    writer.write_u32::<LittleEndian>(geom.num_points().try_into().unwrap())?;

    for point in geom.points() {
        write_point(writer, &point, endianness)?;
    }

    Ok(())
}
