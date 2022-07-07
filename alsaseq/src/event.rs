// SPDX-License-Identifier: MIT
use super::*;

impl Event {
    /// Get the address data of event, available when [`event_type()`][Self::event_type()] results in one of:
    ///
    /// - [`EventType`][crate::EventType].CLIENT_START
    /// - [`EventType`][crate::EventType].CLIENT_EXIT
    /// - [`EventType`][crate::EventType].CLIENT_CHANGE
    /// - [`EventType`][crate::EventType].PORT_START
    /// - [`EventType`][crate::EventType].PORT_EXIT
    /// - [`EventType`][crate::EventType].PORT_CHANGE
    /// - [`EventType`][crate::EventType].USR0
    /// - [`EventType`][crate::EventType].USR1
    /// - [`EventType`][crate::EventType].USR2
    /// - [`EventType`][crate::EventType].USR3
    /// - [`EventType`][crate::EventType].USR4
    /// - [`EventType`][crate::EventType].USR5
    /// - [`EventType`][crate::EventType].USR6
    /// - [`EventType`][crate::EventType].USR7
    /// - [`EventType`][crate::EventType].USR8
    /// - [`EventType`][crate::EventType].USR9
    ///
    /// # Returns
    ///
    /// [`true`] when the overall operation finishes successfully, else [`false`].
    ///
    /// ## `data`
    /// The address data of event.
    #[doc(alias = "alsaseq_event_get_addr_data")]
    #[doc(alias = "get_addr_data")]
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

    /// Refer to the blob data, available when [`event_type()`][Self::event_type()] results in one of:
    ///
    /// - [`EventType`][crate::EventType].SYSEX
    /// - [`EventType`][crate::EventType].BOUNCE
    /// - [`EventType`][crate::EventType].USR_VAR0
    /// - [`EventType`][crate::EventType].USR_VAR1
    /// - [`EventType`][crate::EventType].USR_VAR2
    /// - [`EventType`][crate::EventType].USR_VAR3
    /// - [`EventType`][crate::EventType].USR_VAR4
    ///
    /// # Returns
    ///
    /// [`true`] when the overall operation finishes successfully, else [`false`].
    ///
    /// ## `data`
    /// The pointer to blob data.
    #[doc(alias = "alsaseq_event_get_blob_data")]
    #[doc(alias = "get_blob_data")]
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

    /// Get the byte data, available when [`event_type()`][Self::event_type()] results in one of:
    ///
    /// - [`EventType`][crate::EventType].USR0
    /// - [`EventType`][crate::EventType].USR1
    /// - [`EventType`][crate::EventType].USR2
    /// - [`EventType`][crate::EventType].USR3
    /// - [`EventType`][crate::EventType].USR4
    /// - [`EventType`][crate::EventType].USR5
    /// - [`EventType`][crate::EventType].USR6
    /// - [`EventType`][crate::EventType].USR7
    /// - [`EventType`][crate::EventType].USR8
    /// - [`EventType`][crate::EventType].USR9
    ///
    /// # Returns
    ///
    /// [`true`] when the overall operation finishes successfully, else [`false`].
    ///
    /// ## `data`
    /// The byte data of event.
    #[doc(alias = "alsaseq_event_get_byte_data")]
    #[doc(alias = "get_byte_data")]
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

    /// Get the connect data of event, available when [`event_type()`][Self::event_type()] results in one of:
    ///
    /// - [`EventType`][crate::EventType].PORT_SUBSCRIBED
    /// - [`EventType`][crate::EventType].PORT_UNSUBSCRIBED
    /// - [`EventType`][crate::EventType].USR0
    /// - [`EventType`][crate::EventType].USR1
    /// - [`EventType`][crate::EventType].USR2
    /// - [`EventType`][crate::EventType].USR3
    /// - [`EventType`][crate::EventType].USR4
    /// - [`EventType`][crate::EventType].USR5
    /// - [`EventType`][crate::EventType].USR6
    /// - [`EventType`][crate::EventType].USR7
    /// - [`EventType`][crate::EventType].USR8
    /// - [`EventType`][crate::EventType].USR9
    ///
    /// # Returns
    ///
    /// [`true`] when the overall operation finishes successfully, else [`false`].
    ///
    /// ## `data`
    /// The connect data of event.
    #[doc(alias = "alsaseq_event_get_connect_data")]
    #[doc(alias = "get_connect_data")]
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

