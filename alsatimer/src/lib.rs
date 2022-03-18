#[macro_use]
extern crate glib;
#[macro_use]
extern crate bitflags;
extern crate alsatimer_sys;
extern crate glib_sys;
extern crate gobject_sys;
extern crate libc;

mod auto;
mod event;
mod event_data_tstamp;
mod instance_params;
mod instance_status;
mod query;
mod user_instance;

pub use crate::{
    auto::*, event::*, event_data_tstamp::*, instance_params::*, instance_status::*, query::*,
    user_instance::*,
};

use glib::{object::IsA, translate::*};
