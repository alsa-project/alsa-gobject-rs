// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{ffi, ClientInfo, ClientPool, PortInfo, QueueInfo, SystemInfo};
use glib::translate::*;

/// Get the list of clients as the numeric identifier.
///
/// The call of function executes `open(2)``, ``close(2)``, and ``ioctl(2)`` system calls with
/// `SNDRV_SEQ_IOCTL_CLIENT_ID`, `SNDRV_SEQ_IOCTL_SYSTEM_INFO`, and
/// `SNDRV_SEQ_IOCTL_QUERY_NEXT_CLIENT` command for ALSA sequencer character device.
///
/// # Returns
///
/// [`true`] when the overall operation finishes successfully, else [`false`].
///
/// ## `entries`
/// The array with elements for numeric identified of
///           client. One of [`SpecificClientId`][crate::SpecificClientId] can be included in result as well as any
///           numeric value.
#[doc(alias = "alsaseq_get_client_id_list")]
#[doc(alias = "get_client_id_list")]
pub fn client_id_list() -> Result<Vec<u8>, glib::Error> {
    unsafe {
        let mut entries = std::ptr::null_mut();
        let mut entry_count = std::mem::MaybeUninit::uninit();
        let mut error = std::ptr::null_mut();
        let is_ok =
            ffi::alsaseq_get_client_id_list(&mut entries, entry_count.as_mut_ptr(), &mut error);
        debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
        if error.is_null() {
            Ok(FromGlibContainer::from_glib_full_num(
                entries,
                entry_count.assume_init() as _,
            ))
        } else {
            Err(from_glib_full(error))
        }
    }
}

/// Get the information of client according to the numeric ID.
///
/// The call of function executes `open(2)``, ``close(2)``, and ``ioctl(2)`` system calls with
/// `SNDRV_SEQ_IOCTL_GET_CLIENT_INFO` command for ALSA sequencer character device.
/// ## `client_id`
/// The numeric identifier of client to query. One of [`SpecificClientId`][crate::SpecificClientId] is
///             available as well as any numeric value.
///
/// # Returns
///
/// [`true`] when the overall operation finishes successfully, else [`false`].
///
/// ## `client_info`
/// A [`ClientInfo`][crate::ClientInfo] for the client.
#[doc(alias = "alsaseq_get_client_info")]
#[doc(alias = "get_client_info")]
pub fn client_info(client_id: u8) -> Result<ClientInfo, glib::Error> {
    unsafe {
        let mut client_info = std::ptr::null_mut();
        let mut error = std::ptr::null_mut();
        let is_ok = ffi::alsaseq_get_client_info(client_id, &mut client_info, &mut error);
        debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
        if error.is_null() {
            Ok(from_glib_full(client_info))
        } else {
            Err(from_glib_full(error))
        }
    }
}

/// Get statistical information of memory pool for the given client.
///
/// The call of function executes `open(2)`, `close(2)`, and `ioctl(2)` system calls with
/// `SNDRV_SEQ_IOCTL_GET_CLIENT_POOL` command for ALSA sequencer character device.
/// ## `client_id`
/// The numeric ID of client to query. One of [`SpecificClientId`][crate::SpecificClientId] is available as
///             well as any numeric value.
///
/// # Returns
///
/// [`true`] when the overall operation finishes successfully, else [`false`].
///
/// ## `client_pool`
/// The information of memory pool for the client.
#[doc(alias = "alsaseq_get_client_pool")]
#[doc(alias = "get_client_pool")]
pub fn client_pool(client_id: u8) -> Result<ClientPool, glib::Error> {
    unsafe {
        let mut client_pool = std::ptr::null_mut();
        let mut error = std::ptr::null_mut();
        let is_ok = ffi::alsaseq_get_client_pool(client_id, &mut client_pool, &mut error);
        debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
        if error.is_null() {
            Ok(from_glib_full(client_pool))
        } else {
            Err(from_glib_full(error))
        }
    }
}

