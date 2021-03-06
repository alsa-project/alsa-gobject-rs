use glib::translate::*;

use alsatimer_sys;

use EventDataTstamp;

impl EventDataTstamp {
    pub fn get_tstamp(&mut self, tstamp: &mut [i64;2]) {
        unsafe {
            let ptr: *mut [i64;2] = tstamp;
            alsatimer_sys::alsatimer_event_data_tstamp_get_tstamp(
                self.to_glib_none_mut().0,
                &ptr,
            );
        }
    }
}

#[cfg(test)]
mod test {
    use Event;

    #[test]
    fn test_manual_bindings() {
        let mut tstamp = [0;2];

        let mut ev = Event::new();
        let mut data = ev.get_tstamp_data();
        data.get_tstamp(&mut tstamp);

        assert_eq!([0, 0], tstamp);
    }
}
