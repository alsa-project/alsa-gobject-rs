// SPDX-License-Identifier: MIT

use super::*;

/// Trait which should be implemented by subclass of [`UserInstance`][crate::UserInstance].
pub trait UserInstanceImpl: ObjectImpl + UserInstanceImplExt {
    fn handle_tick_time_event(&self, user_instance: &Self::Type, event: &TickTimeEvent) {
        self.parent_handle_tick_time_event(user_instance, event)
    }

    fn handle_real_time_event(&self, user_instance: &Self::Type, event: &RealTimeEvent) {
        self.parent_handle_real_time_event(user_instance, event)
    }

    fn handle_disconnection(&self, user_instance: &Self::Type) {
        self.parent_handle_disconnection(user_instance)
    }
}

/// Trait which is automatically implemented to implementator of
/// [`UserInstanceImpl`][self::UserInstanceImpl].
pub trait UserInstanceImplExt: ObjectSubclass {
    fn parent_handle_tick_time_event(&self, user_instance: &Self::Type, event: &TickTimeEvent);
    fn parent_handle_real_time_event(&self, user_instance: &Self::Type, event: &RealTimeEvent);
    fn parent_handle_disconnection(&self, user_instance: &Self::Type);
}

impl<T: UserInstanceImpl> UserInstanceImplExt for T {
    fn parent_handle_tick_time_event(&self, user_instance: &Self::Type, event: &TickTimeEvent) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::ALSATimerUserInstanceClass;
            let f = (*parent_class)
                .handle_tick_time_event
                .expect("No parent class implementation for \"handle_tick_time_event\"");
            f(
                user_instance
                    .unsafe_cast_ref::<UserInstance>()
                    .to_glib_none()
                    .0,
                event.to_glib_none().0,
            )
        }
    }

    fn parent_handle_real_time_event(&self, user_instance: &Self::Type, event: &RealTimeEvent) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::ALSATimerUserInstanceClass;
            let f = (*parent_class)
                .handle_real_time_event
                .expect("No parent class implementation for \"handle_real_time_event\"");
            f(
                user_instance
                    .unsafe_cast_ref::<UserInstance>()
                    .to_glib_none()
                    .0,
                event.to_glib_none().0,
            )
        }
    }

    fn parent_handle_disconnection(&self, user_instance: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::ALSATimerUserInstanceClass;
            let f = (*parent_class)
                .handle_disconnection
                .expect("No parent class implementation for \"handle_disconnection\"");
            f(user_instance
                .unsafe_cast_ref::<UserInstance>()
                .to_glib_none()
                .0)
        }
    }
}

unsafe impl<T: UserInstanceImpl> IsSubclassable<T> for UserInstance {
    fn class_init(class: &mut Class<Self>) {
        Self::parent_class_init::<T>(class);

        let klass = class.as_mut();
        klass.handle_tick_time_event = Some(user_instance_handle_tick_time_event::<T>);
        klass.handle_real_time_event = Some(user_instance_handle_real_time_event::<T>);
        klass.handle_disconnection = Some(user_instance_handle_disconnection::<T>);
    }
}

unsafe extern "C" fn user_instance_handle_tick_time_event<T: UserInstanceImpl>(
    ptr: *mut ffi::ALSATimerUserInstance,
    event: *const ffi::ALSATimerTickTimeEvent,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<UserInstance> = from_glib_borrow(ptr);

    imp.handle_tick_time_event(wrap.unsafe_cast_ref(), &from_glib_none(event))
}

unsafe extern "C" fn user_instance_handle_real_time_event<T: UserInstanceImpl>(
    ptr: *mut ffi::ALSATimerUserInstance,
    event: *const ffi::ALSATimerRealTimeEvent,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<UserInstance> = from_glib_borrow(ptr);

    imp.handle_real_time_event(wrap.unsafe_cast_ref(), &from_glib_none(event))
}

unsafe extern "C" fn user_instance_handle_disconnection<T: UserInstanceImpl>(
    ptr: *mut ffi::ALSATimerUserInstance,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<UserInstance> = from_glib_borrow(ptr);

    imp.handle_disconnection(wrap.unsafe_cast_ref())
}
