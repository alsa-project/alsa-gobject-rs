// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{ffi, ElemInfoCommon, ElemInfoSingleArray};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    /// An object to express information for enumerated type of element.
    ///
    /// A `GObject::Object` derived object class for enumerated type of element.
    ///
    /// The object wraps `struct snd_ctl_elem_info` in UAPI of Linux sound subsystem.
    ///
    /// ## Properties
    ///
    ///
    /// #### `labels`
    ///  The list of indexed labels for the element. There is limitation that:
    ///
    ///  - The length of label including terminator should be within 64 bytes.
    ///  - The total length of labels including terminators should be within (64 * 1024) bytes.
    ///
    /// Readable | Writeable
    /// <details><summary><h4>ElemInfoCommon</h4></summary>
    ///
    ///
    /// #### `access`
    ///  The access permission for the element with [`ElemAccessFlag`][crate::ElemAccessFlag].
    ///
    /// Readable | Writeable
    ///
    ///
    /// #### `elem-id`
    ///  The identifier of element.
    ///
    /// Readable
    ///
    ///
    /// #### `elem-type`
    ///  The type of element, one of [`ElemType`][crate::ElemType].
    ///
    /// Readable | Writeable | Construct Only
    ///
    ///
    /// #### `owner`
    ///  The value of PID for process to own the element.
    ///
    /// Readable
    /// </details>
    /// <details><summary><h4>ElemInfoSingleArray</h4></summary>
    ///
    ///
    /// #### `value-count`
    ///  The count of elements in value array of the element.
    ///
    /// Readable | Writeable
    /// </details>
    ///
    /// # Implements
    ///
    /// [`ElemInfoEnumeratedExt`][trait@crate::prelude::ElemInfoEnumeratedExt], [`ElemInfoCommonExt`][trait@crate::prelude::ElemInfoCommonExt], [`ElemInfoSingleArrayExt`][trait@crate::prelude::ElemInfoSingleArrayExt]
    #[doc(alias = "ALSACtlElemInfoEnumerated")]
    pub struct ElemInfoEnumerated(Object<ffi::ALSACtlElemInfoEnumerated, ffi::ALSACtlElemInfoEnumeratedClass>) @implements ElemInfoCommon, ElemInfoSingleArray;

    match fn {
        type_ => || ffi::alsactl_elem_info_enumerated_get_type(),
    }
}

impl ElemInfoEnumerated {
    pub const NONE: Option<&'static ElemInfoEnumerated> = None;

    /// Allocate and return an instance of [`ElemInfoEnumerated`][crate::ElemInfoEnumerated].
    ///
    /// # Returns
    ///
    /// An instance of [`ElemInfoEnumerated`][crate::ElemInfoEnumerated].
    #[doc(alias = "alsactl_elem_info_enumerated_new")]
    pub fn new() -> ElemInfoEnumerated {
        unsafe { from_glib_full(ffi::alsactl_elem_info_enumerated_new()) }
    }
}

impl Default for ElemInfoEnumerated {
    fn default() -> Self {
        Self::new()
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::ElemInfoEnumerated>> Sealed for T {}
}

/// Trait containing all [`struct@ElemInfoEnumerated`] methods.
///
/// # Implementors
///
/// [`ElemInfoEnumerated`][struct@crate::ElemInfoEnumerated]
pub trait ElemInfoEnumeratedExt: IsA<ElemInfoEnumerated> + sealed::Sealed + 'static {
    /// The list of indexed labels for the element. There is limitation that:
    ///
    ///  - The length of label including terminator should be within 64 bytes.
    ///  - The total length of labels including terminators should be within (64 * 1024) bytes.
    fn labels(&self) -> Vec<glib::GString> {
        ObjectExt::property(self.as_ref(), "labels")
    }

    /// The list of indexed labels for the element. There is limitation that:
    ///
    ///  - The length of label including terminator should be within 64 bytes.
    ///  - The total length of labels including terminators should be within (64 * 1024) bytes.
    fn set_labels(&self, labels: &[&str]) {
        ObjectExt::set_property(self.as_ref(), "labels", labels)
    }

    #[doc(alias = "labels")]
    fn connect_labels_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_labels_trampoline<
            P: IsA<ElemInfoEnumerated>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::ALSACtlElemInfoEnumerated,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ElemInfoEnumerated::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::labels\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_labels_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<ElemInfoEnumerated>> ElemInfoEnumeratedExt for O {}
