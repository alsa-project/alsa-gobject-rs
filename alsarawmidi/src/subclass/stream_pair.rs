// SPDX-License-Identifier: MIT

use super::*;

/// Trait which should be implemented by subclass of [`StreamPair`][crate::StreamPair].
pub trait StreamPairImpl: ObjectImpl + StreamPairImplExt {
    /// Class closure for the [`handle-messages`][struct@crate::StreamPair#handle-messages] singal.
    fn handle_messages(&self, stream_pair: &Self::Type) {
        self.parent_handle_messages(stream_pair)
    }
    /// Class closure for the [`handle-disconnection`][struct@crate::StreamPair#handle-disconnection] signal.
    fn handle_disconnection(&self, stream_pair: &Self::Type) {
        self.parent_handle_disconnection(stream_pair)
    }
}

/// Trait which is automatically implemented to implementator of
/// [`StreamPairImpl`][self::StreamPairImpl].
pub trait StreamPairImplExt: ObjectSubclass {
    fn parent_handle_messages(&self, stream_pair: &Self::Type);
    fn parent_handle_disconnection(&self, stream_pair: &Self::Type);
}

impl<T: StreamPairImpl> StreamPairImplExt for T {
    fn parent_handle_messages(&self, stream_pair: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::ALSARawmidiStreamPairClass;
            let f = (*parent_class)
                .handle_messages
                .expect("No parent class implementation for \"handle_messages\"");
            f(stream_pair.unsafe_cast_ref::<StreamPair>().to_glib_none().0)
        }
    }

    fn parent_handle_disconnection(&self, stream_pair: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::ALSARawmidiStreamPairClass;
            let f = (*parent_class)
                .handle_disconnection
                .expect("No parent class implementation for \"handle_disconnection\"");
            f(stream_pair.unsafe_cast_ref::<StreamPair>().to_glib_none().0)
        }
    }
}

unsafe impl<T: StreamPairImpl> IsSubclassable<T> for StreamPair {
    fn class_init(class: &mut Class<Self>) {
        Self::parent_class_init::<T>(class);
        let klass = class.as_mut();
        klass.handle_messages = Some(stream_pair_handle_messages::<T>);
        klass.handle_disconnection = Some(stream_pair_handle_disconnection::<T>);
    }
}

unsafe extern "C" fn stream_pair_handle_messages<T: StreamPairImpl>(
    ptr: *mut ffi::ALSARawmidiStreamPair,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<StreamPair> = from_glib_borrow(ptr);

    imp.handle_messages(wrap.unsafe_cast_ref())
}

unsafe extern "C" fn stream_pair_handle_disconnection<T: StreamPairImpl>(
    ptr: *mut ffi::ALSARawmidiStreamPair,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<StreamPair> = from_glib_borrow(ptr);

    imp.handle_disconnection(wrap.unsafe_cast_ref())
}
