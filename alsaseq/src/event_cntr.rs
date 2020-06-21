use glib::object::IsA;
use glib::translate::*;

use EventCntr;
use Addr;
use EventDataConnect;
use EventDataCtl;
use EventDataNote;
use EventDataQueue;
use EventDataResult;
use Tstamp;

pub trait EventCntrExtManual {
    fn get_addr_data(&self, index: usize) -> Result<Addr, glib::Error>;
    fn get_blob_data(&self, index: usize) -> Result<&[u8], glib::Error>;
    fn get_byte_data(&self, index: usize) -> Result<&[u8;12], glib::Error>;
    fn get_connect_data(&self, index: usize) -> Result<EventDataConnect, glib::Error>;
    fn get_ctl_data(&self, index: usize) -> Result<EventDataCtl, glib::Error>;
    fn get_dst(&self, index: usize) -> Result<Addr, glib::Error>;
    fn get_src(&self, index: usize) -> Result<Addr, glib::Error>;
    fn get_note_data(&self, index: usize) -> Result<EventDataNote, glib::Error>;
    fn get_quadlet_data(&self, index: usize) -> Result<&[u32;3], glib::Error>;
    fn get_queue_data(&self, index: usize) -> Result<EventDataQueue, glib::Error>;
    fn get_result_data(&self, index: usize) -> Result<EventDataResult, glib::Error>;
    fn get_tstamp(&self, index: usize) -> Result<Tstamp, glib::Error>;
    fn get_tstamp_data(&self, index: usize) -> Result<Tstamp, glib::Error>;
    fn set_byte_data(&self, index: usize, data: &[u8;12]) -> Result<(), glib::Error>;
    fn set_quadlet_data(&self, index: usize, data: &[u32;3]) -> Result<(), glib::Error>;
}

