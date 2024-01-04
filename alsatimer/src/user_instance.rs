// SPDX-License-Identifier: MIT
use super::*;

/// Trait containing the rest of [`struct@UserInstance`] methods.
///
/// # Implementors
///
/// [`UserInstance`][struct@crate::UserInstance]
pub trait UserInstanceExtManual {
    /// Get the version of timer protocol currently used. The version is expressed as the array with
    /// three elements; major, minor, and micro version in the order. The length of major version is
    /// 16 bit, the length of minor and micro version is 8 bit each.
    ///
    /// # Returns
    ///
    /// [`true`] when the overall operation finishes successfully, else [`false`].
    ///
    /// ## `proto_ver_triplet`
    /// The version of protocol currently
    ///                     used.
    #[doc(alias = "alsatimer_user_instance_get_protocol_version")]
    #[doc(alias = "get_protocol_version")]
    fn protocol_version(&self) -> Result<&[u16; 3], glib::Error>;

    /// Configure the instance with the parameters and return the latest parameters.
    ///
    /// The call of function executes `ioctl(2)` system call with `SNDRV_TIMER_IOCTL_PARAMS` command
    /// for ALSA timer character device.
    /// ## `instance_params`
    /// A [`InstanceParams`][crate::InstanceParams].
    ///
    /// # Returns
    ///
    /// [`true`] when the overall operation finishes successfully, else [`false`].
    #[doc(alias = "alsatimer_user_instance_set_params")]
    fn set_params<P: IsA<InstanceParams>>(&self, params: &mut P) -> Result<(), glib::Error>;

    /// Get the latest status of instance.
    ///
    /// The call of function executes `ioctl(2)` system call with `SNDRV_TIMER_IOCTL_STATUS` command
    /// for ALSA timer character device.
    /// ## `instance_status`
    /// A [`InstanceStatus`][crate::InstanceStatus].
    ///
    /// # Returns
    ///
    /// [`true`] when the overall operation finishes successfully, else [`false`].
    #[doc(alias = "alsatimer_user_instance_get_status")]
    #[doc(alias = "get_status")]
    fn status<P: IsA<InstanceStatus>>(&self, status: &mut P) -> Result<(), glib::Error>;
}

impl<O: IsA<UserInstance>> UserInstanceExtManual for O {
    fn protocol_version(&self) -> Result<&[u16; 3], glib::Error> {
        unsafe {
            let mut triplet = std::ptr::null_mut() as *const [u16; 3];
            let mut error = std::ptr::null_mut();

            let is_ok = ffi::alsatimer_user_instance_get_protocol_version(
                self.as_ref().to_glib_none().0,
                &mut triplet as *mut *const [u16; 3],
                &mut error,
            );
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());

            if error.is_null() {
                Ok(&*triplet)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn set_params<P: IsA<InstanceParams>>(&self, params: &mut P) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();

            let is_ok = ffi::alsatimer_user_instance_set_params(
                self.as_ref().to_glib_none().0,
                &mut params.as_ref().to_glib_none().0,
                &mut error,
            );
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());

            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn status<P: IsA<InstanceStatus>>(&self, status: &mut P) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::alsatimer_user_instance_get_status(
                self.as_ref().to_glib_none().0,
                &mut status.as_ref().to_glib_none().0,
                &mut error,
            );
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());

            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}
