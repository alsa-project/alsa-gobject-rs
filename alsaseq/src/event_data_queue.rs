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

    pub fn get_real_time_param(&mut self) -> &[u32; 2] {
        unsafe {
            let mut ptr = std::ptr::null_mut() as *const [u32; 2];
            alsaseq_sys::alsaseq_event_data_queue_get_real_time_param(
                self.to_glib_none_mut().0,
                &mut ptr as *mut *const [u32; 2],
            );
            &*ptr
        }
    }

    pub fn set_real_time_param(&mut self, real_time: &[u32; 2]) {
        unsafe {
            alsaseq_sys::alsaseq_event_data_queue_set_real_time_param(
                self.to_glib_none_mut().0,
                real_time,
            );
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
        let mut data = ev.get_queue_data().unwrap();
        let bytes_orig = data.get_byte_param().clone();
        data.set_byte_param(&bytes_expected);
        ev.set_queue_data(&data).unwrap();
        let mut data = ev.get_queue_data().unwrap();
        let bytes_target = data.get_byte_param();
        assert_ne!(bytes_expected, bytes_orig);
        assert_eq!(&bytes_expected, bytes_target);

        let quads_expected = [54321, 12345];
        let mut ev = Event::new(EventType::Start);
        let mut data = ev.get_queue_data().unwrap();
        let quads_orig = data.get_quadlet_param().clone();
        data.set_quadlet_param(&quads_expected);
        ev.set_queue_data(&data).unwrap();
        let mut data = ev.get_queue_data().unwrap();
        let quads_target = data.get_quadlet_param();
        assert_ne!(quads_expected, quads_orig);
        assert_eq!(&quads_expected, quads_target);

        let skew_expected = [45678, 987654];
        let mut ev = Event::new(EventType::Start);
        let mut data = ev.get_queue_data().unwrap();
        let skew_orig = data.get_skew_param().clone();
        data.set_skew_param(&skew_expected);
        ev.set_queue_data(&data).unwrap();
        let mut data = ev.get_queue_data().unwrap();
        let skew_target = data.get_skew_param();
        assert_ne!(skew_expected, skew_orig);
        assert_eq!(&skew_expected, skew_target);

        let tick_time_expected = 123456789;
        let mut ev = Event::new(EventType::Start);
        let mut data = ev.get_queue_data().unwrap();
        let tick_time_orig = data.get_tick_time_param();
        data.set_tick_time_param(tick_time_expected);
        ev.set_queue_data(&data).unwrap();
        let data = ev.get_queue_data().unwrap();
        let tick_time = data.get_tick_time_param();
        assert_ne!(tick_time_orig, tick_time);
        assert_eq!(tick_time_expected, tick_time);
    }
}
