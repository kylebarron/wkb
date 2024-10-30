# `wkb`

Transitory crate for reading/writing [Well-Known Binary](https://libgeos.org/specifications/wkb/).

This is being refactored out of https://github.com/geoarrow/geoarrow-rs in the hope that it can be a standalone [georust](https://github.com/georust) repository.

Notes:

- Implement `GeometryTrait` for each scalar type.
- Implement M and ZM variants.
