#![allow(clippy::trivially_copy_pass_by_ref)]
#![deny(missing_copy_implementations, missing_debug_implementations)]

#[macro_use]
mod macros;

pub(in crate) mod de;
pub(in crate) mod error;
pub mod packets;
pub(in crate) mod ser;
pub(in crate) mod util;

pub use crate::packets::PacketId;
pub(in crate) use crate::util::*;

pub use self::{
    de::{from_bytes, Deserializer},
    error::{Error, Result},
    ser::{to_bytes, to_writer, Serializer},
};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default, PartialOrd, PartialEq)]
pub struct Vector2 {
    x: f32,
    y: f32,
}

#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default, PartialOrd, PartialEq)]
pub struct Vector3 {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Deserialize, Serialize, Copy, Clone, Debug, Default, PartialOrd, PartialEq)]
pub struct Vector4 {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}
