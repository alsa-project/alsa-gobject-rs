// SPDX-License-Identifier: MIT
use super::*;

/// Trait which should be implemented by subclass of [`DeviceCommon`][crate::DeviceCommon].
pub trait DeviceCommonImpl: ObjectImpl + ObjectSubclass {
    fn open(&self, device: &Self::Type, path: &str, open_flag: i32) -> Result<(), Error>;
    fn get_protocol_version(
        &self,
        device: &Self::Type,
        proto_ver_triplet: &mut [u16; 3],
    ) -> Result<(), Error>;
    fn get_device_info(&self, device: &Self::Type) -> Result<DeviceInfo, Error>;
    fn create_source(&self, device: &Self::Type) -> Result<Source, Error>;
    fn handle_disconnection(&self, device: &Self::Type);
}

/// Trait which is automatically implemented to implementator of
/// [`DeviceCommonImpl`][self::DeviceCommonImpl].
pub trait DeviceCommonImplExt: ObjectSubclass {
    fn parent_open(&self, device: &Self::Type, path: &str, open_flag: i32) -> Result<(), Error>;
    fn parent_get_protocol_version(
        &self,
        device: &Self::Type,
        proto_ver_triplet: &mut [u16; 3],
    ) -> Result<(), Error>;
    fn parent_get_device_info(&self, device: &Self::Type) -> Result<DeviceInfo, Error>;
    fn parent_create_source(&self, device: &Self::Type) -> Result<Source, Error>;
    fn parent_handle_disconnection(&self, device: &Self::Type);
}

