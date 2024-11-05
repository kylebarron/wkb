use std::io::Cursor;
use std::marker::PhantomData;

use byteorder::{ByteOrder, ReadBytesExt};
use geo_traits::{CoordTrait, Dimensions};

const F64_WIDTH: u64 = 8;

/// A coordinate in a WKB buffer.
///
/// Note that according to the WKB specification this is called `"Point"`, which is **not** the
/// same as a WKB "framed" `Point`. In particular, a "framed" `Point` has framing that includes the
/// byte order and geometry type of the WKB buffer. In contrast, this `Coord` is the building block
/// of two to four f64 numbers that can occur within any geometry type.
///
/// See page 65 of <https://portal.ogc.org/files/?artifact_id=25355>.
#[derive(Debug, Clone, Copy)]
pub struct Coord<'a, B: ByteOrder> {
    /// The underlying WKB buffer
    buf: &'a [u8],

    /// The offset into the buffer where this coordinate is located
    ///
    /// Note that this does not have to be immediately after the WKB header! For a `Point`, the
    /// `Point` is immediately after the header, but the `Point` also appears in other geometry
    /// types. I.e. the `LineString` has a header, then the number of points, then a sequence of
    /// `Point` objects.
    offset: u64,

    dim: Dimensions,

    byte_order: PhantomData<B>,
}

impl<'a, B: ByteOrder> Coord<'a, B> {
    pub(crate) fn new(buf: &'a [u8], offset: u64, dim: Dimensions) -> Self {
        Self {
            buf,
            offset,
            dim,
            byte_order: PhantomData,
        }
    }

    fn get_x(&self) -> f64 {
        let mut reader = Cursor::new(self.buf);
        reader.set_position(self.offset);
        reader.read_f64::<B>().unwrap()
    }

    fn get_y(&self) -> f64 {
        let mut reader = Cursor::new(self.buf);
        reader.set_position(self.offset + F64_WIDTH);
        reader.read_f64::<B>().unwrap()
    }

    fn get_nth_unchecked(&self, n: usize) -> f64 {
        debug_assert!(n < self.dim.size());
        let mut reader = Cursor::new(self.buf);
        reader.set_position(self.offset + (n as u64 * F64_WIDTH));
        reader.read_f64::<B>().unwrap()
    }

    /// The number of bytes in this object
    ///
    /// Note that this is not the same as the length of the underlying buffer
    pub fn size(&self) -> u64 {
        // A 2D Coord is just two f64s
        self.dim.size() as u64 * 8
    }
}

impl<'a, B: ByteOrder> CoordTrait for Coord<'a, B> {
    type T = f64;

    fn dim(&self) -> Dimensions {
        self.dim
    }

    fn nth_unchecked(&self, n: usize) -> Self::T {
        self.get_nth_unchecked(n)
    }

    fn x(&self) -> Self::T {
        self.get_x()
    }

    fn y(&self) -> Self::T {
        self.get_y()
    }
}
