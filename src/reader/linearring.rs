use std::io::Cursor;
use std::marker::PhantomData;

use crate::reader::coord::Coord;
use byteorder::{ByteOrder, ReadBytesExt};
use geo_traits::Dimensions;
use geo_traits::LineStringTrait;

/// A linear ring in a WKB buffer.
///
/// This has been preprocessed, so access to any internal coordinate is `O(1)`.
///
/// See page 65 of <https://portal.ogc.org/files/?artifact_id=25355>.
#[derive(Debug, Clone, Copy)]
pub struct WKBLinearRing<'a, B: ByteOrder> {
    /// The underlying WKB buffer
    buf: &'a [u8],

    /// The byte order of this WKB buffer
    byte_order: PhantomData<B>,

    /// The offset into the buffer where this linear ring is located
    ///
    /// Note that this does not have to be immediately after the WKB header! For a `Point`, the
    /// `Point` is immediately after the header, but the `Point` also appears in other geometry
    /// types. I.e. the `LineString` has a header, then the number of points, then a sequence of
    /// `Point` objects.
    offset: u64,

    /// The number of points in this linear ring
    num_points: usize,

    dim: Dimensions,
}

impl<'a, B: ByteOrder> WKBLinearRing<'a, B> {
    pub fn new(buf: &'a [u8], offset: u64, dim: Dimensions) -> Self {
        let mut reader = Cursor::new(buf);
        reader.set_position(offset);
        let num_points = reader.read_u32::<B>().unwrap().try_into().unwrap();

        Self {
            buf,
            offset,
            num_points,
            dim,
            byte_order: PhantomData,
        }
    }

    /// The number of bytes in this object, including any header
    ///
    /// Note that this is not the same as the length of the underlying buffer
    pub fn size(&self) -> u64 {
        // - 4: numPoints
        // - 2 * 8 * self.num_points: two f64s for each coordinate
        4 + (self.dim.size() as u64 * 8 * self.num_points as u64)
    }

    /// The offset into this buffer of any given coordinate
    pub fn coord_offset(&self, i: u64) -> u64 {
        self.offset + 4 + (self.dim.size() as u64 * 8 * i)
    }
}

impl<'a, B: ByteOrder> LineStringTrait for WKBLinearRing<'a, B> {
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
