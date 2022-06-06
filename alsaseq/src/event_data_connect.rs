// SPDX-License-Identifier: MIT
use super::*;

impl EventDataConnect {
    pub fn get_dst(&mut self) -> Addr {
        unsafe {
            let mut dst = std::ptr::null_mut() as *const alsaseq_sys::ALSASeqAddr;
            alsaseq_sys::alsaseq_event_data_connect_get_dst(self.to_glib_none_mut().0, &mut dst);
            from_glib_none(dst)
        }
    }

    pub fn get_src(&mut self) -> Addr {
        unsafe {
            let mut src = std::ptr::null_mut() as *const alsaseq_sys::ALSASeqAddr;
            alsaseq_sys::alsaseq_event_data_connect_get_src(self.to_glib_none_mut().0, &mut src);
            from_glib_none(src)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_manual_bindings() {
        let dst_expected = Addr::new(192, 168);
        let src_expected = Addr::new(169, 254);

        let cntr = EventCntr::new(1).unwrap();
        let mut data = cntr.get_connect_data(0).unwrap();
        let dst_orig = data.get_dst();
        let src_orig = data.get_src();

        let mut conn = data.clone();
        conn.set_dst(&dst_expected);
        conn.set_src(&src_expected);

        cntr.set_connect_data(0, &conn).unwrap();
        let mut data = cntr.get_connect_data(0).unwrap();
        let dst_target = data.get_dst();
        let src_target = data.get_src();

        assert_ne!(dst_expected, dst_orig);
        assert_ne!(src_expected, src_orig);

        assert_eq!(dst_expected, dst_target);
        assert_eq!(src_expected, src_target);
    }
}
