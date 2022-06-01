// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use alsatimer_sys;
use glib;
use glib::translate::*;
use glib::GString;
use std::mem;
use std::ptr;
use DeviceId;
use DeviceInfo;

pub fn get_device_info(device_id: &mut DeviceId) -> Result<DeviceInfo, glib::Error> {
    unsafe {
        let mut device_info = ptr::null_mut();
        let mut error = ptr::null_mut();
        let _ = alsatimer_sys::alsatimer_get_device_info(
            device_id.to_glib_none_mut().0,
            &mut device_info,
            &mut error,
        );
        if error.is_null() {
            Ok(from_glib_full(device_info))
        } else {
            Err(from_glib_full(error))
        }
    }
}

//pub fn get_device_status<P: IsA<DeviceStatus>>(device_id: &mut DeviceId, device_status: &P) -> Result<(), glib::Error> {
//    unsafe { TODO: call alsatimer_sys:alsatimer_get_device_status() }
//}

pub fn get_devnode() -> Result<GString, glib::Error> {
    unsafe {
        let mut devnode = ptr::null_mut();
        let mut error = ptr::null_mut();
        let _ = alsatimer_sys::alsatimer_get_devnode(&mut devnode, &mut error);
        if error.is_null() {
            Ok(from_glib_full(devnode))
        } else {
            Err(from_glib_full(error))
        }
    }
}

pub fn get_sysname() -> Result<GString, glib::Error> {
    unsafe {
        let mut sysname = ptr::null_mut();
        let mut error = ptr::null_mut();
        let _ = alsatimer_sys::alsatimer_get_sysname(&mut sysname, &mut error);
        if error.is_null() {
            Ok(from_glib_full(sysname))
        } else {
            Err(from_glib_full(error))
        }
    }
}

pub fn get_tstamp_source() -> Result<i32, glib::Error> {
    unsafe {
        let mut clock_id = mem::MaybeUninit::uninit();
        let mut error = ptr::null_mut();
        let _ = alsatimer_sys::alsatimer_get_tstamp_source(clock_id.as_mut_ptr(), &mut error);
        let clock_id = clock_id.assume_init();
        if error.is_null() {
            Ok(clock_id)
        } else {
            Err(from_glib_full(error))
        }
    }
}
