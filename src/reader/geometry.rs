use std::io::Cursor;

use byteorder::ReadBytesExt;

use crate::common::{WKBDimension, WKBType};
use crate::error::WKBResult;
use crate::reader::{
    GeometryCollection, LineString, MultiLineString, MultiPoint, MultiPolygon, Point, Polygon,
};
use crate::Endianness;
use geo_traits::{
    Dimensions, GeometryTrait, UnimplementedLine, UnimplementedRect, UnimplementedTriangle,
};

#[derive(Debug, Clone)]
pub enum Wkb<'a> {
    Point(Point<'a>),
    LineString(LineString<'a>),
    Polygon(Polygon<'a>),
    MultiPoint(MultiPoint<'a>),
    MultiLineString(MultiLineString<'a>),
    MultiPolygon(MultiPolygon<'a>),
    GeometryCollection(GeometryCollection<'a>),
}

impl<'a> Wkb<'a> {
    pub fn try_new(buf: &'a [u8]) -> WKBResult<Self> {
        let mut reader = Cursor::new(buf);
        let byte_order = Endianness::try_from(reader.read_u8()?).unwrap();
        let wkb_type = WKBType::from_buffer(buf)?;

        let out = match wkb_type {
            WKBType::Point(dim) => Wkb::Point(Point::new(buf, byte_order, 0, dim)),
            WKBType::LineString(dim) => Wkb::LineString(LineString::new(buf, byte_order, 0, dim)),
            WKBType::Polygon(dim) => Wkb::Polygon(Polygon::new(buf, byte_order, 0, dim)),
            WKBType::MultiPoint(dim) => Wkb::MultiPoint(MultiPoint::new(buf, byte_order, dim)),
            WKBType::MultiLineString(dim) => {
                Wkb::MultiLineString(MultiLineString::new(buf, byte_order, dim))
            }
            WKBType::MultiPolygon(dim) => {
                Wkb::MultiPolygon(MultiPolygon::new(buf, byte_order, dim))
            }
            WKBType::GeometryCollection(dim) => {
                Wkb::GeometryCollection(GeometryCollection::try_new(buf, byte_order, dim)?)
            }
        };
        Ok(out)
    }

    pub fn dimension(&self) -> WKBDimension {
        use Wkb::*;
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
        use Wkb::*;
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

impl<'a> GeometryTrait for Wkb<'a> {
    type T = f64;
    type PointType<'b>
        = Point<'a>
    where
        Self: 'b;
    type LineStringType<'b>
        = LineString<'a>
    where
        Self: 'b;
    type PolygonType<'b>
        = Polygon<'a>
    where
        Self: 'b;
    type MultiPointType<'b>
        = MultiPoint<'a>
    where
        Self: 'b;
    type MultiLineStringType<'b>
        = MultiLineString<'a>
    where
        Self: 'b;
    type MultiPolygonType<'b>
        = MultiPolygon<'a>
    where
        Self: 'b;
    type GeometryCollectionType<'b>
        = GeometryCollection<'a>
    where
        Self: 'b;
    type RectType<'b>
        = UnimplementedRect<f64>
    where
        Self: 'b;
    type TriangleType<'b>
        = UnimplementedTriangle<f64>
    where
        Self: 'b;
    type LineType<'b>
        = UnimplementedLine<f64>
    where
        Self: 'b;

    fn dim(&self) -> Dimensions {
        self.dimension().into()
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
        use Wkb as A;
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

impl<'a> GeometryTrait for &'a Wkb<'a> {
    type T = f64;
    type PointType<'b>
        = Point<'a>
    where
        Self: 'b;
    type LineStringType<'b>
        = LineString<'a>
    where
        Self: 'b;
    type PolygonType<'b>
        = Polygon<'a>
    where
        Self: 'b;
    type MultiPointType<'b>
        = MultiPoint<'a>
    where
        Self: 'b;
    type MultiLineStringType<'b>
        = MultiLineString<'a>
    where
        Self: 'b;
    type MultiPolygonType<'b>
        = MultiPolygon<'a>
    where
        Self: 'b;
    type GeometryCollectionType<'b>
        = GeometryCollection<'a>
    where
        Self: 'b;
    type RectType<'b>
        = UnimplementedRect<f64>
    where
        Self: 'b;
    type TriangleType<'b>
        = UnimplementedTriangle<f64>
    where
        Self: 'b;
    type LineType<'b>
        = UnimplementedLine<f64>
    where
        Self: 'b;

    fn dim(&self) -> Dimensions {
        self.dimension().into()
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
        use Wkb as A;
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

// Specialized implementations on each WKT concrete type.

macro_rules! impl_specialization {
    ($geometry_type:ident) => {
        impl GeometryTrait for $geometry_type<'_> {
            type T = f64;
            type PointType<'b>
                = Point<'b>
            where
                Self: 'b;
            type LineStringType<'b>
                = LineString<'b>
            where
                Self: 'b;
            type PolygonType<'b>
                = Polygon<'b>
            where
                Self: 'b;
            type MultiPointType<'b>
                = MultiPoint<'b>
            where
                Self: 'b;
            type MultiLineStringType<'b>
                = MultiLineString<'b>
            where
                Self: 'b;
            type MultiPolygonType<'b>
                = MultiPolygon<'b>
            where
                Self: 'b;
            type GeometryCollectionType<'b>
                = GeometryCollection<'b>
            where
                Self: 'b;
            type RectType<'b>
                = geo_traits::UnimplementedRect<f64>
            where
                Self: 'b;
            type LineType<'b>
                = geo_traits::UnimplementedLine<f64>
            where
                Self: 'b;
            type TriangleType<'b>
                = geo_traits::UnimplementedTriangle<f64>
            where
                Self: 'b;

            fn dim(&self) -> geo_traits::Dimensions {
                self.dimension().into()
            }

            fn as_type(
                &self,
            ) -> geo_traits::GeometryType<
                '_,
                Point,
                LineString,
                Polygon,
                MultiPoint,
                MultiLineString,
                MultiPolygon,
                GeometryCollection,
                Self::RectType<'_>,
                Self::TriangleType<'_>,
                Self::LineType<'_>,
            > {
                geo_traits::GeometryType::$geometry_type(self)
            }
        }

        impl<'a> GeometryTrait for &'a $geometry_type<'_> {
            type T = f64;
            type PointType<'b>
                = Point<'b>
            where
                Self: 'b;
            type LineStringType<'b>
                = LineString<'b>
            where
                Self: 'b;
            type PolygonType<'b>
                = Polygon<'b>
            where
                Self: 'b;
            type MultiPointType<'b>
                = MultiPoint<'b>
            where
                Self: 'b;
            type MultiLineStringType<'b>
                = MultiLineString<'b>
            where
                Self: 'b;
            type MultiPolygonType<'b>
                = MultiPolygon<'b>
            where
                Self: 'b;
            type GeometryCollectionType<'b>
                = GeometryCollection<'b>
            where
                Self: 'b;
            type RectType<'b>
                = geo_traits::UnimplementedRect<f64>
            where
                Self: 'b;
            type LineType<'b>
                = geo_traits::UnimplementedLine<f64>
            where
                Self: 'b;
            type TriangleType<'b>
                = geo_traits::UnimplementedTriangle<f64>
            where
                Self: 'b;

            fn dim(&self) -> geo_traits::Dimensions {
                self.dimension().into()
            }

            fn as_type(
                &self,
            ) -> geo_traits::GeometryType<
                '_,
                Point,
                LineString,
                Polygon,
                MultiPoint,
                MultiLineString,
                MultiPolygon,
                GeometryCollection,
                Self::RectType<'_>,
                Self::TriangleType<'_>,
                Self::LineType<'_>,
            > {
                geo_traits::GeometryType::$geometry_type(self)
            }
        }
    };
}

impl_specialization!(Point);
impl_specialization!(LineString);
impl_specialization!(Polygon);
impl_specialization!(MultiPoint);
impl_specialization!(MultiLineString);
impl_specialization!(MultiPolygon);
impl_specialization!(GeometryCollection);
