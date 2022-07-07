// SPDX-License-Identifier: MIT
use super::*;

pub trait UserInstanceExtManual {
    #[doc(alias = "alsatimer_user_instance_get_protocol_version")]
    #[doc(alias = "get_protocol_version")]
    fn protocol_version(&self) -> Result<&[u16; 3], glib::Error>;
    #[doc(alias = "alsatimer_user_instance_set_params")]
    fn set_params<P: IsA<InstanceParams>>(&self, params: &mut P) -> Result<(), glib::Error>;
    #[doc(alias = "alsatimer_user_instance_get_status")]
    #[doc(alias = "get_status")]
    fn status<P: IsA<InstanceStatus>>(&self, status: &mut P) -> Result<(), glib::Error>;
}

impl<O: IsA<UserInstance>> UserInstanceExtManual for O {
    fn protocol_version(&self) -> Result<&[u16; 3], glib::Error> {
        unsafe {
            let mut triplet = std::ptr::null_mut() as *const [u16; 3];
            let mut error = std::ptr::null_mut();

            let _ = ffi::alsatimer_user_instance_get_protocol_version(
                self.as_ref().to_glib_none().0,
                &mut triplet as *mut *const [u16; 3],
                &mut error,
            );

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

            let _ = ffi::alsatimer_user_instance_set_params(
                self.as_ref().to_glib_none().0,
                &mut params.as_ref().to_glib_none().0,
                &mut error,
            );

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
            let _ = ffi::alsatimer_user_instance_get_status(
                self.as_ref().to_glib_none().0,
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
}
