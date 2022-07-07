// SPDX-License-Identifier: MIT

mod user_client;

pub mod prelude {
    pub use super::user_client::*;
}

use {
    super::*,
    glib::{subclass::prelude::*, translate::*, ObjectClass},
};
