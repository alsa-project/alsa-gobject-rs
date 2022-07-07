// SPDX-License-Identifier: MIT
use super::*;

/// Trait containing the rest of [`struct@RemoveFilter`] methods.
pub trait RemoveFilterExtManual: 'static {
    /// The tag of event as filter condition. This is evaluated with
    /// [`RemoveFilterFlag`][crate::RemoveFilterFlag].TAG_MATCH at call of
    /// [`UserClientExt::remove_events()`][crate::prelude::UserClientExt::remove_events()].
    fn tag(&self) -> i8;

    /// The tag of event as filter condition. This is evaluated with
    /// [`RemoveFilterFlag`][crate::RemoveFilterFlag].TAG_MATCH at call of
    /// [`UserClientExt::remove_events()`][crate::prelude::UserClientExt::remove_events()].
    fn set_tag(&self, tag: i8);

    /// Refer to doublet of real time in internal storage. The call works expectedly as long as
    /// `property::RemoveFilter::flags` doesn't contain [`RemoveFilterFlag`][crate::RemoveFilterFlag].TICK. This is evaluated
    /// with [`RemoveFilterFlag`][crate::RemoveFilterFlag].TIME_BEFORE and [`RemoveFilterFlag`][crate::RemoveFilterFlag].TIME_AFTER at call of
    /// [`UserClientExt::remove_events()`][crate::prelude::UserClientExt::remove_events()].
    ///
    /// # Returns
    ///
    ///
    /// ## `real_time`
    /// The real time data of event.
    #[doc(alias = "alsaseq_remove_filter_get_real_time")]
    #[doc(alias = "get_real_time")]
    fn real_time(&self) -> &[u32];

    /// Copy doublet of real time into internal storage. The call works expectedly as long as
    /// `property::RemoveFilter::flags` doesn't contain [`RemoveFilterFlag`][crate::RemoveFilterFlag].TICK. This is evaluated
    /// with [`RemoveFilterFlag`][crate::RemoveFilterFlag].TIME_BEFORE and [`RemoveFilterFlag`][crate::RemoveFilterFlag].TIME_AFTER at call of
    /// [`UserClientExt::remove_events()`][crate::prelude::UserClientExt::remove_events()].
    /// ## `real_time`
    /// The real time data of event.
    #[doc(alias = "alsaseq_remove_filter_set_real_time")]
    fn set_real_time(&self, real_time: &[u32; 2]);
}

impl<O: IsA<RemoveFilter>> RemoveFilterExtManual for O {
    fn tag(&self) -> i8 {
        unsafe {
            let mut value = Value::from_type(<i8 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"tag\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get::<i8>()
                .expect("Return Value for property `tag` getter")
        }
    }

    fn set_tag(&self, tag: i8) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"tag\0".as_ptr() as *const _,
                Value::from(&tag).to_glib_none().0,
            );
        }
    }

    fn real_time(&self) -> &[u32] {
        unsafe {
            let mut data = std::ptr::null_mut() as *const [u32; 2];
            let _ =
                ffi::alsaseq_remove_filter_get_real_time(self.as_ref().to_glib_none().0, &mut data);
            &*data
        }
    }

    fn set_real_time(&self, real_time: &[u32; 2]) {
        unsafe {
            let _ =
                ffi::alsaseq_remove_filter_set_real_time(self.as_ref().to_glib_none().0, real_time);
        }
    }
}
