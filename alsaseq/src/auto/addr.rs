// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use alsaseq_sys;
use glib::translate::*;
use gobject_sys;
use std::mem;

glib_wrapper! {
    #[derive(Debug, PartialOrd, Ord, Hash)]
    pub struct Addr(Boxed<alsaseq_sys::ALSASeqAddr>);

    match fn {
        copy => |ptr| gobject_sys::g_boxed_copy(alsaseq_sys::alsaseq_addr_get_type(), ptr as *mut _) as *mut alsaseq_sys::ALSASeqAddr,
        free => |ptr| gobject_sys::g_boxed_free(alsaseq_sys::alsaseq_addr_get_type(), ptr as *mut _),
        get_type => || alsaseq_sys::alsaseq_addr_get_type(),
    }
}

impl Addr {
    pub fn new(client_id: u8, port_id: u8) -> Addr {
        unsafe {
            from_glib_full(alsaseq_sys::alsaseq_addr_new(client_id, port_id))
        }
    }

    fn equal(&self, target: &Addr) -> bool {
        unsafe {
            from_glib(alsaseq_sys::alsaseq_addr_equal(self.to_glib_none().0, target.to_glib_none().0))
        }
    }

    pub fn get_client_id(&self) -> u8 {
        unsafe {
            let mut client_id = mem::MaybeUninit::uninit();
            alsaseq_sys::alsaseq_addr_get_client_id(self.to_glib_none().0, client_id.as_mut_ptr());
            let client_id = client_id.assume_init();
            client_id
        }
    }

    pub fn get_port_id(&self) -> u8 {
        unsafe {
            let mut port_id = mem::MaybeUninit::uninit();
            alsaseq_sys::alsaseq_addr_get_port_id(self.to_glib_none().0, port_id.as_mut_ptr());
            let port_id = port_id.assume_init();
            port_id
        }
    }
}

impl PartialEq for Addr {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.equal(other)
    }
}

impl Eq for Addr {}