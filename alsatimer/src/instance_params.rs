// SPDX-License-Identifier: MIT
use crate::*;

pub trait InstanceParamsExtManual {
    fn set_event_filter(&self, event_filter: &Vec<EventType>) -> Result<(), glib::Error>;
    fn get_event_filter(&self) -> Result<Vec<EventType>, glib::Error>;
}

impl<O: IsA<InstanceParams>> InstanceParamsExtManual for O {
    fn set_event_filter(&self, event_filter: &Vec<EventType>) -> Result<(), glib::Error> {
        unsafe {
            let entry_count = event_filter.len();
            let mut entries = Vec::<alsatimer_sys::ALSATimerEventType>::with_capacity(entry_count);
            let mut error = std::ptr::null_mut();

            for entry in event_filter {
                entries.push(EventType::to_glib(entry))
            }

            alsatimer_sys::alsatimer_instance_params_set_event_filter(
                self.as_ref().to_glib_none().0,
                entries.as_ptr(),
                entry_count,
                &mut error,
            );

            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn get_event_filter(&self) -> Result<Vec<EventType>, glib::Error> {
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
                let entries: Vec<alsatimer_sys::ALSATimerEventType> =
                    FromGlibContainer::from_glib_full_num(entries, entry_count);
                let mut return_entries = Vec::<EventType>::with_capacity(entries.len());

                for entry in entries.into_iter() {
                    return_entries.push(EventType::from_glib(entry))
                }

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
            EventType::Tick,
            EventType::Start,
            EventType::Stop,
            EventType::Suspend,
            EventType::Resume,
        ];

        let params = InstanceParams::new();
        let filter_orig = params.get_event_filter().unwrap();
        params.set_event_filter(&filter_expected).unwrap();
        let filter_target = params.get_event_filter().unwrap();
        assert_ne!(filter_expected, filter_orig);
        assert_eq!(filter_expected, filter_target);
    }
}
