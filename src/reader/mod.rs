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

pub fn read_wkb(buf: &[u8]) -> WKBResult<impl GeometryTrait + use<'_>> {
    Wkb::try_new(buf)
}
