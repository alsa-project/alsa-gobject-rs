// SPDX-License-Identifier: MIT
use super::*;

pub trait DeviceCommonExtManual {
    #[doc(alias = "alsahwdep_device_common_get_protocol_version")]
    fn protocol_version(&self) -> Result<[u16; 3], glib::Error>;
}

impl<O: IsA<DeviceCommon>> DeviceCommonExtManual for O {
    fn protocol_version(&self) -> Result<[u16; 3], glib::Error> {
        unsafe {
            let mut triplet = [0u16; 3];
            let mut error = std::ptr::null_mut();

            let _ = ffi::alsahwdep_device_common_get_protocol_version(
                self.as_ref().to_glib_none().0,
                &(triplet.as_mut_ptr() as *mut [u16; 3]),
                &mut error,
            );

            if error.is_null() {
                Ok(triplet)
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}
