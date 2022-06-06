// SPDX-License-Identifier: MIT
use super::*;

glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Event(Boxed<alsatimer_sys::ALSATimerEvent>);

    match fn {
        copy => |ptr| gobject_sys::g_boxed_copy(alsatimer_sys::alsatimer_event_get_type(), ptr as *mut _) as *mut alsatimer_sys::ALSATimerEvent,
        free => |ptr| gobject_sys::g_boxed_free(alsatimer_sys::alsatimer_event_get_type(), ptr as *mut _),
        get_type => || alsatimer_sys::alsatimer_event_get_type(),
    }
}

impl Event {
    pub fn new() -> Event {
        unsafe { from_glib_full(alsatimer_sys::alsatimer_event_new()) }
    }

    pub fn get_tick_data(&mut self) -> EventDataTick {
        unsafe {
            let mut data = std::ptr::null_mut() as *const alsatimer_sys::ALSATimerEventDataTick;
            alsatimer_sys::alsatimer_event_get_tick_data(self.to_glib_none_mut().0, &mut data);
            from_glib_none(data)
        }
    }

    pub fn get_tstamp_data(&mut self) -> EventDataTstamp {
        unsafe {
            let mut data = std::ptr::null_mut() as *const alsatimer_sys::ALSATimerEventDataTstamp;
            alsatimer_sys::alsatimer_event_get_tstamp_data(self.to_glib_none_mut().0, &mut data);
            from_glib_none(data)
        }
    }
}

unsafe impl Send for Event {}
