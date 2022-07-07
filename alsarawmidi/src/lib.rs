// SPDX-License-Identifier: MIT

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

use glib::{object::IsA, translate::*};
