// SPDX-License-Identifier: MIT
use super::*;

/// Trait containing the rest of [`struct@InstanceStatus`] methods.
///
/// # Implementors
///
/// [`InstanceStatus`][struct@crate::InstanceStatus]
pub trait InstanceStatusExtManual {
    /// Get real time at which the timer starts, stops, pauses, and continues.
    ///
    /// # Returns
    ///
    ///
    /// ## `real_time`
    /// The array with two elements for the
    ///             seconds and nanoseconds parts of timestamp when the instance queues the latest
    ///             event.
    #[doc(alias = "alsatimer_instance_status_get_time")]
    fn time(&self) -> &[i64; 2];
}

impl<O: IsA<InstanceStatus>> InstanceStatusExtManual for O {
    fn time(&self) -> &[i64; 2] {
        unsafe {
            let mut tstamp = std::ptr::null_mut() as *const [i64; 2];

            ffi::alsatimer_instance_status_get_time(
                self.as_ref().to_glib_none().0,
                &mut tstamp as *mut *const [i64; 2],
            );

            &*tstamp
        }
    }
}
