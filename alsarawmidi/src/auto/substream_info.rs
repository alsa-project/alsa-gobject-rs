// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use alsarawmidi_sys;
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
use StreamDirection;
use StreamPairInfoFlag;

glib_wrapper! {
    pub struct SubstreamInfo(Object<alsarawmidi_sys::ALSARawmidiSubstreamInfo, alsarawmidi_sys::ALSARawmidiSubstreamInfoClass, SubstreamInfoClass>);

    match fn {
        get_type => || alsarawmidi_sys::alsarawmidi_substream_info_get_type(),
    }
}

pub const NONE_SUBSTREAM_INFO: Option<&SubstreamInfo> = None;

pub trait SubstreamInfoExt: 'static {
    fn get_property_card_id(&self) -> i32;

    fn get_property_device_id(&self) -> u32;

    fn get_property_direction(&self) -> StreamDirection;

    fn get_property_flags(&self) -> StreamPairInfoFlag;

    fn get_property_id(&self) -> Option<GString>;

    fn get_property_name(&self) -> Option<GString>;

    fn get_property_subdevice_id(&self) -> u32;

    fn get_property_subdevice_name(&self) -> Option<GString>;

    fn get_property_subdevices_avail(&self) -> u32;

    fn get_property_subdevices_count(&self) -> u32;

    fn connect_property_card_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_device_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_direction_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_flags_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_subdevice_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_subdevice_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_subdevices_avail_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_subdevices_count_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<SubstreamInfo>> SubstreamInfoExt for O {
    fn get_property_card_id(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"card-id\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `card-id` getter").unwrap()
        }
    }

    fn get_property_device_id(&self) -> u32 {
        unsafe {
            let mut value = Value::from_type(<u32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"device-id\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `device-id` getter").unwrap()
        }
    }

    fn get_property_direction(&self) -> StreamDirection {
        unsafe {
            let mut value = Value::from_type(<StreamDirection as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"direction\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `direction` getter").unwrap()
        }
    }

    fn get_property_flags(&self) -> StreamPairInfoFlag {
        unsafe {
            let mut value = Value::from_type(<StreamPairInfoFlag as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"flags\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `flags` getter").unwrap()
        }
    }

    fn get_property_id(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"id\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `id` getter")
        }
    }

    fn get_property_name(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"name\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `name` getter")
        }
    }

    fn get_property_subdevice_id(&self) -> u32 {
        unsafe {
            let mut value = Value::from_type(<u32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"subdevice-id\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `subdevice-id` getter").unwrap()
        }
    }

    fn get_property_subdevice_name(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"subdevice-name\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `subdevice-name` getter")
        }
    }

    fn get_property_subdevices_avail(&self) -> u32 {
        unsafe {
            let mut value = Value::from_type(<u32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"subdevices-avail\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `subdevices-avail` getter").unwrap()
        }
    }

    fn get_property_subdevices_count(&self) -> u32 {
        unsafe {
            let mut value = Value::from_type(<u32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"subdevices-count\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `subdevices-count` getter").unwrap()
        }
    }

    fn connect_property_card_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_card_id_trampoline<P, F: Fn(&P) + 'static>(this: *mut alsarawmidi_sys::ALSARawmidiSubstreamInfo, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<SubstreamInfo>
        {
            let f: &F = &*(f as *const F);
            f(&SubstreamInfo::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::card-id\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_card_id_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_device_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_device_id_trampoline<P, F: Fn(&P) + 'static>(this: *mut alsarawmidi_sys::ALSARawmidiSubstreamInfo, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<SubstreamInfo>
        {
            let f: &F = &*(f as *const F);
            f(&SubstreamInfo::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::device-id\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_device_id_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_direction_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_direction_trampoline<P, F: Fn(&P) + 'static>(this: *mut alsarawmidi_sys::ALSARawmidiSubstreamInfo, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<SubstreamInfo>
        {
            let f: &F = &*(f as *const F);
            f(&SubstreamInfo::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::direction\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_direction_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_flags_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_flags_trampoline<P, F: Fn(&P) + 'static>(this: *mut alsarawmidi_sys::ALSARawmidiSubstreamInfo, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<SubstreamInfo>
        {
            let f: &F = &*(f as *const F);
            f(&SubstreamInfo::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::flags\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_flags_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_id_trampoline<P, F: Fn(&P) + 'static>(this: *mut alsarawmidi_sys::ALSARawmidiSubstreamInfo, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<SubstreamInfo>
        {
            let f: &F = &*(f as *const F);
            f(&SubstreamInfo::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::id\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_id_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_name_trampoline<P, F: Fn(&P) + 'static>(this: *mut alsarawmidi_sys::ALSARawmidiSubstreamInfo, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<SubstreamInfo>
        {
            let f: &F = &*(f as *const F);
            f(&SubstreamInfo::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::name\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_name_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_subdevice_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_subdevice_id_trampoline<P, F: Fn(&P) + 'static>(this: *mut alsarawmidi_sys::ALSARawmidiSubstreamInfo, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<SubstreamInfo>
        {
            let f: &F = &*(f as *const F);
            f(&SubstreamInfo::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::subdevice-id\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_subdevice_id_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_subdevice_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_subdevice_name_trampoline<P, F: Fn(&P) + 'static>(this: *mut alsarawmidi_sys::ALSARawmidiSubstreamInfo, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<SubstreamInfo>
        {
            let f: &F = &*(f as *const F);
            f(&SubstreamInfo::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::subdevice-name\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_subdevice_name_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_subdevices_avail_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_subdevices_avail_trampoline<P, F: Fn(&P) + 'static>(this: *mut alsarawmidi_sys::ALSARawmidiSubstreamInfo, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<SubstreamInfo>
        {
            let f: &F = &*(f as *const F);
            f(&SubstreamInfo::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::subdevices-avail\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_subdevices_avail_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_subdevices_count_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_subdevices_count_trampoline<P, F: Fn(&P) + 'static>(this: *mut alsarawmidi_sys::ALSARawmidiSubstreamInfo, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<SubstreamInfo>
        {
            let f: &F = &*(f as *const F);
            f(&SubstreamInfo::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::subdevices-count\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_subdevices_count_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for SubstreamInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SubstreamInfo")
    }
}
