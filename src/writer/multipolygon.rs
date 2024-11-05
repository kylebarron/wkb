use crate::common::WKBType;
use crate::error::WKBResult;
use crate::writer::polygon::{polygon_wkb_size, write_polygon};
use crate::Endianness;
use byteorder::{LittleEndian, WriteBytesExt};
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
    writer: &mut W,
    geom: &impl MultiPolygonTrait<T = f64>,
    endianness: Endianness,
) -> WKBResult<()> {
    // Byte order
    writer.write_u8(endianness.into())?;

    let wkb_type = WKBType::MultiPolygon(geom.dim().try_into()?);
    writer.write_u32::<LittleEndian>(wkb_type.into())?;

    // numPolygons
    writer.write_u32::<LittleEndian>(geom.num_polygons().try_into().unwrap())?;

    for polygon in geom.polygons() {
        write_polygon(writer, &polygon, endianness)?;
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
