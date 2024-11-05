use byteorder::ByteOrder;

use crate::common::WKBType;
use crate::error::WKBResult;
use crate::reader::{
    GeometryCollection, LineString, MultiLineString, MultiPoint, MultiPolygon, Point, Polygon,
};
use geo_traits::{
    Dimensions, GeometryTrait, UnimplementedLine, UnimplementedRect, UnimplementedTriangle,
};

#[derive(Debug, Clone)]
pub enum Wkb<'a, B: ByteOrder> {
    Point(Point<'a, B>),
    LineString(LineString<'a, B>),
    Polygon(Polygon<'a, B>),
    MultiPoint(MultiPoint<'a, B>),
    MultiLineString(MultiLineString<'a, B>),
    MultiPolygon(MultiPolygon<'a, B>),
    GeometryCollection(GeometryCollection<'a, B>),
}

impl<'a, B: ByteOrder> Wkb<'a, B> {
    pub fn try_new(buf: &'a [u8]) -> WKBResult<Self> {
        let wkb_type = WKBType::from_buffer(buf)?;

        use Dimensions::*;

        let out = match wkb_type {
            WKBType::Point => Wkb::Point(Point::new(buf, 0, Xy)),
            WKBType::LineString => Wkb::LineString(LineString::new(buf, 0, Xy)),
            WKBType::Polygon => Wkb::Polygon(Polygon::new(buf, 0, Xy)),
            WKBType::MultiPoint => Wkb::MultiPoint(MultiPoint::new(buf, Xy)),
            WKBType::MultiLineString => Wkb::MultiLineString(MultiLineString::new(buf, Xy)),
            WKBType::MultiPolygon => Wkb::MultiPolygon(MultiPolygon::new(buf, Xy)),
            WKBType::GeometryCollection => {
                Wkb::GeometryCollection(GeometryCollection::try_new(buf, Xy)?)
            }
            WKBType::PointZ => Wkb::Point(Point::new(buf, 0, Xyz)),
            WKBType::LineStringZ => Wkb::LineString(LineString::new(buf, 0, Xyz)),
            WKBType::PolygonZ => Wkb::Polygon(Polygon::new(buf, 0, Xyz)),
            WKBType::MultiPointZ => Wkb::MultiPoint(MultiPoint::new(buf, Xyz)),
            WKBType::MultiLineStringZ => Wkb::MultiLineString(MultiLineString::new(buf, Xyz)),
            WKBType::MultiPolygonZ => Wkb::MultiPolygon(MultiPolygon::new(buf, Xyz)),
            WKBType::GeometryCollectionZ => {
                Wkb::GeometryCollection(GeometryCollection::try_new(buf, Xyz)?)
            }
        };
        Ok(out)
    }

    pub fn dimension(&self) -> Dimensions {
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

impl<'a, B: ByteOrder> GeometryTrait for Wkb<'a, B> {
    type T = f64;
    type PointType<'b> = Point<'a, B> where Self: 'b;
    type LineStringType<'b> = LineString<'a, B> where Self: 'b;
    type PolygonType<'b> = Polygon<'a, B> where Self: 'b;
    type MultiPointType<'b> = MultiPoint<'a, B> where Self: 'b;
    type MultiLineStringType<'b> = MultiLineString<'a, B> where Self: 'b;
    type MultiPolygonType<'b> = MultiPolygon<'a, B> where Self: 'b;
    type GeometryCollectionType<'b> = GeometryCollection<'a, B> where Self: 'b;
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
        Point<'a, B>,
        LineString<'a, B>,
        Polygon<'a, B>,
        MultiPoint<'a, B>,
        MultiLineString<'a, B>,
        MultiPolygon<'a, B>,
        GeometryCollection<'a, B>,
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

impl<'a, B: ByteOrder> GeometryTrait for &'a Wkb<'a, B> {
    type T = f64;
    type PointType<'b> = Point<'a, B> where Self: 'b;
    type LineStringType<'b> = LineString<'a, B> where Self: 'b;
    type PolygonType<'b> = Polygon<'a, B> where Self: 'b;
    type MultiPointType<'b> = MultiPoint<'a, B> where Self: 'b;
    type MultiLineStringType<'b> = MultiLineString<'a, B> where Self: 'b;
    type MultiPolygonType<'b> = MultiPolygon<'a, B> where Self: 'b;
    type GeometryCollectionType<'b> = GeometryCollection<'a, B> where Self: 'b;
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
        Point<'a, B>,
        LineString<'a, B>,
        Polygon<'a, B>,
        MultiPoint<'a, B>,
        MultiLineString<'a, B>,
        MultiPolygon<'a, B>,
        GeometryCollection<'a, B>,
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
        impl<B: ByteOrder> GeometryTrait for $geometry_type<'_, B> {
            type T = f64;
            type PointType<'b> = Point<'b, B> where Self: 'b;
            type LineStringType<'b> = LineString<'b, B> where Self: 'b;
            type PolygonType<'b> = Polygon<'b, B> where Self: 'b;
            type MultiPointType<'b> = MultiPoint<'b, B> where Self: 'b;
            type MultiLineStringType<'b> = MultiLineString<'b, B> where Self: 'b;
            type MultiPolygonType<'b> = MultiPolygon<'b, B> where Self: 'b;
            type GeometryCollectionType<'b> = GeometryCollection<'b, B> where Self: 'b;
            type RectType<'b> = geo_traits::UnimplementedRect<f64> where Self: 'b;
            type LineType<'b> = geo_traits::UnimplementedLine<f64> where Self: 'b;
            type TriangleType<'b> = geo_traits::UnimplementedTriangle<f64> where Self: 'b;

            fn dim(&self) -> geo_traits::Dimensions {
                self.dimension()
            }

            fn as_type(
                &self,
            ) -> geo_traits::GeometryType<
                '_,
                Point<B>,
                LineString<B>,
                Polygon<B>,
                MultiPoint<B>,
                MultiLineString<B>,
                MultiPolygon<B>,
                GeometryCollection<B>,
                Self::RectType<'_>,
                Self::TriangleType<'_>,
                Self::LineType<'_>,
            > {
                geo_traits::GeometryType::$geometry_type(self)
            }
        }

        impl<'a, B: ByteOrder> GeometryTrait for &'a $geometry_type<'_, B> {
            type T = f64;
            type PointType<'b> = Point<'b, B> where Self: 'b;
            type LineStringType<'b> = LineString<'b, B> where Self: 'b;
            type PolygonType<'b> = Polygon<'b, B> where Self: 'b;
            type MultiPointType<'b> = MultiPoint<'b, B> where Self: 'b;
            type MultiLineStringType<'b> = MultiLineString<'b, B> where Self: 'b;
            type MultiPolygonType<'b> = MultiPolygon<'b, B> where Self: 'b;
            type GeometryCollectionType<'b> = GeometryCollection<'b, B> where Self: 'b;
            type RectType<'b> = geo_traits::UnimplementedRect<f64> where Self: 'b;
            type LineType<'b> = geo_traits::UnimplementedLine<f64> where Self: 'b;
            type TriangleType<'b> = geo_traits::UnimplementedTriangle<f64> where Self: 'b;

            fn dim(&self) -> geo_traits::Dimensions {
                self.dimension()
            }

            fn as_type(
                &self,
            ) -> geo_traits::GeometryType<
                '_,
                Point<B>,
                LineString<B>,
                Polygon<B>,
                MultiPoint<B>,
                MultiLineString<B>,
                MultiPolygon<B>,
                GeometryCollection<B>,
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
