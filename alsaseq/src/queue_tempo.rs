use crate::*;

pub trait QueueTempoExtManual {
    fn get_skew(&self) -> &[u32; 2];
    fn set_skew(&self, skew: &[u32; 2]);
}

impl<O: IsA<QueueTempo>> QueueTempoExtManual for O {
    fn get_skew(&self) -> &[u32; 2] {
        unsafe {
            let mut ptr = std::ptr::null_mut() as *const [u32; 2];
            alsaseq_sys::alsaseq_queue_tempo_get_skew(
                self.as_ref().to_glib_none().0,
                &mut ptr as *mut *const [u32; 2],
            );
            &*ptr
        }
    }

    fn set_skew(&self, skew: &[u32; 2]) {
        unsafe {
            alsaseq_sys::alsaseq_queue_tempo_set_skew(self.as_ref().to_glib_none().0, skew);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_manual_bindings() {
        let skew_expected = [123, 456];
        let tempo = QueueTempo::new();
        let skew_orig = tempo.get_skew().clone();
        tempo.set_skew(&skew_expected);
        let skew_target = tempo.get_skew();
        assert_ne!(skew_expected, skew_orig);
        assert_eq!(&skew_expected, skew_target);
    }
}
