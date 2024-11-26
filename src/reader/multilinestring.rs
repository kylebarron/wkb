use std::io::Cursor;

use crate::common::WKBDimension;
use crate::reader::linestring::LineString;
use crate::reader::util::ReadBytesExt;
use crate::Endianness;
use geo_traits::Dimensions;
use geo_traits::MultiLineStringTrait;

const HEADER_BYTES: u64 = 5;

/// A WKB MultiLineString
///
/// This has been preprocessed, so access to any internal coordinate is `O(1)`.
#[derive(Debug, Clone)]
pub struct MultiLineString<'a> {
    /// A LineString object for each of the internal line strings
    wkb_line_strings: Vec<LineString<'a>>,
    dim: WKBDimension,
}

impl<'a> MultiLineString<'a> {
    pub(crate) fn new(buf: &'a [u8], byte_order: Endianness, dim: WKBDimension) -> Self {
        let mut reader = Cursor::new(buf);
        reader.set_position(HEADER_BYTES);
        let num_line_strings = reader.read_u32(byte_order).unwrap().try_into().unwrap();

        // - 1: byteOrder
        // - 4: wkbType
        // - 4: numLineStrings
        let mut line_string_offset = 1 + 4 + 4;
        let mut wkb_line_strings = Vec::with_capacity(num_line_strings);
        for _ in 0..num_line_strings {
            let ls = LineString::new(buf, byte_order, line_string_offset, dim);
            wkb_line_strings.push(ls);
            line_string_offset += ls.size();
        }

        Self {
            wkb_line_strings,
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
        // - Point::size() * self.num_points: the size of each Point for each point
        self.wkb_line_strings
            .iter()
            .fold(1 + 4 + 4, |acc, ls| acc + ls.size())
    }

    pub fn dimension(&self) -> WKBDimension {
        self.dim
    }
}

impl<'a> MultiLineStringTrait for MultiLineString<'a> {
    type T = f64;
    type LineStringType<'b> = LineString<'a> where Self: 'b;

    fn dim(&self) -> Dimensions {
        self.dim.into()
    }

    fn num_line_strings(&self) -> usize {
        self.wkb_line_strings.len()
    }

    unsafe fn line_string_unchecked(&self, i: usize) -> Self::LineStringType<'_> {
        *self.wkb_line_strings.get_unchecked(i)
    }
}

impl<'a> MultiLineStringTrait for &'a MultiLineString<'a> {
    type T = f64;
    type LineStringType<'b> = LineString<'a> where Self: 'b;

    fn dim(&self) -> Dimensions {
        self.dim.into()
    }

    fn num_line_strings(&self) -> usize {
        self.wkb_line_strings.len()
    }

    unsafe fn line_string_unchecked(&self, i: usize) -> Self::LineStringType<'_> {
        *self.wkb_line_strings.get_unchecked(i)
    }
}
