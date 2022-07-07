// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Addr;
use crate::EventTstampMode;
use crate::PortAttrFlag;
use crate::PortCapFlag;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "ALSASeqPortInfo")]
    pub struct PortInfo(Object<ffi::ALSASeqPortInfo, ffi::ALSASeqPortInfoClass>);

    match fn {
        type_ => || ffi::alsaseq_port_info_get_type(),
    }
}

impl PortInfo {
    pub const NONE: Option<&'static PortInfo> = None;

    #[doc(alias = "alsaseq_port_info_new")]
    pub fn new() -> PortInfo {
        unsafe { from_glib_full(ffi::alsaseq_port_info_new()) }
    }
}

impl Default for PortInfo {
    fn default() -> Self {
        Self::new()
    }
}

pub trait PortInfoExt: 'static {
    fn addr(&self) -> Option<Addr>;

    fn attrs(&self) -> PortAttrFlag;

    fn set_attrs(&self, attrs: PortAttrFlag);

    fn caps(&self) -> PortCapFlag;

    fn set_caps(&self, caps: PortCapFlag);

    #[doc(alias = "midi-channels")]
    fn midi_channels(&self) -> i32;

    #[doc(alias = "midi-channels")]
    fn set_midi_channels(&self, midi_channels: i32);

    #[doc(alias = "midi-voices")]
    fn midi_voices(&self) -> i32;

    #[doc(alias = "midi-voices")]
    fn set_midi_voices(&self, midi_voices: i32);

    fn name(&self) -> Option<glib::GString>;

    fn set_name(&self, name: Option<&str>);

    #[doc(alias = "queue-id")]
    fn queue_id(&self) -> u8;

    #[doc(alias = "queue-id")]
    fn set_queue_id(&self, queue_id: u8);

    #[doc(alias = "read-users")]
    fn read_users(&self) -> i32;

    #[doc(alias = "synth-voices")]
    fn synth_voices(&self) -> i32;

    #[doc(alias = "synth-voices")]
    fn set_synth_voices(&self, synth_voices: i32);

    #[doc(alias = "tstamp-mode")]
    fn tstamp_mode(&self) -> EventTstampMode;

    #[doc(alias = "tstamp-mode")]
    fn set_tstamp_mode(&self, tstamp_mode: EventTstampMode);

    #[doc(alias = "tstamp-overwrite")]
    fn is_tstamp_overwrite(&self) -> bool;

    #[doc(alias = "tstamp-overwrite")]
    fn set_tstamp_overwrite(&self, tstamp_overwrite: bool);

    #[doc(alias = "write-users")]
    fn write_users(&self) -> i32;

    #[doc(alias = "attrs")]
    fn connect_attrs_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "caps")]
    fn connect_caps_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "midi-channels")]
    fn connect_midi_channels_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "midi-voices")]
    fn connect_midi_voices_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "name")]
    fn connect_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "queue-id")]
    fn connect_queue_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "read-users")]
    fn connect_read_users_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "synth-voices")]
    fn connect_synth_voices_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "tstamp-mode")]
    fn connect_tstamp_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "tstamp-overwrite")]
    fn connect_tstamp_overwrite_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "write-users")]
    fn connect_write_users_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<PortInfo>> PortInfoExt for O {
    fn addr(&self) -> Option<Addr> {
        glib::ObjectExt::property(self.as_ref(), "addr")
    }

    fn attrs(&self) -> PortAttrFlag {
        glib::ObjectExt::property(self.as_ref(), "attrs")
    }

    fn set_attrs(&self, attrs: PortAttrFlag) {
        glib::ObjectExt::set_property(self.as_ref(), "attrs", &attrs)
    }

    fn caps(&self) -> PortCapFlag {
        glib::ObjectExt::property(self.as_ref(), "caps")
    }

    fn set_caps(&self, caps: PortCapFlag) {
        glib::ObjectExt::set_property(self.as_ref(), "caps", &caps)
    }

    fn midi_channels(&self) -> i32 {
        glib::ObjectExt::property(self.as_ref(), "midi-channels")
    }

    fn set_midi_channels(&self, midi_channels: i32) {
        glib::ObjectExt::set_property(self.as_ref(), "midi-channels", &midi_channels)
    }

    fn midi_voices(&self) -> i32 {
        glib::ObjectExt::property(self.as_ref(), "midi-voices")
    }

    fn set_midi_voices(&self, midi_voices: i32) {
        glib::ObjectExt::set_property(self.as_ref(), "midi-voices", &midi_voices)
    }

    fn name(&self) -> Option<glib::GString> {
        glib::ObjectExt::property(self.as_ref(), "name")
    }

    fn set_name(&self, name: Option<&str>) {
        glib::ObjectExt::set_property(self.as_ref(), "name", &name)
    }

    fn queue_id(&self) -> u8 {
        glib::ObjectExt::property(self.as_ref(), "queue-id")
    }

    fn set_queue_id(&self, queue_id: u8) {
        glib::ObjectExt::set_property(self.as_ref(), "queue-id", &queue_id)
    }

    fn read_users(&self) -> i32 {
        glib::ObjectExt::property(self.as_ref(), "read-users")
    }

    fn synth_voices(&self) -> i32 {
        glib::ObjectExt::property(self.as_ref(), "synth-voices")
    }

    fn set_synth_voices(&self, synth_voices: i32) {
        glib::ObjectExt::set_property(self.as_ref(), "synth-voices", &synth_voices)
    }

    fn tstamp_mode(&self) -> EventTstampMode {
        glib::ObjectExt::property(self.as_ref(), "tstamp-mode")
    }

    fn set_tstamp_mode(&self, tstamp_mode: EventTstampMode) {
        glib::ObjectExt::set_property(self.as_ref(), "tstamp-mode", &tstamp_mode)
    }

    fn is_tstamp_overwrite(&self) -> bool {
        glib::ObjectExt::property(self.as_ref(), "tstamp-overwrite")
    }

    fn set_tstamp_overwrite(&self, tstamp_overwrite: bool) {
        glib::ObjectExt::set_property(self.as_ref(), "tstamp-overwrite", &tstamp_overwrite)
    }

    fn write_users(&self) -> i32 {
        glib::ObjectExt::property(self.as_ref(), "write-users")
    }

    fn connect_attrs_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_attrs_trampoline<P: IsA<PortInfo>, F: Fn(&P) + 'static>(
            this: *mut ffi::ALSASeqPortInfo,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(PortInfo::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::attrs\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_attrs_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_caps_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_caps_trampoline<P: IsA<PortInfo>, F: Fn(&P) + 'static>(
            this: *mut ffi::ALSASeqPortInfo,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(PortInfo::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::caps\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_caps_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_midi_channels_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_midi_channels_trampoline<
            P: IsA<PortInfo>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::ALSASeqPortInfo,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(PortInfo::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::midi-channels\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_midi_channels_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_midi_voices_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_midi_voices_trampoline<
            P: IsA<PortInfo>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::ALSASeqPortInfo,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(PortInfo::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::midi-voices\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_midi_voices_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_name_trampoline<P: IsA<PortInfo>, F: Fn(&P) + 'static>(
            this: *mut ffi::ALSASeqPortInfo,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(PortInfo::from_glib_borrow(this).unsafe_cast_ref())
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

    fn connect_queue_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_queue_id_trampoline<P: IsA<PortInfo>, F: Fn(&P) + 'static>(
            this: *mut ffi::ALSASeqPortInfo,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(PortInfo::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::queue-id\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_queue_id_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_read_users_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_read_users_trampoline<P: IsA<PortInfo>, F: Fn(&P) + 'static>(
            this: *mut ffi::ALSASeqPortInfo,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(PortInfo::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::read-users\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_read_users_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_synth_voices_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_synth_voices_trampoline<
            P: IsA<PortInfo>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::ALSASeqPortInfo,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(PortInfo::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::synth-voices\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_synth_voices_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_tstamp_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_tstamp_mode_trampoline<
            P: IsA<PortInfo>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::ALSASeqPortInfo,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(PortInfo::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::tstamp-mode\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_tstamp_mode_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_tstamp_overwrite_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_tstamp_overwrite_trampoline<
            P: IsA<PortInfo>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::ALSASeqPortInfo,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(PortInfo::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::tstamp-overwrite\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_tstamp_overwrite_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_write_users_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_write_users_trampoline<
            P: IsA<PortInfo>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::ALSASeqPortInfo,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(PortInfo::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::write-users\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_write_users_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for PortInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("PortInfo")
    }
}