    /// Refer to the control data, available when [`event_type()`][Self::event_type()] results in one of:
    ///
    /// - [`EventType`][crate::EventType].CONTROLLER
    /// - [`EventType`][crate::EventType].PGMCHANGE
    /// - [`EventType`][crate::EventType].CHANPRESS
    /// - [`EventType`][crate::EventType].PITCHBEND
    /// - [`EventType`][crate::EventType].CONTROL14
    /// - [`EventType`][crate::EventType].NONREGPARAM
    /// - [`EventType`][crate::EventType].REGPARAM
    /// - [`EventType`][crate::EventType].SONGPOS
    /// - [`EventType`][crate::EventType].SONGSEL
    /// - [`EventType`][crate::EventType].QFRAME
    /// - [`EventType`][crate::EventType].TIMESIGN
    /// - [`EventType`][crate::EventType].KEYSIGN
    /// - [`EventType`][crate::EventType].USR0
    /// - [`EventType`][crate::EventType].USR1
    /// - [`EventType`][crate::EventType].USR2
    /// - [`EventType`][crate::EventType].USR3
    /// - [`EventType`][crate::EventType].USR4
    /// - [`EventType`][crate::EventType].USR5
    /// - [`EventType`][crate::EventType].USR6
    /// - [`EventType`][crate::EventType].USR7
    /// - [`EventType`][crate::EventType].USR8
    /// - [`EventType`][crate::EventType].USR9
    ///
    /// # Returns
    ///
    /// [`true`] when the overall operation finishes successfully, else [`false`].
    ///
    /// ## `data`
    /// The control data of event.
    #[doc(alias = "alsaseq_event_get_ctl_data")]
    #[doc(alias = "get_ctl_data")]
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

    /// Refer to the destination address of event.
    ///
    /// # Returns
    ///
    ///
    /// ## `addr`
    /// A [`Addr`][crate::Addr] for event destination.
    #[doc(alias = "alsaseq_event_get_destination")]
    #[doc(alias = "get_destination")]
    pub fn destination(&self) -> Addr {
        unsafe {
            let mut addr = std::ptr::null_mut() as *const ffi::ALSASeqAddr;
            ffi::alsaseq_event_get_destination(self.to_glib_none().0, &mut addr);
            from_glib_none(addr)
        }
    }

    /// Refer to the note data, available when [`event_type()`][Self::event_type()] results in one of:
    ///
    /// - [`EventType`][crate::EventType].NOTE
    /// - [`EventType`][crate::EventType].NOTEON
    /// - [`EventType`][crate::EventType].NOTEOFF
    /// - [`EventType`][crate::EventType].KEYPRESS
    /// - [`EventType`][crate::EventType].USR0
    /// - [`EventType`][crate::EventType].USR1
    /// - [`EventType`][crate::EventType].USR2
    /// - [`EventType`][crate::EventType].USR3
    /// - [`EventType`][crate::EventType].USR4
    /// - [`EventType`][crate::EventType].USR5
    /// - [`EventType`][crate::EventType].USR6
    /// - [`EventType`][crate::EventType].USR7
    /// - [`EventType`][crate::EventType].USR8
    /// - [`EventType`][crate::EventType].USR9
    ///
    /// # Returns
    ///
    /// [`true`] when the overall operation finishes successfully, else [`false`].
    ///
    /// ## `data`
    /// The note data of event.
    #[doc(alias = "alsaseq_event_get_note_data")]
    #[doc(alias = "get_note_data")]
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

    /// Get the quadlet data of event, available when [`event_type()`][Self::event_type()] results in one of:
    ///
    /// - [`EventType`][crate::EventType].USR0
    /// - [`EventType`][crate::EventType].USR1
    /// - [`EventType`][crate::EventType].USR2
    /// - [`EventType`][crate::EventType].USR3
    /// - [`EventType`][crate::EventType].USR4
    /// - [`EventType`][crate::EventType].USR5
    /// - [`EventType`][crate::EventType].USR6
    /// - [`EventType`][crate::EventType].USR7
    /// - [`EventType`][crate::EventType].USR8
    /// - [`EventType`][crate::EventType].USR9
    ///
    /// # Returns
    ///
    /// [`true`] when the overall operation finishes successfully, else [`false`].
    ///
    /// ## `data`
    /// The quadlet data of event.
    #[doc(alias = "alsaseq_event_get_quadlet_data")]
    #[doc(alias = "get_quadlet_data")]
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

