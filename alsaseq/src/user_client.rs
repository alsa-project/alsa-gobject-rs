// SPDX-License-Identifier: MIT
use super::*;

pub trait UserClientExtManual {
    fn protocol_version(&self) -> Result<&[u16; 3], Error>;

    fn create_port<P: IsA<PortInfo>>(&self, port_info: &mut P) -> Result<(), Error>;

    fn create_port_at<P: IsA<PortInfo>>(&self, port_info: &mut P, port_id: u8)
        -> Result<(), Error>;

    fn create_queue<P: IsA<QueueInfo>>(&self, queue_info: &mut P) -> Result<(), Error>;

    fn info<P: IsA<ClientInfo>>(&self, client_info: &mut P) -> Result<(), Error>;

    fn pool<P: IsA<ClientPool>>(&self, client_pool: &mut P) -> Result<(), Error>;

    fn queue_timer(&self, queue_id: u8) -> Result<QueueTimer, Error>;

    fn set_queue_timer(&self, queue_id: u8, queue_timer: &QueueTimer) -> Result<(), Error>;

    fn schedule_events(&self, events: &[Event]) -> Result<usize, Error>;
}

impl<O: IsA<UserClient>> UserClientExtManual for O {
    fn protocol_version(&self) -> Result<&[u16; 3], Error> {
        unsafe {
            let mut triplet = std::ptr::null_mut() as *const [u16; 3];
            let mut error = std::ptr::null_mut();

            let _ = ffi::alsaseq_user_client_get_protocol_version(
                self.as_ref().to_glib_none().0,
                &mut triplet as *mut *const [u16; 3],
                &mut error,
            );

            if error.is_null() {
                Ok(&*triplet)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn create_port<P: IsA<PortInfo>>(&self, port_info: &mut P) -> Result<(), Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let _ = ffi::alsaseq_user_client_create_port(
                self.as_ref().to_glib_none().0,
                &mut port_info.as_ref().to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn create_port_at<P: IsA<PortInfo>>(
        &self,
        port_info: &mut P,
        port_id: u8,
    ) -> Result<(), Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let _ = ffi::alsaseq_user_client_create_port_at(
                self.as_ref().to_glib_none().0,
                &mut port_info.as_ref().to_glib_none().0,
                port_id,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn create_queue<P: IsA<QueueInfo>>(&self, queue_info: &mut P) -> Result<(), Error> {
        unsafe {
            let mut error = std::ptr::null_mut();

            let _ = ffi::alsaseq_user_client_create_queue(
                self.as_ref().to_glib_none().0,
                &mut queue_info.as_ref().to_glib_none().0,
                &mut error,
            );

            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn info<P: IsA<ClientInfo>>(&self, client_info: &mut P) -> Result<(), Error> {
        unsafe {
            let mut error = std::ptr::null_mut();

            let _ = ffi::alsaseq_user_client_get_info(
                self.as_ref().to_glib_none().0,
                &mut client_info.as_ref().to_glib_none().0,
                &mut error,
            );

            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn pool<P: IsA<ClientPool>>(&self, client_pool: &mut P) -> Result<(), Error> {
        unsafe {
            let mut error = std::ptr::null_mut();

            let _ = ffi::alsaseq_user_client_get_pool(
                self.as_ref().to_glib_none().0,
                &mut client_pool.as_ref().to_glib_none().0,
                &mut error,
            );

            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn queue_timer(&self, queue_id: u8) -> Result<QueueTimer, Error> {
        unsafe {
            let mut queue_timer = std::ptr::null_mut();
            let mut error = std::ptr::null_mut();
            let _ = ffi::alsaseq_user_client_get_queue_timer(
                self.as_ref().to_glib_none().0,
                queue_id,
                &mut queue_timer,
                &mut error,
            );
            if error.is_null() {
                let obj = QueueTimerCommon::from_glib_full(queue_timer);
                let queue_timer = match obj.timer_type() {
                    QueueTimerType::Alsa => {
                        let timer = obj.downcast::<QueueTimerAlsa>().unwrap();
                        QueueTimer::Alsa(timer)
                    }
                    _ => unreachable!(),
                };
                Ok(queue_timer)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn set_queue_timer(&self, queue_id: u8, queue_timer: &QueueTimer) -> Result<(), Error> {
        let inst = match &queue_timer {
            QueueTimer::Alsa(inst) => inst.upcast_ref::<QueueTimerCommon>(),
        };

        unsafe {
            let mut error = std::ptr::null_mut();
            let _ = ffi::alsaseq_user_client_set_queue_timer(
                self.as_ref().to_glib_none().0,
                queue_id,
                inst.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    // MEMO: the issue #42 in gtk-rs-core affects the conversion to `*const GList`. See:
    // https://github.com/gtk-rs/gtk-rs-core/issues/42.
    fn schedule_events(&self, events: &[Event]) -> Result<usize, Error> {
        unsafe {
            let mut entries: *mut glib::ffi::GList = std::ptr::null_mut();
            let mut count = std::mem::MaybeUninit::uninit();
            let mut error = std::ptr::null_mut();

            events.iter().for_each(|event| {
                // MEMO: This is the cause of error. The g_list_append() requires mutual pointer
                // (= `glib::ffi::gpointer` = `*mut libc::c_void`) while each entry should be
                // immutable in my case... As workaround, annotate the pointer as mutual in safe
                // scope. I guarantee that alsaseq_user_client_schedule_events() handles element
                // of list as immutable.
                let ptr = event.to_glib_none().0 as glib::ffi::gpointer;
                entries = glib::ffi::g_list_append(entries, ptr);
            });

            let is_ok = ffi::alsaseq_user_client_schedule_events(
                self.as_ref().to_glib_none().0,
                entries,
                count.as_mut_ptr(),
                &mut error,
            );
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(count.assume_init())
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}
