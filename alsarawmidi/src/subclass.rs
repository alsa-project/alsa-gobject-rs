// SPDX-License-Identifier: MIT

pub mod stream_pair;

pub mod prelude {
    pub use super::stream_pair::StreamPairImpl;
}

use {
    super::*,
    glib::{subclass::prelude::*, translate::*, ObjectClass},
};