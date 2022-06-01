// SPDX-License-Identifier: MIT
use crate::*;

pub trait InstanceStatusExtManual {
    fn get_tstamp(&self) -> &[i64; 2];
}

impl<O: IsA<InstanceStatus>> InstanceStatusExtManual for O {
    fn get_tstamp(&self) -> &[i64; 2] {
        unsafe {
            let mut tstamp = std::ptr::null_mut() as *const [i64; 2];

            alsatimer_sys::alsatimer_instance_status_get_tstamp(
                self.as_ref().to_glib_none().0,
                &mut tstamp as *mut *const [i64; 2],
            );

            &*tstamp
        }
    }
}
