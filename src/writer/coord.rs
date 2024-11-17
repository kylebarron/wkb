use std::io::Write;

use byteorder::{ByteOrder, WriteBytesExt};
use geo_traits::CoordTrait;

use crate::error::WKBResult;

/// Write a coordinate to a Writer encoded as WKB
pub(crate) fn write_coord<W: Write, B: ByteOrder>(
    writer: &mut W,
    coord: &impl CoordTrait<T = f64>,
) -> WKBResult<()> {
    for i in 0..coord.dim().size() {
        writer.write_f64::<B>(coord.nth_unchecked(i))?;
    }

    Ok(())
}
