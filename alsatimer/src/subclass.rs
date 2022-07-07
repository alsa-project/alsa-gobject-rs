// SPDX-License-Identifier: MIT

mod user_instance;

pub mod prelude {
    pub use super::user_instance::UserInstanceImpl;
}

use {
    super::*,
    glib::{subclass::prelude::*, translate::*, ObjectClass},
};
