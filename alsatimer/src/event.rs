// SPDX-License-Identifier: MIT
use crate::*;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Event(Boxed<ffi::ALSATimerEvent>);

    match fn {
        copy => |ptr| glib::gobject_ffi::g_boxed_copy(ffi::alsatimer_event_get_type(), ptr as *mut _) as *mut ffi::ALSATimerEvent,
        free => |ptr| glib::gobject_ffi::g_boxed_free(ffi::alsatimer_event_get_type(), ptr as *mut _),
        type_ => || ffi::alsatimer_event_get_type(),
    }
}

impl Event {
    pub fn new() -> Event {
        unsafe { from_glib_full(ffi::alsatimer_event_new()) }
    }

    pub fn get_tick_data(&mut self) -> EventDataTick {
        unsafe {
            let mut data = std::ptr::null_mut() as *const ffi::ALSATimerEventDataTick;
            ffi::alsatimer_event_get_tick_data(self.to_glib_none_mut().0, &mut data);
            from_glib_none(data)
        }
    }

    pub fn get_tstamp_data(&mut self) -> EventDataTstamp {
        unsafe {
            let mut data = std::ptr::null_mut() as *const ffi::ALSATimerEventDataTstamp;
            ffi::alsatimer_event_get_tstamp_data(self.to_glib_none_mut().0, &mut data);
            from_glib_none(data)
        }
    }
}

unsafe impl Send for Event {}
