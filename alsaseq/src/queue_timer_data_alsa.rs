// SPDX-License-Identifier: MIT
use crate::*;

impl QueueTimerDataAlsa {
    pub fn device_id(&mut self) -> alsatimer::DeviceId {
        unsafe {
            let mut device_id = std::ptr::null_mut() as *const alsatimer::ffi::ALSATimerDeviceId;
            ffi::alsaseq_queue_timer_data_alsa_get_device_id(
                self.to_glib_none_mut().0,
                &mut device_id,
            );
            from_glib_none(device_id)
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
        let orig_device_id = data.device_id();

        data.set_device_id(&expected_device_id);
        timer.set_alsa_data(&data);

        let mut data = timer.get_alsa_data();
        let curr_device_id = data.device_id();

        assert_ne!(expected_device_id.class(), orig_device_id.class());
        assert_ne!(expected_device_id.card_id(), orig_device_id.card_id());
        assert_ne!(expected_device_id.device_id(), orig_device_id.device_id());
        assert_ne!(
            expected_device_id.subdevice_id(),
            orig_device_id.subdevice_id()
        );

        assert_eq!(expected_device_id.class(), curr_device_id.class());
        assert_eq!(expected_device_id.card_id(), curr_device_id.card_id());
        assert_eq!(expected_device_id.device_id(), curr_device_id.device_id());
        assert_eq!(
            expected_device_id.subdevice_id(),
            curr_device_id.subdevice_id()
        );
    }
}
