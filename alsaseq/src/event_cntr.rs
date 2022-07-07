// SPDX-License-Identifier: MIT
use super::*;

impl EventCntr {
    /// Retrieve `GLib::List` including batch of deserialized [`Event`][crate::Event].
    ///
    /// # Returns
    ///
    ///
    /// ## `events`
    /// The list of deserialized events.
    #[doc(alias = "alsaseq_event_cntr_deserialize")]
    pub fn deserialize(&self) -> Vec<Event> {
        unsafe {
            let mut entries = std::ptr::null_mut();
            let _ = ffi::alsaseq_event_cntr_deserialize(self.to_glib_none().0, &mut entries);
            let count = glib::ffi::g_list_length(entries) as usize;
            FromGlibContainer::from_glib_container_num(entries, count)
        }
    }
}
