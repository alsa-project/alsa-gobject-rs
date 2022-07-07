// SPDX-License-Identifier: MIT

use super::*;

/// Trait which should be implemented by subclass of [`UserClient`][crate::UserClient].
pub trait UserClientImpl: ObjectImpl + UserClientImplExt {
    fn handle_event(&self, user_client: &Self::Type, event_cntr: &EventCntr) {
        self.parent_handle_event(user_client, event_cntr)
    }
}

/// Trait which is automatically implemented to implementator of
/// [`UserClientImpl`][self::UserClientImpl].
pub trait UserClientImplExt: ObjectSubclass {
    fn parent_handle_event(&self, user_client: &Self::Type, event_cntr: &EventCntr);
}

impl<T: UserClientImpl> UserClientImplExt for T {
    fn parent_handle_event(&self, user_client: &Self::Type, event_cntr: &EventCntr) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::ALSASeqUserClientClass;
            let f = (*parent_class)
                .handle_event
                .expect("No parent class implementation for \"handle_event\"");
            f(
                user_client.unsafe_cast_ref::<UserClient>().to_glib_none().0,
                event_cntr.to_glib_none().0,
            )
        }
    }
}

unsafe impl<T: UserClientImpl> IsSubclassable<T> for UserClient {
    fn class_init(class: &mut Class<Self>) {
        Self::parent_class_init::<T>(class);

        let klass = class.as_mut();
        klass.handle_event = Some(user_client_handle_event::<T>);
    }
}

unsafe extern "C" fn user_client_handle_event<T: UserClientImpl>(
    ptr: *mut ffi::ALSASeqUserClient,
    event_cntr: *const ffi::ALSASeqEventCntr,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<UserClient> = from_glib_borrow(ptr);

    imp.handle_event(wrap.unsafe_cast_ref(), &from_glib_none(event_cntr))
}
