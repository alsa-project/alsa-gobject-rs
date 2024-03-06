// SPDX-License-Identifier: MIT

mod stream_pair;

/// For convenience to provide traits and their blanket implementations to write subclass.
pub mod prelude {
    pub use super::stream_pair::*;
}

use {
    super::*,
    glib::{prelude::Cast, subclass::prelude::*, translate::*, Class},
};
