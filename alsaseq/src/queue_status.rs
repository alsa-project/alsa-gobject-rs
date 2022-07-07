// SPDX-License-Identifier: MIT
use super::*;

pub trait QueueStatusExtManual {
    fn real_time(&self) -> &[u32; 2];
}

impl<O: IsA<QueueStatus>> QueueStatusExtManual for O {
    fn real_time(&self) -> &[u32; 2] {
        unsafe {
            let mut ptr = std::ptr::null_mut() as *const [u32; 2];
            ffi::alsaseq_queue_status_get_real_time(
                self.as_ref().to_glib_none().0,
                &mut ptr as *mut *const [u32; 2],
            );
            &*ptr
        }
    }
}
