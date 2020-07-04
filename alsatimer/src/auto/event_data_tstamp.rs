// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use alsatimer_sys;
use glib::translate::*;
use gobject_sys;
use std::mem;
use EventType;

glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct EventDataTstamp(Boxed<alsatimer_sys::ALSATimerEventDataTstamp>);

    match fn {
        copy => |ptr| gobject_sys::g_boxed_copy(alsatimer_sys::alsatimer_event_data_tstamp_get_type(), ptr as *mut _) as *mut alsatimer_sys::ALSATimerEventDataTstamp,
        free => |ptr| gobject_sys::g_boxed_free(alsatimer_sys::alsatimer_event_data_tstamp_get_type(), ptr as *mut _),
        get_type => || alsatimer_sys::alsatimer_event_data_tstamp_get_type(),
    }
}

impl EventDataTstamp {
    pub fn get_event(&self) -> EventType {
        unsafe {
            let mut event = mem::MaybeUninit::uninit();
            alsatimer_sys::alsatimer_event_data_tstamp_get_event(self.to_glib_none().0, event.as_mut_ptr());
            let event = event.assume_init();
            from_glib(event)
        }
    }

    pub fn get_val(&self) -> u32 {
        unsafe {
            let mut val = mem::MaybeUninit::uninit();
            alsatimer_sys::alsatimer_event_data_tstamp_get_val(self.to_glib_none().0, val.as_mut_ptr());
            let val = val.assume_init();
            val
        }
    }
}

unsafe impl Send for EventDataTstamp {}
