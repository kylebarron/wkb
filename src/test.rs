use geo_traits::to_geo::ToGeoGeometry;
use geo_types::{
    line_string, point, polygon, Geometry, GeometryCollection, LineString, MultiLineString,
    MultiPoint, MultiPolygon, Point, Polygon,
};

use crate::reader::read_wkb;
use crate::writer::{
    write_geometry_collection, write_line_string, write_multi_line_string, write_multi_point,
    write_multi_polygon, write_point, write_polygon,
};
use crate::Endianness;

fn point_2d() -> Point {
    point!(
        x: 0., y: 1.
    )
}

fn linestring_2d() -> LineString {
    line_string![
        (x: 0., y: 1.),
        (x: 1., y: 2.)
    ]
}

fn polygon_2d() -> Polygon {
    polygon![
        (x: -111., y: 45.),
        (x: -111., y: 41.),
        (x: -104., y: 41.),
        (x: -104., y: 45.),
    ]
}

fn polygon_2d_with_interior() -> Polygon {
    polygon!(
        exterior: [
            (x: -111., y: 45.),
            (x: -111., y: 41.),
            (x: -104., y: 41.),
            (x: -104., y: 45.),
        ],
        interiors: [
            [
                (x: -110., y: 44.),
                (x: -110., y: 42.),
                (x: -105., y: 42.),
                (x: -105., y: 44.),
            ],
        ],
    )
}

fn multi_point_2d() -> MultiPoint {
    MultiPoint::new(vec![
        point!(
            x: 0., y: 1.
        ),
        point!(
            x: 1., y: 2.
        ),
    ])
}

fn multi_line_string_2d() -> MultiLineString {
    MultiLineString::new(vec![
        line_string![
            (x: -111., y: 45.),
            (x: -111., y: 41.),
            (x: -104., y: 41.),
            (x: -104., y: 45.),
        ],
        line_string![
            (x: -110., y: 44.),
            (x: -110., y: 42.),
            (x: -105., y: 42.),
            (x: -105., y: 44.),
        ],
    ])
}

fn multi_polygon_2d() -> MultiPolygon {
    MultiPolygon::new(vec![
        polygon![
            (x: -111., y: 45.),
            (x: -111., y: 41.),
            (x: -104., y: 41.),
            (x: -104., y: 45.),
        ],
        polygon!(
            exterior: [
                (x: -111., y: 45.),
                (x: -111., y: 41.),
                (x: -104., y: 41.),
                (x: -104., y: 45.),
            ],
            interiors: [
                [
                    (x: -110., y: 44.),
                    (x: -110., y: 42.),
                    (x: -105., y: 42.),
                    (x: -105., y: 44.),
                ],
            ],
        ),
    ])
}

#[test]
fn round_trip_point() {
    let orig = point_2d();
    let mut buf = Vec::new();
    write_point(&mut buf, &orig, Endianness::LittleEndian).unwrap();
    let retour = read_wkb(&buf).unwrap();
    assert_eq!(Geometry::Point(orig), retour.to_geometry());

    // Big endian
    let mut buf = Vec::new();
    write_point(&mut buf, &orig, Endianness::BigEndian).unwrap();
    let retour = read_wkb(&buf).unwrap();
    assert_eq!(Geometry::Point(orig), retour.to_geometry());
}

#[test]
fn round_trip_line_string() {
    let orig = linestring_2d();

    let mut buf = Vec::new();
    write_line_string(&mut buf, &orig, Endianness::LittleEndian).unwrap();
    let retour = read_wkb(&buf).unwrap();
    assert_eq!(Geometry::LineString(orig.clone()), retour.to_geometry());

    // Big endian
    let mut buf = Vec::new();
    write_line_string(&mut buf, &orig, Endianness::BigEndian).unwrap();
    let retour = read_wkb(&buf).unwrap();
    assert_eq!(Geometry::LineString(orig), retour.to_geometry());
}

