// SPDX-License-Identifier: MIT
mod auto;
mod stream_pair;

pub use crate::{auto::*, stream_pair::*};
pub use ffi;

use glib::{translate::*, IsA};
