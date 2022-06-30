// SPDX-License-Identifier: MIT

use super::*;

pub trait UserInstanceImpl: ObjectImpl + UserInstanceImplExt {
    fn handle_tick_time_event(&self, user_instance: &UserInstance, event: &TickTimeEvent) {
        self.parent_handle_tick_time_event(user_instance, event)
    }

    fn handle_real_time_event(&self, user_instance: &UserInstance, event: &RealTimeEvent) {
        self.parent_handle_real_time_event(user_instance, event)
    }

    fn handle_disconnection(&self, user_instance: &UserInstance) {
        self.parent_handle_disconnection(user_instance)
    }
}

pub trait UserInstanceImplExt: ObjectSubclass {
    fn parent_handle_tick_time_event(&self, user_instance: &UserInstance, event: &TickTimeEvent);
    fn parent_handle_real_time_event(&self, user_instance: &UserInstance, event: &RealTimeEvent);
    fn parent_handle_disconnection(&self, user_instance: &UserInstance);
}

impl<T: UserInstanceImpl> UserInstanceImplExt for T {
    fn parent_handle_tick_time_event(&self, user_instance: &UserInstance, event: &TickTimeEvent) {
        unsafe {
            let data = T::type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut alsatimer_sys::ALSATimerUserInstanceClass;
            let f = (*parent_class)
                .handle_tick_time_event
                .expect("No parent class implementation for \"handle_tick_time_event\"");
            f(user_instance.to_glib_none().0, event.to_glib_none().0)
        }
    }

    fn parent_handle_real_time_event(&self, user_instance: &UserInstance, event: &RealTimeEvent) {
        unsafe {
            let data = T::type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut alsatimer_sys::ALSATimerUserInstanceClass;
            let f = (*parent_class)
                .handle_real_time_event
                .expect("No parent class implementation for \"handle_real_time_event\"");
            f(user_instance.to_glib_none().0, event.to_glib_none().0)
        }
    }

    fn parent_handle_disconnection(&self, user_instance: &UserInstance) {
        unsafe {
            let data = T::type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut alsatimer_sys::ALSATimerUserInstanceClass;
            let f = (*parent_class)
                .handle_disconnection
                .expect("No parent class implementation for \"handle_disconnection\"");
            f(user_instance.to_glib_none().0)
        }
    }
}

unsafe impl<T: UserInstanceImpl> IsSubclassable<T> for UserInstanceClass {
    fn override_vfuncs(&mut self) {
        <ObjectClass as IsSubclassable<T>>::override_vfuncs(self);
        unsafe {
            let klass = &mut *(self as *mut Self as *mut alsatimer_sys::ALSATimerUserInstanceClass);
            klass.handle_tick_time_event = Some(user_instance_handle_tick_time_event::<T>);
            klass.handle_real_time_event = Some(user_instance_handle_real_time_event::<T>);
            klass.handle_disconnection = Some(user_instance_handle_disconnection::<T>);
        }
    }
}

unsafe extern "C" fn user_instance_handle_tick_time_event<T: UserInstanceImpl>(
    ptr: *mut alsatimer_sys::ALSATimerUserInstance,
    event: *const alsatimer_sys::ALSATimerTickTimeEvent,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<UserInstance> = from_glib_borrow(ptr);

    imp.handle_tick_time_event(&wrap, &from_glib_none(event))
}

unsafe extern "C" fn user_instance_handle_real_time_event<T: UserInstanceImpl>(
    ptr: *mut alsatimer_sys::ALSATimerUserInstance,
    event: *const alsatimer_sys::ALSATimerRealTimeEvent,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<UserInstance> = from_glib_borrow(ptr);

    imp.handle_real_time_event(&wrap, &from_glib_none(event))
}

unsafe extern "C" fn user_instance_handle_disconnection<T: UserInstanceImpl>(
    ptr: *mut alsatimer_sys::ALSATimerUserInstance,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<UserInstance> = from_glib_borrow(ptr);

    imp.handle_disconnection(&wrap)
}
