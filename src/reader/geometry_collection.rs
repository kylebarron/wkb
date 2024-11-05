use std::io::Cursor;

use crate::error::WKBResult;
use crate::reader::geometry::Wkb;
use byteorder::{ByteOrder, ReadBytesExt};
use geo_traits::{Dimensions, GeometryCollectionTrait};

/// skip endianness and wkb type
const HEADER_BYTES: u64 = 5;

/// A WKB GeometryCollection
#[derive(Debug, Clone)]
pub struct GeometryCollection<'a, B: ByteOrder> {
    /// A WKB object for each of the internal geometries
    geometries: Vec<Wkb<'a, B>>,
    dim: Dimensions,
}

impl<'a, B: ByteOrder> GeometryCollection<'a, B> {
    pub fn try_new(buf: &'a [u8], dim: Dimensions) -> WKBResult<Self> {
        let mut reader = Cursor::new(buf);
        reader.set_position(HEADER_BYTES);
        let num_geometries = reader.read_u32::<B>().unwrap().try_into().unwrap();

        // - 1: byteOrder
        // - 4: wkbType
        // - 4: numGeometries
        let mut geometry_offset = 1 + 4 + 4;
        let mut geometries = Vec::with_capacity(num_geometries);
        for _ in 0..num_geometries {
            let geometry = Wkb::try_new(&buf[geometry_offset..])?;
            geometry_offset += geometry.size() as usize;
            geometries.push(geometry);
        }

        Ok(Self { geometries, dim })
    }

    pub fn dimension(&self) -> Dimensions {
        self.dim
    }

    pub fn size(&self) -> u64 {
        // - 1: byteOrder
        // - 4: wkbType
        // - 4: numGeometries
        self.geometries
            .iter()
            .fold(1 + 4 + 4, |acc, x| acc + x.size())
    }
}

impl<'a, B: ByteOrder> GeometryCollectionTrait for GeometryCollection<'a, B> {
    type T = f64;
    type GeometryType<'b> = &'b Wkb<'b, B> where Self: 'b;

    fn dim(&self) -> Dimensions {
        self.dim
    }

    fn num_geometries(&self) -> usize {
        self.geometries.len()
    }

    unsafe fn geometry_unchecked(&self, i: usize) -> Self::GeometryType<'_> {
        self.geometries.get_unchecked(i)
    }
}
