// SPDX-License-Identifier: MIT

/// For subclass of [`DeviceCommon`][crate::DeviceCommon].
mod device_common;

/// For convenience to provide traits and their blanket implementations to write subclass.
pub mod prelude {
    pub use super::device_common::*;
}

use {
    super::*,
    glib::{subclass::prelude::*, translate::*, Cast, Error, Interface, Source},
    libc::*,
};
