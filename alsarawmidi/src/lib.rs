// SPDX-License-Identifier: MIT

//! Rust bindings for alsarawmidi library
//!
//! Rust bindings and wrappers for alsarawmidi library in
//! [alsa-gobject](https://github.com/alsa-project/alsa-gobject) to operate ALSA Rawmidi character
//! device.
//!
//! The alsarawmidi library in alsa-gobject v0.3.0 is the minimum supported version for underlying
//! library.
//!
//! The crate depends on [glib crate v0.15](https://crates.io/crates/glib) provided by
//! [gtk-rs project](https://gtk-rs.org/) for type/object system, event loop, and dispacher.
//!
//! # License
//!
//! Released under MIT license.

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

// To access to alsarawmidi-sys crate for FFI.
pub use ffi;

// For links in documentation.
use glib;

use glib::{object::IsA, translate::*};
