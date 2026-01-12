// SPDX-License-Identifier: MIT

#![doc = include_str!("../README.md")]

mod auto;
mod stream_pair;

// For convenience to provide structures and functions.
pub use auto::{functions::*, *};

/// For convenience to provide auto-generated/manual traits, and their blanket implementations.
pub mod prelude {
    pub use crate::{auto::traits::*, stream_pair::*};
}

/// For subclass implementations derived from provided class.
pub mod subclass;

// For applications to handle stuffs in the sys crate.
pub use alsarawmidi_sys as ffi;

// For links in documentation.
pub(crate) use glib;

use glib::{object::IsA, translate::*};
