// SPDX-License-Identifier: MIT
use super::*;

pub trait RemoveFilterExtManual: 'static {
    fn get_property_tag(&self) -> i8;
    fn set_property_tag(&self, tag: i8);

    fn get_real_time(&self) -> &[u32];
    fn set_real_time(&self, real_time: &[u32; 2]);
}

impl<O: IsA<RemoveFilter>> RemoveFilterExtManual for O {
    fn get_property_tag(&self) -> i8 {
        unsafe {
            let mut value = Value::from_type(<i8 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"tag\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `tag` getter")
                .unwrap()
        }
    }

    fn set_property_tag(&self, tag: i8) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"tag\0".as_ptr() as *const _,
                Value::from(&tag).to_glib_none().0,
            );
        }
    }

    fn get_real_time(&self) -> &[u32] {
        unsafe {
            let mut data = std::ptr::null_mut() as *const [u32; 2];
            let _ = alsaseq_sys::alsaseq_remove_filter_get_real_time(
                self.as_ref().to_glib_none().0,
                &mut data,
            );
            &*data
        }
    }

    fn set_real_time(&self, real_time: &[u32; 2]) {
        unsafe {
            let _ = alsaseq_sys::alsaseq_remove_filter_set_real_time(
                self.as_ref().to_glib_none().0,
                real_time,
            );
        }
    }
}
