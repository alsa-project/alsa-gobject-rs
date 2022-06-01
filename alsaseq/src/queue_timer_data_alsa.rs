// SPDX-License-Identifier: MIT
use crate::*;

impl QueueTimerDataAlsa {
    pub fn get_device_id(&mut self) -> alsatimer::DeviceId {
        unsafe {
            let mut device_id = std::ptr::null_mut() as *const alsatimer_sys::ALSATimerDeviceId;
            alsaseq_sys::alsaseq_queue_timer_data_alsa_get_device_id(
                self.to_glib_none_mut().0,
                &mut device_id,
            );
            from_glib_none(device_id)
        }
    }

    pub fn set_device_id(&mut self, device_id: &alsatimer::DeviceId) {
        unsafe {
            alsaseq_sys::alsaseq_queue_timer_data_alsa_set_device_id(
                self.to_glib_none_mut().0,
                device_id.to_glib_none().0,
            );
        }
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_manual_bindings() {
        let expected_device_id = alsatimer::DeviceId::new(alsatimer::Class::Pcm, 10, 20, 30);

        let timer = QueueTimer::new();
        let mut data = timer.get_alsa_data();
        let orig_device_id = data.get_device_id();

        data.set_device_id(&expected_device_id);
        timer.set_alsa_data(&data);

        let mut data = timer.get_alsa_data();
        let curr_device_id = data.get_device_id();

        assert_ne!(expected_device_id.get_class(), orig_device_id.get_class());
        assert_ne!(
            expected_device_id.get_card_id(),
            orig_device_id.get_card_id()
        );
        assert_ne!(
            expected_device_id.get_device_id(),
            orig_device_id.get_device_id()
        );
        assert_ne!(
            expected_device_id.get_subdevice_id(),
            orig_device_id.get_subdevice_id()
        );

        assert_eq!(expected_device_id.get_class(), curr_device_id.get_class());
        assert_eq!(
            expected_device_id.get_card_id(),
            curr_device_id.get_card_id()
        );
        assert_eq!(
            expected_device_id.get_device_id(),
            curr_device_id.get_device_id()
        );
        assert_eq!(
            expected_device_id.get_subdevice_id(),
            curr_device_id.get_subdevice_id()
        );
    }
}
