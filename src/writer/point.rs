use crate::common::WKBType;
use crate::error::WKBResult;
use crate::Endianness;
use byteorder::{LittleEndian, WriteBytesExt};
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
pub fn write_point_as_wkb<W: Write>(
    mut writer: W,
    geom: &impl PointTrait<T = f64>,
) -> WKBResult<()> {
    use geo_traits::Dimensions;

    // Byte order
    writer.write_u8(Endianness::LittleEndian.into())?;

    match geom.dim() {
        Dimensions::Xy | Dimensions::Unknown(2) => {
            writer.write_u32::<LittleEndian>(WKBType::Point.into())?;
        }
        Dimensions::Xyz | Dimensions::Unknown(3) => {
            writer.write_u32::<LittleEndian>(WKBType::PointZ.into())?;
        }
        _ => panic!(),
    }

    if let Some(coord) = geom.coord() {
        writer.write_f64::<LittleEndian>(coord.x())?;
        writer.write_f64::<LittleEndian>(coord.y())?;

        if coord.dim().size() == 3 {
            writer.write_f64::<LittleEndian>(coord.nth_unchecked(2))?;
        }
    } else {
        // Write POINT EMPTY as f64::NAN values
        for _ in 0..geom.dim().size() {
            writer.write_f64::<LittleEndian>(f64::NAN)?;
        }
    }

    Ok(())
}

// #[cfg(test)]
// mod test {
//     use super::*;
//     use crate::test::point::{p0, p1, p2};

//     #[test]
//     fn round_trip() {
//         // TODO: test with nulls
//         let orig_arr: PointArray<2> = vec![Some(p0()), Some(p1()), Some(p2())].into();
//         let wkb_arr: WKBArray<i32> = (&orig_arr).into();
//         let new_arr: PointArray<2> = wkb_arr.try_into().unwrap();

//         assert_eq!(orig_arr, new_arr);
//     }

//     #[test]
//     fn round_trip_with_null() {
//         let orig_arr: PointArray<2> = vec![Some(p0()), None, Some(p1()), None, Some(p2())].into();
//         let wkb_arr: WKBArray<i32> = (&orig_arr).into();
//         let new_arr: PointArray<2> = wkb_arr.try_into().unwrap();

//         assert_eq!(orig_arr, new_arr);
//     }
// }
