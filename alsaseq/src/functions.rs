// SPDX-License-Identifier: MIT
use super::*;

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
