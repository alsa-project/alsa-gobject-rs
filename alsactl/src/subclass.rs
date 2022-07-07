// SPDX-License-Identifier: MIT

mod card;

pub mod prelude {
    pub use super::card::*;
}

use {
    super::*,
    glib::{subclass::prelude::*, translate::*, ObjectClass},
};
