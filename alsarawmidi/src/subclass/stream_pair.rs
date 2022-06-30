// SPDX-License-Identifier: MIT

use super::*;

pub trait StreamPairImpl: ObjectImpl + StreamPairImplExt {
    fn handle_messages(&self, stream_pair: &StreamPair) {
        self.parent_handle_messages(stream_pair)
    }
    fn handle_disconnection(&self, stream_pair: &StreamPair) {
        self.parent_handle_disconnection(stream_pair)
    }
}

pub trait StreamPairImplExt: ObjectSubclass {
    fn parent_handle_messages(&self, stream_pair: &StreamPair);
    fn parent_handle_disconnection(&self, stream_pair: &StreamPair);
}

impl<T: StreamPairImpl> StreamPairImplExt for T {
    fn parent_handle_messages(&self, stream_pair: &StreamPair) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class()
                as *mut alsarawmidi_sys::ALSARawmidiStreamPairClass;
            let f = (*parent_class)
                .handle_messages
                .expect("No parent class implementation for \"handle_messages\"");
            f(stream_pair.to_glib_none().0)
        }
    }

    fn parent_handle_disconnection(&self, stream_pair: &StreamPair) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class()
                as *mut alsarawmidi_sys::ALSARawmidiStreamPairClass;
            let f = (*parent_class)
                .handle_disconnection
                .expect("No parent class implementation for \"handle_disconnection\"");
            f(stream_pair.to_glib_none().0)
        }
    }
}

unsafe impl<T: StreamPairImpl> IsSubclassable<T> for StreamPairClass {
    fn override_vfuncs(&mut self) {
        <ObjectClass as IsSubclassable<T>>::override_vfuncs(self);
        unsafe {
            let klass =
                &mut *(self as *mut Self as *mut alsarawmidi_sys::ALSARawmidiStreamPairClass);
            klass.handle_messages = Some(stream_pair_handle_messages::<T>);
            klass.handle_disconnection = Some(stream_pair_handle_disconnection::<T>);
        }
    }
}

unsafe extern "C" fn stream_pair_handle_messages<T: StreamPairImpl>(
    ptr: *mut alsarawmidi_sys::ALSARawmidiStreamPair,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<StreamPair> = from_glib_borrow(ptr);

    imp.handle_messages(&wrap)
}

unsafe extern "C" fn stream_pair_handle_disconnection<T: StreamPairImpl>(
    ptr: *mut alsarawmidi_sys::ALSARawmidiStreamPair,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<StreamPair> = from_glib_borrow(ptr);

    imp.handle_disconnection(&wrap)
}
