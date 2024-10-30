use std::io::Cursor;

use byteorder::ReadBytesExt;

use crate::common::WKBType;
use crate::error::WKBResult;
use crate::reader::{
    GeometryCollection, LineString, MultiLineString, MultiPoint, MultiPolygon, Point, Polygon,
};
use geo_traits::{
    Dimensions, GeometryTrait, UnimplementedLine, UnimplementedRect, UnimplementedTriangle,
};

#[derive(Debug, Clone)]
pub enum WKB<'a> {
    Point(Point<'a>),
    LineString(LineString<'a>),
    Polygon(Polygon<'a>),
    MultiPoint(MultiPoint<'a>),
    MultiLineString(MultiLineString<'a>),
    MultiPolygon(MultiPolygon<'a>),
    GeometryCollection(GeometryCollection<'a>),
}

impl<'a> WKB<'a> {
    pub fn try_new(buf: &'a [u8]) -> WKBResult<Self> {
        let mut reader = Cursor::new(buf);
        let byte_order = reader.read_u8().unwrap();
        let wkb_type = WKBType::from_buffer(buf)?;

        use Dimensions::*;

        let out = match wkb_type {
            WKBType::Point => WKB::Point(Point::new(buf, byte_order.into(), 0, Xy)),
            WKBType::LineString => WKB::LineString(LineString::new(buf, byte_order.into(), 0, Xy)),
            WKBType::Polygon => WKB::Polygon(Polygon::new(buf, byte_order.into(), 0, Xy)),
            WKBType::MultiPoint => WKB::MultiPoint(MultiPoint::new(buf, byte_order.into(), Xy)),
            WKBType::MultiLineString => {
                WKB::MultiLineString(MultiLineString::new(buf, byte_order.into(), Xy))
            }
            WKBType::MultiPolygon => {
                WKB::MultiPolygon(MultiPolygon::new(buf, byte_order.into(), Xy))
            }
            WKBType::GeometryCollection => {
                WKB::GeometryCollection(GeometryCollection::try_new(buf, byte_order.into(), Xy)?)
            }
            WKBType::PointZ => WKB::Point(Point::new(buf, byte_order.into(), 0, Xyz)),
            WKBType::LineStringZ => {
                WKB::LineString(LineString::new(buf, byte_order.into(), 0, Xyz))
            }
            WKBType::PolygonZ => WKB::Polygon(Polygon::new(buf, byte_order.into(), 0, Xyz)),
            WKBType::MultiPointZ => WKB::MultiPoint(MultiPoint::new(buf, byte_order.into(), Xyz)),
            WKBType::MultiLineStringZ => {
                WKB::MultiLineString(MultiLineString::new(buf, byte_order.into(), Xyz))
            }
            WKBType::MultiPolygonZ => {
                WKB::MultiPolygon(MultiPolygon::new(buf, byte_order.into(), Xyz))
            }
            WKBType::GeometryCollectionZ => {
                WKB::GeometryCollection(GeometryCollection::try_new(buf, byte_order.into(), Xyz)?)
            }
        };
        Ok(out)
    }

    pub fn into_point(self) -> Point<'a> {
        match self {
            WKB::Point(geom) => geom,
            _ => panic!(),
        }
    }

    pub fn into_line_string(self) -> LineString<'a> {
        match self {
            WKB::LineString(geom) => geom,
            _ => panic!(),
        }
    }

    pub fn into_polygon(self) -> Polygon<'a> {
        match self {
            WKB::Polygon(geom) => geom,
            _ => panic!(),
        }
    }

    pub fn into_multi_point(self) -> MultiPoint<'a> {
        match self {
            WKB::MultiPoint(geom) => geom,
            _ => panic!(),
        }
    }

    pub fn into_multi_line_string(self) -> MultiLineString<'a> {
        match self {
            WKB::MultiLineString(geom) => geom,
            _ => panic!(),
        }
    }

    pub fn into_multi_polygon(self) -> MultiPolygon<'a> {
        match self {
            WKB::MultiPolygon(geom) => geom,
            _ => panic!(),
        }
    }

    pub fn dimension(&self) -> Dimensions {
        use WKB::*;
        match self {
            Point(g) => g.dimension(),
            LineString(g) => g.dimension(),
            Polygon(g) => g.dimension(),
            MultiPoint(g) => g.dimension(),
            MultiLineString(g) => g.dimension(),
            MultiPolygon(g) => g.dimension(),
            GeometryCollection(g) => g.dimension(),
        }
    }

    pub fn size(&self) -> u64 {
        use WKB::*;
        match self {
            Point(g) => g.size(),
            LineString(g) => g.size(),
            Polygon(g) => g.size(),
            MultiPoint(g) => g.size(),
            MultiLineString(g) => g.size(),
            MultiPolygon(g) => g.size(),
            GeometryCollection(g) => g.size(),
        }
    }
}

