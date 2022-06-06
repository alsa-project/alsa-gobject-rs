// SPDX-License-Identifier: MIT
use super::*;

impl EventDataQueue {
    pub fn get_byte_param(&mut self) -> &[u8; 8] {
        unsafe {
            let mut ptr = std::ptr::null_mut() as *const [u8; 8];
            alsaseq_sys::alsaseq_event_data_queue_get_byte_param(
                self.to_glib_none_mut().0,
                &mut ptr as *mut *const [u8; 8],
            );
            &*ptr
        }
    }

    pub fn get_skew_param(&mut self) -> &[u32; 2] {
        unsafe {
            let mut ptr = std::ptr::null_mut() as *const [u32; 2];
            alsaseq_sys::alsaseq_event_data_queue_get_skew_param(
                self.to_glib_none_mut().0,
                &mut ptr as *mut *const [u32; 2],
            );
            &*ptr
        }
    }

    pub fn get_quadlet_param(&mut self) -> &[u32; 2] {
        unsafe {
            let mut ptr = std::ptr::null_mut() as *const [u32; 2];
            alsaseq_sys::alsaseq_event_data_queue_get_quadlet_param(
                self.to_glib_none_mut().0,
                &mut ptr as *mut *const [u32; 2],
            );
            &*ptr
        }
    }

    pub fn set_byte_param(&mut self, bytes: &[u8; 8]) {
        unsafe {
            alsaseq_sys::alsaseq_event_data_queue_set_byte_param(self.to_glib_none_mut().0, bytes);
        }
    }

    pub fn set_skew_param(&mut self, skew: &[u32; 2]) {
        unsafe {
            alsaseq_sys::alsaseq_event_data_queue_set_skew_param(self.to_glib_none_mut().0, skew);
        }
    }

    pub fn set_quadlet_param(&mut self, quadlets: &[u32; 2]) {
        unsafe {
            alsaseq_sys::alsaseq_event_data_queue_set_quadlet_param(
                self.to_glib_none_mut().0,
                quadlets as *const [u32; 2],
            );
        }
    }

    pub fn get_tstamp_param(&mut self) -> Tstamp {
        unsafe {
            let mut tstamp = std::ptr::null_mut() as *const alsaseq_sys::ALSASeqTstamp;
            alsaseq_sys::alsaseq_event_data_queue_get_tstamp_param(
                self.to_glib_none_mut().0,
                &mut tstamp,
            );
            from_glib_none(tstamp)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_manual_bindings() {
        let bytes_expected = [9, 2, 4, 7, 1, 8, 5, 3];
        let cntr = EventCntr::new(1).unwrap();
        let mut data = cntr.get_queue_data(0).unwrap();
        let bytes_orig = data.get_byte_param().clone();
        data.set_byte_param(&bytes_expected);
        cntr.set_queue_data(0, &data).unwrap();
        let mut data = cntr.get_queue_data(0).unwrap();
        let bytes_target = data.get_byte_param();
        assert_ne!(bytes_expected, bytes_orig);
        assert_eq!(&bytes_expected, bytes_target);

        let quads_expected = [54321, 12345];
        let cntr = EventCntr::new(1).unwrap();
        let mut data = cntr.get_queue_data(0).unwrap();
        let quads_orig = data.get_quadlet_param().clone();
        data.set_quadlet_param(&quads_expected);
        cntr.set_queue_data(0, &data).unwrap();
        let mut data = cntr.get_queue_data(0).unwrap();
        let quads_target = data.get_quadlet_param();
        assert_ne!(quads_expected, quads_orig);
        assert_eq!(&quads_expected, quads_target);

        let skew_expected = [45678, 987654];
        let cntr = EventCntr::new(1).unwrap();
        let mut data = cntr.get_queue_data(0).unwrap();
        let skew_orig = data.get_skew_param().clone();
        data.set_skew_param(&skew_expected);
        cntr.set_queue_data(0, &data).unwrap();
        let mut data = cntr.get_queue_data(0).unwrap();
        let skew_target = data.get_skew_param();
        assert_ne!(skew_expected, skew_orig);
        assert_eq!(&skew_expected, skew_target);

        let cntr = EventCntr::new(1).unwrap();
        let mut data = cntr.get_queue_data(0).unwrap();
        let tstamp_orig = data.get_tstamp_param();
        let mut tstamp_expected = tstamp_orig.clone();
        tstamp_expected.set_tick_time(123456789);
        data.set_tstamp_param(&tstamp_expected);
        cntr.set_queue_data(0, &data).unwrap();
        let mut data = cntr.get_queue_data(0).unwrap();
        let tstamp_target = data.get_tstamp_param();
        assert_ne!(tstamp_expected.get_tick_time(), tstamp_orig.get_tick_time());
        assert_eq!(
            tstamp_expected.get_tick_time(),
            tstamp_target.get_tick_time()
        );
    }
}
