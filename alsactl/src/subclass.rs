// SPDX-License-Identifier: MIT

/// For subclass of [`Card`][crate::Card].
mod card;

/// For convenience to provide traits and their blanket implementations to write subclass.
pub mod prelude {
    pub use super::card::*;
}

use {
    super::*,
    glib::{subclass::prelude::*, translate::*, Class},
};
