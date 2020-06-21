// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use alsatimer_sys;
use glib;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::ptr;
use DeviceId;
use Event;
use EventDataType;
use InstanceInfo;
use SlaveClass;

glib_wrapper! {
    pub struct UserInstance(Object<alsatimer_sys::ALSATimerUserInstance, alsatimer_sys::ALSATimerUserInstanceClass, UserInstanceClass>);

    match fn {
        get_type => || alsatimer_sys::alsatimer_user_instance_get_type(),
    }
}

impl UserInstance {
    pub fn new() -> UserInstance {
        unsafe {
            from_glib_full(alsatimer_sys::alsatimer_user_instance_new())
        }
    }
}

impl Default for UserInstance {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_USER_INSTANCE: Option<&UserInstance> = None;

pub trait UserInstanceExt: 'static {
    fn attach(&self, device_id: &mut DeviceId) -> Result<(), glib::Error>;

    fn attach_as_slave(&self, slave_class: SlaveClass, slave_id: i32) -> Result<(), glib::Error>;

    fn choose_event_data_type(&self, event_data_type: EventDataType) -> Result<(), glib::Error>;

    fn continue_(&self) -> Result<(), glib::Error>;

    fn create_source(&self) -> Result<glib::Source, glib::Error>;

    fn get_info(&self) -> Result<InstanceInfo, glib::Error>;

    fn open(&self, open_flag: i32) -> Result<(), glib::Error>;

    fn pause(&self) -> Result<(), glib::Error>;

    fn start(&self) -> Result<(), glib::Error>;

    fn stop(&self) -> Result<(), glib::Error>;

    fn connect_handle_disconnection<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_handle_event<F: Fn(&Self, &Event) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<UserInstance>> UserInstanceExt for O {
    fn attach(&self, device_id: &mut DeviceId) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = alsatimer_sys::alsatimer_user_instance_attach(self.as_ref().to_glib_none().0, device_id.to_glib_none_mut().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn attach_as_slave(&self, slave_class: SlaveClass, slave_id: i32) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = alsatimer_sys::alsatimer_user_instance_attach_as_slave(self.as_ref().to_glib_none().0, slave_class.to_glib(), slave_id, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn choose_event_data_type(&self, event_data_type: EventDataType) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = alsatimer_sys::alsatimer_user_instance_choose_event_data_type(self.as_ref().to_glib_none().0, event_data_type.to_glib(), &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn continue_(&self) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = alsatimer_sys::alsatimer_user_instance_continue(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn create_source(&self) -> Result<glib::Source, glib::Error> {
        unsafe {
            let mut gsrc = ptr::null_mut();
            let mut error = ptr::null_mut();
            let _ = alsatimer_sys::alsatimer_user_instance_create_source(self.as_ref().to_glib_none().0, &mut gsrc, &mut error);
            if error.is_null() { Ok(from_glib_full(gsrc)) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_info(&self) -> Result<InstanceInfo, glib::Error> {
        unsafe {
            let mut instance_info = ptr::null_mut();
            let mut error = ptr::null_mut();
            let _ = alsatimer_sys::alsatimer_user_instance_get_info(self.as_ref().to_glib_none().0, &mut instance_info, &mut error);
            if error.is_null() { Ok(from_glib_full(instance_info)) } else { Err(from_glib_full(error)) }
        }
    }

    fn open(&self, open_flag: i32) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = alsatimer_sys::alsatimer_user_instance_open(self.as_ref().to_glib_none().0, open_flag, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn pause(&self) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = alsatimer_sys::alsatimer_user_instance_pause(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn start(&self) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = alsatimer_sys::alsatimer_user_instance_start(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn stop(&self) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = alsatimer_sys::alsatimer_user_instance_stop(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn connect_handle_disconnection<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn handle_disconnection_trampoline<P, F: Fn(&P) + 'static>(this: *mut alsatimer_sys::ALSATimerUserInstance, f: glib_sys::gpointer)
            where P: IsA<UserInstance>
        {
            let f: &F = &*(f as *const F);
            f(&UserInstance::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"handle-disconnection\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(handle_disconnection_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_handle_event<F: Fn(&Self, &Event) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn handle_event_trampoline<P, F: Fn(&P, &Event) + 'static>(this: *mut alsatimer_sys::ALSATimerUserInstance, event: *mut alsatimer_sys::ALSATimerEvent, f: glib_sys::gpointer)
            where P: IsA<UserInstance>
        {
            let f: &F = &*(f as *const F);
            f(&UserInstance::from_glib_borrow(this).unsafe_cast_ref(), &from_glib_none(event))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"handle-event\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(handle_event_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for UserInstance {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "UserInstance")
    }
}
