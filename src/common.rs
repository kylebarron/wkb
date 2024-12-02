use std::io::Cursor;

use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use num_enum::{IntoPrimitive, TryFromPrimitive};

use crate::error::{WKBError, WKBResult};

/// Supported WKB dimensions
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKBDimension {
    Xy,
    Xyz,
    Xym,
    Xyzm,
}

impl WKBDimension {
    fn as_u32_offset(&self) -> u32 {
        match self {
            Self::Xy => 0,
            Self::Xyz => 1000,
            Self::Xym => 2000,
            Self::Xyzm => 3000,
        }
    }

    pub(crate) fn size(&self) -> usize {
        match self {
            Self::Xy => 2,
            Self::Xyz | Self::Xym => 3,
            Self::Xyzm => 4,
        }
    }
}

impl TryFrom<geo_traits::Dimensions> for WKBDimension {
    type Error = WKBError;

    fn try_from(value: geo_traits::Dimensions) -> Result<Self, Self::Error> {
        use geo_traits::Dimensions::*;

        let result = match value {
            Xy | Unknown(2) => Self::Xy,
            Xyz | Unknown(3) => Self::Xyz,
            Xym => Self::Xym,
            Xyzm | Unknown(4) => Self::Xyzm,
            Unknown(n_dim) => {
                return Err(WKBError::General(format!(
                    "Unsupported number of dimensions: {}",
                    n_dim
                )))
            }
        };
        Ok(result)
    }
}

impl From<WKBDimension> for geo_traits::Dimensions {
    fn from(value: WKBDimension) -> Self {
        match value {
            WKBDimension::Xy => Self::Xy,
            WKBDimension::Xyz => Self::Xyz,
            WKBDimension::Xym => Self::Xym,
            WKBDimension::Xyzm => Self::Xyzm,
        }
    }
}

/// The various WKB types supported by this crate
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKBType {
    /// A WKB Point
    Point(WKBDimension),
    /// A WKB LineString
    LineString(WKBDimension),
    /// A WKB Polygon
    Polygon(WKBDimension),
    /// A WKB MultiPoint
    MultiPoint(WKBDimension),
    /// A WKB MultiLineString
    MultiLineString(WKBDimension),
    /// A WKB MultiPolygon
    MultiPolygon(WKBDimension),
    /// A WKB GeometryCollection
    GeometryCollection(WKBDimension),
}

impl WKBType {
    /// Construct from a byte slice representing a WKB geometry
    pub fn from_buffer(buf: &[u8]) -> WKBResult<Self> {
        let mut reader = Cursor::new(buf);
        let byte_order = reader.read_u8().unwrap();
        let geometry_type = match byte_order {
            0 => reader.read_u32::<BigEndian>().unwrap(),
            1 => reader.read_u32::<LittleEndian>().unwrap(),
            other => {
                return Err(WKBError::General(format!(
                    "Unexpected byte order: {}",
                    other
                )))
            }
        };
        Self::try_from_u32(geometry_type)
    }

    pub fn try_from_u32(value: u32) -> WKBResult<Self> {
        // Values 1, 2, 3 are 2D,
        // 1001, 1002, 1003 are XYZ,
        // 2001 etc are XYM,
        // 3001 etc are XYZM
        let dim = match value / 1000 {
            0 => WKBDimension::Xy,
            1 => WKBDimension::Xyz,
            2 => WKBDimension::Xym,
            3 => WKBDimension::Xyzm,
            _ => {
                return Err(WKBError::General(format!(
                    "WKB dimension value out of range. Got: {}",
                    value
                )))
            }
        };
        let typ = match value % 1000 {
            1 => WKBType::Point(dim),
            2 => WKBType::LineString(dim),
            3 => WKBType::Polygon(dim),
            4 => WKBType::MultiPoint(dim),
            5 => WKBType::MultiLineString(dim),
            6 => WKBType::MultiPolygon(dim),
            7 => WKBType::GeometryCollection(dim),
            _ => {
                return Err(WKBError::General(format!(
                    "WKB type value out of range. Got: {}",
                    value
                )))
            }
        };
        Ok(typ)
    }

    pub fn as_u32(&self) -> u32 {
        match self {
            Self::Point(dim) => 1 + dim.as_u32_offset(),
            Self::LineString(dim) => 2 + dim.as_u32_offset(),
            Self::Polygon(dim) => 3 + dim.as_u32_offset(),
            Self::MultiPoint(dim) => 4 + dim.as_u32_offset(),
            Self::MultiLineString(dim) => 5 + dim.as_u32_offset(),
            Self::MultiPolygon(dim) => 6 + dim.as_u32_offset(),
            Self::GeometryCollection(dim) => 7 + dim.as_u32_offset(),
        }
    }
}

impl From<WKBType> for u32 {
    fn from(value: WKBType) -> Self {
        value.as_u32()
    }
}

/// Endianness
#[derive(Debug, Clone, Copy, Default, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum Endianness {
    BigEndian = 0,
    #[default]
    LittleEndian = 1,
}
