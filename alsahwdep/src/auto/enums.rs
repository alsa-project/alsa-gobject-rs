// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::translate::*;
use glib::value::FromValue;
use glib::value::ToValue;
use glib::StaticType;
use glib::Type;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "ALSAHwdepIfaceType")]
pub enum IfaceType {
    #[doc(alias = "ALSAHWDEP_IFACE_TYPE_OPL2")]
    Opl2,
    #[doc(alias = "ALSAHWDEP_IFACE_TYPE_OPL3")]
    Opl3,
    #[doc(alias = "ALSAHWDEP_IFACE_TYPE_OPL4")]
    Opl4,
    #[doc(alias = "ALSAHWDEP_IFACE_TYPE_SB16CSP")]
    Sb16csp,
    #[doc(alias = "ALSAHWDEP_IFACE_TYPE_EMU10K1")]
    Emu10k1,
    #[doc(alias = "ALSAHWDEP_IFACE_TYPE_YSS225")]
    Yss225,
    #[doc(alias = "ALSAHWDEP_IFACE_TYPE_ICS2115")]
    Ics2115,
    #[doc(alias = "ALSAHWDEP_IFACE_TYPE_SSCAPE")]
    Sscape,
    #[doc(alias = "ALSAHWDEP_IFACE_TYPE_VX")]
    Vx,
    #[doc(alias = "ALSAHWDEP_IFACE_TYPE_MIXART")]
    Mixart,
    #[doc(alias = "ALSAHWDEP_IFACE_TYPE_USX2Y")]
    Usx2y,
    #[doc(alias = "ALSAHWDEP_IFACE_TYPE_EMUX_WAVETABLE")]
    EmuxWavetable,
    #[doc(alias = "ALSAHWDEP_IFACE_TYPE_BLUETOOTH")]
    Bluetooth,
    #[doc(alias = "ALSAHWDEP_IFACE_TYPE_USX2Y_PCM")]
    Usx2yPcm,
    #[doc(alias = "ALSAHWDEP_IFACE_TYPE_PCXHR")]
    Pcxhr,
    #[doc(alias = "ALSAHWDEP_IFACE_TYPE_SB_RC")]
    SbRc,
    #[doc(alias = "ALSAHWDEP_IFACE_TYPE_HDA")]
    Hda,
    #[doc(alias = "ALSAHWDEP_IFACE_TYPE_USB_STREAM")]
    UsbStream,
    #[doc(alias = "ALSAHWDEP_IFACE_TYPE_FW_DICE")]
    FwDice,
    #[doc(alias = "ALSAHWDEP_IFACE_TYPE_FW_FIREWORKS")]
    FwFireworks,
    #[doc(alias = "ALSAHWDEP_IFACE_TYPE_FW_BEBOB")]
    FwBebob,
    #[doc(alias = "ALSAHWDEP_IFACE_TYPE_FW_OXFW")]
    FwOxfw,
    #[doc(alias = "ALSAHWDEP_IFACE_TYPE_FW_DIGI00X")]
    FwDigi00x,
    #[doc(alias = "ALSAHWDEP_IFACE_TYPE_FW_TASCAM")]
    FwTascam,
    #[doc(alias = "ALSAHWDEP_IFACE_TYPE_LINE6")]
    Line6,
    #[doc(alias = "ALSAHWDEP_IFACE_TYPE_FW_MOTU")]
    FwMotu,
    #[doc(alias = "ALSAHWDEP_IFACE_TYPE_FW_FIREFACE")]
    FwFireface,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for IfaceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "IfaceType::{}",
            match *self {
                Self::Opl2 => "Opl2",
                Self::Opl3 => "Opl3",
                Self::Opl4 => "Opl4",
                Self::Sb16csp => "Sb16csp",
                Self::Emu10k1 => "Emu10k1",
                Self::Yss225 => "Yss225",
                Self::Ics2115 => "Ics2115",
                Self::Sscape => "Sscape",
                Self::Vx => "Vx",
                Self::Mixart => "Mixart",
                Self::Usx2y => "Usx2y",
                Self::EmuxWavetable => "EmuxWavetable",
                Self::Bluetooth => "Bluetooth",
                Self::Usx2yPcm => "Usx2yPcm",
                Self::Pcxhr => "Pcxhr",
                Self::SbRc => "SbRc",
                Self::Hda => "Hda",
                Self::UsbStream => "UsbStream",
                Self::FwDice => "FwDice",
                Self::FwFireworks => "FwFireworks",
                Self::FwBebob => "FwBebob",
                Self::FwOxfw => "FwOxfw",
                Self::FwDigi00x => "FwDigi00x",
                Self::FwTascam => "FwTascam",
                Self::Line6 => "Line6",
                Self::FwMotu => "FwMotu",
                Self::FwFireface => "FwFireface",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl IntoGlib for IfaceType {
    type GlibType = ffi::ALSAHwdepIfaceType;

    fn into_glib(self) -> ffi::ALSAHwdepIfaceType {
        match self {
            Self::Opl2 => ffi::ALSAHWDEP_IFACE_TYPE_OPL2,
            Self::Opl3 => ffi::ALSAHWDEP_IFACE_TYPE_OPL3,
            Self::Opl4 => ffi::ALSAHWDEP_IFACE_TYPE_OPL4,
            Self::Sb16csp => ffi::ALSAHWDEP_IFACE_TYPE_SB16CSP,
            Self::Emu10k1 => ffi::ALSAHWDEP_IFACE_TYPE_EMU10K1,
            Self::Yss225 => ffi::ALSAHWDEP_IFACE_TYPE_YSS225,
            Self::Ics2115 => ffi::ALSAHWDEP_IFACE_TYPE_ICS2115,
            Self::Sscape => ffi::ALSAHWDEP_IFACE_TYPE_SSCAPE,
            Self::Vx => ffi::ALSAHWDEP_IFACE_TYPE_VX,
            Self::Mixart => ffi::ALSAHWDEP_IFACE_TYPE_MIXART,
            Self::Usx2y => ffi::ALSAHWDEP_IFACE_TYPE_USX2Y,
            Self::EmuxWavetable => ffi::ALSAHWDEP_IFACE_TYPE_EMUX_WAVETABLE,
            Self::Bluetooth => ffi::ALSAHWDEP_IFACE_TYPE_BLUETOOTH,
            Self::Usx2yPcm => ffi::ALSAHWDEP_IFACE_TYPE_USX2Y_PCM,
            Self::Pcxhr => ffi::ALSAHWDEP_IFACE_TYPE_PCXHR,
            Self::SbRc => ffi::ALSAHWDEP_IFACE_TYPE_SB_RC,
            Self::Hda => ffi::ALSAHWDEP_IFACE_TYPE_HDA,
            Self::UsbStream => ffi::ALSAHWDEP_IFACE_TYPE_USB_STREAM,
            Self::FwDice => ffi::ALSAHWDEP_IFACE_TYPE_FW_DICE,
            Self::FwFireworks => ffi::ALSAHWDEP_IFACE_TYPE_FW_FIREWORKS,
            Self::FwBebob => ffi::ALSAHWDEP_IFACE_TYPE_FW_BEBOB,
            Self::FwOxfw => ffi::ALSAHWDEP_IFACE_TYPE_FW_OXFW,
            Self::FwDigi00x => ffi::ALSAHWDEP_IFACE_TYPE_FW_DIGI00X,
            Self::FwTascam => ffi::ALSAHWDEP_IFACE_TYPE_FW_TASCAM,
            Self::Line6 => ffi::ALSAHWDEP_IFACE_TYPE_LINE6,
            Self::FwMotu => ffi::ALSAHWDEP_IFACE_TYPE_FW_MOTU,
            Self::FwFireface => ffi::ALSAHWDEP_IFACE_TYPE_FW_FIREFACE,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::ALSAHwdepIfaceType> for IfaceType {
    unsafe fn from_glib(value: ffi::ALSAHwdepIfaceType) -> Self {
        match value {
            ffi::ALSAHWDEP_IFACE_TYPE_OPL2 => Self::Opl2,
            ffi::ALSAHWDEP_IFACE_TYPE_OPL3 => Self::Opl3,
            ffi::ALSAHWDEP_IFACE_TYPE_OPL4 => Self::Opl4,
            ffi::ALSAHWDEP_IFACE_TYPE_SB16CSP => Self::Sb16csp,
            ffi::ALSAHWDEP_IFACE_TYPE_EMU10K1 => Self::Emu10k1,
            ffi::ALSAHWDEP_IFACE_TYPE_YSS225 => Self::Yss225,
            ffi::ALSAHWDEP_IFACE_TYPE_ICS2115 => Self::Ics2115,
            ffi::ALSAHWDEP_IFACE_TYPE_SSCAPE => Self::Sscape,
            ffi::ALSAHWDEP_IFACE_TYPE_VX => Self::Vx,
            ffi::ALSAHWDEP_IFACE_TYPE_MIXART => Self::Mixart,
            ffi::ALSAHWDEP_IFACE_TYPE_USX2Y => Self::Usx2y,
            ffi::ALSAHWDEP_IFACE_TYPE_EMUX_WAVETABLE => Self::EmuxWavetable,
            ffi::ALSAHWDEP_IFACE_TYPE_BLUETOOTH => Self::Bluetooth,
            ffi::ALSAHWDEP_IFACE_TYPE_USX2Y_PCM => Self::Usx2yPcm,
            ffi::ALSAHWDEP_IFACE_TYPE_PCXHR => Self::Pcxhr,
            ffi::ALSAHWDEP_IFACE_TYPE_SB_RC => Self::SbRc,
            ffi::ALSAHWDEP_IFACE_TYPE_HDA => Self::Hda,
            ffi::ALSAHWDEP_IFACE_TYPE_USB_STREAM => Self::UsbStream,
            ffi::ALSAHWDEP_IFACE_TYPE_FW_DICE => Self::FwDice,
            ffi::ALSAHWDEP_IFACE_TYPE_FW_FIREWORKS => Self::FwFireworks,
            ffi::ALSAHWDEP_IFACE_TYPE_FW_BEBOB => Self::FwBebob,
            ffi::ALSAHWDEP_IFACE_TYPE_FW_OXFW => Self::FwOxfw,
            ffi::ALSAHWDEP_IFACE_TYPE_FW_DIGI00X => Self::FwDigi00x,
            ffi::ALSAHWDEP_IFACE_TYPE_FW_TASCAM => Self::FwTascam,
            ffi::ALSAHWDEP_IFACE_TYPE_LINE6 => Self::Line6,
            ffi::ALSAHWDEP_IFACE_TYPE_FW_MOTU => Self::FwMotu,
            ffi::ALSAHWDEP_IFACE_TYPE_FW_FIREFACE => Self::FwFireface,
            value => Self::__Unknown(value),
        }
    }
}

impl StaticType for IfaceType {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::alsahwdep_iface_type_get_type()) }
    }
}

impl glib::value::ValueType for IfaceType {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for IfaceType {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for IfaceType {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}
