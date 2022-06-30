// SPDX-License-Identifier: MIT
use super::*;

impl EventCntr {
    pub fn deserialize(&self) -> Vec<Event> {
        unsafe {
            let mut entries = std::ptr::null_mut();
            let _ =
                alsaseq_sys::alsaseq_event_cntr_deserialize(self.to_glib_none().0, &mut entries);
            let count = glib_sys::g_list_length(entries) as usize;
            FromGlibContainer::from_glib_container_num(entries, count)
        }
    }
}
