// SPDX-License-Identifier: MIT

//! Rust bindings for alsaseq library
//!
//! Rust bindings and wrappers for alsaseq library in
//! [alsa-gobject](https://github.com/alsa-project/alsa-gobject) to operate ALSA Sequencer
//! character device.
//!
//! The alsaseq library in alsa-gobject v0.3.0 is the minimum supported version for underlying
//! library.
//!
//! The crate depends on [glib crate v0.15](https://crates.io/crates/glib) provided by
//! [gtk-rs project](https://gtk-rs.org/) for type/object system, event loop, and dispacher.
//!
//! # License
//!
//! Released under MIT license.
//!
//! # Sample programs
//!
//! Some programs are available under `examples` directory.
//!
//! `dump-event-data`
//! : demonstration to dump events received at port registered in ALSA Sequencer.

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

// For links in documentation.
use glib;

use glib::{object::IsA, translate::*, Cast, Error, StaticType, Value};

use crate::prelude::*;

/// A set of enumerations for timer backend of queue.
pub enum QueueTimer {
    /// Timer backend is an instance of ALSA Timer.
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
