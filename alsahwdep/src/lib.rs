// SPDX-License-Identifier: MIT

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

// To access to alsahwdep-sys crate for FFI.
pub use ffi;

use glib::{translate::*, IsA};
