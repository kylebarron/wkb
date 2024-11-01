use std::io::Cursor;

use byteorder::{BigEndian, LittleEndian, ReadBytesExt};

use crate::error::WKBResult;
use crate::reader::geometry::Wkb;
use crate::Endianness;
use geo_traits::{Dimensions, GeometryCollectionTrait};

/// skip endianness and wkb type
const HEADER_BYTES: u64 = 5;

/// A WKB GeometryCollection
#[derive(Debug, Clone)]
pub struct GeometryCollection<'a> {
    /// A WKB object for each of the internal geometries
    geometries: Vec<Wkb<'a>>,
    dim: Dimensions,
}

impl<'a> GeometryCollection<'a> {
    pub fn try_new(buf: &'a [u8], byte_order: Endianness, dim: Dimensions) -> WKBResult<Self> {
        let mut reader = Cursor::new(buf);
        reader.set_position(HEADER_BYTES);
        let num_geometries = match byte_order {
            Endianness::BigEndian => reader.read_u32::<BigEndian>().unwrap().try_into().unwrap(),
            Endianness::LittleEndian => reader
                .read_u32::<LittleEndian>()
                .unwrap()
                .try_into()
                .unwrap(),
        };

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

impl<'a> GeometryCollectionTrait for GeometryCollection<'a> {
    type T = f64;
    type GeometryType<'b> = &'b Wkb<'b> where Self: 'b;

    fn dim(&self) -> Dimensions {
        self.dim
    }

    fn num_geometries(&self) -> usize {
        self.geometries.len()
    }

    unsafe fn geometry_unchecked(&self, i: usize) -> Self::GeometryType<'_> {
        &self.geometries[i]
    }
}
