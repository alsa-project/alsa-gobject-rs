// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use alsaseq_sys;
use glib::translate::*;
use gobject_sys;
use std::mem;

glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct EventDataNote(Boxed<alsaseq_sys::ALSASeqEventDataNote>);

    match fn {
        copy => |ptr| gobject_sys::g_boxed_copy(alsaseq_sys::alsaseq_event_data_note_get_type(), ptr as *mut _) as *mut alsaseq_sys::ALSASeqEventDataNote,
        free => |ptr| gobject_sys::g_boxed_free(alsaseq_sys::alsaseq_event_data_note_get_type(), ptr as *mut _),
        get_type => || alsaseq_sys::alsaseq_event_data_note_get_type(),
    }
}

impl EventDataNote {
    pub fn get_channel(&self) -> u8 {
        unsafe {
            let mut channel = mem::MaybeUninit::uninit();
            alsaseq_sys::alsaseq_event_data_note_get_channel(self.to_glib_none().0, channel.as_mut_ptr());
            let channel = channel.assume_init();
            channel
        }
    }

    pub fn get_duration(&self) -> u8 {
        unsafe {
            let mut duration = mem::MaybeUninit::uninit();
            alsaseq_sys::alsaseq_event_data_note_get_duration(self.to_glib_none().0, duration.as_mut_ptr());
            let duration = duration.assume_init();
            duration
        }
    }

    pub fn get_note(&self) -> u8 {
        unsafe {
            let mut note = mem::MaybeUninit::uninit();
            alsaseq_sys::alsaseq_event_data_note_get_note(self.to_glib_none().0, note.as_mut_ptr());
            let note = note.assume_init();
            note
        }
    }

    pub fn get_off_velocity(&self) -> u8 {
        unsafe {
            let mut off_velocity = mem::MaybeUninit::uninit();
            alsaseq_sys::alsaseq_event_data_note_get_off_velocity(self.to_glib_none().0, off_velocity.as_mut_ptr());
            let off_velocity = off_velocity.assume_init();
            off_velocity
        }
    }

    pub fn get_velocity(&self) -> u8 {
        unsafe {
            let mut velocity = mem::MaybeUninit::uninit();
            alsaseq_sys::alsaseq_event_data_note_get_velocity(self.to_glib_none().0, velocity.as_mut_ptr());
            let velocity = velocity.assume_init();
            velocity
        }
    }

    pub fn set_channel(&mut self, channel: u8) {
        unsafe {
            alsaseq_sys::alsaseq_event_data_note_set_channel(self.to_glib_none_mut().0, channel);
        }
    }

    pub fn set_duration(&mut self, duration: u8) {
        unsafe {
            alsaseq_sys::alsaseq_event_data_note_set_duration(self.to_glib_none_mut().0, duration);
        }
    }

    pub fn set_note(&mut self, note: u8) {
        unsafe {
            alsaseq_sys::alsaseq_event_data_note_set_note(self.to_glib_none_mut().0, note);
        }
    }

    pub fn set_off_velocity(&mut self, off_velocity: u8) {
        unsafe {
            alsaseq_sys::alsaseq_event_data_note_set_off_velocity(self.to_glib_none_mut().0, off_velocity);
        }
    }

    pub fn set_velocity(&mut self, velocity: u8) {
        unsafe {
            alsaseq_sys::alsaseq_event_data_note_set_velocity(self.to_glib_none_mut().0, velocity);
        }
    }
}

unsafe impl Send for EventDataNote {}
