// SPDX-License-Identifier: MIT
use super::*;

pub trait ClientInfoExtManual {
    fn get_event_filter(&self) -> Result<Vec<EventType>, glib::Error>;
    fn set_event_filter(&self, entries: &[EventType]) -> Result<(), glib::Error>;
}

impl<O: IsA<ClientInfo>> ClientInfoExtManual for O {
    fn get_event_filter(&self) -> Result<Vec<EventType>, glib::Error> {
        unsafe {
            let mut ptr = std::ptr::null_mut() as *mut alsaseq_sys::ALSASeqEventType;
            let mut len = 0 as usize;
            let mut error = std::ptr::null_mut();

            alsaseq_sys::alsaseq_client_info_get_event_filter(
                self.as_ref().to_glib_none().0,
                &mut ptr,
                &mut len,
                &mut error,
            );

            if error.is_null() {
                let array = std::slice::from_raw_parts(ptr, len);
                let mut entries = Vec::new();
                for &entry in array {
                    entries.push(EventType::from_glib(entry));
                }
                Ok(entries)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn set_event_filter(&self, entries: &[EventType]) -> Result<(), glib::Error> {
        unsafe {
            let mut array = Vec::new();
            let mut error = std::ptr::null_mut();

            for &entry in entries {
                array.push(entry.to_glib());
            }

            alsaseq_sys::alsaseq_client_info_set_event_filter(
                self.as_ref().to_glib_none().0,
                array.as_ptr(),
                array.len(),
                &mut error,
            );

            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{prelude::*, *};

    #[test]
    fn test_manual_bindings() {
        let filter_expected = vec![
            EventType::QueueSkew,
            EventType::Sensing,
            EventType::ClientStart,
            EventType::PortStart,
            EventType::PortSubscribed,
            EventType::Usr1,
            EventType::Usr4,
            EventType::Usr7,
            EventType::Sysex,
            EventType::UsrVar2,
            EventType::UsrVar3,
            EventType::UsrVar4,
        ];

        let info = ClientInfo::new();
        let filter_orig = info.get_event_filter().unwrap();

        info.set_event_filter(&filter_expected).unwrap();

        let filter_target = info.get_event_filter().unwrap();

        assert_ne!(filter_expected, filter_orig);
        assert_eq!(filter_expected, filter_target);
    }
}