    /// Get the queue data of event, available when [`event_type()`][Self::event_type()] results in one of:
    ///
    /// - [`EventType`][crate::EventType].START
    /// - [`EventType`][crate::EventType].CONTINUE
    /// - [`EventType`][crate::EventType].STOP
    /// - [`EventType`][crate::EventType].SETPOS_TICK
    /// - [`EventType`][crate::EventType].SETPOS_TIME
    /// - [`EventType`][crate::EventType].TEMPO
    /// - [`EventType`][crate::EventType].CLOCK
    /// - [`EventType`][crate::EventType].TICK
    /// - [`EventType`][crate::EventType].QUEUE_SKEW
    /// - [`EventType`][crate::EventType].USR0
    /// - [`EventType`][crate::EventType].USR1
    /// - [`EventType`][crate::EventType].USR2
    /// - [`EventType`][crate::EventType].USR3
    /// - [`EventType`][crate::EventType].USR4
    /// - [`EventType`][crate::EventType].USR5
    /// - [`EventType`][crate::EventType].USR6
    /// - [`EventType`][crate::EventType].USR7
    /// - [`EventType`][crate::EventType].USR8
    /// - [`EventType`][crate::EventType].USR9
    ///
    /// # Returns
    ///
    /// [`true`] when the overall operation finishes successfully, else [`false`].
    ///
    /// ## `data`
    /// The queue data of event.
    #[doc(alias = "alsaseq_event_get_queue_data")]
    #[doc(alias = "get_queue_data")]
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

    /// Copy the real time to the event, available only when [`EventTstampMode`][crate::EventTstampMode].REAL is retrieved by
    /// [`tstamp_mode()`][Self::tstamp_mode()].
    ///
    /// # Returns
    ///
    /// [`true`] when the overall operation finishes successfully, else [`false`].
    ///
    /// ## `real_time`
    /// The real time of event.
    #[doc(alias = "alsaseq_event_get_real_time")]
    #[doc(alias = "get_real_time")]
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

    /// Get the real time data of event, available when [`tstamp_mode()`][Self::tstamp_mode()] is
    /// [`EventTstampMode`][crate::EventTstampMode].REAL and [`event_type()`][Self::event_type()] results in one of:
    ///
    /// - [`EventType`][crate::EventType].USR0
    /// - [`EventType`][crate::EventType].USR1
    /// - [`EventType`][crate::EventType].USR2
    /// - [`EventType`][crate::EventType].USR3
    /// - [`EventType`][crate::EventType].USR4
    /// - [`EventType`][crate::EventType].USR5
    /// - [`EventType`][crate::EventType].USR6
    /// - [`EventType`][crate::EventType].USR7
    /// - [`EventType`][crate::EventType].USR8
    /// - [`EventType`][crate::EventType].USR9
    ///
    /// # Returns
    ///
    /// [`true`] when the overall operation finishes successfully, else [`false`].
    ///
    /// ## `real_time`
    /// The real time data of event.
    #[doc(alias = "alsaseq_event_get_real_time_data")]
    #[doc(alias = "get_real_time_data")]
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

    /// Get the result data of event, available when [`event_type()`][Self::event_type()] results in one of:
    ///
    /// - [`EventType`][crate::EventType].SYSTEM
    /// - [`EventType`][crate::EventType].RESULT
    /// - [`EventType`][crate::EventType].USR0
    /// - [`EventType`][crate::EventType].USR1
    /// - [`EventType`][crate::EventType].USR2
    /// - [`EventType`][crate::EventType].USR3
    /// - [`EventType`][crate::EventType].USR4
    /// - [`EventType`][crate::EventType].USR5
    /// - [`EventType`][crate::EventType].USR6
    /// - [`EventType`][crate::EventType].USR7
    /// - [`EventType`][crate::EventType].USR8
    /// - [`EventType`][crate::EventType].USR9
    ///
    /// # Returns
    ///
    /// [`true`] when the overall operation finishes successfully, else [`false`].
    ///
    /// ## `data`
    /// The result data of event.
    #[doc(alias = "alsaseq_event_get_result_data")]
    #[doc(alias = "get_result_data")]
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

    /// Refer to the source address of event.
    ///
    /// # Returns
    ///
    ///
    /// ## `addr`
    /// A [`Addr`][crate::Addr] for source address.
    #[doc(alias = "alsaseq_event_get_source")]
    #[doc(alias = "get_source")]
    pub fn source(&self) -> Addr {
        unsafe {
            let mut addr = std::ptr::null_mut() as *const ffi::ALSASeqAddr;
            ffi::alsaseq_event_get_source(self.to_glib_none().0, &mut addr);
            from_glib_none(addr)
        }
    }

