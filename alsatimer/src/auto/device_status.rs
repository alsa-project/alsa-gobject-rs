// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use alsatimer_sys;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::Value;
use glib_sys;
use gobject_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct DeviceStatus(Object<alsatimer_sys::ALSATimerDeviceStatus, alsatimer_sys::ALSATimerDeviceStatusClass, DeviceStatusClass>);

    match fn {
        get_type => || alsatimer_sys::alsatimer_device_status_get_type(),
    }
}

impl DeviceStatus {
    pub fn new() -> DeviceStatus {
        unsafe {
            from_glib_full(alsatimer_sys::alsatimer_device_status_new())
        }
    }
}

impl Default for DeviceStatus {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_DEVICE_STATUS: Option<&DeviceStatus> = None;

pub trait DeviceStatusExt: 'static {
    fn get_property_resolution(&self) -> u64;

    fn get_property_resolution_denominator(&self) -> u64;

    fn get_property_resolution_numerator(&self) -> u64;

    fn connect_property_resolution_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_resolution_denominator_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_resolution_numerator_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DeviceStatus>> DeviceStatusExt for O {
    fn get_property_resolution(&self) -> u64 {
        unsafe {
            let mut value = Value::from_type(<u64 as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"resolution\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `resolution` getter").unwrap()
        }
    }

    fn get_property_resolution_denominator(&self) -> u64 {
        unsafe {
            let mut value = Value::from_type(<u64 as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"resolution-denominator\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `resolution-denominator` getter").unwrap()
        }
    }

    fn get_property_resolution_numerator(&self) -> u64 {
        unsafe {
            let mut value = Value::from_type(<u64 as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"resolution-numerator\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `resolution-numerator` getter").unwrap()
        }
    }

    fn connect_property_resolution_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_resolution_trampoline<P, F: Fn(&P) + 'static>(this: *mut alsatimer_sys::ALSATimerDeviceStatus, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<DeviceStatus>
        {
            let f: &F = &*(f as *const F);
            f(&DeviceStatus::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::resolution\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_resolution_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_resolution_denominator_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_resolution_denominator_trampoline<P, F: Fn(&P) + 'static>(this: *mut alsatimer_sys::ALSATimerDeviceStatus, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<DeviceStatus>
        {
            let f: &F = &*(f as *const F);
            f(&DeviceStatus::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::resolution-denominator\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_resolution_denominator_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_resolution_numerator_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_resolution_numerator_trampoline<P, F: Fn(&P) + 'static>(this: *mut alsatimer_sys::ALSATimerDeviceStatus, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<DeviceStatus>
        {
            let f: &F = &*(f as *const F);
            f(&DeviceStatus::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::resolution-numerator\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_resolution_numerator_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for DeviceStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DeviceStatus")
    }
}