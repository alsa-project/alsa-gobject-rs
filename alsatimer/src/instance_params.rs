// SPDX-License-Identifier: MIT
use super::*;

pub trait InstanceParamsExtManual {
    fn set_event_filter(&self, event_filter: &[RealTimeEventType]) -> Result<(), glib::Error>;
    fn get_event_filter(&self) -> Result<Vec<RealTimeEventType>, glib::Error>;
}

impl<O: IsA<InstanceParams>> InstanceParamsExtManual for O {
    fn set_event_filter(&self, event_filter: &[RealTimeEventType]) -> Result<(), glib::Error> {
        unsafe {
            let entries: Vec<alsatimer_sys::ALSATimerRealTimeEventType> =
                event_filter.iter().map(|entry| entry.to_glib()).collect();
            let mut error = std::ptr::null_mut();

            alsatimer_sys::alsatimer_instance_params_set_event_filter(
                self.as_ref().to_glib_none().0,
                entries.as_ptr(),
                event_filter.len(),
                &mut error,
            );

            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn get_event_filter(&self) -> Result<Vec<RealTimeEventType>, glib::Error> {
        unsafe {
            let mut entries = std::ptr::null_mut();
            let mut entry_count = 0 as usize;
            let mut error = std::ptr::null_mut();

            alsatimer_sys::alsatimer_instance_params_get_event_filter(
                self.as_ref().to_glib_none().0,
                &mut entries,
                &mut entry_count,
                &mut error,
            );

            if error.is_null() {
                let entries: Vec<alsatimer_sys::ALSATimerRealTimeEventType> =
                    FromGlibContainer::from_glib_full_num(entries, entry_count);
                let return_entries: Vec<RealTimeEventType> = entries
                    .iter()
                    .map(|&entry| RealTimeEventType::from_glib(entry))
                    .collect();

                Ok(return_entries)
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_manual_bindings() {
        let filter_expected = vec![
            RealTimeEventType::Tick,
            RealTimeEventType::Start,
            RealTimeEventType::Stop,
            RealTimeEventType::Suspend,
            RealTimeEventType::Resume,
        ];

        let params = InstanceParams::new();
        let filter_orig = params.get_event_filter().unwrap();
        params.set_event_filter(&filter_expected).unwrap();
        let filter_target = params.get_event_filter().unwrap();
        assert_ne!(filter_expected, filter_orig);
        assert_eq!(filter_expected, filter_target);
    }
}
