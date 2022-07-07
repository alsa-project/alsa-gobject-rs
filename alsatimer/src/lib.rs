// SPDX-License-Identifier: MIT

//! Rust bindings for alsatimer library
//!
//! Rust bindings and wrappers for alsatimer library in
//! [alsa-gobject](https://github.com/alsa-project/alsa-gobject) to operate ALSA Timer character
//! device.
//!
//! The alsatimer library in alsa-gobject v0.3.0 is the minimum supported version for underlying
//! library.
//!
//! The crate depends on [glib crate v0.15](https://crates.io/crates/glib) provided by
//! [gtk-rs project](https://gtk-rs.org/) for type/object system, event loop, and dispacher.
//!
//! # License
//!
//! Released under MIT license.

mod auto;
mod functions;
mod instance_params;
mod instance_status;
mod real_time_event;
mod user_instance;

// For convenience to provide structures and functions.
pub use {
    auto::{functions::*, *},
    functions::*,
};

/// For convenience to provide auto-generated/manual traits, and their blanket implementations.
pub mod prelude {
    pub use crate::{
        auto::traits::*, instance_params::*, instance_status::*, real_time_event::*,
        user_instance::*,
    };
}

/// For subclass implementations derived from provided class.
pub mod subclass;

// To access to alsatimer-sys crate for FFI.
pub use ffi;

// For links in documentation.
use glib;

use glib::{object::IsA, translate::*, Cast};
