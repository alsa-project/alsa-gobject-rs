// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{ffi, ClientType, FilterAttrFlag};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    /// A GObject-derived object to express information of client.
    ///
    /// A [`ClientInfo`][crate::ClientInfo] is a GObject-derived object to express information of client. The call
    /// of [`client_info()`][crate::client_info()] returns the instance of object.  The call of
    /// [`UserClientExt::set_info()`][crate::prelude::UserClientExt::set_info()] and [`UserClientExtManual::info()`][crate::prelude::UserClientExtManual::info()] require the instance of object.
    ///
    /// The object wraps `struct snd_seq_client_info` in UAPI of Linux sound subsystem.
    ///
    /// ## Properties
    ///
    ///
    /// #### `card-id`
    ///  The numeric ID of sound card. Available in Linux kernel 4.6.0 or later.
    ///
    /// Readable | Writeable
    ///
    ///
    /// #### `client-id`
    ///  The numeric ID of client. One of [`SpecificClientId`][crate::SpecificClientId] is available as well as any
    /// numeric value.
    ///
    /// Readable | Writeable
    ///
    ///
    /// #### `filter-attributes`
    ///  The attributes for event filter.
    ///
    /// Readable | Writeable
    ///
    ///
    /// #### `lost-count`
    ///  The number of lost events.
    ///
    /// Readable
    ///
    ///
    /// #### `name`
    ///  The name of client.
    ///
    /// Readable | Writeable
    ///
    ///
    /// #### `port-count`
    ///  The number of ports opened by the client.
    ///
    /// Readable
    ///
    ///
    /// #### `process-id`
    ///  The process ID for user client, otherwise -1. Available in Linux kernel 4.6.0 or later.
    ///
    /// Readable
    ///
    ///
    /// #### `type`
    ///  The type of client, one of [`ClientType`][crate::ClientType].
    ///
    /// Readable | Writeable
    ///
    ///
    /// #### `use-filter`
    ///  Whether using filter to receive event or not.
    ///
    /// Readable | Writeable
    ///
    /// # Implements
    ///
    /// [`ClientInfoExt`][trait@crate::prelude::ClientInfoExt], [`ClientInfoExtManual`][trait@crate::prelude::ClientInfoExtManual]
    #[doc(alias = "ALSASeqClientInfo")]
    pub struct ClientInfo(Object<ffi::ALSASeqClientInfo, ffi::ALSASeqClientInfoClass>);

    match fn {
        type_ => || ffi::alsaseq_client_info_get_type(),
    }
}

impl ClientInfo {
    pub const NONE: Option<&'static ClientInfo> = None;

    /// Allocate and return an instance of [`ClientInfo`][crate::ClientInfo].
    ///
    /// # Returns
    ///
    /// An instance of [`ClientInfo`][crate::ClientInfo].
    #[doc(alias = "alsaseq_client_info_new")]
    pub fn new() -> ClientInfo {
        unsafe { from_glib_full(ffi::alsaseq_client_info_new()) }
    }
}

impl Default for ClientInfo {
    fn default() -> Self {
        Self::new()
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::ClientInfo>> Sealed for T {}
}

/// Trait containing the part of [`struct@ClientInfo`] methods.
///
/// # Implementors
///
/// [`ClientInfo`][struct@crate::ClientInfo]
pub trait ClientInfoExt: IsA<ClientInfo> + sealed::Sealed + 'static {
    /// The numeric ID of sound card. Available in Linux kernel 4.6.0 or later.
    #[doc(alias = "card-id")]
    fn card_id(&self) -> i32 {
        ObjectExt::property(self.as_ref(), "card-id")
    }

    /// The numeric ID of sound card. Available in Linux kernel 4.6.0 or later.
    #[doc(alias = "card-id")]
    fn set_card_id(&self, card_id: i32) {
        ObjectExt::set_property(self.as_ref(), "card-id", card_id)
    }

    /// The numeric ID of client. One of [`SpecificClientId`][crate::SpecificClientId] is available as well as any
    /// numeric value.
    #[doc(alias = "client-id")]
    fn client_id(&self) -> u8 {
        ObjectExt::property(self.as_ref(), "client-id")
    }

    /// The numeric ID of client. One of [`SpecificClientId`][crate::SpecificClientId] is available as well as any
    /// numeric value.
    #[doc(alias = "client-id")]
    fn set_client_id(&self, client_id: u8) {
        ObjectExt::set_property(self.as_ref(), "client-id", client_id)
    }

