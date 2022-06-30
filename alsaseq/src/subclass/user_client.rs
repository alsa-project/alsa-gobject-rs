// SPDX-License-Identifier: MIT

use super::*;

pub trait UserClientImpl: ObjectImpl + UserClientImplExt {
    fn handle_event(&self, user_client: &UserClient, event_cntr: &EventCntr) {
        self.parent_handle_event(user_client, event_cntr)
    }
}

pub trait UserClientImplExt: ObjectSubclass {
    fn parent_handle_event(&self, user_client: &UserClient, event_cntr: &EventCntr);
}

impl<T: UserClientImpl> UserClientImplExt for T {
    fn parent_handle_event(&self, user_client: &UserClient, event_cntr: &EventCntr) {
        unsafe {
            let data = T::type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut alsaseq_sys::ALSASeqUserClientClass;
            let f = (*parent_class)
                .handle_event
                .expect("No parent class implementation for \"handle_event\"");
            f(user_client.to_glib_none().0, event_cntr.to_glib_none().0)
        }
    }
}

unsafe impl<T: UserClientImpl> IsSubclassable<T> for UserClientClass {
    fn override_vfuncs(&mut self) {
        <ObjectClass as IsSubclassable<T>>::override_vfuncs(self);
        unsafe {
            let klass = &mut *(self as *mut Self as *mut alsaseq_sys::ALSASeqUserClientClass);
            klass.handle_event = Some(user_client_handle_event::<T>);
        }
    }
}

unsafe extern "C" fn user_client_handle_event<T: UserClientImpl>(
    ptr: *mut alsaseq_sys::ALSASeqUserClient,
    event_cntr: *const alsaseq_sys::ALSASeqEventCntr,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<UserClient> = from_glib_borrow(ptr);

    imp.handle_event(&wrap, &from_glib_none(event_cntr))
}
