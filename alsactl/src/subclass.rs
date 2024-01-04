// SPDX-License-Identifier: MIT

mod card;

/// For convenience to provide traits and their blanket implementations to write subclass.
pub mod prelude {
    pub use super::card::*;
}

use {
    super::*,
    glib::{subclass::prelude::*, translate::*, Class},
};
