use crate::reader::coord::Coord;
use crate::Endianness;
use geo_traits::Dimensions;
use geo_traits::{CoordTrait, MultiPointTrait, PointTrait};

/// A WKB Point.
///
/// This has been preprocessed, so access to any internal coordinate is `O(1)`.
///
/// See page 66 of <https://portal.ogc.org/files/?artifact_id=25355>.
#[derive(Debug, Clone, Copy)]
pub struct Point<'a> {
    /// The coordinate inside this Point
    coord: Coord<'a>,
    dim: Dimensions,
    is_empty: bool,
}

impl<'a> Point<'a> {
    pub fn new(buf: &'a [u8], byte_order: Endianness, offset: u64, dim: Dimensions) -> Self {
        // The space of the byte order + geometry type
        let offset = offset + 5;
        let coord = Coord::new(buf, byte_order, offset, dim);
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

impl<'a> PointTrait for Point<'a> {
    type T = f64;
    type CoordType<'b> = Coord<'a> where Self: 'b;

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

impl<'a> PointTrait for &Point<'a> {
    type T = f64;
    type CoordType<'b> = Coord<'a> where Self: 'b;

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

impl<'a> MultiPointTrait for Point<'a> {
    type T = f64;
    type PointType<'b> = Point<'a> where Self: 'b;

    fn dim(&self) -> Dimensions {
        self.dim
    }

    fn num_points(&self) -> usize {
        1
    }

    unsafe fn point_unchecked(&self, _i: usize) -> Self::PointType<'_> {
        *self
    }
}

impl<'a> MultiPointTrait for &'a Point<'a> {
    type T = f64;
    type PointType<'b> = Point<'a> where Self: 'b;

    fn dim(&self) -> Dimensions {
        self.dim
    }

    fn num_points(&self) -> usize {
        1
    }

    unsafe fn point_unchecked(&self, _i: usize) -> Self::PointType<'_> {
        **self
    }
}
