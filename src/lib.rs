//! An optimized implementation of reading and writing ISO-flavored WKB-encoded geometries.

mod common;
pub mod error;
pub mod reader;
pub mod writer;

pub use common::{Endianness, WKBType};
pub use reader::WKBGeometry as WKB;
