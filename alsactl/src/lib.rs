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
mod elem_info;
mod elem_value;

pub use crate::{auto::*, card::*, elem_info::*, elem_value::*};

use glib::{object::IsA, translate::*};
