// This file was generated by gir (https://github.com/gtk-rs/gir)
// from
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ffi;
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    /// A GObject-derived object to express status of substream.
    ///
    /// A [`SubstreamStatus`][crate::SubstreamStatus] is a GObject-derived object to express status of substream attached
    /// to the pair of stream. The call of [`StreamPairExtManual::substream_status()`][crate::prelude::StreamPairExtManual::substream_status()] returns the instance
    /// of object.
    ///
    /// The object wraps `struct snd_rawmidi_status` in UAPI of Linux sound subsystem.
    ///
    /// ## Properties
    ///
    ///
    /// #### `avail`
    ///  The size of available space in intermediate buffer.
    ///
    /// Readable
    ///
    ///
    /// #### `xruns`
    ///  The count of XRUNs since the last query of status.
    ///
    /// Readable
    ///
    /// # Implements
    ///
    /// [`SubstreamStatusExt`][trait@crate::prelude::SubstreamStatusExt]
    #[doc(alias = "ALSARawmidiSubstreamStatus")]
    pub struct SubstreamStatus(Object<ffi::ALSARawmidiSubstreamStatus, ffi::ALSARawmidiSubstreamStatusClass>);

    match fn {
        type_ => || ffi::alsarawmidi_substream_status_get_type(),
    }
}

impl SubstreamStatus {
    pub const NONE: Option<&'static SubstreamStatus> = None;

    /// Allocate and return an instance of [`SubstreamStatus`][crate::SubstreamStatus].
    ///
    /// # Returns
    ///
    /// An instance of [`SubstreamStatus`][crate::SubstreamStatus].
    #[doc(alias = "alsarawmidi_substream_status_new")]
    pub fn new() -> SubstreamStatus {
        unsafe { from_glib_full(ffi::alsarawmidi_substream_status_new()) }
    }
}

impl Default for SubstreamStatus {
    fn default() -> Self {
        Self::new()
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::SubstreamStatus>> Sealed for T {}
}

/// Trait containing all [`struct@SubstreamStatus`] methods.
///
/// # Implementors
///
/// [`SubstreamStatus`][struct@crate::SubstreamStatus]
pub trait SubstreamStatusExt: IsA<SubstreamStatus> + sealed::Sealed + 'static {
    /// The size of available space in intermediate buffer.
    fn avail(&self) -> u64 {
        ObjectExt::property(self.as_ref(), "avail")
    }

    /// The count of XRUNs since the last query of status.
    fn xruns(&self) -> u64 {
        ObjectExt::property(self.as_ref(), "xruns")
    }

    #[doc(alias = "avail")]
    fn connect_avail_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_avail_trampoline<
            P: IsA<SubstreamStatus>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::ALSARawmidiSubstreamStatus,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(SubstreamStatus::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::avail\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_avail_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "xruns")]
    fn connect_xruns_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_xruns_trampoline<
            P: IsA<SubstreamStatus>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::ALSARawmidiSubstreamStatus,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(SubstreamStatus::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::xruns\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_xruns_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<SubstreamStatus>> SubstreamStatusExt for O {}
