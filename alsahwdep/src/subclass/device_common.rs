// SPDX-License-Identifier: MIT
use super::*;

pub trait DeviceCommonImpl: ObjectImpl + ObjectSubclass {
    fn open(&self, device: &DeviceCommon, path: &str, open_flag: i32) -> Result<(), Error>;
    fn get_protocol_version(
        &self,
        device: &DeviceCommon,
        proto_ver_triplet: &mut [u16; 3],
    ) -> Result<(), Error>;
    fn get_device_info(&self, device: &DeviceCommon) -> Result<DeviceInfo, Error>;
    fn create_source(&self, device: &DeviceCommon) -> Result<Source, Error>;
    fn handle_disconnection(&self, device: &DeviceCommon);
}

unsafe impl<T: DeviceCommonImpl> IsImplementable<T> for DeviceCommon {
    unsafe extern "C" fn interface_init(
        iface: glib_sys::gpointer,
        _iface_data: glib_sys::gpointer,
    ) {
        let iface = &mut *(iface as *mut alsahwdep_sys::ALSAHwdepDeviceCommonInterface);
        iface.open = Some(device_common_open::<T>);
        iface.get_protocol_version = Some(device_common_get_protocol_version::<T>);
        iface.get_device_info = Some(device_common_get_device_info::<T>);
        iface.create_source = Some(device_common_create_source::<T>);
        iface.handle_disconnection = Some(device_common_handle_disconnection::<T>);
    }
}

unsafe extern "C" fn device_common_open<T: DeviceCommonImpl>(
    device: *mut alsahwdep_sys::ALSAHwdepDeviceCommon,
    path: *const c_char,
    open_flag: c_int,
    error: *mut *mut glib_sys::GError,
) -> glib_sys::gboolean {
    let instance = &*(device as *mut T::Instance);
    let imp = instance.get_impl();
    match imp.open(
        &from_glib_borrow(device),
        std::ffi::CStr::from_ptr(path).to_str().unwrap(),
        open_flag,
    ) {
        Ok(()) => glib_sys::GTRUE,
        Err(err) => {
            let mut e = std::mem::ManuallyDrop::new(err);
            *error = e.to_glib_none_mut().0;
            glib_sys::GFALSE
        }
    }
}

unsafe extern "C" fn device_common_get_protocol_version<T: DeviceCommonImpl>(
    device: *mut alsahwdep_sys::ALSAHwdepDeviceCommon,
    proto_ver_triplet: *const *mut [u16; 3],
    error: *mut *mut glib_sys::GError,
) -> glib_sys::gboolean {
    let instance = &*(device as *mut T::Instance);
    let imp = instance.get_impl();
    match imp.get_protocol_version(&from_glib_borrow(device), &mut (**proto_ver_triplet)) {
        Ok(()) => glib_sys::GTRUE,
        Err(err) => {
            let mut e = std::mem::ManuallyDrop::new(err);
            *error = e.to_glib_none_mut().0;
            glib_sys::GFALSE
        }
    }
}

unsafe extern "C" fn device_common_get_device_info<T: DeviceCommonImpl>(
    device: *mut alsahwdep_sys::ALSAHwdepDeviceCommon,
    device_info: *mut *mut alsahwdep_sys::ALSAHwdepDeviceInfo,
    error: *mut *mut glib_sys::GError,
) -> glib_sys::gboolean {
    let instance = &*(device as *mut T::Instance);
    let imp = instance.get_impl();
    match imp.get_device_info(&from_glib_borrow(device)) {
        Ok(info) => {
            *device_info = info.to_glib_full();
            glib_sys::GTRUE
        }
        Err(err) => {
            let mut e = std::mem::ManuallyDrop::new(err);
            *error = e.to_glib_none_mut().0;
            glib_sys::GFALSE
        }
    }
}

unsafe extern "C" fn device_common_create_source<T: DeviceCommonImpl>(
    device: *mut alsahwdep_sys::ALSAHwdepDeviceCommon,
    source: *mut *mut glib_sys::GSource,
    error: *mut *mut glib_sys::GError,
) -> glib_sys::gboolean {
    let instance = &*(device as *mut T::Instance);
    let imp = instance.get_impl();
    match imp.create_source(&from_glib_borrow(device)) {
        Ok(src) => {
            *source = src.to_glib_none().0;
            glib_sys::GTRUE
        }
        Err(err) => {
            let mut e = std::mem::ManuallyDrop::new(err);
            *error = e.to_glib_none_mut().0;
            glib_sys::GFALSE
        }
    }
}

