// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use alsactl_sys;
use glib;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;
use glib::StaticType;
use glib::Value;
use glib_sys;
use gobject_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::ptr;
use ElemAccessFlag;
use ElemId;
use ElemType;

glib_wrapper! {
    pub struct ElemInfo(Object<alsactl_sys::ALSACtlElemInfo, alsactl_sys::ALSACtlElemInfoClass, ElemInfoClass>);

    match fn {
        get_type => || alsactl_sys::alsactl_elem_info_get_type(),
    }
}

impl ElemInfo {
    pub fn new(elem_type: ElemType) -> Result<ElemInfo, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = alsactl_sys::alsactl_elem_info_new(elem_type.to_glib(), &mut error);
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}

pub const NONE_ELEM_INFO: Option<&ElemInfo> = None;

pub trait ElemInfoExt: 'static {
    fn get_enum_data(&self) -> Result<Vec<GString>, glib::Error>;

    fn set_enum_data(&self, data: &[&str]) -> Result<(), glib::Error>;

    fn get_property_access(&self) -> ElemAccessFlag;

    fn set_property_access(&self, access: ElemAccessFlag);

    fn get_property_elem_id(&self) -> Option<ElemId>;

    fn get_property_owner(&self) -> i32;

    fn get_property_type(&self) -> ElemType;

    fn get_property_value_count(&self) -> u32;

    fn set_property_value_count(&self, value_count: u32);

    fn connect_property_access_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_elem_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_owner_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_value_count_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<ElemInfo>> ElemInfoExt for O {
    fn get_enum_data(&self) -> Result<Vec<GString>, glib::Error> {
        unsafe {
            let mut data = ptr::null_mut();
            let mut error = ptr::null_mut();
            let _ = alsactl_sys::alsactl_elem_info_get_enum_data(
                self.as_ref().to_glib_none().0,
                &mut data,
                &mut error,
            );
            if error.is_null() {
                Ok(FromGlibPtrContainer::from_glib_none(data))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn set_enum_data(&self, data: &[&str]) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = alsactl_sys::alsactl_elem_info_set_enum_data(
                self.as_ref().to_glib_none().0,
                data.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn get_property_access(&self) -> ElemAccessFlag {
        unsafe {
            let mut value = Value::from_type(<ElemAccessFlag as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"access\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `access` getter")
                .unwrap()
        }
    }

    fn set_property_access(&self, access: ElemAccessFlag) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"access\0".as_ptr() as *const _,
                Value::from(&access).to_glib_none().0,
            );
        }
    }

    fn get_property_elem_id(&self) -> Option<ElemId> {
        unsafe {
            let mut value = Value::from_type(<ElemId as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"elem-id\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `elem-id` getter")
        }
    }

    fn get_property_owner(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"owner\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `owner` getter")
                .unwrap()
        }
    }

    fn get_property_type(&self) -> ElemType {
        unsafe {
            let mut value = Value::from_type(<ElemType as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"type\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `type` getter")
                .unwrap()
        }
    }

    fn get_property_value_count(&self) -> u32 {
        unsafe {
            let mut value = Value::from_type(<u32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"value-count\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `value-count` getter")
                .unwrap()
        }
    }

    fn set_property_value_count(&self, value_count: u32) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"value-count\0".as_ptr() as *const _,
                Value::from(&value_count).to_glib_none().0,
            );
        }
    }

    fn connect_property_access_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_access_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut alsactl_sys::ALSACtlElemInfo,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ElemInfo>,
        {
            let f: &F = &*(f as *const F);
            f(&ElemInfo::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::access\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_access_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_elem_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_elem_id_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut alsactl_sys::ALSACtlElemInfo,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ElemInfo>,
        {
            let f: &F = &*(f as *const F);
            f(&ElemInfo::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::elem-id\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_elem_id_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_owner_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_owner_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut alsactl_sys::ALSACtlElemInfo,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ElemInfo>,
        {
            let f: &F = &*(f as *const F);
            f(&ElemInfo::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::owner\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_owner_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_value_count_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_value_count_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut alsactl_sys::ALSACtlElemInfo,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<ElemInfo>,
        {
            let f: &F = &*(f as *const F);
            f(&ElemInfo::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::value-count\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_value_count_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for ElemInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ElemInfo")
    }
}
