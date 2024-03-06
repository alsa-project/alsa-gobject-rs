// SPDX-License-Identifier: MIT

#![doc = include_str!("../README.md")]

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
    pub use crate::{auto::traits::*, instance_params::*, instance_status::*, user_instance::*};
}

/// For subclass implementations derived from provided class.
pub mod subclass;

// To access to alsatimer-sys crate for FFI.
pub use ffi;

// For links in documentation.
pub(crate) use glib;

use glib::{object::IsA, prelude::Cast, translate::*};