    /// The attributes for event filter.
    #[doc(alias = "filter-attributes")]
    fn filter_attributes(&self) -> FilterAttrFlag {
        ObjectExt::property(self.as_ref(), "filter-attributes")
    }

    /// The attributes for event filter.
    #[doc(alias = "filter-attributes")]
    fn set_filter_attributes(&self, filter_attributes: FilterAttrFlag) {
        ObjectExt::set_property(self.as_ref(), "filter-attributes", filter_attributes)
    }

    /// The number of lost events.
    #[doc(alias = "lost-count")]
    fn lost_count(&self) -> i32 {
        ObjectExt::property(self.as_ref(), "lost-count")
    }

    /// The name of client.
    fn name(&self) -> Option<glib::GString> {
        ObjectExt::property(self.as_ref(), "name")
    }

    /// The name of client.
    fn set_name(&self, name: Option<&str>) {
        ObjectExt::set_property(self.as_ref(), "name", name)
    }

    /// The number of ports opened by the client.
    #[doc(alias = "port-count")]
    fn port_count(&self) -> i32 {
        ObjectExt::property(self.as_ref(), "port-count")
    }

    /// The process ID for user client, otherwise -1. Available in Linux kernel 4.6.0 or later.
    #[doc(alias = "process-id")]
    fn process_id(&self) -> i64 {
        ObjectExt::property(self.as_ref(), "process-id")
    }

    /// The type of client, one of [`ClientType`][crate::ClientType].
    #[doc(alias = "type")]
    fn type_(&self) -> ClientType {
        ObjectExt::property(self.as_ref(), "type")
    }

    /// The type of client, one of [`ClientType`][crate::ClientType].
    #[doc(alias = "type")]
    fn set_type(&self, type_: ClientType) {
        ObjectExt::set_property(self.as_ref(), "type", type_)
    }

    /// Whether using filter to receive event or not.
    #[doc(alias = "use-filter")]
    fn uses_filter(&self) -> bool {
        ObjectExt::property(self.as_ref(), "use-filter")
    }

    /// Whether using filter to receive event or not.
    #[doc(alias = "use-filter")]
    fn set_use_filter(&self, use_filter: bool) {
        ObjectExt::set_property(self.as_ref(), "use-filter", use_filter)
    }

    #[doc(alias = "card-id")]
    fn connect_card_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_card_id_trampoline<P: IsA<ClientInfo>, F: Fn(&P) + 'static>(
            this: *mut ffi::ALSASeqClientInfo,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ClientInfo::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::card-id\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_card_id_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "client-id")]
    fn connect_client_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_client_id_trampoline<
            P: IsA<ClientInfo>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::ALSASeqClientInfo,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ClientInfo::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::client-id\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_client_id_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "filter-attributes")]
    fn connect_filter_attributes_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_filter_attributes_trampoline<
            P: IsA<ClientInfo>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::ALSASeqClientInfo,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ClientInfo::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::filter-attributes\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_filter_attributes_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "lost-count")]
    fn connect_lost_count_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_lost_count_trampoline<
            P: IsA<ClientInfo>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::ALSASeqClientInfo,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ClientInfo::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::lost-count\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_lost_count_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "name")]
    fn connect_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_name_trampoline<P: IsA<ClientInfo>, F: Fn(&P) + 'static>(
            this: *mut ffi::ALSASeqClientInfo,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ClientInfo::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::name\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_name_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "port-count")]
    fn connect_port_count_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_port_count_trampoline<
            P: IsA<ClientInfo>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::ALSASeqClientInfo,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ClientInfo::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::port-count\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_port_count_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "process-id")]
    fn connect_process_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_process_id_trampoline<
            P: IsA<ClientInfo>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::ALSASeqClientInfo,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ClientInfo::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::process-id\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_process_id_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "type")]
    fn connect_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_type_trampoline<P: IsA<ClientInfo>, F: Fn(&P) + 'static>(
            this: *mut ffi::ALSASeqClientInfo,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ClientInfo::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::type\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_type_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "use-filter")]
    fn connect_use_filter_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_use_filter_trampoline<
            P: IsA<ClientInfo>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::ALSASeqClientInfo,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ClientInfo::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::use-filter\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_use_filter_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<ClientInfo>> ClientInfoExt for O {}