#[test]
fn round_trip_polygon() {
    let orig = polygon_2d();

    let mut buf = Vec::new();
    write_polygon(&mut buf, &orig, Endianness::LittleEndian).unwrap();
    let retour = read_wkb(&buf).unwrap();
    assert_eq!(Geometry::Polygon(orig.clone()), retour.to_geometry());

    // Big endian
    let mut buf = Vec::new();
    write_polygon(&mut buf, &orig, Endianness::BigEndian).unwrap();
    let retour = read_wkb(&buf).unwrap();
    assert_eq!(Geometry::Polygon(orig), retour.to_geometry());
}

#[test]
fn round_trip_polygon_with_interior() {
    let orig = polygon_2d_with_interior();

    let mut buf = Vec::new();
    write_polygon(&mut buf, &orig, Endianness::LittleEndian).unwrap();
    let retour = read_wkb(&buf).unwrap();
    assert_eq!(Geometry::Polygon(orig.clone()), retour.to_geometry());

    // Big endian
    let mut buf = Vec::new();
    write_polygon(&mut buf, &orig, Endianness::BigEndian).unwrap();
    let retour = read_wkb(&buf).unwrap();
    assert_eq!(Geometry::Polygon(orig), retour.to_geometry());
}

#[test]
fn round_trip_multi_point() {
    let orig = multi_point_2d();

    let mut buf = Vec::new();
    write_multi_point(&mut buf, &orig, Endianness::LittleEndian).unwrap();
    let retour = read_wkb(&buf).unwrap();
    assert_eq!(Geometry::MultiPoint(orig.clone()), retour.to_geometry());

    // Big endian
    let mut buf = Vec::new();
    write_multi_point(&mut buf, &orig, Endianness::BigEndian).unwrap();
    let retour = read_wkb(&buf).unwrap();
    assert_eq!(Geometry::MultiPoint(orig), retour.to_geometry());
}

#[test]
fn round_trip_multi_line_string() {
    let orig = multi_line_string_2d();

    let mut buf = Vec::new();
    write_multi_line_string(&mut buf, &orig, Endianness::LittleEndian).unwrap();
    let retour = read_wkb(&buf).unwrap();
    assert_eq!(
        Geometry::MultiLineString(orig.clone()),
        retour.to_geometry()
    );

    // Big endian
    let mut buf = Vec::new();
    write_multi_line_string(&mut buf, &orig, Endianness::BigEndian).unwrap();
    let retour = read_wkb(&buf).unwrap();
    assert_eq!(Geometry::MultiLineString(orig), retour.to_geometry());
}

#[test]
fn round_trip_multi_polygon() {
    let orig = multi_polygon_2d();

    let mut buf = Vec::new();
    write_multi_polygon(&mut buf, &orig, Endianness::LittleEndian).unwrap();
    let retour = read_wkb(&buf).unwrap();
    assert_eq!(Geometry::MultiPolygon(orig.clone()), retour.to_geometry());

    // Big endian
    let mut buf = Vec::new();
    write_multi_polygon(&mut buf, &orig, Endianness::BigEndian).unwrap();
    let retour = read_wkb(&buf).unwrap();
    assert_eq!(Geometry::MultiPolygon(orig), retour.to_geometry());
}

#[test]
fn round_trip_geometry_collection() {
    let orig = GeometryCollection::new_from(vec![
        Geometry::Point(point_2d()),
        Geometry::LineString(linestring_2d()),
        Geometry::Polygon(polygon_2d()),
        Geometry::Polygon(polygon_2d_with_interior()),
        Geometry::MultiPoint(multi_point_2d()),
        Geometry::MultiLineString(multi_line_string_2d()),
        Geometry::MultiPolygon(multi_polygon_2d()),
    ]);

    let mut buf = Vec::new();
    write_geometry_collection(&mut buf, &orig, Endianness::LittleEndian).unwrap();
    let retour = read_wkb(&buf).unwrap();
    assert_eq!(
        Geometry::GeometryCollection(orig.clone()),
        retour.to_geometry()
    );

    // Big endian
    let mut buf = Vec::new();
    write_geometry_collection(&mut buf, &orig, Endianness::BigEndian).unwrap();
    let retour = read_wkb(&buf).unwrap();
    assert_eq!(Geometry::GeometryCollection(orig), retour.to_geometry());
}
