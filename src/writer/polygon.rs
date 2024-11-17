use crate::common::WKBType;
use crate::error::WKBResult;
use crate::writer::coord::write_coord;
use crate::Endianness;
use byteorder::{BigEndian, ByteOrder, LittleEndian, WriteBytesExt};
use geo_traits::{LineStringTrait, PolygonTrait};
use std::io::Write;

/// The byte length of a Polygon
pub fn polygon_wkb_size(geom: &impl PolygonTrait) -> usize {
    let mut sum = 1 + 4 + 4;

    let each_coord = geom.dim().size() * 8;

    if let Some(ext_ring) = geom.exterior() {
        sum += 4 + (ext_ring.num_coords() * each_coord);
    }

    for int_ring in geom.interiors() {
        sum += 4 + (int_ring.num_coords() * each_coord);
    }

    sum
}

/// Write a Polygon geometry to a Writer encoded as WKB
pub fn write_polygon<W: Write>(
    writer: &mut W,
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
    writer: &mut W,
    geom: &impl PolygonTrait<T = f64>,
) -> WKBResult<()> {
    let wkb_type = WKBType::Polygon(geom.dim().try_into()?);
    writer.write_u32::<LittleEndian>(wkb_type.into())?;

    // numRings
    let num_rings = if geom.exterior().is_some() {
        1 + geom.num_interiors()
    } else {
        0
    };
    writer.write_u32::<B>(num_rings.try_into().unwrap())?;

    if let Some(ext_ring) = geom.exterior() {
        writer.write_u32::<B>(ext_ring.num_coords().try_into().unwrap())?;

        for coord in ext_ring.coords() {
            write_coord::<W, B>(writer, &coord)?;
        }
    }

    for int_ring in geom.interiors() {
        writer.write_u32::<B>(int_ring.num_coords().try_into().unwrap())?;

        for coord in int_ring.coords() {
            write_coord::<W, B>(writer, &coord)?;
        }
    }

    Ok(())
}
