use byteorder::ByteOrder;
use geo_traits::Dimensions;
use geo_traits::{CoordTrait, PointTrait};

use crate::reader::coord::Coord;

/// A WKB Point.
///
/// This has been preprocessed, so access to any internal coordinate is `O(1)`.
///
/// See page 66 of <https://portal.ogc.org/files/?artifact_id=25355>.
#[derive(Debug, Clone, Copy)]
pub struct Point<'a, B: ByteOrder> {
    /// The coordinate inside this Point
    coord: Coord<'a, B>,
    dim: Dimensions,
    is_empty: bool,
}

impl<'a, B: ByteOrder> Point<'a, B> {
    pub fn new(buf: &'a [u8], offset: u64, dim: Dimensions) -> Self {
        // The space of the byte order + geometry type
        let offset = offset + 5;
        let coord = Coord::new(buf, offset, dim);
        let is_empty =
            (0..coord.dim().size()).all(|coord_dim| coord.nth_unchecked(coord_dim).is_nan());
        Self {
            coord,
            dim,
            is_empty,
        }
    }

    /// The number of bytes in this object, including any header
    ///
    /// Note that this is not the same as the length of the underlying buffer
    pub fn size(&self) -> u64 {
        // - 1: byteOrder
        // - 4: wkbType
        // - 4: numPoints
        // - dim size * 8: two f64s
        1 + 4 + (self.dim.size() as u64 * 8)
    }

    pub fn dimension(&self) -> Dimensions {
        self.dim
    }
}

impl<'a, B: ByteOrder> PointTrait for Point<'a, B> {
    type T = f64;
    type CoordType<'b> = Coord<'a, B> where Self: 'b;

    fn dim(&self) -> Dimensions {
        self.dim
    }

    fn coord(&self) -> Option<Self::CoordType<'_>> {
        if self.is_empty {
            None
        } else {
            Some(self.coord)
        }
    }
}

impl<'a, B: ByteOrder> PointTrait for &Point<'a, B> {
    type T = f64;
    type CoordType<'b> = Coord<'a, B> where Self: 'b;

    fn dim(&self) -> Dimensions {
        self.dim
    }

    fn coord(&self) -> Option<Self::CoordType<'_>> {
        if self.is_empty {
            None
        } else {
            Some(self.coord)
        }
    }
}
