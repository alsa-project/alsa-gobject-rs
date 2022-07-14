// SPDX-License-Identifier: MIT
use super::*;

/// Trait containing the rest of [`struct@DeviceCommon`] methods.
pub trait DeviceCommonExtManual {
    /// Get the version of hwdep protocol used currently. The version is expressed as the array with
    /// three elements; major, minor, and micro version in the order. The length of major version is
    /// 16 bit, the length of minor and micro version is 8 bit each.
    /// ## `proto_ver_triplet`
    /// The version of protocol used currently.
    ///
    /// # Returns
    ///
    /// [`true`] when the overall operation finishes successfully, else [`false`].
    ///
    /// ## `proto_ver_triplet`
    /// The version of protocol currently used.
    #[doc(alias = "alsahwdep_device_common_get_protocol_version")]
    fn protocol_version(&self) -> Result<[u16; 3], glib::Error>;
}

impl<O: IsA<DeviceCommon>> DeviceCommonExtManual for O {
    fn protocol_version(&self) -> Result<[u16; 3], glib::Error> {
        unsafe {
            let mut triplet = [0u16; 3];
            let mut error = std::ptr::null_mut();

            let is_ok = ffi::alsahwdep_device_common_get_protocol_version(
                self.as_ref().to_glib_none().0,
                &(triplet.as_mut_ptr() as *mut [u16; 3]),
                &mut error,
            );
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());

            if error.is_null() {
                Ok(triplet)
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}