    /// Get the tick time of event, available only when [`EventTstampMode`][crate::EventTstampMode].TICK is retrieved by
    /// [`tstamp_mode()`][Self::tstamp_mode()].
    ///
    /// # Returns
    ///
    /// [`true`] when the overall operation finishes successfully, else [`false`].
    ///
    /// ## `tick_time`
    /// The tick time of event.
    #[doc(alias = "alsaseq_event_get_tick_time")]
    #[doc(alias = "get_tick_time")]
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

    /// Get the tick time data of event, available when [`tstamp_mode()`][Self::tstamp_mode()] is
    /// [`EventTstampMode`][crate::EventTstampMode].TICK and [`event_type()`][Self::event_type()] results in one of:
    ///
    /// - [`EventType`][crate::EventType].USR0
    /// - [`EventType`][crate::EventType].USR1
    /// - [`EventType`][crate::EventType].USR2
    /// - [`EventType`][crate::EventType].USR3
    /// - [`EventType`][crate::EventType].USR4
    /// - [`EventType`][crate::EventType].USR5
    /// - [`EventType`][crate::EventType].USR6
    /// - [`EventType`][crate::EventType].USR7
    /// - [`EventType`][crate::EventType].USR8
    /// - [`EventType`][crate::EventType].USR9
    ///
    /// # Returns
    ///
    /// [`true`] when the overall operation finishes successfully, else [`false`].
    ///
    /// ## `tick_time`
    /// The tick time data of event.
    #[doc(alias = "alsaseq_event_get_tick_time_data")]
    #[doc(alias = "get_tick_time_data")]
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

    /// Copy the byte data, available when [`event_type()`][Self::event_type()] results in one of:
    ///
    /// - [`EventType`][crate::EventType].USR0
    /// - [`EventType`][crate::EventType].USR1
    /// - [`EventType`][crate::EventType].USR2
    /// - [`EventType`][crate::EventType].USR3
    /// - [`EventType`][crate::EventType].USR4
    /// - [`EventType`][crate::EventType].USR5
    /// - [`EventType`][crate::EventType].USR6
    /// - [`EventType`][crate::EventType].USR7
    /// - [`EventType`][crate::EventType].USR8
    /// - [`EventType`][crate::EventType].USR9
    /// ## `data`
    /// The byte data of event.
    ///
    /// # Returns
    ///
    /// [`true`] when the overall operation finishes successfully, else [`false`].
    #[doc(alias = "alsaseq_event_set_byte_data")]
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

    /// Copy the quadlet data to the event, available when [`event_type()`][Self::event_type()] results in one of:
    ///
    /// - [`EventType`][crate::EventType].USR0
    /// - [`EventType`][crate::EventType].USR1
    /// - [`EventType`][crate::EventType].USR2
    /// - [`EventType`][crate::EventType].USR3
    /// - [`EventType`][crate::EventType].USR4
    /// - [`EventType`][crate::EventType].USR5
    /// - [`EventType`][crate::EventType].USR6
    /// - [`EventType`][crate::EventType].USR7
    /// - [`EventType`][crate::EventType].USR8
    /// - [`EventType`][crate::EventType].USR9
    /// ## `data`
    /// The quadlet data of event.
    ///
    /// # Returns
    ///
    /// [`true`] when the overall operation finishes successfully, else [`false`].
    #[doc(alias = "alsaseq_event_set_quadlet_data")]
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

    /// Copy the real time to the event and set [`EventTstampMode`][crate::EventTstampMode].REAL.
    /// ## `real_time`
    /// The real time of event.
    ///
    /// # Returns
    ///
    /// [`true`] when the overall operation finishes successfully, else [`false`].
    #[doc(alias = "alsaseq_event_set_real_time")]
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

    /// Copy the real time data to the event, available [`tstamp_mode()`][Self::tstamp_mode()] is
    /// [`EventTstampMode`][crate::EventTstampMode].REAL and [`event_type()`][Self::event_type()] results in one of:
    ///
    /// - [`EventType`][crate::EventType].USR0
    /// - [`EventType`][crate::EventType].USR1
    /// - [`EventType`][crate::EventType].USR2
    /// - [`EventType`][crate::EventType].USR3
    /// - [`EventType`][crate::EventType].USR4
    /// - [`EventType`][crate::EventType].USR5
    /// - [`EventType`][crate::EventType].USR6
    /// - [`EventType`][crate::EventType].USR7
    /// - [`EventType`][crate::EventType].USR8
    /// - [`EventType`][crate::EventType].USR9
    /// ## `real_time`
    /// The real time data of event.
    ///
    /// # Returns
    ///
    /// [`true`] when the overall operation finishes successfully, else [`false`].
    #[doc(alias = "alsaseq_event_set_real_time_data")]
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