impl<'a> GeometryTrait for WKB<'a> {
    type T = f64;
    type PointType<'b> = Point<'a> where Self: 'b;
    type LineStringType<'b> = LineString<'a> where Self: 'b;
    type PolygonType<'b> = Polygon<'a> where Self: 'b;
    type MultiPointType<'b> = MultiPoint<'a> where Self: 'b;
    type MultiLineStringType<'b> = MultiLineString<'a> where Self: 'b;
    type MultiPolygonType<'b> = MultiPolygon<'a> where Self: 'b;
    type GeometryCollectionType<'b> = GeometryCollection<'a> where Self: 'b;
    type RectType<'b> = UnimplementedRect<f64> where Self: 'b;
    type TriangleType<'b> = UnimplementedTriangle<f64> where Self: 'b;
    type LineType<'b> = UnimplementedLine<f64> where Self: 'b;

    fn dim(&self) -> Dimensions {
        self.dimension()
    }

    fn as_type(
        &self,
    ) -> geo_traits::GeometryType<
        '_,
        Point<'a>,
        LineString<'a>,
        Polygon<'a>,
        MultiPoint<'a>,
        MultiLineString<'a>,
        MultiPolygon<'a>,
        GeometryCollection<'a>,
        UnimplementedRect<f64>,
        UnimplementedTriangle<f64>,
        UnimplementedLine<f64>,
    > {
        use geo_traits::GeometryType as B;
        use WKB as A;
        match self {
            A::Point(p) => B::Point(p),
            A::LineString(ls) => B::LineString(ls),
            A::Polygon(ls) => B::Polygon(ls),
            A::MultiPoint(ls) => B::MultiPoint(ls),
            A::MultiLineString(ls) => B::MultiLineString(ls),
            A::MultiPolygon(ls) => B::MultiPolygon(ls),
            A::GeometryCollection(gc) => B::GeometryCollection(gc),
        }
    }
}

impl<'a> GeometryTrait for &'a WKB<'a> {
    type T = f64;
    type PointType<'b> = Point<'a> where Self: 'b;
    type LineStringType<'b> = LineString<'a> where Self: 'b;
    type PolygonType<'b> = Polygon<'a> where Self: 'b;
    type MultiPointType<'b> = MultiPoint<'a> where Self: 'b;
    type MultiLineStringType<'b> = MultiLineString<'a> where Self: 'b;
    type MultiPolygonType<'b> = MultiPolygon<'a> where Self: 'b;
    type GeometryCollectionType<'b> = GeometryCollection<'a> where Self: 'b;
    type RectType<'b> = UnimplementedRect<f64> where Self: 'b;
    type TriangleType<'b> = UnimplementedTriangle<f64> where Self: 'b;
    type LineType<'b> = UnimplementedLine<f64> where Self: 'b;

    fn dim(&self) -> Dimensions {
        self.dimension()
    }

    fn as_type(
        &self,
    ) -> geo_traits::GeometryType<
        '_,
        Point<'a>,
        LineString<'a>,
        Polygon<'a>,
        MultiPoint<'a>,
        MultiLineString<'a>,
        MultiPolygon<'a>,
        GeometryCollection<'a>,
        UnimplementedRect<f64>,
        UnimplementedTriangle<f64>,
        UnimplementedLine<f64>,
    > {
        use geo_traits::GeometryType as B;
        use WKB as A;
        match self {
            A::Point(p) => B::Point(p),
            A::LineString(ls) => B::LineString(ls),
            A::Polygon(ls) => B::Polygon(ls),
            A::MultiPoint(ls) => B::MultiPoint(ls),
            A::MultiLineString(ls) => B::MultiLineString(ls),
            A::MultiPolygon(ls) => B::MultiPolygon(ls),
            A::GeometryCollection(gc) => B::GeometryCollection(gc),
        }
    }
}
