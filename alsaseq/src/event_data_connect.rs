// SPDX-License-Identifier: MIT
use super::*;

impl EventDataConnect {
    #[doc(alias = "alsaseq_event_data_connect_get_dst")]
    #[doc(alias = "get_dst")]
    pub fn dst(&mut self) -> Addr {
        unsafe {
            let mut dst = std::ptr::null_mut() as *const ffi::ALSASeqAddr;
            ffi::alsaseq_event_data_connect_get_dst(self.to_glib_none_mut().0, &mut dst);
            from_glib_none(dst)
        }
    }

    #[doc(alias = "alsaseq_event_data_connect_get_src")]
    #[doc(alias = "get_src")]
    pub fn src(&mut self) -> Addr {
        unsafe {
            let mut src = std::ptr::null_mut() as *const ffi::ALSASeqAddr;
            ffi::alsaseq_event_data_connect_get_src(self.to_glib_none_mut().0, &mut src);
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

        let mut ev = Event::new(EventType::PortSubscribed);
        let mut data = ev.connect_data().unwrap();
        let dst_orig = data.dst();
        let src_orig = data.src();

        let mut conn = data.clone();
        conn.set_dst(&dst_expected);
        conn.set_src(&src_expected);

        ev.set_connect_data(&conn).unwrap();
        let mut data = ev.connect_data().unwrap();
        let dst_target = data.dst();
        let src_target = data.src();

        assert_ne!(dst_expected, dst_orig);
        assert_ne!(src_expected, src_orig);

        assert_eq!(dst_expected, dst_target);
        assert_eq!(src_expected, src_target);
    }
}
