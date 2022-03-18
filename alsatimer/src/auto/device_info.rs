// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::DeviceInfoFlag;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "ALSATimerDeviceInfo")]
    pub struct DeviceInfo(Object<ffi::ALSATimerDeviceInfo, ffi::ALSATimerDeviceInfoClass>);

    match fn {
        type_ => || ffi::alsatimer_device_info_get_type(),
    }
}

impl DeviceInfo {
    pub const NONE: Option<&'static DeviceInfo> = None;
}

pub trait DeviceInfoExt: 'static {
    #[doc(alias = "card-id")]
    fn card_id(&self) -> i32;

    fn flags(&self) -> DeviceInfoFlag;

    fn id(&self) -> Option<glib::GString>;

    #[doc(alias = "instance-count")]
    fn instance_count(&self) -> u32;

    fn name(&self) -> Option<glib::GString>;

    fn resolution(&self) -> u64;

    #[doc(alias = "resolution-max")]
    fn resolution_max(&self) -> u64;

    #[doc(alias = "resolution-min")]
    fn resolution_min(&self) -> u64;

    #[doc(alias = "card-id")]
    fn connect_card_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "flags")]
    fn connect_flags_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "id")]
    fn connect_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "instance-count")]
    fn connect_instance_count_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "name")]
    fn connect_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "resolution")]
    fn connect_resolution_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "resolution-max")]
    fn connect_resolution_max_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "resolution-min")]
    fn connect_resolution_min_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DeviceInfo>> DeviceInfoExt for O {
    fn card_id(&self) -> i32 {
        glib::ObjectExt::property(self.as_ref(), "card-id")
    }

    fn flags(&self) -> DeviceInfoFlag {
        glib::ObjectExt::property(self.as_ref(), "flags")
    }

    fn id(&self) -> Option<glib::GString> {
        glib::ObjectExt::property(self.as_ref(), "id")
    }

    fn instance_count(&self) -> u32 {
        glib::ObjectExt::property(self.as_ref(), "instance-count")
    }

    fn name(&self) -> Option<glib::GString> {
        glib::ObjectExt::property(self.as_ref(), "name")
    }

    fn resolution(&self) -> u64 {
        glib::ObjectExt::property(self.as_ref(), "resolution")
    }

    fn resolution_max(&self) -> u64 {
        glib::ObjectExt::property(self.as_ref(), "resolution-max")
    }

    fn resolution_min(&self) -> u64 {
        glib::ObjectExt::property(self.as_ref(), "resolution-min")
    }

    fn connect_card_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_card_id_trampoline<P: IsA<DeviceInfo>, F: Fn(&P) + 'static>(
            this: *mut ffi::ALSATimerDeviceInfo,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DeviceInfo::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::card-id\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_card_id_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_flags_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_flags_trampoline<P: IsA<DeviceInfo>, F: Fn(&P) + 'static>(
            this: *mut ffi::ALSATimerDeviceInfo,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DeviceInfo::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::flags\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_flags_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_id_trampoline<P: IsA<DeviceInfo>, F: Fn(&P) + 'static>(
            this: *mut ffi::ALSATimerDeviceInfo,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DeviceInfo::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::id\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_id_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_instance_count_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_instance_count_trampoline<
            P: IsA<DeviceInfo>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::ALSATimerDeviceInfo,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DeviceInfo::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::instance-count\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_instance_count_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_name_trampoline<P: IsA<DeviceInfo>, F: Fn(&P) + 'static>(
            this: *mut ffi::ALSATimerDeviceInfo,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DeviceInfo::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::name\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_name_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_resolution_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_resolution_trampoline<
            P: IsA<DeviceInfo>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::ALSATimerDeviceInfo,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DeviceInfo::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::resolution\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_resolution_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_resolution_max_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_resolution_max_trampoline<
            P: IsA<DeviceInfo>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::ALSATimerDeviceInfo,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DeviceInfo::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::resolution-max\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_resolution_max_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_resolution_min_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_resolution_min_trampoline<
            P: IsA<DeviceInfo>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::ALSATimerDeviceInfo,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(DeviceInfo::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::resolution-min\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_resolution_min_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for DeviceInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DeviceInfo")
    }
}
