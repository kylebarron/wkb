use crate::error::WKBResult;
use crate::writer::{
    geometry_collection_wkb_size, line_string_wkb_size, multi_line_string_wkb_size,
    multi_point_wkb_size, multi_polygon_wkb_size, point_wkb_size, polygon_wkb_size,
    write_geometry_collection, write_line_string, write_multi_line_string, write_multi_point,
    write_multi_polygon, write_point, write_polygon,
};
use geo_traits::{GeometryTrait, GeometryType};
use std::io::Write;

/// The byte length of a Geometry
pub fn geometry_wkb_size(geom: &impl GeometryTrait) -> usize {
    use GeometryType::*;
    match geom.as_type() {
        Point(_) => point_wkb_size(geom.dim()),
        LineString(ls) => line_string_wkb_size(ls),
        Polygon(p) => polygon_wkb_size(p),
        MultiPoint(mp) => multi_point_wkb_size(mp),
        MultiLineString(ml) => multi_line_string_wkb_size(ml),
        MultiPolygon(mp) => multi_polygon_wkb_size(mp),
        GeometryCollection(gc) => geometry_collection_wkb_size(gc),
        Rect(_) => todo!(),
        Triangle(_) => todo!(),
        Line(_) => todo!(),
    }
}

/// Write a Geometry to a Writer encoded as WKB
pub fn write_geometry<W: Write>(writer: W, geom: &impl GeometryTrait<T = f64>) -> WKBResult<()> {
    use GeometryType::*;
    match geom.as_type() {
        Point(p) => write_point(writer, p),
        LineString(ls) => write_line_string(writer, ls),
        Polygon(p) => write_polygon(writer, p),
        MultiPoint(mp) => write_multi_point(writer, mp),
        MultiLineString(ml) => write_multi_line_string(writer, ml),
        MultiPolygon(mp) => write_multi_polygon(writer, mp),
        GeometryCollection(gc) => {
            // todo!()
            // error[E0275]: overflow evaluating the requirement `&mut std::io::Cursor<std::vec::Vec<u8>>: std::io::Write`
            // https://stackoverflow.com/a/31197781/7319250
            write_geometry_collection(writer, gc)
        }
        Rect(_) => todo!(),
        Triangle(_) => todo!(),
        Line(_) => todo!(),
    }
}
