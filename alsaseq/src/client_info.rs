// SPDX-License-Identifier: MIT
use super::*;

/// Trait containing the rest of [`struct@ClientInfo`] methods.
pub trait ClientInfoExtManual {
    /// Get the list of type of events configured to be listen.
    ///
    /// # Returns
    ///
    /// [`true`] when the overall operation finishes successfully, else [`false`].
    ///
    /// ## `event_types`
    /// The array with elements for the type of event to listen.
    #[doc(alias = "alsaseq_client_info_get_event_filter")]
    #[doc(alias = "get_event_filter")]
    fn event_filter(&self) -> Result<Vec<EventType>, glib::Error>;

    /// Set the list of type of events configured to be listen.
    /// ## `event_types`
    /// The array with elements for the type of event to listen.
    ///
    /// # Returns
    ///
    /// [`true`] when the overall operation finishes successfully, else [`false`].
    #[doc(alias = "alsaseq_client_info_set_event_filter")]
    fn set_event_filter(&self, entries: &[EventType]) -> Result<(), glib::Error>;
}

impl<O: IsA<ClientInfo>> ClientInfoExtManual for O {
    fn event_filter(&self) -> Result<Vec<EventType>, glib::Error> {
        unsafe {
            let mut ptr = std::ptr::null_mut() as *mut ffi::ALSASeqEventType;
            let mut len = 0 as usize;
            let mut error = std::ptr::null_mut();

            let is_ok = ffi::alsaseq_client_info_get_event_filter(
                self.as_ref().to_glib_none().0,
                &mut ptr,
                &mut len,
                &mut error,
            );
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());

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
                array.push(entry.into_glib());
            }

            let is_ok = ffi::alsaseq_client_info_set_event_filter(
                self.as_ref().to_glib_none().0,
                array.as_ptr(),
                array.len(),
                &mut error,
            );
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());

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
        let filter_orig = info.event_filter().unwrap();

        info.set_event_filter(&filter_expected).unwrap();

        let filter_target = info.event_filter().unwrap();

        assert_ne!(filter_expected, filter_orig);
        assert_eq!(filter_expected, filter_target);
    }
}
