//! Parse WKB buffers.
//!
//! Each of the data structures in this module is intended to mirror the [WKB
//! spec](https://portal.ogc.org/files/?artifact_id=25355).
//!
//! Each of these data structures implement traits from [geo-traits] for interoperability. These
//! traits are the standard way to access coordinates.

mod coord;
mod geometry;
mod geometry_collection;
mod linearring;
mod linestring;
mod multilinestring;
mod multipoint;
mod multipolygon;
mod point;
mod polygon;
mod util;

use std::io::Cursor;

use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use geometry::Wkb;
use geometry_collection::GeometryCollection;
use linestring::LineString;
use multilinestring::MultiLineString;
use multipoint::MultiPoint;
use multipolygon::MultiPolygon;
use point::Point;
use polygon::Polygon;

use geo_traits::GeometryTrait;

use crate::error::WKBResult;
use crate::Endianness;

pub fn read_wkb(buf: &[u8]) -> WKBResult<impl GeometryTrait + use<'_>> {
    let mut reader = Cursor::new(buf);
    let byte_order = Endianness::try_from(reader.read_u8()?).unwrap();
    match byte_order {
        Endianness::LittleEndian => Wkb::<LittleEndian>::try_new(buf),
        Endianness::BigEndian => Wkb::<BigEndian>::try_new(buf),
    }
}
