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
use glib;

use crate::prelude::*;
use glib::{object::IsA, translate::*, Cast};

/// A set of enumerations for information of element.
pub enum ElemInfo {
    /// For element with IEC 60958 data.
    Iec60958(ElemInfoIec60958),
    /// For element with boolean values.
    Boolean(ElemInfoBoolean),
    /// For element with unsigned 8 bit integer values.
    Bytes(ElemInfoBytes),
    /// For element with signed 32 bit integer values.
    Integer(ElemInfoInteger),
    /// For element with signed 64 bit integer values.
    Integer64(ElemInfoInteger64),
    /// For element with enumerated index values.
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
