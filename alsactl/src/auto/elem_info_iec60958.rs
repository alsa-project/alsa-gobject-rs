// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{ffi, ElemInfoCommon};
use glib::translate::*;

glib::wrapper! {
    /// An object to express information for iec60958 type of element.
    ///
    /// A `GObject::Object` derived object class for iec60958 type of element.
    ///
    /// The object wraps `struct snd_ctl_elem_info` in UAPI of Linux sound subsystem.
    ///
    /// # Implements
    ///
    /// [`ElemInfoCommonExt`][trait@crate::prelude::ElemInfoCommonExt]
    #[doc(alias = "ALSACtlElemInfoIec60958")]
    pub struct ElemInfoIec60958(Object<ffi::ALSACtlElemInfoIec60958, ffi::ALSACtlElemInfoIec60958Class>) @implements ElemInfoCommon;

    match fn {
        type_ => || ffi::alsactl_elem_info_iec60958_get_type(),
    }
}

impl ElemInfoIec60958 {
    pub const NONE: Option<&'static ElemInfoIec60958> = None;

    /// Allocate and return an instance of [`ElemInfoIec60958`][crate::ElemInfoIec60958].
    ///
    /// # Returns
    ///
    /// An instance of [`ElemInfoIec60958`][crate::ElemInfoIec60958].
    #[doc(alias = "alsactl_elem_info_iec60958_new")]
    pub fn new() -> ElemInfoIec60958 {
        unsafe { from_glib_full(ffi::alsactl_elem_info_iec60958_new()) }
    }
}

impl Default for ElemInfoIec60958 {
    fn default() -> Self {
        Self::new()
    }
}