impl<O: IsA<EventCntr>> EventCntrExtManual for O {
    fn get_addr_data(&self, index: usize) -> Result<Addr, glib::Error> {
        unsafe {
            let mut data = std::ptr::null_mut() as *const alsaseq_sys::ALSASeqAddr;
            let mut error = std::ptr::null_mut();

            let _ = alsaseq_sys::alsaseq_event_cntr_get_addr_data(
                self.as_ref().to_glib_none().0,
                index,
                &mut data,
                &mut error,
            );

            if error.is_null() {
                Ok(from_glib_none(data))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn get_blob_data(&self, index: usize) -> Result<&[u8], glib::Error> {
        unsafe {
            let mut data = std::ptr::null_mut() as *const u8;
            let mut size = 0 as usize;
            let mut error = std::ptr::null_mut();

            let _ = alsaseq_sys::alsaseq_event_cntr_get_blob_data(
                self.as_ref().to_glib_none().0,
                index,
                &mut data,
                &mut size,
                &mut error,
            );

            if error.is_null() {
                Ok(std::slice::from_raw_parts(data, size))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn get_byte_data(&self, index: usize) -> Result<&[u8;12], glib::Error> {
        unsafe {
            let mut ptr = std::ptr::null_mut() as *const [u8;12];
            let mut error = std::ptr::null_mut();

            alsaseq_sys::alsaseq_event_cntr_get_byte_data(
                self.as_ref().to_glib_none().0,
                index,
                &mut ptr,
                &mut error,
            );

            if error.is_null() {
                Ok(&*ptr)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn get_connect_data(&self, index: usize) -> Result<EventDataConnect, glib::Error> {
        unsafe {
            let mut data = std::ptr::null_mut() as *const alsaseq_sys::ALSASeqEventDataConnect;
            let mut error = std::ptr::null_mut();

            let _ = alsaseq_sys::alsaseq_event_cntr_get_connect_data(
                self.as_ref().to_glib_none().0,
                index,
                &mut data,
                &mut error,
            );

            if error.is_null() {
                Ok(from_glib_none(data))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn get_ctl_data(&self, index: usize) -> Result<EventDataCtl, glib::Error> {
        unsafe {
            let mut data = std::ptr::null_mut() as *const alsaseq_sys::ALSASeqEventDataCtl;
            let mut error = std::ptr::null_mut();

            let _ = alsaseq_sys::alsaseq_event_cntr_get_ctl_data(
                self.as_ref().to_glib_none().0,
                index,
                &mut data,
                &mut error,
            );

            if error.is_null() {
                Ok(from_glib_none(data))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn get_dst(&self, index: usize) -> Result<Addr, glib::Error> {
        unsafe {
            let mut dst = std::ptr::null_mut() as *const alsaseq_sys::ALSASeqAddr;
            let mut error = std::ptr::null_mut();

            let _ = alsaseq_sys::alsaseq_event_cntr_get_dst(
                self.as_ref().to_glib_none().0,
                index,
                &mut dst,
                &mut error,
            );

            if error.is_null() {
                Ok(from_glib_none(dst))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn get_src(&self, index: usize) -> Result<Addr, glib::Error> {
        unsafe {
            let mut src = std::ptr::null_mut() as *const alsaseq_sys::ALSASeqAddr;
            let mut error = std::ptr::null_mut();

            let _ = alsaseq_sys::alsaseq_event_cntr_get_src(
                self.as_ref().to_glib_none().0,
                index,
                &mut src,
                &mut error,
            );

            if error.is_null() {
                Ok(from_glib_none(src))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn get_note_data(&self, index: usize) -> Result<EventDataNote, glib::Error> {
        unsafe {
            let mut data = std::ptr::null_mut() as *const alsaseq_sys::ALSASeqEventDataNote;
            let mut error = std::ptr::null_mut();

            let _ = alsaseq_sys::alsaseq_event_cntr_get_note_data(
                self.as_ref().to_glib_none().0,
                index,
                &mut data,
                &mut error,
            );

            if error.is_null() {
                Ok(from_glib_none(data))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn get_quadlet_data(&self, index: usize) -> Result<&[u32;3], glib::Error> {
        unsafe {
            let mut ptr = std::ptr::null_mut() as *const [u32;3];
            let mut error = std::ptr::null_mut();

            alsaseq_sys::alsaseq_event_cntr_get_quadlet_data(
                self.as_ref().to_glib_none().0,
                index,
                &mut ptr,
                &mut error,
            );

            if error.is_null() {
                Ok(&*ptr)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn get_queue_data(&self, index: usize) -> Result<EventDataQueue, glib::Error> {
        unsafe {
            let mut data = std::ptr::null_mut() as *const alsaseq_sys::ALSASeqEventDataQueue;
            let mut error = std::ptr::null_mut();

            let _ = alsaseq_sys::alsaseq_event_cntr_get_queue_data(
                self.as_ref().to_glib_none().0,
                index,
                &mut data,
                &mut error,
            );

            if error.is_null() {
                Ok(from_glib_none(data))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn get_result_data(&self, index: usize) -> Result<EventDataResult, glib::Error> {
        unsafe {
            let mut data = std::ptr::null_mut() as *const alsaseq_sys::ALSASeqEventDataResult;
            let mut error = std::ptr::null_mut();

            let _ = alsaseq_sys::alsaseq_event_cntr_get_result_data(
                self.as_ref().to_glib_none().0,
                index,
                &mut data,
                &mut error,
            );

            if error.is_null() {
                Ok(from_glib_none(data))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn get_tstamp(&self, index: usize) -> Result<Tstamp, glib::Error> {
        unsafe {
            let mut data = std::ptr::null_mut() as *const alsaseq_sys::ALSASeqTstamp;
            let mut error = std::ptr::null_mut();

            let _ = alsaseq_sys::alsaseq_event_cntr_get_tstamp(
                self.as_ref().to_glib_none().0,
                index,
                &mut data,
                &mut error);

            if error.is_null() {
                Ok(from_glib_none(data))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn get_tstamp_data(&self, index: usize) -> Result<Tstamp, glib::Error> {
        unsafe {
            let mut data = std::ptr::null_mut() as *const alsaseq_sys::ALSASeqTstamp;
            let mut error = std::ptr::null_mut();

            let _ = alsaseq_sys::alsaseq_event_cntr_get_tstamp_data(
                self.as_ref().to_glib_none().0,
                index,
                &mut data,
                &mut error);

            if error.is_null() {
                Ok(from_glib_none(data))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn set_byte_data(&self, index: usize, data: &[u8;12]) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();

            alsaseq_sys::alsaseq_event_cntr_set_byte_data(
                self.as_ref().to_glib_none().0,
                index,
                data,
                &mut error,
            );

            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn set_quadlet_data(&self, index: usize, data: &[u32;3]) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();

            alsaseq_sys::alsaseq_event_cntr_set_quadlet_data(
                self.as_ref().to_glib_none().0,
                index,
                data,
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
    use Addr;
    use EventCntr;
    use EventCntrExt;
    use EventCntrExtManual;

    #[test]
    fn test_manual_bindings() {
        let cntr = EventCntr::new(1).unwrap();

        // Addr data.
        let addr = Addr::new(12, 31);
        cntr.set_addr_data(0, &addr).unwrap();
        let data = cntr.get_addr_data(0).unwrap();
        assert_eq!(&addr, &data);

        // Blob data.
        let blob = [21, 22, 25, 98, 10, 12, 99];
        cntr.set_blob_data(0, &blob).unwrap();
        let data = cntr.get_blob_data(0).unwrap();
        assert_eq!(&blob, data);

        // Byte data.
        let bytes = [1, 9, 2, 0, 17, 9, 5, 99, 210, 4, 112, 77];
        cntr.set_byte_data(0, &bytes).unwrap();
        let data = cntr.get_byte_data(0).unwrap();
        assert_eq!(&bytes, data);

        // Connect data.
        let src = Addr::new(91, 11);
        let dst = Addr::new(173, 251);
        let data = cntr.get_connect_data(0).unwrap();
        let mut conn = data.clone();
        conn.set_src(&src);
        conn.set_dst(&dst);
        cntr.set_connect_data(0, &conn).unwrap();
        let mut data = cntr.get_connect_data(0).unwrap();
        assert_eq!(data.get_src(), src);
        assert_eq!(data.get_dst(), dst);

        // Control data.
        let data = cntr.get_ctl_data(0).unwrap();
        let mut ctl = data.clone();
        ctl.set_channel(172);
        ctl.set_param(169);
        ctl.set_value(254);
        cntr.set_ctl_data(0, &ctl).unwrap();
        let data = cntr.get_ctl_data(0).unwrap();
        assert_eq!(data.get_channel(), ctl.get_channel());
        assert_eq!(data.get_param(), ctl.get_param());
        assert_eq!(data.get_value(), ctl.get_value());

        // Destination data.
        let dst = Addr::new(102, 57);
        cntr.set_dst(0, &dst).unwrap();
        let data = cntr.get_dst(0).unwrap();
        assert_eq!(dst, data);

        // Source data.
        let src = Addr::new(13, 37);
        cntr.set_src(0, &src).unwrap();
        let data = cntr.get_src(0).unwrap();
        assert_eq!(src, data);

        // Note data.
        let data = cntr.get_note_data(0).unwrap();
        let mut note = data.clone();
        note.set_channel(11);
        note.set_note(12);
        note.set_velocity(13);
        note.set_off_velocity(14);
        note.set_duration(15);
        cntr.set_note_data(0, &note).unwrap();
        let data = cntr.get_note_data(0).unwrap();
        assert_eq!(data.get_channel(), note.get_channel());
        assert_eq!(data.get_note(), note.get_note());
        assert_eq!(data.get_velocity(), note.get_velocity());
        assert_eq!(data.get_off_velocity(), note.get_off_velocity());
        assert_eq!(data.get_duration(), note.get_duration());

        // Quadlet data.
        let quadlets = [11111, 22222, 33333];
        cntr.set_quadlet_data(0, &quadlets).unwrap();
        let data = cntr.get_quadlet_data(0).unwrap();
        assert_eq!(&quadlets, data);

        // Queue data.
        let data = cntr.get_queue_data(0).unwrap();
        let mut queue_ctl = data.clone();
        queue_ctl.set_position_param(793);
        cntr.set_queue_data(0, &queue_ctl).unwrap();
        let data = cntr.get_queue_data(0).unwrap();
        assert_eq!(queue_ctl.get_position_param(), data.get_position_param());

        // Result data.
        let data = cntr.get_result_data(0).unwrap();
        let mut result = data.clone();
        result.set_result(-123);
        cntr.set_result_data(0, &result).unwrap();
        let data = cntr.get_result_data(0).unwrap();
        assert_eq!(data.get_result(), result.get_result());

        // Timestamp.
        let data = cntr.get_tstamp(0).unwrap();
        let mut tstamp = data.clone();
        tstamp.set_tick_time(315289);
        cntr.set_tstamp(0, &tstamp).unwrap();
        let data = cntr.get_tstamp(0).unwrap();
        assert_eq!(data.get_tick_time(), tstamp.get_tick_time());

        // Timestamp data.
        let data = cntr.get_tstamp_data(0).unwrap();
        let mut tstamp = data.clone();
        tstamp.set_tick_time(315289);
        cntr.set_tstamp_data(0, &tstamp).unwrap();
        let data = cntr.get_tstamp_data(0).unwrap();
        assert_eq!(data.get_tick_time(), tstamp.get_tick_time());
    }
}
