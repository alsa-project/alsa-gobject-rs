// SPDX-License-Identifier: MIT
mod auto;
mod card;
mod elem_info;
mod elem_value;

pub use crate::{auto::*, card::*, elem_info::*, elem_value::*};
pub use ffi;

use glib::{translate::*, IsA};
