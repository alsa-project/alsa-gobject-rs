// SPDX-License-Identifier: MIT
use super::*;

/// Trait containing the rest of [`struct@UserClient`] methods.
pub trait UserClientExtManual {
    /// Get the version of sequencer protocol currently used. The version is expressed as the array
    /// with three elements; major, minor, and micro version in the order. The length of major version
    /// is 16 bit, the length of minor and micro version is 8 bit each.
    ///
    /// # Returns
    ///
    /// [`true`] when the overall operation finishes successfully, else [`false`].
    ///
    /// ## `proto_ver_triplet`
    /// The version of protocol currently used.
    #[doc(alias = "alsaseq_user_client_get_protocol_version")]
    #[doc(alias = "get_protocol_version")]
    fn protocol_version(&self) -> Result<&[u16; 3], Error>;

    /// Create a port into the client.
    ///
    /// The call of function executes `ioctl(2)` system call with `SNDRV_SEQ_IOCTL_CREATE_PORT` command
    /// for ALSA sequencer character device.
    /// ## `port_info`
    /// A [`PortInfo`][crate::PortInfo].
    ///
    /// # Returns
    ///
    /// [`true`] when the overall operation finishes successfully, else [`false`].
    #[doc(alias = "alsaseq_user_client_create_port")]
    fn create_port<P: IsA<PortInfo>>(&self, port_info: &mut P) -> Result<(), Error>;

    /// Create a port into the client with the given numeric port ID.
    ///
    /// The call of function executes `ioctl(2)` system call with `SNDRV_SEQ_IOCTL_CREATE_PORT` command
    /// for ALSA sequencer character device.
    /// ## `port_info`
    /// A [`PortInfo`][crate::PortInfo].
    /// ## `port_id`
    /// The numeric ID of port to create.
    ///
    /// # Returns
    ///
    /// [`true`] when the overall operation finishes successfully, else [`false`].
    #[doc(alias = "alsaseq_user_client_create_port_at")]
    fn create_port_at<P: IsA<PortInfo>>(&self, port_info: &mut P, port_id: u8)
        -> Result<(), Error>;

    /// Create a new queue owned by the client. The content of information is updated if success.
    ///
    /// The call of function executes `ioctl(2)` system call with `SNDRV_SEQ_IOCTL_CREATE_QUEUE` command
    /// for ALSA sequencer character device.
    /// ## `queue_info`
    /// The information of queue to add.
    ///
    /// # Returns
    ///
    /// [`true`] when the overall operation finishes successfully, else [`false`].
    #[doc(alias = "alsaseq_user_client_create_queue")]
    fn create_queue<P: IsA<QueueInfo>>(&self, queue_info: &mut P) -> Result<(), Error>;

    /// Set client information.
    ///
    /// The call of function executes `ioctl(2)` system call with `SNDRV_SEQ_IOCTL_GET_CLIENT_INFO`
    /// command for ALSA sequencer character device.
    /// ## `client_info`
    /// A [`ClientInfo`][crate::ClientInfo].
    ///
    /// # Returns
    ///
    /// [`true`] when the overall operation finishes successfully, else [`false`].
    #[doc(alias = "alsaseq_user_client_get_info")]
    #[doc(alias = "get_info")]
    fn info<P: IsA<ClientInfo>>(&self, client_info: &mut P) -> Result<(), Error>;

    /// Get information of memory pool in the client.
    ///
    /// The call of function executes `ioctl(2)` system call with `SNDRV_SEQ_IOCTL_GET_CLIENT_POOL`
    /// command for ALSA sequencer character device.
    /// ## `client_pool`
    /// A [`ClientPool`][crate::ClientPool] to be configured for the client.
    ///
    /// # Returns
    ///
    /// [`true`] when the overall operation finishes successfully, else [`false`].
    #[doc(alias = "alsaseq_user_client_get_pool")]
    #[doc(alias = "get_pool")]
    fn pool<P: IsA<ClientPool>>(&self, client_pool: &mut P) -> Result<(), Error>;

    /// Get the data of timer for the queue.
    ///
    /// The call of function executes `ioctl(2)` system call with `SNDRV_SEQ_IOCTL_GET_QUEUE_TIMER`
    /// command for ALSA sequencer character device.
    /// ## `queue_id`
    /// The numeric ID of queue. An entry of [`SpecificQueueId`][crate::SpecificQueueId] is available as well.
    ///
    /// # Returns
    ///
    /// [`true`] when the overall operation finishes successfully, else [`false`].
    ///
    /// ## `queue_timer`
    /// The data of timer for queue, which implements [`QueueTimerCommon`][crate::QueueTimerCommon].
    #[doc(alias = "alsaseq_user_client_get_queue_timer")]
    #[doc(alias = "get_queue_timer")]
    fn queue_timer(&self, queue_id: u8) -> Result<QueueTimer, Error>;

    /// Set the data of timer for the queue.
    ///
    /// The call of function executes `ioctl(2)` system call with `SNDRV_SEQ_IOCTL_SET_QUEUE_TIMER`
    /// command for ALSA sequencer character device.
    /// ## `queue_id`
    /// The numeric ID of queue. An entry of [`SpecificQueueId`][crate::SpecificQueueId] is available as well.
    /// ## `queue_timer`
    /// The data of timer for queue, which implements [`QueueTimerCommon`][crate::QueueTimerCommon].
    ///
    /// # Returns
    ///
    /// [`true`] when the overall operation finishes successfully, else [`false`].
    #[doc(alias = "alsaseq_user_client_set_queue_timer")]
    fn set_queue_timer(&self, queue_id: u8, queue_timer: &QueueTimer) -> Result<(), Error>;

    /// Deliver the events immediately, or schedule it into memory pool of the client.
    ///
    /// The call of function executes `write(2)` system call for ALSA sequencer character device. When
    /// `property::ClientPool::output-free` is less than sum of [`Event::calculate_pool_consumption()`][crate::Event::calculate_pool_consumption()]
    /// and [`UserClientExt::open()`][crate::prelude::UserClientExt::open()] is called without non-blocking flag, the user process can be
    /// blocked untill enough number of cells becomes available.
    /// ## `events`
    /// The list of [`Event`][crate::Event].
    ///
    /// # Returns
    ///
    /// [`true`] when the overall operation finishes successfully, else [`false`].
    ///
    /// ## `count`
    /// The number of events to be scheduled.
    #[doc(alias = "alsaseq_user_client_schedule_events")]
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
