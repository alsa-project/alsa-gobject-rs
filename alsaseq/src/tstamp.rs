// SPDX-License-Identifier: MIT
use super::*;

glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Tstamp(Boxed<alsaseq_sys::ALSASeqTstamp>);

    match fn {
        copy => |ptr| gobject_sys::g_boxed_copy(alsaseq_sys::alsaseq_tstamp_get_type(), ptr as *mut _) as *mut alsaseq_sys::ALSASeqTstamp,
        free => |ptr| gobject_sys::g_boxed_free(alsaseq_sys::alsaseq_tstamp_get_type(), ptr as *mut _),
        get_type => || alsaseq_sys::alsaseq_tstamp_get_type(),
    }
}

impl Tstamp {
    pub fn get_tick_time(&self) -> u32 {
        unsafe {
            let mut tick_time = 0;
            alsaseq_sys::alsaseq_tstamp_get_tick_time(self.to_glib_none().0, &mut tick_time);
            tick_time
        }
    }

    pub fn set_tick_time(&mut self, tick_time: u32) {
        unsafe {
            alsaseq_sys::alsaseq_tstamp_set_tick_time(self.to_glib_none_mut().0, tick_time);
        }
    }

    pub fn get_real_time(&self) -> &[u32; 2] {
        unsafe {
            let mut ptr = std::ptr::null_mut() as *const [u32; 2];
            alsaseq_sys::alsaseq_tstamp_get_real_time(
                self.to_glib_none().0,
                &mut ptr as *mut *const [u32; 2],
            );
            &*ptr
        }
    }

    pub fn set_real_time(&mut self, real_time: &[u32; 2]) {
        unsafe {
            alsaseq_sys::alsaseq_tstamp_set_real_time(self.to_glib_none_mut().0, real_time);
        }
    }
}

unsafe impl Send for Tstamp {}

#[cfg(test)]
mod test {
    use crate::*;
    #[test]
    fn test_manual_bindings() {
        let cntr = EventCntr::new(1).unwrap();
        let mut tstamp = cntr.get_tstamp_data(0).unwrap();

        let real_time_expected = [4321, 9876];
        let real_time_orig = tstamp.get_real_time().clone();
        tstamp.set_real_time(&real_time_expected);
        let real_time_target = tstamp.get_real_time().clone();
        assert_ne!(real_time_expected, real_time_orig);
        assert_eq!(real_time_expected, real_time_target);

        let tick_time_expected = 968275;
        let tick_time_orig = tstamp.get_tick_time();
        tstamp.set_tick_time(tick_time_expected);
        let tick_time_target = tstamp.get_tick_time().clone();
        assert_ne!(tick_time_expected, tick_time_orig);
        assert_eq!(tick_time_expected, tick_time_target);
    }
}
