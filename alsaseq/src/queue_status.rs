// SPDX-License-Identifier: MIT
use super::*;

/// Trait containing the rest of[`struct@QueueStatus`] methods.
pub trait QueueStatusExtManual {
    /// Get time as wall-clock time.
    ///
    /// # Returns
    ///
    ///
    /// ## `real_time`
    /// The array with two elements for sec part and nsec part of real time.
    #[doc(alias = "alsaseq_queue_status_get_real_time")]
    #[doc(alias = "get_real_time")]
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
