// SPDX-License-Identifier: MIT

use super::*;

impl DeviceCommonError {
    pub fn to_label(&self) -> &'static str {
        unsafe {
            let mut ptr = std::ptr::null_mut() as *const i8;
            alsahwdep_sys::alsahwdep_device_common_error_to_label(self.to_glib(), &mut ptr);
            std::ffi::CStr::from_ptr(ptr).to_str().unwrap()
        }
    }
}
