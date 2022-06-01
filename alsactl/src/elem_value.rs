// SPDX-License-Identifier: MIT
use crate::*;

pub trait ElemValueExtManual {
    fn set_bool(&self, values: &[bool]);
    fn get_bool(&self, values: &mut [bool]);

    fn get_bytes(&self, values: &mut [u8]);

    fn get_int(&self, values: &mut [i32]);

    fn get_enum(&self, values: &mut [u32]);

    fn get_int64(&self, values: &mut [i64]);

    fn get_iec60958_channel_status(&self, status: &mut [u8]);
    fn get_iec60958_user_data(&self, data: &mut [u8]);
}

impl<O: IsA<ElemValue>> ElemValueExtManual for O {
    // The type of 'gboolean' is alias to 'int'.
    fn set_bool(&self, values: &[bool]) {
        let value_count = values.len() as usize;
        let mut int_values = Vec::<i32>::with_capacity(value_count);

        for &val in values.into_iter() {
            int_values.push(val as i32);
        }

        unsafe {
            alsactl_sys::alsactl_elem_value_set_bool(
                self.as_ref().to_glib_none().0,
                int_values.as_ptr(),
                value_count,
            );
        }
    }

    fn get_bool(&self, values: &mut [bool]) {
        let mut value_count = values.len() as usize;
        let mut int_values = Vec::<i32>::with_capacity(value_count);

        for &mut val in values.into_iter() {
            int_values.push(val as i32)
        }

        unsafe {
            alsactl_sys::alsactl_elem_value_get_bool(
                self.as_ref().to_glib_none().0,
                &int_values.as_mut_ptr(),
                &mut value_count,
            );

            for (i, val) in int_values.into_iter().enumerate() {
                values[i] = val != 0;
            }
        }
    }

    fn get_bytes(&self, values: &mut [u8]) {
        unsafe {
            let mut value_count = values.len() as usize;

            alsactl_sys::alsactl_elem_value_get_bytes(
                self.as_ref().to_glib_none().0,
                &values.as_mut_ptr(),
                &mut value_count,
            );
        }
    }

    fn get_int(&self, values: &mut [i32]) {
        unsafe {
            let mut value_count = values.len() as usize;

            alsactl_sys::alsactl_elem_value_get_int(
                self.as_ref().to_glib_none().0,
                &values.as_mut_ptr(),
                &mut value_count,
            );
        }
    }

    fn get_enum(&self, values: &mut [u32]) {
        unsafe {
            let mut value_count = values.len() as usize;

            alsactl_sys::alsactl_elem_value_get_enum(
                self.as_ref().to_glib_none().0,
                &values.as_mut_ptr(),
                &mut value_count,
            );
        }
    }

    fn get_int64(&self, values: &mut [i64]) {
        unsafe {
            let mut value_count = values.len() as usize;

            alsactl_sys::alsactl_elem_value_get_int64(
                self.as_ref().to_glib_none().0,
                &values.as_mut_ptr(),
                &mut value_count,
            );
        }
    }

    fn get_iec60958_channel_status(&self, status: &mut [u8]) {
        unsafe {
            let mut count = status.len() as usize;

            alsactl_sys::alsactl_elem_value_get_iec60958_channel_status(
                self.as_ref().to_glib_none().0,
                &status.as_mut_ptr(),
                &mut count,
            );
        }
    }

    fn get_iec60958_user_data(&self, data: &mut [u8]) {
        unsafe {
            let mut count = data.len() as usize;

            alsactl_sys::alsactl_elem_value_get_iec60958_user_data(
                self.as_ref().to_glib_none().0,
                &data.as_mut_ptr(),
                &mut count,
            );
        }
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_manual_bindings() {
        let val = ElemValue::new();

        let bool_expected = [false, true, false, true, true, false];
        let mut bool_orig = vec![false; bool_expected.len()];
        val.get_bool(&mut bool_orig);
        val.set_bool(&bool_expected);
        let mut bool_target = vec![false; bool_expected.len()];
        val.get_bool(&mut bool_target);
        assert_ne!(&bool_expected.to_vec(), &bool_orig);
        assert_eq!(&bool_expected.to_vec(), &bool_target);

        let bytes_expected = [5, 4, 0, 2, 1, 8, 19, 21, 128, 212, 192];
        let mut bytes_orig = vec![0; bytes_expected.len()];
        val.get_bytes(&mut bytes_orig);
        val.set_bytes(&bytes_expected);
        let mut bytes_target = vec![0; bytes_expected.len()];
        val.get_bytes(&mut bytes_target);
        assert_ne!(&bytes_expected.to_vec(), &bytes_orig);
        assert_eq!(&bytes_expected.to_vec(), &bytes_target);

        let int_expected = [5, -4, 0, 2, 1, 8, 19, 21, -128, 212, -192000];
        let mut int_orig = vec![0; int_expected.len()];
        val.get_int(&mut int_orig);
        val.set_int(&int_expected);
        let mut int_target = vec![0; int_expected.len()];
        val.get_int(&mut int_target);
        assert_ne!(&int_expected.to_vec(), &int_orig);
        assert_eq!(&int_expected.to_vec(), &int_target);

        let enum_expected = [5, 4, 0, 2, 1, 8, 19, 21, 128, 212, 192];
        let mut enum_orig = vec![0; enum_expected.len()];
        val.get_enum(&mut enum_orig);
        val.set_enum(&enum_expected);
        let mut enum_target = vec![0; enum_expected.len()];
        val.get_enum(&mut enum_target);
        assert_ne!(&enum_expected.to_vec(), &enum_orig);
        assert_eq!(&enum_expected.to_vec(), &enum_target);

        let int64_expected = [5, 4, 0, 2, 1, 8, -1938754, 21, 128, 212, -92854];
        let mut int64_orig = vec![0; int64_expected.len()];
        val.get_int64(&mut int64_orig);
        val.set_int64(&int64_expected);
        let mut int64_target = vec![0; int64_expected.len()];
        val.get_int64(&mut int64_target);
        assert_ne!(&int64_expected.to_vec(), &int64_orig);
        assert_eq!(&int64_expected.to_vec(), &int64_target);

        let status_expected = [9, 7, 4, 124, 67];
        let mut status_orig = vec![0; status_expected.len()];
        val.get_iec60958_channel_status(&mut status_orig);
        val.set_iec60958_channel_status(&status_expected);
        let mut status_target = vec![0; status_expected.len()];
        val.get_iec60958_channel_status(&mut status_target);
        assert_ne!(&status_expected.to_vec(), &status_orig);
        assert_eq!(&status_expected.to_vec(), &status_target);

        let data_expected = [31, 211, 198, 90, 28, 8, 49];
        let mut data_orig = vec![0; data_expected.len()];
        val.get_iec60958_user_data(&mut data_orig);
        val.set_iec60958_user_data(&data_expected);
        let mut data_target = vec![0; data_expected.len()];
        val.get_iec60958_user_data(&mut data_target);
        assert_ne!(&data_expected.to_vec(), &data_orig);
        assert_eq!(&data_expected.to_vec(), &data_target);
    }
}
