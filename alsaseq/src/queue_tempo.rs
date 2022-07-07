// SPDX-License-Identifier: MIT
use super::*;

/// Trait containing the rest of [`struct@QueueTempo`] methods.
pub trait QueueTempoExtManual {
    /// Refer to numerator and denominator of fraction for skew.
    ///
    /// # Returns
    ///
    ///
    /// ## `skew`
    /// The array with two elements for numerator and denominator of fraction for skew.
    #[doc(alias = "alsaseq_queue_tempo_get_skew")]
    #[doc(alias = "get_skew")]
    fn skew(&self) -> &[u32; 2];

    /// Copy numerator and denominator of fraction for skew.
    /// ## `skew`
    /// The array with two elements for numerator and denominator of fraction for skew.
    #[doc(alias = "alsaseq_queue_tempo_set_skew")]
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
