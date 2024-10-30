use crate::common::WKBType;
use crate::error::WKBResult;
use crate::Endianness;
use byteorder::{LittleEndian, WriteBytesExt};
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
pub fn write_polygon_as_wkb<W: Write>(
    mut writer: W,
    geom: &impl PolygonTrait<T = f64>,
) -> WKBResult<()> {
    use geo_traits::Dimensions;

    // Byte order
    writer.write_u8(Endianness::LittleEndian.into()).unwrap();

    match geom.dim() {
        Dimensions::Xy | Dimensions::Unknown(2) => {
            writer
                .write_u32::<LittleEndian>(WKBType::Polygon.into())
                .unwrap();
        }
        Dimensions::Xyz | Dimensions::Unknown(3) => {
            writer
                .write_u32::<LittleEndian>(WKBType::PolygonZ.into())
                .unwrap();
        }
        _ => panic!(),
    }

    // numRings
    // TODO: support empty polygons where this will panic
    let num_rings = 1 + geom.num_interiors();
    writer
        .write_u32::<LittleEndian>(num_rings.try_into().unwrap())
        .unwrap();

    let ext_ring = geom.exterior().unwrap();
    writer
        .write_u32::<LittleEndian>(ext_ring.num_coords().try_into().unwrap())
        .unwrap();

    for coord in ext_ring.coords() {
        writer.write_f64::<LittleEndian>(coord.x()).unwrap();
        writer.write_f64::<LittleEndian>(coord.y()).unwrap();
        if geom.dim().size() == 3 {
            writer
                .write_f64::<LittleEndian>(coord.nth_unchecked(2))
                .unwrap();
        }
    }

    for int_ring in geom.interiors() {
        writer
            .write_u32::<LittleEndian>(int_ring.num_coords().try_into().unwrap())
            .unwrap();

        for coord in int_ring.coords() {
            writer.write_f64::<LittleEndian>(coord.x()).unwrap();
            writer.write_f64::<LittleEndian>(coord.y()).unwrap();
            if geom.dim().size() == 3 {
                writer
                    .write_f64::<LittleEndian>(coord.nth_unchecked(2))
                    .unwrap();
            }
        }
    }

    Ok(())
}
