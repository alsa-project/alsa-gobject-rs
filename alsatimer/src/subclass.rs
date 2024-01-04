// SPDX-License-Identifier: MIT

mod user_instance;

/// For convenience to provide traits and their blanket implementations to write subclass.
pub mod prelude {
    pub use super::user_instance::*;
}

use {
    super::*,
    glib::{subclass::prelude::*, translate::*, Class},
};
