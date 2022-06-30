// SPDX-License-Identifier: MIT

pub mod card;

pub mod prelude {
    pub use super::card::CardImpl;
}

use {
    super::*,
    glib::{subclass::prelude::*, translate::*, ObjectClass},
};
