// SPDX-License-Identifier: MIT
#[macro_use]
extern crate glib;
#[macro_use]
extern crate bitflags;
extern crate alsaseq_sys;
extern crate alsatimer;
extern crate glib_sys;
extern crate gobject_sys;
extern crate libc;

mod auto;
mod client_info;
mod event_cntr;
mod event_data_connect;
mod event_data_queue;
mod query;
mod queue_status;
mod queue_tempo;
mod queue_timer;
mod queue_timer_data_alsa;
mod tstamp;
mod user_client;

pub use crate::{
    auto::*, client_info::*, event_cntr::*, event_data_connect::*, event_data_queue::*, query::*,
    queue_status::*, queue_tempo::*, queue_timer::*, queue_timer_data_alsa::*, tstamp::*,
    user_client::*,
};

use glib::{object::IsA, translate::*};
