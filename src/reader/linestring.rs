use std::io::Cursor;
use std::marker::PhantomData;

use crate::reader::coord::Coord;
use byteorder::{ByteOrder, ReadBytesExt};
use geo_traits::Dimensions;
use geo_traits::LineStringTrait;

const HEADER_BYTES: u64 = 5;

/// A WKB LineString
///
/// This has been preprocessed, so access to any internal coordinate is `O(1)`.
#[derive(Debug, Clone, Copy)]
pub struct LineString<'a, B: ByteOrder> {
    buf: &'a [u8],
    byte_order: PhantomData<B>,

    /// The number of points in this LineString WKB
    num_points: usize,

    /// This offset will be 0 for a single LineString but it will be non zero for a
    /// LineString contained within a MultiLineString
    offset: u64,
    dim: Dimensions,
}

impl<'a, B: ByteOrder> LineString<'a, B> {
    pub fn new(buf: &'a [u8], offset: u64, dim: Dimensions) -> Self {
        let mut reader = Cursor::new(buf);
        reader.set_position(HEADER_BYTES + offset);
        let num_points = reader.read_u32::<B>().unwrap().try_into().unwrap();

        Self {
            buf,
            byte_order: PhantomData,
            num_points,
            offset,
            dim,
        }
    }

    /// The number of bytes in this object, including any header
    ///
    /// Note that this is not the same as the length of the underlying buffer
    pub fn size(&self) -> u64 {
        // - 1: byteOrder
        // - 4: wkbType
        // - 4: numPoints
        // - 2 * 8 * self.num_points: two f64s for each coordinate
        1 + 4 + 4 + (self.dim.size() as u64 * 8 * self.num_points as u64)
    }

    /// The offset into this buffer of any given coordinate
    pub fn coord_offset(&self, i: u64) -> u64 {
        self.offset + 1 + 4 + 4 + (self.dim.size() as u64 * 8 * i)
    }

    pub fn dimension(&self) -> Dimensions {
        self.dim
    }
}

impl<'a, B: ByteOrder> LineStringTrait for LineString<'a, B> {
    type T = f64;
    type CoordType<'b> = Coord<'a, B> where Self: 'b;

    fn dim(&self) -> Dimensions {
        self.dim
    }

    fn num_coords(&self) -> usize {
        self.num_points
    }

    unsafe fn coord_unchecked(&self, i: usize) -> Self::CoordType<'_> {
        Coord::new(self.buf, self.coord_offset(i.try_into().unwrap()), self.dim)
    }
}

impl<'a, B: ByteOrder> LineStringTrait for &'a LineString<'a, B> {
    type T = f64;
    type CoordType<'b> = Coord<'a, B> where Self: 'b;

    fn dim(&self) -> Dimensions {
        self.dim
    }

    fn num_coords(&self) -> usize {
        self.num_points
    }

    unsafe fn coord_unchecked(&self, i: usize) -> Self::CoordType<'_> {
        Coord::new(self.buf, self.coord_offset(i.try_into().unwrap()), self.dim)
    }
}
