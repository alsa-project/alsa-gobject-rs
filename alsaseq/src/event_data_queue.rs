// SPDX-License-Identifier: MIT
use super::*;

impl EventDataQueue {
    /// Refer to eight bytes as the parameter of queue event.
    ///
    /// # Returns
    ///
    ///
    /// ## `bytes`
    /// The array with eight elements for bytes
    ///         parameter of the queue event.
    #[doc(alias = "alsaseq_event_data_queue_get_byte_param")]
    #[doc(alias = "get_byte_param")]
    pub fn byte_param(&mut self) -> &[u8; 8] {
        unsafe {
            let mut ptr = std::ptr::null_mut() as *const [u8; 8];
            ffi::alsaseq_event_data_queue_get_byte_param(
                self.to_glib_none_mut().0,
                &mut ptr as *mut *const [u8; 8],
            );
            &*ptr
        }
    }

    /// Refer to numerator and denominator of fraction for skew as the parameter of queue event.
    ///
    /// # Returns
    ///
    ///
    /// ## `skew`
    /// The array with two elements for numerator and
    ///        denominator of fraction for skew.
    #[doc(alias = "alsaseq_event_data_queue_get_skew_param")]
    #[doc(alias = "get_skew_param")]
    pub fn skew_param(&mut self) -> &[u32; 2] {
        unsafe {
            let mut ptr = std::ptr::null_mut() as *const [u32; 2];
            ffi::alsaseq_event_data_queue_get_skew_param(
                self.to_glib_none_mut().0,
                &mut ptr as *mut *const [u32; 2],
            );
            &*ptr
        }
    }

    /// Refer to two quadlets as the parameter of queue event.
    ///
    /// # Returns
    ///
    ///
    /// ## `quadlets`
    /// The array with two elements for quadlets as
    ///            the parameter of queue event.
    #[doc(alias = "alsaseq_event_data_queue_get_quadlet_param")]
    #[doc(alias = "get_quadlet_param")]
    pub fn quadlet_param(&mut self) -> &[u32; 2] {
        unsafe {
            let mut ptr = std::ptr::null_mut() as *const [u32; 2];
            ffi::alsaseq_event_data_queue_get_quadlet_param(
                self.to_glib_none_mut().0,
                &mut ptr as *mut *const [u32; 2],
            );
            &*ptr
        }
    }

    /// Copy eight bytes from the given buffer as the parameter of queue event.
    /// ## `bytes`
    /// The array with eight elements for bytes parameter
    ///         of the queue event.
    #[doc(alias = "alsaseq_event_data_queue_set_byte_param")]
    pub fn set_byte_param(&mut self, bytes: &[u8; 8]) {
        unsafe {
            ffi::alsaseq_event_data_queue_set_byte_param(self.to_glib_none_mut().0, bytes);
        }
    }

    /// Copy numerator and denominator of fraction for skew from the given buffer as the parameter of
    /// queue event.
    /// ## `skew`
    /// The array with two elements for numerator and
    ///        denominator of fraction for skew.
    #[doc(alias = "alsaseq_event_data_queue_set_skew_param")]
    pub fn set_skew_param(&mut self, skew: &[u32; 2]) {
        unsafe {
            ffi::alsaseq_event_data_queue_set_skew_param(self.to_glib_none_mut().0, skew);
        }
    }

    /// Set two quadlets from the given buffer as the parameter of queue event.
    /// ## `quadlets`
    /// The array with two elements for quadlets as the
    ///            parameter of queue event.
    #[doc(alias = "alsaseq_event_data_queue_set_quadlet_param")]
    pub fn set_quadlet_param(&mut self, quadlets: &[u32; 2]) {
        unsafe {
            ffi::alsaseq_event_data_queue_set_quadlet_param(
                self.to_glib_none_mut().0,
                quadlets as *const [u32; 2],
            );
        }
    }

    /// Get the real time as parameter of the queue event.
    ///
    /// # Returns
    ///
    ///
    /// ## `real_time`
    /// The real time as param of the queue
    ///             event.
    #[doc(alias = "alsaseq_event_data_queue_get_real_time_param")]
    #[doc(alias = "get_real_time_param")]
    pub fn real_time_param(&mut self) -> &[u32; 2] {
        unsafe {
            let mut ptr = std::ptr::null_mut() as *const [u32; 2];
            ffi::alsaseq_event_data_queue_get_real_time_param(
                self.to_glib_none_mut().0,
                &mut ptr as *mut *const [u32; 2],
            );
            &*ptr
        }
    }

    /// Set the real time as parameter of the queue event.
    /// ## `real_time`
    /// The real time as param of the queue event.
    #[doc(alias = "alsaseq_event_data_queue_set_real_time_param")]
    pub fn set_real_time_param(&mut self, real_time: &[u32; 2]) {
        unsafe {
            ffi::alsaseq_event_data_queue_set_real_time_param(self.to_glib_none_mut().0, real_time);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_manual_bindings() {
        let bytes_expected = [9, 2, 4, 7, 1, 8, 5, 3];
        let mut ev = Event::new(EventType::Start);
        let mut data = ev.queue_data().unwrap();
        let bytes_orig = data.byte_param().clone();
        data.set_byte_param(&bytes_expected);
        ev.set_queue_data(&data).unwrap();
        let mut data = ev.queue_data().unwrap();
        let bytes_target = data.byte_param();
        assert_ne!(bytes_expected, bytes_orig);
        assert_eq!(&bytes_expected, bytes_target);

        let quads_expected = [54321, 12345];
        let mut ev = Event::new(EventType::Start);
        let mut data = ev.queue_data().unwrap();
        let quads_orig = data.quadlet_param().clone();
        data.set_quadlet_param(&quads_expected);
        ev.set_queue_data(&data).unwrap();
        let mut data = ev.queue_data().unwrap();
        let quads_target = data.quadlet_param();
        assert_ne!(quads_expected, quads_orig);
        assert_eq!(&quads_expected, quads_target);

        let skew_expected = [45678, 987654];
        let mut ev = Event::new(EventType::Start);
        let mut data = ev.queue_data().unwrap();
        let skew_orig = data.skew_param().clone();
        data.set_skew_param(&skew_expected);
        ev.set_queue_data(&data).unwrap();
        let mut data = ev.queue_data().unwrap();
        let skew_target = data.skew_param();
        assert_ne!(skew_expected, skew_orig);
        assert_eq!(&skew_expected, skew_target);

        let tick_time_expected = 123456789;
        let mut ev = Event::new(EventType::Start);
        let mut data = ev.queue_data().unwrap();
        let tick_time_orig = data.tick_time_param();
        data.set_tick_time_param(tick_time_expected);
        ev.set_queue_data(&data).unwrap();
        let data = ev.queue_data().unwrap();
        let tick_time = data.tick_time_param();
        assert_ne!(tick_time_orig, tick_time);
        assert_eq!(tick_time_expected, tick_time);
    }
}
