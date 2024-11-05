use std::io::Cursor;

use crate::reader::polygon::Polygon;
use byteorder::{ByteOrder, ReadBytesExt};
use geo_traits::Dimensions;
use geo_traits::MultiPolygonTrait;

/// skip endianness and wkb type
const HEADER_BYTES: u64 = 5;

/// A WKB MultiPolygon
#[derive(Debug, Clone)]
pub struct MultiPolygon<'a, B: ByteOrder> {
    /// A Polygon object for each of the internal line strings
    wkb_polygons: Vec<Polygon<'a, B>>,

    dim: Dimensions,
}

impl<'a, B: ByteOrder> MultiPolygon<'a, B> {
    pub(crate) fn new(buf: &'a [u8], dim: Dimensions) -> Self {
        let mut reader = Cursor::new(buf);
        reader.set_position(HEADER_BYTES);
        let num_polygons = reader.read_u32::<B>().unwrap().try_into().unwrap();

        // - 1: byteOrder
        // - 4: wkbType
        // - 4: numLineStrings
        let mut polygon_offset = 1 + 4 + 4;
        let mut wkb_polygons = Vec::with_capacity(num_polygons);
        for _ in 0..num_polygons {
            let polygon = Polygon::new(buf, polygon_offset, dim);
            polygon_offset += polygon.size();
            wkb_polygons.push(polygon);
        }

        Self { wkb_polygons, dim }
    }

    /// The number of bytes in this object, including any header
    ///
    /// Note that this is not the same as the length of the underlying buffer
    pub fn size(&self) -> u64 {
        // - 1: byteOrder
        // - 4: wkbType
        // - 4: numPolygons
        self.wkb_polygons
            .iter()
            .fold(1 + 4 + 4, |acc, x| acc + x.size())
    }

    pub fn dimension(&self) -> Dimensions {
        self.dim
    }
}

impl<'a, B: ByteOrder> MultiPolygonTrait for MultiPolygon<'a, B> {
    type T = f64;
    type PolygonType<'b> = Polygon<'a, B> where Self: 'b;

    fn dim(&self) -> Dimensions {
        self.dim
    }

    fn num_polygons(&self) -> usize {
        self.wkb_polygons.len()
    }

    unsafe fn polygon_unchecked(&self, i: usize) -> Self::PolygonType<'_> {
        self.wkb_polygons.get_unchecked(i).clone()
    }
}

impl<'a, B: ByteOrder> MultiPolygonTrait for &'a MultiPolygon<'a, B> {
    type T = f64;
    type PolygonType<'b> = Polygon<'a, B> where Self: 'b;

    fn dim(&self) -> Dimensions {
        self.dim
    }

    fn num_polygons(&self) -> usize {
        self.wkb_polygons.len()
    }

    unsafe fn polygon_unchecked(&self, i: usize) -> Self::PolygonType<'_> {
        self.wkb_polygons.get_unchecked(i).clone()
    }
}
