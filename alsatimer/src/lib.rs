// SPDX-License-Identifier: MIT
#[macro_use]
extern crate glib;
#[macro_use]
extern crate bitflags;
extern crate alsatimer_sys;
extern crate glib_sys;
extern crate gobject_sys;
extern crate libc;

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

use glib::{object::IsA, translate::*};
