// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

//! For functions available globally.

use crate::DeviceInfo;
use glib::translate::*;

/// Get the list of numeric ID for available hwdep devices of sound card.
///
/// Nodes under sound subsystem in sysfs are used to gather the information.
/// ## `card_id`
/// The numeric ID of sound card.
///
/// # Returns
///
/// [`true`] when the overall operation finishes successfully, else [`false`].
///
/// ## `entries`
/// The list of numeric ID for hwdep device.
#[doc(alias = "alsahwdep_get_device_id_list")]
#[doc(alias = "get_device_id_list")]
pub fn device_id_list(card_id: u32) -> Result<Vec<u32>, glib::Error> {
    unsafe {
        let mut entries = std::ptr::null_mut();
        let mut entry_count = std::mem::MaybeUninit::uninit();
        let mut error = std::ptr::null_mut();
        let is_ok = ffi::alsahwdep_get_device_id_list(
            card_id,
            &mut entries,
            entry_count.as_mut_ptr(),
            &mut error,
        );
        debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
        if error.is_null() {
            Ok(FromGlibContainer::from_glib_full_num(
                entries,
                entry_count.assume_init() as _,
            ))
        } else {
            Err(from_glib_full(error))
        }
    }
}

/// Get the information according to given numeric IDs for card and device.
///
/// The call of function executes `open(2)`, `close(2)`, and `ioctl(2)` system call
/// with `SNDRV_CTL_IOCTL_HWDEP_INFO` command for ALSA control character device.
/// ## `card_id`
/// The numeric value for sound card to query.
/// ## `device_id`
/// The numeric value of hwdep device to query.
///
/// # Returns
///
/// [`true`] when the overall operation finishes successfully, else [`false`].
///
/// ## `device_info`
/// The information of the device.
#[doc(alias = "alsahwdep_get_device_info")]
#[doc(alias = "get_device_info")]
pub fn device_info(card_id: u32, device_id: u32) -> Result<DeviceInfo, glib::Error> {
    unsafe {
        let mut device_info = std::ptr::null_mut();
        let mut error = std::ptr::null_mut();
        let is_ok =
            ffi::alsahwdep_get_device_info(card_id, device_id, &mut device_info, &mut error);
        debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
        if error.is_null() {
            Ok(from_glib_full(device_info))
        } else {
            Err(from_glib_full(error))
        }
    }
}

/// Allocate devnode string for hwdep device and return it when exists.
///
/// Nodes under sound subsystem in sysfs are used to gather the information.
/// ## `card_id`
/// The numeridcal ID of sound card.
/// ## `device_id`
/// The numeric ID of hwdep device for the sound card.
///
/// # Returns
///
/// [`true`] when the overall operation finishes successfully, else [`false`].
///
/// ## `devnode`
/// The string for devnode of hwdep device.
#[doc(alias = "alsahwdep_get_hwdep_devnode")]
#[doc(alias = "get_hwdep_devnode")]
pub fn hwdep_devnode(card_id: u32, device_id: u32) -> Result<glib::GString, glib::Error> {
    unsafe {
        let mut devnode = std::ptr::null_mut();
        let mut error = std::ptr::null_mut();
        let is_ok = ffi::alsahwdep_get_hwdep_devnode(card_id, device_id, &mut devnode, &mut error);
        debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
        if error.is_null() {
            Ok(from_glib_full(devnode))
        } else {
            Err(from_glib_full(error))
        }
    }
}

/// Allocate sysname for hwdep device and return it when it exists.
///
/// Nodes under sound subsystem in sysfs are used to gather the information.
/// ## `card_id`
/// The numeridcal ID of sound card.
/// ## `device_id`
/// The numeric ID of hwdep device for the sound card.
///
/// # Returns
///
/// [`true`] when the overall operation finishes successfully, else [`false`].
///
/// ## `sysname`
/// The string for sysname of hwdep device.
#[doc(alias = "alsahwdep_get_hwdep_sysname")]
#[doc(alias = "get_hwdep_sysname")]
pub fn hwdep_sysname(card_id: u32, device_id: u32) -> Result<glib::GString, glib::Error> {
    unsafe {
        let mut sysname = std::ptr::null_mut();
        let mut error = std::ptr::null_mut();
        let is_ok = ffi::alsahwdep_get_hwdep_sysname(card_id, device_id, &mut sysname, &mut error);
        debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
        if error.is_null() {
            Ok(from_glib_full(sysname))
        } else {
            Err(from_glib_full(error))
        }
    }
}
