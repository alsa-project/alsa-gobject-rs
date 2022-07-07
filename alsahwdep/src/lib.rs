// SPDX-License-Identifier: MIT
#[macro_use]
extern crate glib;
extern crate alsahwdep_sys;
extern crate bitflags;
extern crate glib_sys;
extern crate gobject_sys;
extern crate libc;

mod auto;
mod device_common;
mod enums;

// For convenience to provide structures and functions.
pub use {
    auto::{functions::*, *},
    enums::*,
};

/// For convenience to provide auto-generated/manual traits, and their blanket implementations.
pub mod prelude {
    pub use crate::{auto::traits::*, device_common::*};
}

/// For subclass implementations derived from provided class.
pub mod subclass;

use glib::{translate::*, IsA};
