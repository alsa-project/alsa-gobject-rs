// SPDX-License-Identifier: MIT
use super::*;

/// Get the list of subscription for given address and query type.
///
/// The call of function executes `open(2)`, `close(2)`, and `ioctl(2)` system calls with
/// `SNDRV_SEQ_IOCTL_QUERY_SUBS` command for ALSA sequencer character device.
/// ## `addr`
/// A [`Addr`][crate::Addr] to query.
/// ## `query_type`
/// The type of query, one of [`QuerySubscribeType`][crate::QuerySubscribeType].
///
/// # Returns
///
/// [`true`] when the overall operation finishes successfully, else [`false`].
///
/// ## `entries`
/// The array with element for subscription
///           data.
#[doc(alias = "alsaseq_get_subscription_list")]
#[doc(alias = "get_subscription_list")]
pub fn subscription_list(
    addr: &Addr,
    query_type: QuerySubscribeType,
) -> Result<Vec<SubscribeData>, glib::Error> {
    unsafe {
        let mut entries = std::ptr::null_mut();
        let mut error = std::ptr::null_mut();

        ffi::alsaseq_get_subscription_list(
            addr.to_glib_none().0,
            query_type.into_glib(),
            &mut entries,
            &mut error,
        );

        if error.is_null() {
            Ok(FromGlibPtrContainer::from_glib_full(entries))
        } else {
            Err(from_glib_full(error))
        }
    }
}

/// Get current status of queue.
///
/// The call of function executes `open(2)`, `close(2)`, and `ioctl(2)` system calls with
/// `SNDRV_SEQ_IOCTL_GET_QUEUE_STATUS` command for ALSA sequencer character device.
/// ## `queue_id`
/// The numeric ID of queue. One of [`SpecificQueueId`][crate::SpecificQueueId] is available as well.
/// ## `queue_status`
/// The current status of queue.
///
/// # Returns
///
/// [`true`] when the overall operation finishes successfully, else [`false`].
#[doc(alias = "alsaseq_get_queue_status")]
#[doc(alias = "get_queue_status")]
pub fn queue_status<P: IsA<QueueStatus>>(
    queue_id: u8,
    queue_status: &mut P,
) -> Result<(), glib::Error> {
    unsafe {
        let mut error = std::ptr::null_mut();

        ffi::alsaseq_get_queue_status(
            queue_id,
            &mut queue_status.as_ref().to_glib_none().0,
            &mut error,
        );

        if error.is_null() {
            Ok(())
        } else {
            Err(from_glib_full(error))
        }
    }
}