/// Get the list of numeric identifiers for port added by the client.
///
/// The call of function executes `open(2)``, ``close(2)``, and ``ioctl(2)`` system calls with
/// `SNDRV_SEQ_IOCTL_GET_CLIENT_INFO` and `SNDRV_SEQ_IOCTL_QUERY_NEXT_PORT` commands for ALSA
/// sequencer character device.
/// ## `client_id`
/// The numeric ID of client to query. One of [`SpecificClientId`][crate::SpecificClientId] is available as
///             well as any numeric value.
///
/// # Returns
///
///
/// ## `entries`
/// The array with elements for numeric identifier of
///           port. One of [`SpecificPortId`][crate::SpecificPortId] is available as well as any numeric value.
#[doc(alias = "alsaseq_get_port_id_list")]
#[doc(alias = "get_port_id_list")]
pub fn port_id_list(client_id: u8) -> Result<Vec<u8>, glib::Error> {
    unsafe {
        let mut entries = std::ptr::null_mut();
        let mut entry_count = std::mem::MaybeUninit::uninit();
        let mut error = std::ptr::null_mut();
        let is_ok = ffi::alsaseq_get_port_id_list(
            client_id,
            &mut entries,
            entry_count.as_mut_ptr(),
            &mut error,
        );
        debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
        if error.is_null() {
            Ok(FromGlibContainer::from_glib_full_num(
                entries,
                entry_count.assume_init() as _,
            ))
        } else {
            Err(from_glib_full(error))
        }
    }
}

/// Get the information of port in client.
///
/// The call of function executes `open(2)`, `close(2)`, and `ioctl(2)` system calls with
/// `SNDRV_SEQ_IOCTL_GET_PORT_INFO` command for ALSA sequencer character device.
/// ## `client_id`
/// The numeric identifier of client to query. One of [`SpecificClientId`][crate::SpecificClientId] is
///             available as well as any numerica value.
/// ## `port_id`
/// The numeric identifier of port in the client. One of [`SpecificPortId`][crate::SpecificPortId] is
///           available as well as any numeric value.
///
/// # Returns
///
/// [`true`] when the overall operation finishes successfully, else [`false`].
///
/// ## `port_info`
/// A [`PortInfo`][crate::PortInfo] for the port.
#[doc(alias = "alsaseq_get_port_info")]
#[doc(alias = "get_port_info")]
pub fn port_info(client_id: u8, port_id: u8) -> Result<PortInfo, glib::Error> {
    unsafe {
        let mut port_info = std::ptr::null_mut();
        let mut error = std::ptr::null_mut();
        let is_ok = ffi::alsaseq_get_port_info(client_id, port_id, &mut port_info, &mut error);
        debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
        if error.is_null() {
            Ok(from_glib_full(port_info))
        } else {
            Err(from_glib_full(error))
        }
    }
}

/// Get the list of queue in ALSA Sequencer.
///
/// The call of function executes `open(2)`, `close(2)`, and `ioctl(2)` system calls with
/// `SNDRV_SEQ_IOCTL_SYSTEM_INFO` and `SNDRV_SEQ_IOCTL_GET_QUEUE_INFO` commands for ALSA
/// sequencer character device.
///
/// # Returns
///
/// [`true`] when the overall operation finishes successfully, else [`false`].
///
/// ## `entries`
/// The array of elements for numeric identifier of queue.
#[doc(alias = "alsaseq_get_queue_id_list")]
#[doc(alias = "get_queue_id_list")]
pub fn queue_id_list() -> Result<Vec<u8>, glib::Error> {
    unsafe {
        let mut entries = std::ptr::null_mut();
        let mut entry_count = std::mem::MaybeUninit::uninit();
        let mut error = std::ptr::null_mut();
        let is_ok =
            ffi::alsaseq_get_queue_id_list(&mut entries, entry_count.as_mut_ptr(), &mut error);
        debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
        if error.is_null() {
            Ok(FromGlibContainer::from_glib_full_num(
                entries,
                entry_count.assume_init() as _,
            ))
        } else {
            Err(from_glib_full(error))
        }
    }
}

/// Get the information of queue, according to the numeric ID.
///
/// The call of function executes `open(2)`, `close(2)`, and `ioctl(2)` system calls with
/// `SNDRV_SEQ_IOCTL_GET_QUEUE_INFO` command for ALSA sequencer character device.
/// ## `queue_id`
/// The numeric ID of queue. One of [`SpecificQueueId`][crate::SpecificQueueId] is available as well.
///
/// # Returns
///
/// [`true`] when the overall operation finishes successfully, else [`false`].
///
/// ## `queue_info`
/// The information of queue.
#[doc(alias = "alsaseq_get_queue_info_by_id")]
#[doc(alias = "get_queue_info_by_id")]
pub fn queue_info_by_id(queue_id: u8) -> Result<QueueInfo, glib::Error> {
    unsafe {
        let mut queue_info = std::ptr::null_mut();
        let mut error = std::ptr::null_mut();
        let is_ok = ffi::alsaseq_get_queue_info_by_id(queue_id, &mut queue_info, &mut error);
        debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
        if error.is_null() {
            Ok(from_glib_full(queue_info))
        } else {
            Err(from_glib_full(error))
        }
    }
}

