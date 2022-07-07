// SPDX-License-Identifier: MIT

use super::*;

impl DeviceCommonError {
    /// Retrieve the string label of error from code.
    ///
    /// # Returns
    ///
    /// ## `label`
    /// The string label of error.
    #[doc(alias = "alsahwdep_device_common_error_to_label")]
    pub fn to_label(&self) -> &'static str {
        unsafe {
            let mut ptr = std::ptr::null_mut() as *const i8;
            ffi::alsahwdep_device_common_error_to_label(self.into_glib(), &mut ptr);
            std::ffi::CStr::from_ptr(ptr).to_str().unwrap()
        }
    }
}
