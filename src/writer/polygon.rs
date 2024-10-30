use crate::common::WKBType;
use crate::error::WKBResult;
use crate::Endianness;
use byteorder::{BigEndian, ByteOrder, LittleEndian, WriteBytesExt};
use geo_traits::{CoordTrait, LineStringTrait, PolygonTrait};
use std::io::Write;

/// The byte length of a Polygon
pub fn polygon_wkb_size(geom: &impl PolygonTrait) -> usize {
    let mut sum = 1 + 4 + 4;

    let each_coord = geom.dim().size() * 8;

    // TODO: support empty polygons where this will panic
    let ext_ring = geom.exterior().unwrap();
    sum += 4 + (ext_ring.num_coords() * each_coord);

    for int_ring in geom.interiors() {
        sum += 4 + (int_ring.num_coords() * each_coord);
    }

    sum
}

/// Write a Polygon geometry to a Writer encoded as WKB
pub fn write_polygon<W: Write>(
    mut writer: W,
    geom: &impl PolygonTrait<T = f64>,
    endianness: Endianness,
) -> WKBResult<()> {
    // Byte order
    writer.write_u8(endianness.into())?;

    // Content
    match endianness {
        Endianness::LittleEndian => write_polygon_content::<W, LittleEndian>(writer, geom),
        Endianness::BigEndian => write_polygon_content::<W, BigEndian>(writer, geom),
    }
}

fn write_polygon_content<W: Write, B: ByteOrder>(
    mut writer: W,
    geom: &impl PolygonTrait<T = f64>,
) -> WKBResult<()> {
    use geo_traits::Dimensions;

    match geom.dim() {
        Dimensions::Xy | Dimensions::Unknown(2) => {
            writer.write_u32::<B>(WKBType::Polygon.into())?;
        }
        Dimensions::Xyz | Dimensions::Unknown(3) => {
            writer.write_u32::<B>(WKBType::PolygonZ.into())?;
        }
        _ => panic!(),
    }

    // numRings
    // TODO: support empty polygons where this will panic
    let num_rings = 1 + geom.num_interiors();
    writer.write_u32::<B>(num_rings.try_into().unwrap())?;

    let ext_ring = geom.exterior().unwrap();
    writer.write_u32::<B>(ext_ring.num_coords().try_into().unwrap())?;

    for coord in ext_ring.coords() {
        writer.write_f64::<B>(coord.x())?;
        writer.write_f64::<B>(coord.y())?;
        if geom.dim().size() == 3 {
            writer.write_f64::<B>(coord.nth_unchecked(2))?;
        }
    }

    for int_ring in geom.interiors() {
        writer.write_u32::<B>(int_ring.num_coords().try_into().unwrap())?;

        for coord in int_ring.coords() {
            writer.write_f64::<B>(coord.x())?;
            writer.write_f64::<B>(coord.y())?;
            if geom.dim().size() == 3 {
                writer.write_f64::<B>(coord.nth_unchecked(2))?;
            }
        }
    }

    Ok(())
}
