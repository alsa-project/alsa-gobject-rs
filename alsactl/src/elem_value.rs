// SPDX-License-Identifier: MIT
use super::*;

pub trait ElemValueExtManual {
    // NOTE: conversion between gboolean(=i32) and bool(=uchar in most ABIs). Read:
    // https://gitlab.gnome.org/GNOME/gobject-introspection/-/issues/392
    fn set_bool(&self, values: &[bool]);
    fn get_bool(&self) -> Vec<bool>;

    fn get_bytes(&self) -> &[u8];

    fn get_int(&self) -> &[i32];

    fn get_enum(&self) -> &[u32];

    fn get_int64(&self) -> &[i64];

    fn get_iec60958_channel_status(&self) -> &[u8];
    fn get_iec60958_user_data(&self) -> &[u8];
}

impl<O: IsA<ElemValue>> ElemValueExtManual for O {
    fn set_bool(&self, values: &[bool]) {
        let entries: Vec<glib_sys::gboolean> = values.iter().map(|&val| val.to_glib()).collect();

        unsafe {
            alsactl_sys::alsactl_elem_value_set_bool(
                self.as_ref().to_glib_none().0,
                entries.as_ptr(),
                entries.len(),
            );
        }
    }

    fn get_bool(&self) -> Vec<bool> {
        let mut data = std::ptr::null_mut() as *const [glib_sys::gboolean; 128];

        unsafe {
            alsactl_sys::alsactl_elem_value_get_bool(self.as_ref().to_glib_none().0, &mut data);

            (*data).iter().map(|&val| from_glib(val)).collect()
        }
    }

    fn get_bytes(&self) -> &[u8] {
        unsafe {
            let mut data = std::ptr::null_mut() as *const [u8; 512];

            alsactl_sys::alsactl_elem_value_get_bytes(self.as_ref().to_glib_none().0, &mut data);

            &*data
        }
    }

    fn get_int(&self) -> &[i32] {
        unsafe {
            let mut data = std::ptr::null_mut() as *const [i32; 128];

            alsactl_sys::alsactl_elem_value_get_int(self.as_ref().to_glib_none().0, &mut data);

            &*data
        }
    }

    fn get_enum(&self) -> &[u32] {
        unsafe {
            let mut data = std::ptr::null_mut() as *const [u32; 128];

            alsactl_sys::alsactl_elem_value_get_enum(self.as_ref().to_glib_none().0, &mut data);

            &*data
        }
    }

    fn get_int64(&self) -> &[i64] {
        unsafe {
            let mut data = std::ptr::null_mut() as *const [i64; 64];

            alsactl_sys::alsactl_elem_value_get_int64(self.as_ref().to_glib_none().0, &mut data);

            &*data
        }
    }

    fn get_iec60958_channel_status(&self) -> &[u8] {
        unsafe {
            let mut data = std::ptr::null_mut() as *const [u8; 24];

            alsactl_sys::alsactl_elem_value_get_iec60958_channel_status(
                self.as_ref().to_glib_none().0,
                &mut data,
            );

            &*data
        }
    }

    fn get_iec60958_user_data(&self) -> &[u8] {
        unsafe {
            let mut data = std::ptr::null_mut() as *const [u8; 147];

            alsactl_sys::alsactl_elem_value_get_iec60958_user_data(
                self.as_ref().to_glib_none().0,
                &mut data,
            );

            &*data
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{prelude::*, *};

    #[test]
    fn test_manual_bindings() {
        let val = ElemValue::new();

        let bool_expected = [false, true, false, true, true, false];
        val.set_bool(&bool_expected);
        assert_eq!(bool_expected, val.get_bool()[..bool_expected.len()]);

        let bytes_expected = [5, 4, 0, 2, 1, 8, 19, 21, 128, 212, 192];
        val.set_bytes(&bytes_expected);
        assert_eq!(bytes_expected, val.get_bytes()[..bytes_expected.len()]);

        let int_expected = [5, -4, 0, 2, 1, 8, 19, 21, -128, 212, -192000];
        val.set_int(&int_expected);
        assert_eq!(int_expected, val.get_int()[..int_expected.len()]);

        let enum_expected = [5, 4, 0, 2, 1, 8, 19, 21, 128, 212, 192];
        val.set_enum(&enum_expected);
        assert_eq!(enum_expected, val.get_enum()[..enum_expected.len()]);

        let int64_expected = [5, 4, 0, 2, 1, 8, -1938754, 21, 128, 212, -92854];
        val.set_int64(&int64_expected);
        assert_eq!(int64_expected, val.get_int64()[..int64_expected.len()]);

        let status_expected = [9, 7, 4, 124, 67];
        val.set_iec60958_channel_status(&status_expected);
        assert_eq!(
            status_expected,
            val.get_iec60958_channel_status()[..status_expected.len()]
        );

        let data_expected = [31, 211, 198, 90, 28, 8, 49];
        val.set_iec60958_user_data(&data_expected);
        assert_eq!(
            data_expected,
            val.get_iec60958_user_data()[..data_expected.len()]
        );
    }
}
