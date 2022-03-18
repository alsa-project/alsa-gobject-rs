// SPDX-License-Identifier: MIT
mod auto;
mod client_info;
mod event_cntr;
mod event_data_connect;
mod event_data_queue;
mod functions;
mod queue_status;
mod queue_tempo;
mod queue_timer;
mod queue_timer_data_alsa;
mod tstamp;
mod user_client;

pub use crate::{
    auto::{functions::*, *},
    client_info::*,
    event_cntr::*,
    event_data_connect::*,
    event_data_queue::*,
    functions::*,
    queue_status::*,
    queue_tempo::*,
    queue_timer::*,
    queue_timer_data_alsa::*,
    tstamp::*,
    user_client::*,
};
pub use ffi;

use glib::{translate::*, IsA};
