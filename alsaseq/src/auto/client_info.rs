// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use alsaseq_sys;
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
use ClientType;
use FilterAttrFlag;

glib_wrapper! {
    pub struct ClientInfo(Object<alsaseq_sys::ALSASeqClientInfo, alsaseq_sys::ALSASeqClientInfoClass, ClientInfoClass>);

    match fn {
        get_type => || alsaseq_sys::alsaseq_client_info_get_type(),
    }
}

impl ClientInfo {
    pub fn new() -> ClientInfo {
        unsafe {
            from_glib_full(alsaseq_sys::alsaseq_client_info_new())
        }
    }
}

impl Default for ClientInfo {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_CLIENT_INFO: Option<&ClientInfo> = None;

pub trait ClientInfoExt: 'static {
    fn get_property_card_id(&self) -> i32;

    fn set_property_card_id(&self, card_id: i32);

    fn get_property_client_id(&self) -> u8;

    fn set_property_client_id(&self, client_id: u8);

    fn get_property_filter_attributes(&self) -> FilterAttrFlag;

    fn set_property_filter_attributes(&self, filter_attributes: FilterAttrFlag);

    fn get_property_lost_count(&self) -> i32;

    fn get_property_name(&self) -> Option<GString>;

    fn set_property_name(&self, name: Option<&str>);

    fn get_property_port_count(&self) -> i32;

    fn get_property_process_id(&self) -> i64;

    fn get_property_type(&self) -> ClientType;

    fn set_property_type(&self, type_: ClientType);

    fn get_property_use_filter(&self) -> bool;

    fn set_property_use_filter(&self, use_filter: bool);

    fn connect_property_card_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_client_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_filter_attributes_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_lost_count_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_port_count_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_process_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_use_filter_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<ClientInfo>> ClientInfoExt for O {
    fn get_property_card_id(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"card-id\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `card-id` getter").unwrap()
        }
    }

    fn set_property_card_id(&self, card_id: i32) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"card-id\0".as_ptr() as *const _, Value::from(&card_id).to_glib_none().0);
        }
    }

    fn get_property_client_id(&self) -> u8 {
        unsafe {
            let mut value = Value::from_type(<u8 as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"client-id\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `client-id` getter").unwrap()
        }
    }

    fn set_property_client_id(&self, client_id: u8) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"client-id\0".as_ptr() as *const _, Value::from(&client_id).to_glib_none().0);
        }
    }

    fn get_property_filter_attributes(&self) -> FilterAttrFlag {
        unsafe {
            let mut value = Value::from_type(<FilterAttrFlag as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"filter-attributes\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `filter-attributes` getter").unwrap()
        }
    }

    fn set_property_filter_attributes(&self, filter_attributes: FilterAttrFlag) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"filter-attributes\0".as_ptr() as *const _, Value::from(&filter_attributes).to_glib_none().0);
        }
    }

    fn get_property_lost_count(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"lost-count\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `lost-count` getter").unwrap()
        }
    }

    fn get_property_name(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"name\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `name` getter")
        }
    }

    fn set_property_name(&self, name: Option<&str>) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"name\0".as_ptr() as *const _, Value::from(name).to_glib_none().0);
        }
    }

    fn get_property_port_count(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"port-count\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `port-count` getter").unwrap()
        }
    }

    fn get_property_process_id(&self) -> i64 {
        unsafe {
            let mut value = Value::from_type(<i64 as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"process-id\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `process-id` getter").unwrap()
        }
    }

    fn get_property_type(&self) -> ClientType {
        unsafe {
            let mut value = Value::from_type(<ClientType as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"type\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `type` getter").unwrap()
        }
    }

    fn set_property_type(&self, type_: ClientType) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"type\0".as_ptr() as *const _, Value::from(&type_).to_glib_none().0);
        }
    }

    fn get_property_use_filter(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"use-filter\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `use-filter` getter").unwrap()
        }
    }

    fn set_property_use_filter(&self, use_filter: bool) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"use-filter\0".as_ptr() as *const _, Value::from(&use_filter).to_glib_none().0);
        }
    }

    fn connect_property_card_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_card_id_trampoline<P, F: Fn(&P) + 'static>(this: *mut alsaseq_sys::ALSASeqClientInfo, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<ClientInfo>
        {
            let f: &F = &*(f as *const F);
            f(&ClientInfo::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::card-id\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_card_id_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_client_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_client_id_trampoline<P, F: Fn(&P) + 'static>(this: *mut alsaseq_sys::ALSASeqClientInfo, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<ClientInfo>
        {
            let f: &F = &*(f as *const F);
            f(&ClientInfo::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::client-id\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_client_id_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_filter_attributes_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_filter_attributes_trampoline<P, F: Fn(&P) + 'static>(this: *mut alsaseq_sys::ALSASeqClientInfo, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<ClientInfo>
        {
            let f: &F = &*(f as *const F);
            f(&ClientInfo::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::filter-attributes\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_filter_attributes_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_lost_count_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_lost_count_trampoline<P, F: Fn(&P) + 'static>(this: *mut alsaseq_sys::ALSASeqClientInfo, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<ClientInfo>
        {
            let f: &F = &*(f as *const F);
            f(&ClientInfo::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::lost-count\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_lost_count_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_name_trampoline<P, F: Fn(&P) + 'static>(this: *mut alsaseq_sys::ALSASeqClientInfo, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<ClientInfo>
        {
            let f: &F = &*(f as *const F);
            f(&ClientInfo::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::name\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_name_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_port_count_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_port_count_trampoline<P, F: Fn(&P) + 'static>(this: *mut alsaseq_sys::ALSASeqClientInfo, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<ClientInfo>
        {
            let f: &F = &*(f as *const F);
            f(&ClientInfo::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::port-count\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_port_count_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_process_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_process_id_trampoline<P, F: Fn(&P) + 'static>(this: *mut alsaseq_sys::ALSASeqClientInfo, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<ClientInfo>
        {
            let f: &F = &*(f as *const F);
            f(&ClientInfo::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::process-id\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_process_id_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_type_trampoline<P, F: Fn(&P) + 'static>(this: *mut alsaseq_sys::ALSASeqClientInfo, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<ClientInfo>
        {
            let f: &F = &*(f as *const F);
            f(&ClientInfo::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::type\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_type_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_property_use_filter_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_use_filter_trampoline<P, F: Fn(&P) + 'static>(this: *mut alsaseq_sys::ALSASeqClientInfo, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
            where P: IsA<ClientInfo>
        {
            let f: &F = &*(f as *const F);
            f(&ClientInfo::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::use-filter\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_use_filter_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for ClientInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ClientInfo")
    }
}