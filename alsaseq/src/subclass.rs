// SPDX-License-Identifier: MIT

/// For subclass of [`UserClient`][crate::UserClient].
mod user_client;

/// For convenience to provide traits and their blanket implementations to write subclass.
pub mod prelude {
    pub use super::user_client::*;
}

use {
    super::*,
    glib::{subclass::prelude::*, translate::*, Class},
};
