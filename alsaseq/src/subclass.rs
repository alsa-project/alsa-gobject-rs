// SPDX-License-Identifier: MIT

pub mod user_client;

pub mod prelude {
    pub use super::user_client::UserClientImpl;
}

use {
    super::*,
    glib::{subclass::prelude::*, translate::*, ObjectClass},
};
