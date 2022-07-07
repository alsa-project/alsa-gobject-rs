// SPDX-License-Identifier: MIT
use super::*;

pub trait QueueTempoExtManual {
    fn skew(&self) -> &[u32; 2];
    fn set_skew(&self, skew: &[u32; 2]);
}

impl<O: IsA<QueueTempo>> QueueTempoExtManual for O {
    fn skew(&self) -> &[u32; 2] {
        unsafe {
            let mut ptr = std::ptr::null_mut() as *const [u32; 2];
            ffi::alsaseq_queue_tempo_get_skew(
                self.as_ref().to_glib_none().0,
                &mut ptr as *mut *const [u32; 2],
            );
            &*ptr
        }
    }

    fn set_skew(&self, skew: &[u32; 2]) {
        unsafe {
            ffi::alsaseq_queue_tempo_set_skew(self.as_ref().to_glib_none().0, skew);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{prelude::*, *};

    #[test]
    fn test_manual_bindings() {
        let skew_expected = [123, 456];
        let tempo = QueueTempo::new();
        let skew_orig = tempo.skew().clone();
        tempo.set_skew(&skew_expected);
        let skew_target = tempo.skew();
        assert_ne!(skew_expected, skew_orig);
        assert_eq!(&skew_expected, skew_target);
    }
}
