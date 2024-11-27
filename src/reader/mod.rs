//! Parse buffers containing WKB-encoded geometries.

// Each of the data structures in this module is intended to mirror the [WKB
// spec](https://portal.ogc.org/files/?artifact_id=25355).

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

/// Parse a WKB byte slice into a geometry.
///
/// This returns an opaque object that implements [`GeometryTrait`]. Use methods provided by
/// [`geo_traits`] to access the underlying data.
///
/// The contained [dimension][geo_traits::Dimensions] will never be `Unknown`.
///
/// ### Performance
///
/// WKB is not a zero-copy format because coordinates are not 8-byte aligned and because an initial
/// scan needs to take place to know internal buffer offsets.
///
/// This function does an initial pass over the WKB buffer to validate the contents and record the
/// byte offsets for relevant coordinate slices but does not copy the underlying data to an
/// alternate representation. This means that coordinates will **always be constant-time to
/// access** but **not zero-copy**. This is because the raw WKB buffer is not 8-byte aligned, so
/// when accessing a coordinate the underlying bytes need to be copied into a newly-allocated
/// `f64`.
pub fn read_wkb(buf: &[u8]) -> WKBResult<impl GeometryTrait<T = f64> + use<'_>> {
    Wkb::try_new(buf)
}
