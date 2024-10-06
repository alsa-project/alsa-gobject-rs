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
    /// A GObject-derived object to express status of user instance.
    ///
    /// A [`InstanceStatus`][crate::InstanceStatus] is a GObject-derived object to express status of user instance attached
    /// to any timer device or the other instance as slave. The call of [`UserInstanceExtManual::status()`][crate::prelude::UserInstanceExtManual::status()]
    /// returns the instance of object.
    ///
    /// The object wraps `struct snd_timer_status` in UAPI of Linux sound subsystem.
    ///
    /// ## Properties
    ///
    ///
    /// #### `interval`
    ///  The current interval in nano second.
    ///
    /// Readable
    ///
    ///
    /// #### `lost`
    ///  The count of losts master ticks.
    ///
    /// Readable
    ///
    ///
    /// #### `overrun`
    ///  The count of overrun in read queue.
    ///
    /// Readable
    ///
    ///
    /// #### `queue-size`
    ///  The current size of queue.
    ///
    /// Readable
    ///
    /// # Implements
    ///
    /// [`InstanceStatusExt`][trait@crate::prelude::InstanceStatusExt], [`InstanceStatusExtManual`][trait@crate::prelude::InstanceStatusExtManual]
    #[doc(alias = "ALSATimerInstanceStatus")]
    pub struct InstanceStatus(Object<ffi::ALSATimerInstanceStatus, ffi::ALSATimerInstanceStatusClass>);

    match fn {
        type_ => || ffi::alsatimer_instance_status_get_type(),
    }
}

impl InstanceStatus {
    pub const NONE: Option<&'static InstanceStatus> = None;

    /// Allocate and return an instance of [`InstanceStatus`][crate::InstanceStatus].
    ///
    /// # Returns
    ///
    /// A [`InstanceStatus`][crate::InstanceStatus].
    #[doc(alias = "alsatimer_instance_status_new")]
    pub fn new() -> InstanceStatus {
        unsafe { from_glib_full(ffi::alsatimer_instance_status_new()) }
    }
}

impl Default for InstanceStatus {
    fn default() -> Self {
        Self::new()
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::InstanceStatus>> Sealed for T {}
}

/// Trait containing the part of [`struct@InstanceStatus`] methods.
///
/// # Implementors
///
/// [`InstanceStatus`][struct@crate::InstanceStatus]
pub trait InstanceStatusExt: IsA<InstanceStatus> + sealed::Sealed + 'static {
    /// The current interval in nano second.
    fn interval(&self) -> u32 {
        ObjectExt::property(self.as_ref(), "interval")
    }

    /// The count of losts master ticks.
    fn lost(&self) -> u32 {
        ObjectExt::property(self.as_ref(), "lost")
    }

    /// The count of overrun in read queue.
    fn overrun(&self) -> u32 {
        ObjectExt::property(self.as_ref(), "overrun")
    }

    /// The current size of queue.
    #[doc(alias = "queue-size")]
    fn queue_size(&self) -> u32 {
        ObjectExt::property(self.as_ref(), "queue-size")
    }

    #[doc(alias = "interval")]
    fn connect_interval_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_interval_trampoline<
            P: IsA<InstanceStatus>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::ALSATimerInstanceStatus,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(InstanceStatus::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::interval\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_interval_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "lost")]
    fn connect_lost_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_lost_trampoline<P: IsA<InstanceStatus>, F: Fn(&P) + 'static>(
            this: *mut ffi::ALSATimerInstanceStatus,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(InstanceStatus::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::lost\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_lost_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "overrun")]
    fn connect_overrun_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_overrun_trampoline<
            P: IsA<InstanceStatus>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::ALSATimerInstanceStatus,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(InstanceStatus::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::overrun\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_overrun_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "queue-size")]
    fn connect_queue_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_queue_size_trampoline<
            P: IsA<InstanceStatus>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::ALSATimerInstanceStatus,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(InstanceStatus::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::queue-size\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_queue_size_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<InstanceStatus>> InstanceStatusExt for O {}
