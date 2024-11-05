use crate::common::WKBType;
use crate::error::WKBResult;
use crate::writer::geometry::{geometry_wkb_size, write_geometry};
use crate::Endianness;
use byteorder::{LittleEndian, WriteBytesExt};
use geo_traits::GeometryCollectionTrait;
use std::io::Write;

/// The byte length of a GeometryCollection
pub fn geometry_collection_wkb_size(geom: &impl GeometryCollectionTrait) -> usize {
    let mut sum = 1 + 4 + 4;

    for inner_geom in geom.geometries() {
        sum += geometry_wkb_size(&inner_geom);
    }

    sum
}

/// Write a GeometryCollection geometry to a Writer encoded as WKB
pub fn write_geometry_collection<W: Write>(
    writer: &mut W,
    geom: &impl GeometryCollectionTrait<T = f64>,
    endianness: Endianness,
) -> WKBResult<()> {
    use geo_traits::Dimensions;

    // Byte order
    writer.write_u8(Endianness::LittleEndian.into())?;

    match geom.dim() {
        Dimensions::Xy | Dimensions::Unknown(2) => {
            writer.write_u32::<LittleEndian>(WKBType::GeometryCollection.into())?;
        }
        Dimensions::Xyz | Dimensions::Unknown(3) => {
            writer.write_u32::<LittleEndian>(WKBType::GeometryCollectionZ.into())?;
        }
        _ => panic!(),
    }

    // numGeometries
    writer.write_u32::<LittleEndian>(geom.num_geometries().try_into().unwrap())?;

    for inner_geom in geom.geometries() {
        write_geometry(writer, &inner_geom, endianness)?;
    }

    Ok(())
}

// #[cfg(test)]
// mod test {
//     use super::*;
//     use crate::test::multipoint;
//     use crate::test::multipolygon;

//     #[test]
//     fn round_trip() {
//         let gc0 = geo::GeometryCollection::new_from(vec![
//             geo::Geometry::MultiPoint(multipoint::mp0()),
//             geo::Geometry::MultiPolygon(multipolygon::mp0()),
//         ]);
//         let gc1 = geo::GeometryCollection::new_from(vec![
//             geo::Geometry::MultiPoint(multipoint::mp1()),
//             geo::Geometry::MultiPolygon(multipolygon::mp1()),
//         ]);

//         let orig_arr: GeometryCollectionArray<i32> = vec![Some(gc0), Some(gc1), None].into();
//         let wkb_arr: WKBArray<i32> = (&orig_arr).into();
//         let new_arr: GeometryCollectionArray<i32> = wkb_arr.try_into().unwrap();

//         assert_eq!(orig_arr, new_arr);
//     }
// }
