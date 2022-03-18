// SPDX-License-Identifier: MIT
use crate::*;

pub trait ElemInfoExtManual {
    fn get_int_data(&self) -> Result<&[i32;3], glib::Error>;
    fn set_int_data(&self, data: &[i32;3]) -> Result<(), glib::Error>;

    fn get_int64_data(&self) -> Result<&[i64;3], glib::Error>;
    fn set_int64_data(&self, data: &[i64;3]) -> Result<(), glib::Error>;
}

impl<O: IsA<ElemInfo>> ElemInfoExtManual for O {
    fn get_int_data(&self) -> Result<&[i32;3], glib::Error> {
        unsafe {
            let mut data = std::ptr::null_mut() as *const [i32;3];
            let mut error = std::ptr::null_mut();

            alsactl_sys::alsactl_elem_info_get_int_data(
                self.as_ref().to_glib_none().0,
                &mut data as *mut *const [i32;3],
                &mut error,
            );

            if error.is_null() {
                Ok(&*data)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn set_int_data(&self, data: &[i32;3]) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();

            alsactl_sys::alsactl_elem_info_set_int_data(
                self.as_ref().to_glib_none().0,
                data,
                &mut error,
            );

            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn get_int64_data(&self) -> Result<&[i64;3], glib::Error> {
        unsafe {
            let mut data = std::ptr::null_mut() as *const [i64;3];
            let mut error = std::ptr::null_mut();

            alsactl_sys::alsactl_elem_info_get_int64_data(
                self.as_ref().to_glib_none().0,
                &mut data as *mut *const [i64;3],
                &mut error,
            );

            if error.is_null() {
                Ok(&*data)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn set_int64_data(&self, data: &[i64;3]) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();

            alsactl_sys::alsactl_elem_info_set_int64_data(
                self.as_ref().to_glib_none().0,
                data,
                &mut error,
            );

            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_manual_bindings() {
        let expected_int_data = [10, 20, 5];
        let int_info = ElemInfo::new(ElemType::Integer).unwrap();
        let int_data = int_info.get_int_data().unwrap();
        let orig_int_data = int_data.clone();
        int_info.set_int_data(&expected_int_data).unwrap();
        let curr_int_data = int_info.get_int_data().unwrap();

        assert_ne!(expected_int_data, orig_int_data);
        assert_eq!(&expected_int_data, curr_int_data);

        let expected_int64_data = [10, 20, 5];
        let int64_info = ElemInfo::new(ElemType::Integer64).unwrap();
        let int64_data = int64_info.get_int64_data().unwrap();
        let orig_int64_data = int64_data.clone();
        int64_info.set_int64_data(&expected_int64_data).unwrap();
        let curr_int64_data = int64_info.get_int64_data().unwrap();

        assert_ne!(expected_int64_data, orig_int64_data);
        assert_eq!(&expected_int64_data, curr_int64_data);
    }
}
