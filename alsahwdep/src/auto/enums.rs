// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use alsahwdep_sys;
use glib::translate::*;
use glib::value::FromValue;
use glib::value::FromValueOptional;
use glib::value::SetValue;
use glib::value::Value;
use glib::StaticType;
use glib::Type;
use gobject_sys;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
#[non_exhaustive]
pub enum IfaceType {
    Opl2,
    Opl3,
    Opl4,
    Sb16csp,
    Emu10k1,
    Yss225,
    Ics2115,
    Sscape,
    Vx,
    Mixart,
    Usx2y,
    EmuxWavetable,
    Bluetooth,
    Usx2yPcm,
    Pcxhr,
    SbRc,
    Hda,
    UsbStream,
    FwDice,
    FwFireworks,
    FwBebob,
    FwOxfw,
    FwDigi00x,
    FwTascam,
    Line6,
    FwMotu,
    FwFireface,
#[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for IfaceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "IfaceType::{}", match *self {
            IfaceType::Opl2 => "Opl2",
            IfaceType::Opl3 => "Opl3",
            IfaceType::Opl4 => "Opl4",
            IfaceType::Sb16csp => "Sb16csp",
            IfaceType::Emu10k1 => "Emu10k1",
            IfaceType::Yss225 => "Yss225",
            IfaceType::Ics2115 => "Ics2115",
            IfaceType::Sscape => "Sscape",
            IfaceType::Vx => "Vx",
            IfaceType::Mixart => "Mixart",
            IfaceType::Usx2y => "Usx2y",
            IfaceType::EmuxWavetable => "EmuxWavetable",
            IfaceType::Bluetooth => "Bluetooth",
            IfaceType::Usx2yPcm => "Usx2yPcm",
            IfaceType::Pcxhr => "Pcxhr",
            IfaceType::SbRc => "SbRc",
            IfaceType::Hda => "Hda",
            IfaceType::UsbStream => "UsbStream",
            IfaceType::FwDice => "FwDice",
            IfaceType::FwFireworks => "FwFireworks",
            IfaceType::FwBebob => "FwBebob",
            IfaceType::FwOxfw => "FwOxfw",
            IfaceType::FwDigi00x => "FwDigi00x",
            IfaceType::FwTascam => "FwTascam",
            IfaceType::Line6 => "Line6",
            IfaceType::FwMotu => "FwMotu",
            IfaceType::FwFireface => "FwFireface",
            _ => "Unknown",
        })
    }
}

#[doc(hidden)]
impl ToGlib for IfaceType {
    type GlibType = alsahwdep_sys::ALSAHwdepIfaceType;