impl<T: DeviceCommonImpl> DeviceCommonImplExt for T {
    fn parent_open(&self, device: &Self::Type, path: &str, open_flag: i32) -> Result<(), Error> {
        unsafe {
            let data = T::type_data();
            let parent_class =
                data.as_ref().parent_class() as *mut ffi::ALSAHwdepDeviceCommonInterface;
            let f = (*parent_class)
                .open
                .expect("No parent \"open\" implementation");

            let mut error = std::ptr::null_mut();
            let is_ok = f(
                device.unsafe_cast_ref::<DeviceCommon>().to_glib_none().0,
                path.to_glib_none().0,
                open_flag.into(),
                &mut error,
            );
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn parent_get_protocol_version(
        &self,
        device: &Self::Type,
        proto_ver_triplet: &mut [u16; 3],
    ) -> Result<(), Error> {
        unsafe {
            let data = T::type_data();
            let parent_class =
                data.as_ref().parent_class() as *mut ffi::ALSAHwdepDeviceCommonInterface;
            let f = (*parent_class)
                .get_protocol_version
                .expect("No parent \"get_protocol_version\" implementation");

            let ptr: *mut [u16; 3] = proto_ver_triplet;
            let mut error = std::ptr::null_mut();
            let is_ok = f(
                device.unsafe_cast_ref::<DeviceCommon>().to_glib_none().0,
                &ptr,
                &mut error,
            );
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn parent_get_device_info(&self, device: &Self::Type) -> Result<DeviceInfo, Error> {
        unsafe {
            let data = T::type_data();
            let parent_class =
                data.as_ref().parent_class() as *mut ffi::ALSAHwdepDeviceCommonInterface;
            let f = (*parent_class)
                .get_device_info
                .expect("No parent \"get_device_info\" implementation");

            let mut info = std::ptr::null_mut();
            let mut error = std::ptr::null_mut();
            let is_ok = f(
                device.unsafe_cast_ref::<DeviceCommon>().to_glib_none().0,
                &mut info,
                &mut error,
            );
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(from_glib_full(info))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn parent_create_source(&self, device: &Self::Type) -> Result<Source, Error> {
        unsafe {
            let data = T::type_data();
            let parent_class =
                data.as_ref().parent_class() as *mut ffi::ALSAHwdepDeviceCommonInterface;
            let f = (*parent_class)
                .create_source
                .expect("No parent \"create_source\" implementation");

            let mut src = std::ptr::null_mut();
            let mut error = std::ptr::null_mut();
            let is_ok = f(
                device.unsafe_cast_ref::<DeviceCommon>().to_glib_none().0,
                &mut src,
                &mut error,
            );
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(from_glib_full(src))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn parent_handle_disconnection(&self, device: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class =
                data.as_ref().parent_class() as *mut ffi::ALSAHwdepDeviceCommonInterface;
            let f = (*parent_class)
                .handle_disconnection
                .expect("No parent \"handle_disconnection\" implementation");

            f(device.unsafe_cast_ref::<DeviceCommon>().to_glib_none().0);
        }
    }
}

unsafe impl<T: DeviceCommonImpl> IsImplementable<T> for DeviceCommon {
    fn interface_init(iface: &mut Interface<Self>) {
        let iface = iface.as_mut();
        iface.open = Some(device_common_open::<T>);
        iface.get_protocol_version = Some(device_common_get_protocol_version::<T>);
        iface.get_device_info = Some(device_common_get_device_info::<T>);
        iface.create_source = Some(device_common_create_source::<T>);
        iface.handle_disconnection = Some(device_common_handle_disconnection::<T>);
    }
}

unsafe extern "C" fn device_common_open<T: DeviceCommonImpl>(
    device: *mut ffi::ALSAHwdepDeviceCommon,
    path: *const c_char,
    open_flag: c_int,
    error: *mut *mut glib::ffi::GError,
) -> glib::ffi::gboolean {
    let instance = &*(device as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<DeviceCommon> = from_glib_borrow(device);
    match imp.open(
        wrap.unsafe_cast_ref(),
        std::ffi::CStr::from_ptr(path).to_str().unwrap(),
        open_flag,
    ) {
        Ok(()) => glib::ffi::GTRUE,
        Err(err) => {
            if !error.is_null() {
                *error = err.into_glib_ptr();
            }
            glib::ffi::GFALSE
        }
    }
}

unsafe extern "C" fn device_common_get_protocol_version<T: DeviceCommonImpl>(
    device: *mut ffi::ALSAHwdepDeviceCommon,
    proto_ver_triplet: *const *mut [u16; 3],
    error: *mut *mut glib::ffi::GError,
) -> glib::ffi::gboolean {
    let instance = &*(device as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<DeviceCommon> = from_glib_borrow(device);
    match imp.get_protocol_version(wrap.unsafe_cast_ref(), &mut (**proto_ver_triplet)) {
        Ok(()) => glib::ffi::GTRUE,
        Err(err) => {
            if !error.is_null() {
                *error = err.into_glib_ptr();
            }
            glib::ffi::GFALSE
        }
    }
}

unsafe extern "C" fn device_common_get_device_info<T: DeviceCommonImpl>(
    device: *mut ffi::ALSAHwdepDeviceCommon,
    device_info: *mut *mut ffi::ALSAHwdepDeviceInfo,
    error: *mut *mut glib::ffi::GError,
) -> glib::ffi::gboolean {
    let instance = &*(device as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<DeviceCommon> = from_glib_borrow(device);
    match imp.get_device_info(wrap.unsafe_cast_ref()) {
        Ok(info) => {
            *device_info = info.to_glib_full();
            glib::ffi::GTRUE
        }
        Err(err) => {
            if !error.is_null() {
                *error = err.into_glib_ptr();
            }
            glib::ffi::GFALSE
        }
    }
}

unsafe extern "C" fn device_common_create_source<T: DeviceCommonImpl>(
    device: *mut ffi::ALSAHwdepDeviceCommon,
    source: *mut *mut glib::ffi::GSource,
    error: *mut *mut glib::ffi::GError,
) -> glib::ffi::gboolean {
    let instance = &*(device as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<DeviceCommon> = from_glib_borrow(device);
    match imp.create_source(wrap.unsafe_cast_ref()) {
        Ok(src) => {
            *source = src.to_glib_none().0;
            glib::ffi::GTRUE
        }
        Err(err) => {
            if !error.is_null() {
                *error = err.into_glib_ptr();
            }
            glib::ffi::GFALSE
        }
    }
}

unsafe extern "C" fn device_common_handle_disconnection<T: DeviceCommonImpl>(
    device: *mut ffi::ALSAHwdepDeviceCommon,
) {
    let instance = &*(device as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<DeviceCommon> = from_glib_borrow(device);
    imp.handle_disconnection(wrap.unsafe_cast_ref())
}

#[cfg(test)]
mod test {
    use crate::{prelude::*, subclass::prelude::*, *};
    use glib::{subclass::prelude::*, Error, Object, ObjectExt, Properties, Source};

    const NAME: &str = "MyName";

    mod imp {
        use super::*;
        use std::cell::RefCell;

        #[derive(Properties)]
        #[properties(wrapper_type = super::DeviceCommonTest)]
        pub struct DeviceCommonTestPrivate {
            #[property(get, set)]
            name: RefCell<String>,
            #[property(get)]
            handled: RefCell<bool>,
        }

        #[glib::object_subclass]
        impl ObjectSubclass for DeviceCommonTestPrivate {
            const NAME: &'static str = "DeviceCommonTest";
            type Type = super::DeviceCommonTest;
            type Interfaces = (DeviceCommon,);

            fn new() -> Self {
                Self {
                    name: Default::default(),
                    handled: Default::default(),
                }
            }
        }

        #[glib::derived_properties]
        impl ObjectImpl for DeviceCommonTestPrivate {}

        impl DeviceCommonImpl for DeviceCommonTestPrivate {
            fn open(&self, _: &Self::Type, _: &str, _: i32) -> Result<(), Error> {
                Ok(())
            }

            fn get_protocol_version(
                &self,
                _: &Self::Type,
                proto_ver_triplet: &mut [u16; 3],
            ) -> Result<(), Error> {
                proto_ver_triplet.copy_from_slice(&[1, 2, 3]);
                Ok(())
            }

            fn get_device_info(&self, _: &Self::Type) -> Result<DeviceInfo, Error> {
                let device_info = Object::builder::<DeviceInfo>()
                    .property("name", NAME)
                    .build();
                Ok(device_info)
            }

            fn create_source(&self, _: &Self::Type) -> Result<Source, Error> {
                Err(Error::new(DeviceCommonError::IsOpened, "expected"))
            }

            fn handle_disconnection(&self, _: &Self::Type) {
                *self.handled.borrow_mut() = true;
            }
        }
    }

    glib::wrapper! {
        pub struct DeviceCommonTest(ObjectSubclass<imp::DeviceCommonTestPrivate>)
            @implements DeviceCommon;
    }

    #[test]
    fn device_common_iface() {
        let device = Object::builder::<DeviceCommonTest>().build();

        // The '/dev/snd/hwC10000D10000' hardly exists in the system.
        assert!(device.open(10000, 10000, 0).is_err());
        assert_eq!(device.protocol_version(), Ok([1, 2, 3]));
        let device_info = device.device_info().unwrap();
        assert_eq!(device_info.name().unwrap(), NAME);
        assert!(device.create_source().is_err());

        assert_eq!(device.handled(), false);
        device.emit_handle_disconnection();
        assert_eq!(device.handled(), true);
    }
}
