// SPDX-License-Identifier: MIT
#[macro_use]
extern crate glib;
#[macro_use]
extern crate bitflags;
extern crate alsarawmidi_sys;
extern crate glib_sys;
extern crate gobject_sys;
extern crate libc;

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

use glib::{object::IsA, translate::*};
