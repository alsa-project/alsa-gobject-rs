// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use alsactl_sys;
use glib;
use glib::translate::*;
use std::mem;
use std::ptr;

use glib::GString;

pub fn get_card_id_list() -> Result<Vec<u32>, glib::Error> {
    unsafe {
        let mut entries = ptr::null_mut();
        let mut entry_count = mem::MaybeUninit::uninit();
        let mut error = ptr::null_mut();
        let _ = alsactl_sys::alsactl_get_card_id_list(&mut entries, entry_count.as_mut_ptr(), &mut error);
        if error.is_null() { Ok(FromGlibContainer::from_glib_full_num(entries, entry_count.assume_init() as usize)) } else { Err(from_glib_full(error)) }
    }
}

pub fn get_card_sysname(card_id: u32) -> Result<GString, glib::Error> {
    unsafe {
        let mut sysname = ptr::null_mut();
        let mut error = ptr::null_mut();
        let _ = alsactl_sys::alsactl_get_card_sysname(card_id, &mut sysname, &mut error);
        if error.is_null() { Ok(from_glib_full(sysname)) } else { Err(from_glib_full(error)) }
    }
}

pub fn get_control_devnode(card_id: u32) -> Result<GString, glib::Error> {
    unsafe {
        let mut devnode = ptr::null_mut();
        let mut error = ptr::null_mut();
        let _ = alsactl_sys::alsactl_get_control_devnode(card_id, &mut devnode, &mut error);
        if error.is_null() { Ok(from_glib_full(devnode)) } else { Err(from_glib_full(error)) }
    }
}

pub fn get_control_sysname(card_id: u32) -> Result<GString, glib::Error> {
    unsafe {
        let mut sysname = ptr::null_mut();
        let mut error = ptr::null_mut();
        let _ = alsactl_sys::alsactl_get_control_sysname(card_id, &mut sysname, &mut error);
        if error.is_null() { Ok(from_glib_full(sysname)) } else { Err(from_glib_full(error)) }
    }
}

//pub fn sigs_marshal_VOID__BOXED_FLAGS(closure: /*Ignored*/&glib::Closure, return_value: /*Ignored*/&mut glib::Value, n_param_values: u32, param_values: /*Ignored*/&glib::Value, invocation_hint: /*Unimplemented*/Option<Fundamental: Pointer>, marshal_data: /*Unimplemented*/Option<Fundamental: Pointer>) {
//    unsafe { TODO: call alsactl_sys:alsactl_sigs_marshal_VOID__BOXED_FLAGS() }
//}