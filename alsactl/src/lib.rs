// SPDX-License-Identifier: MIT
#[macro_use]
extern crate glib;
#[macro_use]
extern crate bitflags;
extern crate alsactl_sys;
extern crate glib_sys;
extern crate gobject_sys;
extern crate libc;

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

use glib::{object::IsA, translate::*, Cast};

pub enum ElemInfo {
    Iec60958(ElemInfoIec60958),
    Boolean(ElemInfoBoolean),
    Bytes(ElemInfoBytes),
    Integer(ElemInfoInteger),
    Integer64(ElemInfoInteger64),
    Enumerated(ElemInfoEnumerated),
}

impl From<ElemInfoCommon> for ElemInfo {
    fn from(obj: ElemInfoCommon) -> Self {
        match obj.get_property_elem_type() {
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
