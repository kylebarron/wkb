use crate::common::WKBType;
use crate::error::WKBResult;
use crate::writer::polygon::{polygon_wkb_size, write_polygon};
use crate::Endianness;
use byteorder::{BigEndian, ByteOrder, LittleEndian, WriteBytesExt};
use geo_traits::MultiPolygonTrait;
use std::io::Write;

/// The byte length of a MultiPolygon
pub fn multi_polygon_wkb_size(geom: &impl MultiPolygonTrait) -> usize {
    let mut sum = 1 + 4 + 4;
    for polygon in geom.polygons() {
        sum += polygon_wkb_size(&polygon);
    }

    sum
}

/// Write a MultiPolygon geometry to a Writer encoded as WKB
pub fn write_multi_polygon<W: Write>(
    mut writer: W,
    geom: &impl MultiPolygonTrait<T = f64>,
    endianness: Endianness,
) -> WKBResult<()> {
    // Byte order
    writer.write_u8(endianness.into())?;

    // Content
    match endianness {
        Endianness::LittleEndian => {
            write_multi_polygon_content::<W, LittleEndian>(writer, geom, endianness)
        }
        Endianness::BigEndian => {
            write_multi_polygon_content::<W, BigEndian>(writer, geom, endianness)
        }
    }
}

fn write_multi_polygon_content<W: Write, B: ByteOrder>(
    mut writer: W,
    geom: &impl MultiPolygonTrait<T = f64>,
    endianness: Endianness,
) -> WKBResult<()> {
    use geo_traits::Dimensions;

    match geom.dim() {
        Dimensions::Xy | Dimensions::Unknown(2) => {
            writer.write_u32::<B>(WKBType::MultiPolygon.into())?;
        }
        Dimensions::Xyz | Dimensions::Unknown(3) => {
            writer.write_u32::<B>(WKBType::MultiPolygonZ.into())?;
        }
        _ => panic!(),
    }

    // numPolygons
    writer.write_u32::<B>(geom.num_polygons().try_into().unwrap())?;

    for polygon in geom.polygons() {
        write_polygon(&mut writer, &polygon, endianness)?;
    }

    Ok(())
}

// #[cfg(test)]
// mod test {
//     use super::*;
//     use crate::test::multipolygon::{mp0, mp1};

//     #[test]
//     fn round_trip() {
//         let orig_arr: MultiPolygonArray<2> = vec![Some(mp0()), Some(mp1()), None].into();
//         let wkb_arr: WKBArray<i32> = (&orig_arr).into();
//         let new_arr: MultiPolygonArray<2> = wkb_arr.try_into().unwrap();

//         assert_eq!(orig_arr, new_arr);
//     }
// }
