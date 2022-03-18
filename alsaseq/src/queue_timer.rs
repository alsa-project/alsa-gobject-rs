// SPDX-License-Identifier: MIT
use crate::*;

pub trait QueueTimerExtManual {
    fn get_alsa_data(&self) -> QueueTimerDataAlsa;
}

impl<O: IsA<QueueTimer>> QueueTimerExtManual for O {
    fn get_alsa_data(&self) -> QueueTimerDataAlsa {
        unsafe {
            let mut data = std::ptr::null_mut() as *const ffi::ALSASeqQueueTimerDataAlsa;
            ffi::alsaseq_queue_timer_get_alsa_data(self.as_ref().to_glib_none().0, &mut data);
            from_glib_none(data)
        }
    }
}
