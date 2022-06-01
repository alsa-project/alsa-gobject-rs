// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use alsactl_sys;
use glib::error::ErrorDomain;
use glib::translate::*;
use glib::value::FromValue;
use glib::value::FromValueOptional;
use glib::value::SetValue;
use glib::value::Value;
use glib::Quark;
use glib::StaticType;
use glib::Type;
use gobject_sys;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum CardError {
    Failed,
    Disconnected,
    ElemNotFound,
    ElemNotSupported,
    ElemOwned,
    ElemExist,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for CardError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "CardError::{}",
            match *self {
                CardError::Failed => "Failed",
                CardError::Disconnected => "Disconnected",
                CardError::ElemNotFound => "ElemNotFound",
                CardError::ElemNotSupported => "ElemNotSupported",
                CardError::ElemOwned => "ElemOwned",
                CardError::ElemExist => "ElemExist",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for CardError {
    type GlibType = alsactl_sys::ALSACtlCardError;

    fn to_glib(&self) -> alsactl_sys::ALSACtlCardError {
        match *self {
            CardError::Failed => alsactl_sys::ALSACTL_CARD_ERROR_FAILED,
            CardError::Disconnected => alsactl_sys::ALSACTL_CARD_ERROR_DISCONNECTED,
            CardError::ElemNotFound => alsactl_sys::ALSACTL_CARD_ERROR_ELEM_NOT_FOUND,
            CardError::ElemNotSupported => alsactl_sys::ALSACTL_CARD_ERROR_ELEM_NOT_SUPPORTED,
            CardError::ElemOwned => alsactl_sys::ALSACTL_CARD_ERROR_ELEM_OWNED,
            CardError::ElemExist => alsactl_sys::ALSACTL_CARD_ERROR_ELEM_EXIST,
            CardError::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<alsactl_sys::ALSACtlCardError> for CardError {
    fn from_glib(value: alsactl_sys::ALSACtlCardError) -> Self {
        match value {
            0 => CardError::Failed,
            1 => CardError::Disconnected,
            2 => CardError::ElemNotFound,
            3 => CardError::ElemNotSupported,
            4 => CardError::ElemOwned,
            5 => CardError::ElemExist,
            value => CardError::__Unknown(value),
        }
    }
}

impl ErrorDomain for CardError {
    fn domain() -> Quark {
        unsafe { from_glib(alsactl_sys::alsactl_card_error_quark()) }
    }

    fn code(self) -> i32 {
        self.to_glib()
    }

    fn from(code: i32) -> Option<Self> {
        match code {
            0 => Some(CardError::Failed),
            1 => Some(CardError::Disconnected),
            2 => Some(CardError::ElemNotFound),
            3 => Some(CardError::ElemNotSupported),
            4 => Some(CardError::ElemOwned),
            5 => Some(CardError::ElemExist),
            _ => Some(CardError::Failed),
        }
    }
}

impl StaticType for CardError {
    fn static_type() -> Type {
        unsafe { from_glib(alsactl_sys::alsactl_card_error_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for CardError {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for CardError {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for CardError {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum ElemIfaceType {
    Card,
    Hwdep,
    Mixer,
    Pcm,
    Rawmidi,
    Timer,
    Sequencer,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for ElemIfaceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ElemIfaceType::{}",
            match *self {
                ElemIfaceType::Card => "Card",
                ElemIfaceType::Hwdep => "Hwdep",
                ElemIfaceType::Mixer => "Mixer",
                ElemIfaceType::Pcm => "Pcm",
                ElemIfaceType::Rawmidi => "Rawmidi",
                ElemIfaceType::Timer => "Timer",
                ElemIfaceType::Sequencer => "Sequencer",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for ElemIfaceType {
    type GlibType = alsactl_sys::ALSACtlElemIfaceType;

    fn to_glib(&self) -> alsactl_sys::ALSACtlElemIfaceType {
        match *self {
            ElemIfaceType::Card => alsactl_sys::ALSACTL_ELEM_IFACE_TYPE_CARD,
            ElemIfaceType::Hwdep => alsactl_sys::ALSACTL_ELEM_IFACE_TYPE_HWDEP,
            ElemIfaceType::Mixer => alsactl_sys::ALSACTL_ELEM_IFACE_TYPE_MIXER,
            ElemIfaceType::Pcm => alsactl_sys::ALSACTL_ELEM_IFACE_TYPE_PCM,
            ElemIfaceType::Rawmidi => alsactl_sys::ALSACTL_ELEM_IFACE_TYPE_RAWMIDI,
            ElemIfaceType::Timer => alsactl_sys::ALSACTL_ELEM_IFACE_TYPE_TIMER,
            ElemIfaceType::Sequencer => alsactl_sys::ALSACTL_ELEM_IFACE_TYPE_SEQUENCER,
            ElemIfaceType::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<alsactl_sys::ALSACtlElemIfaceType> for ElemIfaceType {
    fn from_glib(value: alsactl_sys::ALSACtlElemIfaceType) -> Self {
        match value {
            0 => ElemIfaceType::Card,
            1 => ElemIfaceType::Hwdep,
            2 => ElemIfaceType::Mixer,
            3 => ElemIfaceType::Pcm,
            4 => ElemIfaceType::Rawmidi,
            5 => ElemIfaceType::Timer,
            6 => ElemIfaceType::Sequencer,
            value => ElemIfaceType::__Unknown(value),
        }
    }
}

impl StaticType for ElemIfaceType {
    fn static_type() -> Type {
        unsafe { from_glib(alsactl_sys::alsactl_elem_iface_type_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for ElemIfaceType {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for ElemIfaceType {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for ElemIfaceType {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum ElemType {
    None,
    Boolean,
    Integer,
    Enumerated,
    Bytes,
    Iec60958,
    Integer64,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for ElemType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ElemType::{}",
            match *self {
                ElemType::None => "None",
                ElemType::Boolean => "Boolean",
                ElemType::Integer => "Integer",
                ElemType::Enumerated => "Enumerated",
                ElemType::Bytes => "Bytes",
                ElemType::Iec60958 => "Iec60958",
                ElemType::Integer64 => "Integer64",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for ElemType {
    type GlibType = alsactl_sys::ALSACtlElemType;

    fn to_glib(&self) -> alsactl_sys::ALSACtlElemType {
        match *self {
            ElemType::None => alsactl_sys::ALSACTL_ELEM_TYPE_NONE,
            ElemType::Boolean => alsactl_sys::ALSACTL_ELEM_TYPE_BOOLEAN,
            ElemType::Integer => alsactl_sys::ALSACTL_ELEM_TYPE_INTEGER,
            ElemType::Enumerated => alsactl_sys::ALSACTL_ELEM_TYPE_ENUMERATED,
            ElemType::Bytes => alsactl_sys::ALSACTL_ELEM_TYPE_BYTES,
            ElemType::Iec60958 => alsactl_sys::ALSACTL_ELEM_TYPE_IEC60958,
            ElemType::Integer64 => alsactl_sys::ALSACTL_ELEM_TYPE_INTEGER64,
            ElemType::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<alsactl_sys::ALSACtlElemType> for ElemType {
    fn from_glib(value: alsactl_sys::ALSACtlElemType) -> Self {
        match value {
            0 => ElemType::None,
            1 => ElemType::Boolean,
            2 => ElemType::Integer,
            3 => ElemType::Enumerated,
            4 => ElemType::Bytes,
            5 => ElemType::Iec60958,
            6 => ElemType::Integer64,
            value => ElemType::__Unknown(value),
        }
    }
}

impl StaticType for ElemType {
    fn static_type() -> Type {
        unsafe { from_glib(alsactl_sys::alsactl_elem_type_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for ElemType {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for ElemType {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for ElemType {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
pub enum EventType {
    Elem,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for EventType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "EventType::{}",
            match *self {
                EventType::Elem => "Elem",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl ToGlib for EventType {
    type GlibType = alsactl_sys::ALSACtlEventType;

    fn to_glib(&self) -> alsactl_sys::ALSACtlEventType {
        match *self {
            EventType::Elem => alsactl_sys::ALSACTL_EVENT_TYPE_ELEM,
            EventType::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<alsactl_sys::ALSACtlEventType> for EventType {
    fn from_glib(value: alsactl_sys::ALSACtlEventType) -> Self {
        match value {
            0 => EventType::Elem,
            value => EventType::__Unknown(value),
        }
    }
}

impl StaticType for EventType {
    fn static_type() -> Type {
        unsafe { from_glib(alsactl_sys::alsactl_event_type_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for EventType {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for EventType {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_sys::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for EventType {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_sys::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}
