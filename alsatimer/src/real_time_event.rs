// SPDX-License-Identifier: MIT
use super::*;

impl RealTimeEvent {
    pub fn time(&mut self) -> [i64; 2] {
        unsafe {
            let mut real_time = [0i64; 2];
            ffi::alsatimer_real_time_event_get_time(
                self.to_glib_none_mut().0,
                &(real_time.as_mut_ptr() as *mut [i64; 2]),
            );
            real_time
        }
    }
}
