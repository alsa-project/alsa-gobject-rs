#[macro_use]
extern crate glib;
#[macro_use]
extern crate bitflags;
extern crate alsarawmidi_sys;
extern crate glib_sys;
extern crate gobject_sys;
extern crate libc;

mod auto;
pub use auto::*;

mod stream_pair;
pub use stream_pair::*;
