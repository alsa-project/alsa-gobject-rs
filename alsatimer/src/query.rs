use crate::*;

pub fn get_device_id_list() -> Result<Vec<DeviceId>, glib::Error> {
    unsafe {
        let mut entries = std::ptr::null_mut();
        let mut error = std::ptr::null_mut();

        alsatimer_sys::alsatimer_get_device_id_list(&mut entries, &mut error);

        if error.is_null() {
            Ok(FromGlibPtrContainer::from_glib_full(entries))
        } else {
            Err(from_glib_full(error))
        }
    }
}

pub fn set_device_params(device_id: &mut DeviceId, device_params: &DeviceParams) -> Result<(), glib::Error> {
    unsafe {
        let mut error = std::ptr::null_mut();

        alsatimer_sys::alsatimer_set_device_params(
            device_id.to_glib_none_mut().0,
            device_params.to_glib_none().0,
            &mut error,
        );

        if error.is_null() {
            Ok(())
        } else {
            Err(from_glib_full(error))
        }
    }
}

pub fn get_device_status<P: IsA<DeviceStatus>>(device_id: &mut DeviceId, status: &mut P) -> Result<(), glib::Error> {
    unsafe {
        let mut error = std::ptr::null_mut();

        alsatimer_sys::alsatimer_get_device_status(
            device_id.to_glib_none_mut().0,
            &mut status.as_ref().to_glib_none().0,
            &mut error,
        );

        if error.is_null() {
            Ok(())
        } else {
            Err(from_glib_full(error))
        }
    }
}
