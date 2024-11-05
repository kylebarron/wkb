use std::io::Cursor;

use crate::reader::linearring::WKBLinearRing;
use byteorder::{ByteOrder, ReadBytesExt};
use geo_traits::Dimensions;
use geo_traits::PolygonTrait;

const WKB_POLYGON_TYPE: u32 = 3;

/// A WKB Polygon
///
/// This has been preprocessed, so access to any internal coordinate is `O(1)`.
#[derive(Debug, Clone)]
pub struct Polygon<'a, B: ByteOrder> {
    wkb_linear_rings: Vec<WKBLinearRing<'a, B>>,
    dim: Dimensions,
}

impl<'a, B: ByteOrder> Polygon<'a, B> {
    pub fn new(buf: &'a [u8], offset: u64, dim: Dimensions) -> Self {
        let mut reader = Cursor::new(buf);
        reader.set_position(1 + offset);

        // Assert that this is indeed a 2D Polygon
        assert_eq!(WKB_POLYGON_TYPE, reader.read_u32::<B>().unwrap());

        let num_rings = reader.read_u32::<B>().unwrap().try_into().unwrap();

        // - existing offset into buffer
        // - 1: byteOrder
        // - 4: wkbType
        // - 4: numLineStrings
        let mut ring_offset = offset + 1 + 4 + 4;
        let mut wkb_linear_rings = Vec::with_capacity(num_rings);
        for _ in 0..num_rings {
            let polygon = WKBLinearRing::new(buf, ring_offset, dim);
            wkb_linear_rings.push(polygon);
            ring_offset += polygon.size();
        }

        Self {
            wkb_linear_rings,
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
        // - size of each linear ring
        self.wkb_linear_rings
            .iter()
            .fold(1 + 4 + 4, |acc, ring| acc + ring.size())
    }

    pub fn is_empty(&self) -> bool {
        self.wkb_linear_rings.len() == 0
    }

    pub fn dimension(&self) -> Dimensions {
        self.dim
    }
}

impl<'a, B: ByteOrder> PolygonTrait for Polygon<'a, B> {
    type T = f64;
    type RingType<'b> = WKBLinearRing<'a, B> where Self: 'b;

    fn dim(&self) -> Dimensions {
        self.dim
    }

    fn num_interiors(&self) -> usize {
        // Support an empty polygon with no rings
        if self.wkb_linear_rings.is_empty() {
            0
        } else {
            self.wkb_linear_rings.len() - 1
        }
    }

    fn exterior(&self) -> Option<Self::RingType<'_>> {
        if self.wkb_linear_rings.is_empty() {
            None
        } else {
            Some(self.wkb_linear_rings[0])
        }
    }

    unsafe fn interior_unchecked(&self, i: usize) -> Self::RingType<'_> {
        *self.wkb_linear_rings.get_unchecked(i + 1)
    }
}

impl<'a, B: ByteOrder> PolygonTrait for &'a Polygon<'a, B> {
    type T = f64;
    type RingType<'b> = WKBLinearRing<'a, B> where Self: 'b;

    fn dim(&self) -> Dimensions {
        self.dim
    }

    fn num_interiors(&self) -> usize {
        // Support an empty polygon with no rings
        if self.wkb_linear_rings.is_empty() {
            0
        } else {
            self.wkb_linear_rings.len() - 1
        }
    }

    fn exterior(&self) -> Option<Self::RingType<'_>> {
        if self.wkb_linear_rings.is_empty() {
            None
        } else {
            Some(self.wkb_linear_rings[0])
        }
    }

    unsafe fn interior_unchecked(&self, i: usize) -> Self::RingType<'_> {
        *self.wkb_linear_rings.get_unchecked(i + 1)
    }
}
