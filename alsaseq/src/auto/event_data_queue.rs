// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ffi;
use glib::translate::*;

glib::wrapper! {
    /// A boxed object to express data of queue event.
    ///
    /// A [`EventDataQueue`][crate::EventDataQueue] is a boxed object to express data of queue event. The instance of
    /// object is one of data properties in event.
    ///
    /// The object wraps `struct snd_seq_ev_queue_control` in UAPI of Linux sound subsystem.
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct EventDataQueue(Boxed<ffi::ALSASeqEventDataQueue>);

    match fn {
        copy => |ptr| glib::gobject_ffi::g_boxed_copy(ffi::alsaseq_event_data_queue_get_type(), ptr as *mut _) as *mut ffi::ALSASeqEventDataQueue,
        free => |ptr| glib::gobject_ffi::g_boxed_free(ffi::alsaseq_event_data_queue_get_type(), ptr as *mut _),
        type_ => || ffi::alsaseq_event_data_queue_get_type(),
    }
}

impl EventDataQueue {
    /// Get the position as param of the queue event.
    ///
    /// # Returns
    ///
    ///
    /// ## `position`
    /// The position as param of the queue event.
    #[doc(alias = "alsaseq_event_data_queue_get_position_param")]
    #[doc(alias = "get_position_param")]
    pub fn position_param(&self) -> u32 {
        unsafe {
            let mut position = std::mem::MaybeUninit::uninit();
            ffi::alsaseq_event_data_queue_get_position_param(
                self.to_glib_none().0,
                position.as_mut_ptr(),
            );
            position.assume_init()
        }
    }

    /// Get the numeric identifier of queue for the event.
    ///
    /// # Returns
    ///
    ///
    /// ## `queue_id`
    /// the numeric identifier of queue for the event.
    #[doc(alias = "alsaseq_event_data_queue_get_queue_id")]
    #[doc(alias = "get_queue_id")]
    pub fn queue_id(&self) -> u8 {
        unsafe {
            let mut queue_id = std::mem::MaybeUninit::uninit();
            ffi::alsaseq_event_data_queue_get_queue_id(
                self.to_glib_none().0,
                queue_id.as_mut_ptr(),
            );
            queue_id.assume_init()
        }
    }

    /// Get the tick time as param of the queue event.
    ///
    /// # Returns
    ///
    ///
    /// ## `tick_time`
    /// The tick time as param of the queue event.
    #[doc(alias = "alsaseq_event_data_queue_get_tick_time_param")]
    #[doc(alias = "get_tick_time_param")]
    pub fn tick_time_param(&self) -> u32 {
        unsafe {
            let mut tick_time = std::mem::MaybeUninit::uninit();
            ffi::alsaseq_event_data_queue_get_tick_time_param(
                self.to_glib_none().0,
                tick_time.as_mut_ptr(),
            );
            tick_time.assume_init()
        }
    }

    /// Get the value as param of the queue event.
    ///
    /// # Returns
    ///
    ///
    /// ## `value`
    /// The value as param of the queue event.
    #[doc(alias = "alsaseq_event_data_queue_get_value_param")]
    #[doc(alias = "get_value_param")]
    pub fn value_param(&self) -> i32 {
        unsafe {
            let mut value = std::mem::MaybeUninit::uninit();
            ffi::alsaseq_event_data_queue_get_value_param(
                self.to_glib_none().0,
                value.as_mut_ptr(),
            );
            value.assume_init()
        }
    }

    /// Set the position as param of the queue event.
    /// ## `position`
    /// the position as param of the queue event.
    #[doc(alias = "alsaseq_event_data_queue_set_position_param")]
    pub fn set_position_param(&mut self, position: u32) {
        unsafe {
            ffi::alsaseq_event_data_queue_set_position_param(self.to_glib_none_mut().0, position);
        }
    }

    /// Se the numeric identifier of queue for the event.
    /// ## `queue_id`
    /// The numeric identifier of queue for the event.
    #[doc(alias = "alsaseq_event_data_queue_set_queue_id")]
    pub fn set_queue_id(&mut self, queue_id: u8) {
        unsafe {
            ffi::alsaseq_event_data_queue_set_queue_id(self.to_glib_none_mut().0, queue_id);
        }
    }

    /// Set the tick time as param of the queue event.
    /// ## `tick_time`
    /// The tick time as param of the queue event.
    #[doc(alias = "alsaseq_event_data_queue_set_tick_time_param")]
    pub fn set_tick_time_param(&mut self, tick_time: u32) {
        unsafe {
            ffi::alsaseq_event_data_queue_set_tick_time_param(self.to_glib_none_mut().0, tick_time);
        }
    }

    /// Set the value as param of the queue event.
    /// ## `value`
    /// The value as param of the queue event.
    #[doc(alias = "alsaseq_event_data_queue_set_value_param")]
    pub fn set_value_param(&mut self, value: i32) {
        unsafe {
            ffi::alsaseq_event_data_queue_set_value_param(self.to_glib_none_mut().0, value);
        }
    }
}

unsafe impl Send for EventDataQueue {}
