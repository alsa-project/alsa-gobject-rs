// SPDX-License-Identifier: MIT

mod auto;
mod client_info;
mod enums;
mod event;
mod event_cntr;
mod event_data_connect;
mod event_data_queue;
mod functions;
mod queue_status;
mod queue_tempo;
mod remove_filter;
mod user_client;

// For convenience to provide structures and functions.
pub use {
    auto::{functions::*, *},
    enums::*,
    event::*,
    event_cntr::*,
    event_data_connect::*,
    event_data_queue::*,
    functions::*,
};

/// For convenience to provide auto-generated/manual traits, and their blanket implementations.
pub mod prelude {
    pub use crate::{
        auto::traits::*, client_info::*, queue_status::*, queue_tempo::*, remove_filter::*,
        user_client::*,
    };
}

/// For subclass implementations derived from provided class.
pub mod subclass;

// To access to alsaseq-sys crate for FFI.
pub use ffi;

use glib::{object::IsA, translate::*, Cast, Error, StaticType, Value};

use crate::prelude::*;

pub enum QueueTimer {
    Alsa(QueueTimerAlsa),
}

impl From<QueueTimerCommon> for QueueTimer {
    fn from(obj: QueueTimerCommon) -> Self {
        match obj.timer_type() {
            QueueTimerType::Alsa => QueueTimer::Alsa(obj.downcast().unwrap()),
            _ => unreachable!(),
        }
    }
}

impl AsRef<QueueTimerCommon> for QueueTimer {
    fn as_ref(&self) -> &QueueTimerCommon {
        match self {
            QueueTimer::Alsa(timer) => timer.upcast_ref(),
        }
    }
}
