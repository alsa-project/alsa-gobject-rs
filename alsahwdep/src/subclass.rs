// SPDX-License-Identifier: MIT

mod device_common;

pub mod prelude {
    pub use super::device_common::*;
}

use {
    super::*,
    glib::{subclass::prelude::*, translate::*, Cast, Error, Interface, Source},
    libc::*,
};
