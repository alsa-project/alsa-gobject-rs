// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ElemInfoCommon;
use crate::ElemInfoSingleArray;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "ALSACtlElemInfoInteger64")]
    pub struct ElemInfoInteger64(Object<ffi::ALSACtlElemInfoInteger64, ffi::ALSACtlElemInfoInteger64Class>) @implements ElemInfoCommon, ElemInfoSingleArray;

    match fn {
        type_ => || ffi::alsactl_elem_info_integer64_get_type(),
    }
}

impl ElemInfoInteger64 {
    pub const NONE: Option<&'static ElemInfoInteger64> = None;

    #[doc(alias = "alsactl_elem_info_integer64_new")]
    pub fn new() -> ElemInfoInteger64 {
        unsafe { from_glib_full(ffi::alsactl_elem_info_integer64_new()) }
    }
}

impl Default for ElemInfoInteger64 {
    fn default() -> Self {
        Self::new()
    }
}

pub trait ElemInfoInteger64Ext: 'static {
    #[doc(alias = "value-max")]
    fn value_max(&self) -> i64;

    #[doc(alias = "value-max")]
    fn set_value_max(&self, value_max: i64);

    #[doc(alias = "value-min")]
    fn value_min(&self) -> i64;

    #[doc(alias = "value-min")]
    fn set_value_min(&self, value_min: i64);

    #[doc(alias = "value-step")]
    fn value_step(&self) -> i64;

    #[doc(alias = "value-step")]
    fn set_value_step(&self, value_step: i64);

    #[doc(alias = "value-max")]
    fn connect_value_max_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "value-min")]
    fn connect_value_min_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "value-step")]
    fn connect_value_step_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<ElemInfoInteger64>> ElemInfoInteger64Ext for O {
    fn value_max(&self) -> i64 {
        glib::ObjectExt::property(self.as_ref(), "value-max")
    }

    fn set_value_max(&self, value_max: i64) {
        glib::ObjectExt::set_property(self.as_ref(), "value-max", &value_max)
    }

    fn value_min(&self) -> i64 {
        glib::ObjectExt::property(self.as_ref(), "value-min")
    }

    fn set_value_min(&self, value_min: i64) {
        glib::ObjectExt::set_property(self.as_ref(), "value-min", &value_min)
    }

    fn value_step(&self) -> i64 {
        glib::ObjectExt::property(self.as_ref(), "value-step")
    }

    fn set_value_step(&self, value_step: i64) {
        glib::ObjectExt::set_property(self.as_ref(), "value-step", &value_step)
    }

    fn connect_value_max_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_value_max_trampoline<
            P: IsA<ElemInfoInteger64>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::ALSACtlElemInfoInteger64,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ElemInfoInteger64::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::value-max\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_value_max_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_value_min_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_value_min_trampoline<
            P: IsA<ElemInfoInteger64>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::ALSACtlElemInfoInteger64,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ElemInfoInteger64::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::value-min\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_value_min_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_value_step_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_value_step_trampoline<
            P: IsA<ElemInfoInteger64>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::ALSACtlElemInfoInteger64,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ElemInfoInteger64::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::value-step\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_value_step_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for ElemInfoInteger64 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ElemInfoInteger64")
    }
}
