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

pub use geometry::WKB;
pub use geometry_collection::GeometryCollection;
pub use linestring::LineString;
pub use multilinestring::MultiLineString;
pub use multipoint::MultiPoint;
pub use multipolygon::MultiPolygon;
pub use point::Point;
pub use polygon::Polygon;