/// Get the information of queue, according to the name string.
///
/// The call of function executes `open(2)`, `close(2)`, and `ioctl(2)` system calls with
/// `SNDRV_SEQ_IOCTL_GET_NAMED_QUEUE` command for ALSA sequencer character device.
/// ## `name`
/// The name string of queue to query.
///
/// # Returns
///
/// [`true`] when the overall operation finishes successfully, else [`false`].
///
/// ## `queue_info`
/// The information of queue.
#[doc(alias = "alsaseq_get_queue_info_by_name")]
#[doc(alias = "get_queue_info_by_name")]
pub fn queue_info_by_name(name: &str) -> Result<QueueInfo, glib::Error> {
    unsafe {
        let mut queue_info = std::ptr::null_mut();
        let mut error = std::ptr::null_mut();
        let is_ok =
            ffi::alsaseq_get_queue_info_by_name(name.to_glib_none().0, &mut queue_info, &mut error);
        debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
        if error.is_null() {
            Ok(from_glib_full(queue_info))
        } else {
            Err(from_glib_full(error))
        }
    }
}

/// Allocate devnode string for ALSA Sequencer and return it when exists.
///
/// Nodes under sound subsystem in sysfs are used to gather the information.
///
/// # Returns
///
/// [`true`] when the overall operation finishes successfully, else [`false`].
///
/// ## `devnode`
/// The devnode of ALSA Sequencer.
#[doc(alias = "alsaseq_get_seq_devnode")]
#[doc(alias = "get_seq_devnode")]
pub fn seq_devnode() -> Result<glib::GString, glib::Error> {
    unsafe {
        let mut devnode = std::ptr::null_mut();
        let mut error = std::ptr::null_mut();
        let is_ok = ffi::alsaseq_get_seq_devnode(&mut devnode, &mut error);
        debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
        if error.is_null() {
            Ok(from_glib_full(devnode))
        } else {
            Err(from_glib_full(error))
        }
    }
}

/// Allocate sysname string for ALSA sequencer and return it when exists.
///
/// Nodes under sound subsystem in sysfs are used to gather the information.
///
/// # Returns
///
/// [`true`] when the overall operation finishes successfully, else [`false`].
///
/// ## `sysname`
/// The sysname of ALSA Sequencer.
#[doc(alias = "alsaseq_get_seq_sysname")]
#[doc(alias = "get_seq_sysname")]
pub fn seq_sysname() -> Result<glib::GString, glib::Error> {
    unsafe {
        let mut sysname = std::ptr::null_mut();
        let mut error = std::ptr::null_mut();
        let is_ok = ffi::alsaseq_get_seq_sysname(&mut sysname, &mut error);
        debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
        if error.is_null() {
            Ok(from_glib_full(sysname))
        } else {
            Err(from_glib_full(error))
        }
    }
}

/// Get information of ALSA Sequencer.
///
/// The call of function executes `open(2)``, ``close(2)``, and ``ioctl(2)`` system calls with
/// `SNDRV_SEQ_IOCTL_SYSTEM_INFO` command for ALSA sequencer character device.
///
/// # Returns
///
/// [`true`] when the overall operation finishes successfully, else [`false`].
///
/// ## `system_info`
/// The information of ALSA Sequencer.
#[doc(alias = "alsaseq_get_system_info")]
#[doc(alias = "get_system_info")]
pub fn system_info() -> Result<SystemInfo, glib::Error> {
    unsafe {
        let mut system_info = std::ptr::null_mut();
        let mut error = std::ptr::null_mut();
        let is_ok = ffi::alsaseq_get_system_info(&mut system_info, &mut error);
        debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
        if error.is_null() {
            Ok(from_glib_full(system_info))
        } else {
            Err(from_glib_full(error))
        }
    }
}