    fn to_glib(&self) -> alsahwdep_sys::ALSAHwdepIfaceType {
        match *self {
            IfaceType::Opl2 => alsahwdep_sys::ALSAHWDEP_IFACE_TYPE_OPL2,
            IfaceType::Opl3 => alsahwdep_sys::ALSAHWDEP_IFACE_TYPE_OPL3,
            IfaceType::Opl4 => alsahwdep_sys::ALSAHWDEP_IFACE_TYPE_OPL4,
            IfaceType::Sb16csp => alsahwdep_sys::ALSAHWDEP_IFACE_TYPE_SB16CSP,
            IfaceType::Emu10k1 => alsahwdep_sys::ALSAHWDEP_IFACE_TYPE_EMU10K1,
            IfaceType::Yss225 => alsahwdep_sys::ALSAHWDEP_IFACE_TYPE_YSS225,
            IfaceType::Ics2115 => alsahwdep_sys::ALSAHWDEP_IFACE_TYPE_ICS2115,
            IfaceType::Sscape => alsahwdep_sys::ALSAHWDEP_IFACE_TYPE_SSCAPE,
            IfaceType::Vx => alsahwdep_sys::ALSAHWDEP_IFACE_TYPE_VX,
            IfaceType::Mixart => alsahwdep_sys::ALSAHWDEP_IFACE_TYPE_MIXART,
            IfaceType::Usx2y => alsahwdep_sys::ALSAHWDEP_IFACE_TYPE_USX2Y,
            IfaceType::EmuxWavetable => alsahwdep_sys::ALSAHWDEP_IFACE_TYPE_EMUX_WAVETABLE,
            IfaceType::Bluetooth => alsahwdep_sys::ALSAHWDEP_IFACE_TYPE_BLUETOOTH,
            IfaceType::Usx2yPcm => alsahwdep_sys::ALSAHWDEP_IFACE_TYPE_USX2Y_PCM,
            IfaceType::Pcxhr => alsahwdep_sys::ALSAHWDEP_IFACE_TYPE_PCXHR,
            IfaceType::SbRc => alsahwdep_sys::ALSAHWDEP_IFACE_TYPE_SB_RC,
            IfaceType::Hda => alsahwdep_sys::ALSAHWDEP_IFACE_TYPE_HDA,
            IfaceType::UsbStream => alsahwdep_sys::ALSAHWDEP_IFACE_TYPE_USB_STREAM,
            IfaceType::FwDice => alsahwdep_sys::ALSAHWDEP_IFACE_TYPE_FW_DICE,
            IfaceType::FwFireworks => alsahwdep_sys::ALSAHWDEP_IFACE_TYPE_FW_FIREWORKS,
            IfaceType::FwBebob => alsahwdep_sys::ALSAHWDEP_IFACE_TYPE_FW_BEBOB,
            IfaceType::FwOxfw => alsahwdep_sys::ALSAHWDEP_IFACE_TYPE_FW_OXFW,
            IfaceType::FwDigi00x => alsahwdep_sys::ALSAHWDEP_IFACE_TYPE_FW_DIGI00X,
            IfaceType::FwTascam => alsahwdep_sys::ALSAHWDEP_IFACE_TYPE_FW_TASCAM,
            IfaceType::Line6 => alsahwdep_sys::ALSAHWDEP_IFACE_TYPE_LINE6,
            IfaceType::FwMotu => alsahwdep_sys::ALSAHWDEP_IFACE_TYPE_FW_MOTU,
            IfaceType::FwFireface => alsahwdep_sys::ALSAHWDEP_IFACE_TYPE_FW_FIREFACE,
            IfaceType::__Unknown(value) => value,
}
    }
}

#[doc(hidden)]
impl FromGlib<alsahwdep_sys::ALSAHwdepIfaceType> for IfaceType {
    fn from_glib(value: alsahwdep_sys::ALSAHwdepIfaceType) -> Self {
        match value {
            0 => IfaceType::Opl2,
            1 => IfaceType::Opl3,
            2 => IfaceType::Opl4,
            3 => IfaceType::Sb16csp,
            4 => IfaceType::Emu10k1,
            5 => IfaceType::Yss225,
            6 => IfaceType::Ics2115,
            7 => IfaceType::Sscape,
            8 => IfaceType::Vx,
            9 => IfaceType::Mixart,
            10 => IfaceType::Usx2y,
            11 => IfaceType::EmuxWavetable,
            12 => IfaceType::Bluetooth,
            13 => IfaceType::Usx2yPcm,
            14 => IfaceType::Pcxhr,
            15 => IfaceType::SbRc,
            16 => IfaceType::Hda,
            17 => IfaceType::UsbStream,
            18 => IfaceType::FwDice,
            19 => IfaceType::FwFireworks,
            20 => IfaceType::FwBebob,
            21 => IfaceType::FwOxfw,
            22 => IfaceType::FwDigi00x,
            23 => IfaceType::FwTascam,
            24 => IfaceType::Line6,
            25 => IfaceType::FwMotu,
            26 => IfaceType::FwFireface,
            value => IfaceType::__Unknown(value),
}
    }
}

impl StaticType for IfaceType {
    fn static_type() -> Type {
        unsafe { from_glib(alsahwdep_sys::alsahwdep_iface_type_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for IfaceType {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for IfaceType {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for IfaceType {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

