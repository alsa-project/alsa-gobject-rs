// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::EventType;
use glib::translate::*;
use std::mem;

glib::wrapper! {
    /// A boxed object to express data of result event.
    ///
    /// A [`EventDataResult`][crate::EventDataResult] is a boxed object to express data of result event. The instance of
    /// object is one of data properties in event.
    ///
    /// The object wraps `struct snd_seq_result` in UAPI of Linux sound subsystem.
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct EventDataResult(Boxed<ffi::ALSASeqEventDataResult>);

    match fn {
        copy => |ptr| glib::gobject_ffi::g_boxed_copy(ffi::alsaseq_event_data_result_get_type(), ptr as *mut _) as *mut ffi::ALSASeqEventDataResult,
        free => |ptr| glib::gobject_ffi::g_boxed_free(ffi::alsaseq_event_data_result_get_type(), ptr as *mut _),
        type_ => || ffi::alsaseq_event_data_result_get_type(),
    }
}

impl EventDataResult {
    /// Get the type of event in which the data results.
    ///
    /// # Returns
    ///
    ///
    /// ## `event_type`
    /// The type of event in which the data results.
    #[doc(alias = "alsaseq_event_data_result_get_event")]
    #[doc(alias = "get_event")]
    pub fn event(&self) -> EventType {
        unsafe {
            let mut event_type = mem::MaybeUninit::uninit();
            ffi::alsaseq_event_data_result_get_event(
                self.to_glib_none().0,
                event_type.as_mut_ptr(),
            );
            from_glib(event_type.assume_init())
        }
    }

    /// Get the status of event.
    ///
    /// # Returns
    ///
    ///
    /// ## `result`
    /// the status of the event.
    #[doc(alias = "alsaseq_event_data_result_get_result")]
    #[doc(alias = "get_result")]
    pub fn result(&self) -> i32 {
        unsafe {
            let mut result = mem::MaybeUninit::uninit();
            ffi::alsaseq_event_data_result_get_result(self.to_glib_none().0, result.as_mut_ptr());
            result.assume_init()
        }
    }

    /// Set the type of event in which the data results.
    /// ## `event_type`
    /// A #ALSASeqEventType.
    #[doc(alias = "alsaseq_event_data_result_set_event")]
    pub fn set_event(&mut self, event_type: EventType) {
        unsafe {
            ffi::alsaseq_event_data_result_set_event(
                self.to_glib_none_mut().0,
                event_type.into_glib(),
            );
        }
    }

    /// Set the status of event.
    /// ## `result`
    /// The status of event.
    #[doc(alias = "alsaseq_event_data_result_set_result")]
    pub fn set_result(&mut self, result: i32) {
        unsafe {
            ffi::alsaseq_event_data_result_set_result(self.to_glib_none_mut().0, result);
        }
    }
}

unsafe impl Send for EventDataResult {}
