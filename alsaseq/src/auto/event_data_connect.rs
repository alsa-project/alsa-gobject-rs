// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Addr;
use glib::translate::*;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct EventDataConnect(Boxed<ffi::ALSASeqEventDataConnect>);

    match fn {
        copy => |ptr| glib::gobject_ffi::g_boxed_copy(ffi::alsaseq_event_data_connect_get_type(), ptr as *mut _) as *mut ffi::ALSASeqEventDataConnect,
        free => |ptr| glib::gobject_ffi::g_boxed_free(ffi::alsaseq_event_data_connect_get_type(), ptr as *mut _),
        type_ => || ffi::alsaseq_event_data_connect_get_type(),
    }
}

impl EventDataConnect {
    #[doc(alias = "alsaseq_event_data_connect_set_dst")]
    pub fn set_dst(&mut self, dst: &Addr) {
        unsafe {
            ffi::alsaseq_event_data_connect_set_dst(
                self.to_glib_none_mut().0,
                dst.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "alsaseq_event_data_connect_set_src")]
    pub fn set_src(&mut self, src: &Addr) {
        unsafe {
            ffi::alsaseq_event_data_connect_set_src(
                self.to_glib_none_mut().0,
                src.to_glib_none().0,
            );
        }
    }
}

unsafe impl Send for EventDataConnect {}
