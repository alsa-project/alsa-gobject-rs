// SPDX-License-Identifier: MIT
use super::*;

impl Event {
    pub fn addr_data(&self) -> Result<Addr, Error> {
        unsafe {
            let mut data = std::ptr::null_mut() as *const ffi::ALSASeqAddr;
            let mut error = std::ptr::null_mut();
            let is_ok =
                ffi::alsaseq_event_get_addr_data(self.to_glib_none().0, &mut data, &mut error);
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());

            if error.is_null() {
                Ok(from_glib_none(data))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    pub fn blob_data(&self) -> Result<&[u8], Error> {
        unsafe {
            let mut data = std::ptr::null_mut() as *const u8;
            let mut length = 0;
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::alsaseq_event_get_blob_data(
                self.to_glib_none().0,
                &mut data,
                &mut length,
                &mut error,
            );
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());

            if error.is_null() {
                Ok(std::slice::from_raw_parts(data, length))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    pub fn byte_data(&self) -> Result<&[u8], Error> {
        unsafe {
            let mut data = std::ptr::null_mut() as *const [u8; 12];
            let mut error = std::ptr::null_mut();
            let is_ok =
                ffi::alsaseq_event_get_byte_data(self.to_glib_none().0, &mut data, &mut error);
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());

            if error.is_null() {
                Ok(&*data)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    pub fn connect_data(&self) -> Result<EventDataConnect, Error> {
        unsafe {
            let mut data = std::ptr::null_mut() as *const ffi::ALSASeqEventDataConnect;
            let mut error = std::ptr::null_mut();
            let is_ok =
                ffi::alsaseq_event_get_connect_data(self.to_glib_none().0, &mut data, &mut error);
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());

            if error.is_null() {
                Ok(from_glib_none(data))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    pub fn ctl_data(&self) -> Result<EventDataCtl, Error> {
        unsafe {
            let mut data = std::ptr::null_mut() as *const ffi::ALSASeqEventDataCtl;
            let mut error = std::ptr::null_mut();
            let is_ok =
                ffi::alsaseq_event_get_ctl_data(self.to_glib_none().0, &mut data, &mut error);
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());

            if error.is_null() {
                Ok(from_glib_none(data))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    pub fn destination(&self) -> Addr {
        unsafe {
            let mut addr = std::ptr::null_mut() as *const ffi::ALSASeqAddr;
            ffi::alsaseq_event_get_destination(self.to_glib_none().0, &mut addr);
            from_glib_none(addr)
        }
    }

    pub fn note_data(&self) -> Result<EventDataNote, Error> {
        unsafe {
            let mut data = std::ptr::null_mut() as *const ffi::ALSASeqEventDataNote;
            let mut error = std::ptr::null_mut();
            let is_ok =
                ffi::alsaseq_event_get_note_data(self.to_glib_none().0, &mut data, &mut error);
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());

            if error.is_null() {
                Ok(from_glib_none(data))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    pub fn quadlet_data(&self) -> Result<&[u32], Error> {
        unsafe {
            let mut data = std::ptr::null_mut() as *const [u32; 3];
            let mut error = std::ptr::null_mut();
            let is_ok =
                ffi::alsaseq_event_get_quadlet_data(self.to_glib_none().0, &mut data, &mut error);
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());

            if error.is_null() {
                Ok(&*data)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    pub fn queue_data(&self) -> Result<EventDataQueue, Error> {
        unsafe {
            let mut data = std::ptr::null_mut() as *const ffi::ALSASeqEventDataQueue;
            let mut error = std::ptr::null_mut();
            let is_ok =
                ffi::alsaseq_event_get_queue_data(self.to_glib_none().0, &mut data, &mut error);
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());

            if error.is_null() {
                Ok(from_glib_none(data))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    pub fn real_time(&self) -> Result<[u32; 2], Error> {
        unsafe {
            let mut data = std::ptr::null_mut() as *const [u32; 2];
            let mut error = std::ptr::null_mut();
            let is_ok =
                ffi::alsaseq_event_get_real_time(self.to_glib_none().0, &mut data, &mut error);
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());

            if error.is_null() {
                let mut real_time = [0u32; 2];
                real_time.copy_from_slice(&*data);
                Ok(real_time)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    pub fn real_time_data(&self) -> Result<[u32; 2], Error> {
        unsafe {
            let mut data = std::ptr::null_mut() as *const [u32; 2];
            let mut error = std::ptr::null_mut();
            let is_ok =
                ffi::alsaseq_event_get_real_time_data(self.to_glib_none().0, &mut data, &mut error);
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());

            if error.is_null() {
                let mut real_time = [0u32; 2];
                real_time.copy_from_slice(&*data);
                Ok(real_time)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    pub fn result_data(&self) -> Result<EventDataResult, Error> {
        unsafe {
            let mut data = std::ptr::null_mut() as *const ffi::ALSASeqEventDataResult;
            let mut error = std::ptr::null_mut();
            let is_ok =
                ffi::alsaseq_event_get_result_data(self.to_glib_none().0, &mut data, &mut error);
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());

            if error.is_null() {
                Ok(from_glib_none(data))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    pub fn source(&self) -> Addr {
        unsafe {
            let mut addr = std::ptr::null_mut() as *const ffi::ALSASeqAddr;
            ffi::alsaseq_event_get_source(self.to_glib_none().0, &mut addr);
            from_glib_none(addr)
        }
    }

    pub fn tick_time(&self) -> Result<u32, Error> {
        unsafe {
            let mut tick_time = std::mem::MaybeUninit::uninit();
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::alsaseq_event_get_tick_time(
                self.to_glib_none().0,
                tick_time.as_mut_ptr(),
                &mut error,
            );
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());

            if error.is_null() {
                Ok(tick_time.assume_init())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    pub fn tick_time_data(&self) -> Result<u32, Error> {
        unsafe {
            let mut tick_time = std::mem::MaybeUninit::uninit();
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::alsaseq_event_get_tick_time_data(
                self.to_glib_none().0,
                tick_time.as_mut_ptr(),
                &mut error,
            );
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());

            if error.is_null() {
                Ok(tick_time.assume_init())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    pub fn set_byte_data(&mut self, data: &[u8; 12]) -> Result<(), Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let is_ok =
                ffi::alsaseq_event_set_byte_data(self.to_glib_none_mut().0, data, &mut error);
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());

            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    pub fn set_quadlet_data(&mut self, data: &[u32; 3]) -> Result<(), Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let is_ok =
                ffi::alsaseq_event_set_quadlet_data(self.to_glib_none_mut().0, data, &mut error);
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());

            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    pub fn set_real_time(&mut self, real_time: &[u32; 2]) -> Result<(), Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let is_ok =
                ffi::alsaseq_event_set_real_time(self.to_glib_none_mut().0, real_time, &mut error);
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());

            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    pub fn set_real_time_data(&mut self, real_time: &[u32; 2]) -> Result<(), Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::alsaseq_event_set_real_time_data(
                self.to_glib_none_mut().0,
                real_time,
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
