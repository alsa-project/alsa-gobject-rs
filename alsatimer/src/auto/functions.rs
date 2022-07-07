// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::DeviceId;
use crate::DeviceInfo;
use glib::translate::*;
use std::mem;
use std::ptr;

#[doc(alias = "alsatimer_get_device_info")]
#[doc(alias = "get_device_info")]
pub fn device_info(device_id: &mut DeviceId) -> Result<DeviceInfo, glib::Error> {
    unsafe {
        let mut device_info = ptr::null_mut();
        let mut error = ptr::null_mut();
        let is_ok = ffi::alsatimer_get_device_info(
            device_id.to_glib_none_mut().0,
            &mut device_info,
            &mut error,
        );
        assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
        if error.is_null() {
            Ok(from_glib_full(device_info))
        } else {
            Err(from_glib_full(error))
        }
    }
}

//#[doc(alias = "alsatimer_get_device_status")]
//#[doc(alias = "get_device_status")]
//pub fn device_status(device_id: &mut DeviceId, device_status: impl IsA<DeviceStatus>) -> Result<(), glib::Error> {
//    unsafe { TODO: call ffi:alsatimer_get_device_status() }
//}

#[doc(alias = "alsatimer_get_devnode")]
#[doc(alias = "get_devnode")]
pub fn devnode() -> Result<glib::GString, glib::Error> {
    unsafe {
        let mut devnode = ptr::null_mut();
        let mut error = ptr::null_mut();
        let is_ok = ffi::alsatimer_get_devnode(&mut devnode, &mut error);
        assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
        if error.is_null() {
            Ok(from_glib_full(devnode))
        } else {
            Err(from_glib_full(error))
        }
    }
}

#[doc(alias = "alsatimer_get_real_time_clock_id")]
#[doc(alias = "get_real_time_clock_id")]
pub fn real_time_clock_id() -> Result<i32, glib::Error> {
    unsafe {
        let mut clock_id = mem::MaybeUninit::uninit();
        let mut error = ptr::null_mut();
        let is_ok = ffi::alsatimer_get_real_time_clock_id(clock_id.as_mut_ptr(), &mut error);
        let clock_id = clock_id.assume_init();
        assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
        if error.is_null() {
            Ok(clock_id)
        } else {
            Err(from_glib_full(error))
        }
    }
}

#[doc(alias = "alsatimer_get_sysname")]
#[doc(alias = "get_sysname")]
pub fn sysname() -> Result<glib::GString, glib::Error> {
    unsafe {
        let mut sysname = ptr::null_mut();
        let mut error = ptr::null_mut();
        let is_ok = ffi::alsatimer_get_sysname(&mut sysname, &mut error);
        assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
        if error.is_null() {
            Ok(from_glib_full(sysname))
        } else {
            Err(from_glib_full(error))
        }
    }
}
