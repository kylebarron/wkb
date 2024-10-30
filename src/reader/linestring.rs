use std::io::Cursor;

use byteorder::{BigEndian, LittleEndian, ReadBytesExt};

use crate::reader::coord::WKBCoord;
use crate::Endianness;
use geo_traits::Dimensions;
use geo_traits::{LineStringTrait, MultiLineStringTrait};

const HEADER_BYTES: u64 = 5;

/// A WKB LineString
///
/// This has been preprocessed, so access to any internal coordinate is `O(1)`.
#[derive(Debug, Clone, Copy)]
pub struct WKBLineString<'a> {
    buf: &'a [u8],
    byte_order: Endianness,

    /// The number of points in this LineString WKB
    num_points: usize,

    /// This offset will be 0 for a single WKBLineString but it will be non zero for a
    /// WKBLineString contained within a WKBMultiLineString
    offset: u64,
    dim: Dimensions,
}

impl<'a> WKBLineString<'a> {
    pub fn new(buf: &'a [u8], byte_order: Endianness, offset: u64, dim: Dimensions) -> Self {
        let mut reader = Cursor::new(buf);
        reader.set_position(HEADER_BYTES + offset);
        let num_points = match byte_order {
            Endianness::BigEndian => reader.read_u32::<BigEndian>().unwrap().try_into().unwrap(),
            Endianness::LittleEndian => reader
                .read_u32::<LittleEndian>()
                .unwrap()
                .try_into()
                .unwrap(),
        };

        Self {
            buf,
            byte_order,
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

impl<'a> LineStringTrait for WKBLineString<'a> {
    type T = f64;
    type CoordType<'b> = WKBCoord<'a> where Self: 'b;

    fn dim(&self) -> Dimensions {
        self.dim
    }

    fn num_coords(&self) -> usize {
        self.num_points
    }

    unsafe fn coord_unchecked(&self, i: usize) -> Self::CoordType<'_> {
        WKBCoord::new(
            self.buf,
            self.byte_order,
            self.coord_offset(i.try_into().unwrap()),
            self.dim,
        )
    }
}

impl<'a> LineStringTrait for &'a WKBLineString<'a> {
    type T = f64;
    type CoordType<'b> = WKBCoord<'a> where Self: 'b;

    fn dim(&self) -> Dimensions {
        self.dim
    }

    fn num_coords(&self) -> usize {
        self.num_points
    }

    unsafe fn coord_unchecked(&self, i: usize) -> Self::CoordType<'_> {
        WKBCoord::new(
            self.buf,
            self.byte_order,
            self.coord_offset(i.try_into().unwrap()),
            self.dim,
        )
    }
}

impl<'a> MultiLineStringTrait for WKBLineString<'a> {
    type T = f64;
    type LineStringType<'b> = WKBLineString<'a> where Self: 'b;

    fn dim(&self) -> Dimensions {
        self.dim
    }

    fn num_line_strings(&self) -> usize {
        1
    }

    unsafe fn line_string_unchecked(&self, _i: usize) -> Self::LineStringType<'_> {
        *self
    }
}

impl<'a> MultiLineStringTrait for &'a WKBLineString<'a> {
    type T = f64;
    type LineStringType<'b> = WKBLineString<'a> where Self: 'b;

    fn dim(&self) -> Dimensions {
        self.dim
    }

    fn num_line_strings(&self) -> usize {
        1
    }

    unsafe fn line_string_unchecked(&self, _i: usize) -> Self::LineStringType<'_> {
        **self
    }
}
