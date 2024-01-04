// SPDX-License-Identifier: MIT
use super::*;

/// Get the list of existent timer device.
///
/// The call of function executes `open(2)`, `close(2)`, and `ioctl(2)` system call with
/// `SNDRV_TIMER_IOCTL_NEXT_DEVICE` command for ALSA timer character device.
///
/// # Returns
///
/// [`true`] when the overall operation finishes successfully, else [`false`].
///
/// ## `entries`
/// The array with entries of [`DeviceId`][crate::DeviceId].
#[doc(alias = "alsatimer_get_device_id_list")]
pub fn device_id_list() -> Result<Vec<DeviceId>, glib::Error> {
    unsafe {
        let mut entries = std::ptr::null_mut();
        let mut error = std::ptr::null_mut();

        ffi::alsatimer_get_device_id_list(&mut entries, &mut error);

        if error.is_null() {
            Ok(FromGlibPtrContainer::from_glib_full(entries))
        } else {
            Err(from_glib_full(error))
        }
    }
}

/// Set the given parameters to the timer indicated by the identifier.
///
/// The call of function executes `open(2)`, `close(2)`, and `ioctl(2)` system call with
/// `SNDRV_TIMER_IOCTL_GPARAMS` command for ALSA timer character device.
/// ## `device_id`
/// A [`DeviceId`][crate::DeviceId] to identify the timer device.
/// ## `device_params`
/// The parameters of timer device.
///
/// # Returns
///
/// [`true`] when the overall operation finishes successfully, else [`false`].
#[doc(alias = "alsatimer_set_device_params")]
pub fn set_device_params(
    device_id: &mut DeviceId,
    device_params: &DeviceParams,
) -> Result<(), glib::Error> {
    unsafe {
        let mut error = std::ptr::null_mut();

        ffi::alsatimer_set_device_params(
            device_id.to_glib_none_mut().0,
            device_params.to_glib_none().0,
            &mut error,
        );

        if error.is_null() {
            Ok(())
        } else {
            Err(from_glib_full(error))
        }
    }
}

/// Get the status of timer device.
///
/// The call of function executes `open(2)`, `close(2)`, and `ioctl(2)` system call with
/// `SNDRV_TIMER_IOCTL_GSTATUS` command for ALSA timer character device.
/// ## `device_id`
/// A [`DeviceId`][crate::DeviceId] to identify the timer device.
/// ## `device_status`
/// The status of timer device.
///
/// # Returns
///
/// [`true`] when the overall operation finishes successfully, else [`false`].
#[doc(alias = "alsatimer_get_device_status")]
pub fn device_status<P: IsA<DeviceStatus>>(
    device_id: &mut DeviceId,
    status: &mut P,
) -> Result<(), glib::Error> {
    unsafe {
        let mut error = std::ptr::null_mut();

        ffi::alsatimer_get_device_status(
            device_id.to_glib_none_mut().0,
            &mut status.as_ref().to_glib_none().0,
            &mut error,
        );

        if error.is_null() {
            Ok(())
        } else {
            Err(from_glib_full(error))
        }
    }
}