unsafe extern "C" fn device_common_handle_disconnection<T: DeviceCommonImpl>(
    device: *mut alsahwdep_sys::ALSAHwdepDeviceCommon,
) {
    let instance = &*(device as *mut T::Instance);
    let imp = instance.get_impl();
    imp.handle_disconnection(&from_glib_borrow(device))
}

#[cfg(test)]
mod test {
    use crate::subclass::device_common::*;
    use crate::{DeviceCommon, DeviceCommonError, DeviceCommonExt};
    use glib::{
        subclass::{
            object::*,
            simple::{ClassStruct, InstanceStruct},
            types::*,
            InitializingType,
        },
        translate::*,
        Cast, Object, ObjectExt, ParamFlags, ParamSpec, StaticType, ToValue, Value,
    };

    mod imp {
        use super::*;
        use std::cell::RefCell;

        #[derive(Default)]
        pub struct DeviceCommonTestPrivate(RefCell<bool>);

        static PROPERTIES: [Property; 1] = [Property("handled", |handled| {
            ParamSpec::boolean(
                handled,
                "handled",
                "handle event or not",
                false,
                ParamFlags::READABLE,
            )
        })];

        impl ObjectSubclass for DeviceCommonTestPrivate {
            const NAME: &'static str = "DeviceCommonTest";
            type ParentType = Object;
            type Instance = InstanceStruct<Self>;
            type Class = ClassStruct<Self>;

            glib_object_subclass!();

            fn new() -> Self {
                Self::default()
            }

            fn class_init(klass: &mut Self::Class) {
                klass.install_properties(&PROPERTIES);
            }

            fn type_init(type_: &mut InitializingType<Self>) {
                type_.add_interface::<DeviceCommon>();
            }
        }

        impl ObjectImpl for DeviceCommonTestPrivate {
            glib_object_impl!();

            fn get_property(&self, _obj: &Object, id: usize) -> Result<Value, ()> {
                let prop = &PROPERTIES[id];

                match *prop {
                    Property("handled", ..) => Ok(self.0.borrow().to_value()),
                    _ => unimplemented!(),
                }
            }
        }

        impl DeviceCommonImpl for DeviceCommonTestPrivate {
            fn open(&self, _: &DeviceCommon, _: &str, _: i32) -> Result<(), Error> {
                unreachable!()
            }

            fn get_protocol_version(
                &self,
                _: &DeviceCommon,
                proto_ver_triplet: &mut [u16; 3],
            ) -> Result<(), Error> {
                proto_ver_triplet.copy_from_slice(&[1, 2, 3]);
                Ok(())
            }

            fn get_device_info(&self, _: &DeviceCommon) -> Result<DeviceInfo, Error> {
                let device_info = Object::new(DeviceInfo::static_type(), &[("name", &"MyName")])
                    .expect("Failed to create DeviceCommon")
                    .downcast()
                    .expect("Created row data is of wrong type");
                Ok(device_info)
            }

            fn create_source(&self, _: &DeviceCommon) -> Result<Source, Error> {
                Err(Error::new(DeviceCommonError::IsOpened, "expected"))
            }

            fn handle_disconnection(&self, _: &DeviceCommon) {
                *self.0.borrow_mut() = true;
            }
        }
    }

    glib_wrapper! {
        pub struct DeviceCommonTest(
            Object<InstanceStruct<imp::DeviceCommonTestPrivate>,
            ClassStruct<imp::DeviceCommonTestPrivate>, DeviceCommonTestClass>
        ) @implements DeviceCommon;

        match fn {
            get_type => || imp::DeviceCommonTestPrivate::get_type().to_glib(),
        }
    }

    impl DeviceCommonTest {
        fn new() -> Self {
            Object::new(Self::static_type(), &[])
                .expect("Failed to create DeviceCommon")
                .downcast()
                .expect("Created row data is of wrong type")
        }

        fn get_property_handled(&self) -> bool {
            self.get_property("handled")
                .expect("Failed to get 'handled' property")
                .get::<bool>()
                .expect("Failed to get boolean from 'handled' property")
                .unwrap()
        }
    }

    #[test]
    fn device_common_iface() {
        let device = DeviceCommonTest::new();

        // The '/dev/snd/hwC10000D10000' hardly exists in the system.
        assert!(device.open(10000, 10000, 0).is_err());
        assert_eq!(device.get_protocol_version(), Ok([1, 2, 3]));
        let device_info = device.get_device_info().unwrap();
        assert_eq!(device_info.get_property_name().unwrap(), "MyName");
        assert!(device.create_source().is_err());

        assert_eq!(device.get_property_handled(), false);
        device.emit_handle_disconnection();
        assert_eq!(device.get_property_handled(), true);
    }
}
