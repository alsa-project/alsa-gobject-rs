// SPDX-License-Identifier: MIT

#![doc = include_str!("../README.md")]

mod auto;
mod card;
mod elem_value;

// For convenience to provide structures and functions.
pub use auto::{functions::*, *};

/// For convenience to provide auto-generated/manual traits, and their blanket implementations.
pub mod prelude {
    pub use crate::{auto::traits::*, card::*, elem_value::*};
}

/// For subclass implementations derived from provided class.
pub mod subclass;

// To access to alsactl-sys crate for FFI.
pub use ffi;

// For links in documentation.
pub(crate) use glib;

use crate::prelude::*;
use glib::{object::IsA, prelude::Cast, translate::*};

/// A set of information about control elements.
pub enum ElemInfo {
    /// IEC 60958.
    Iec60958(ElemInfoIec60958),
    /// Array of boolean.
    Boolean(ElemInfoBoolean),
    /// Array of u8.
    Bytes(ElemInfoBytes),
    /// Array of u32.
    Integer(ElemInfoInteger),
    /// Array of u64.
    Integer64(ElemInfoInteger64),
    /// Array of enumerated items.
    Enumerated(ElemInfoEnumerated),
}

impl From<ElemInfoCommon> for ElemInfo {
    fn from(obj: ElemInfoCommon) -> Self {
        match obj.elem_type() {
            ElemType::Iec60958 => ElemInfo::Iec60958(obj.downcast().unwrap()),
            ElemType::Boolean => ElemInfo::Boolean(obj.downcast().unwrap()),
            ElemType::Bytes => ElemInfo::Bytes(obj.downcast().unwrap()),
            ElemType::Integer => ElemInfo::Integer(obj.downcast().unwrap()),
            ElemType::Integer64 => ElemInfo::Integer64(obj.downcast().unwrap()),
            ElemType::Enumerated => ElemInfo::Enumerated(obj.downcast().unwrap()),
            _ => unreachable!(),
        }
    }
}

impl AsRef<ElemInfoCommon> for ElemInfo {
    fn as_ref(&self) -> &ElemInfoCommon {
        match self {
            ElemInfo::Iec60958(info) => info.upcast_ref(),
            ElemInfo::Boolean(info) => info.upcast_ref(),
            ElemInfo::Bytes(info) => info.upcast_ref(),
            ElemInfo::Integer(info) => info.upcast_ref(),
            ElemInfo::Integer64(info) => info.upcast_ref(),
            ElemInfo::Enumerated(info) => info.upcast_ref(),
        }
    }
}
