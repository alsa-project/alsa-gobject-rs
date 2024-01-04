// SPDX-License-Identifier: MIT

use super::*;

/// Trait which should be implemented by subclass of [`Card`][crate::Card].
pub trait CardImpl: ObjectImpl + CardImplExt {
    /// Class closure for the [`handle-elem-event`][struct@crate::Card#handle-elem-event] signal.
    /// ## `elem_id`
    /// A [`ElemId`][crate::ElemId].
    /// ## `events`
    /// A set of [`ElemEventMask`][crate::ElemEventMask].
    fn handle_elem_event(&self, card: &Self::Type, elem_id: &ElemId, events: ElemEventMask) {
        self.parent_handle_elem_event(card, elem_id, events)
    }
    /// Class closure for the [`handle-disconnection`][struct@crate::Card#handle-disconnection] signal.
    fn handle_disconnection(&self, card: &Self::Type) {
        self.parent_handle_disconnection(card)
    }
}

/// Trait which is automatically implemented to implementator of [`CardImpl`][self::CardImpl].
pub trait CardImplExt: ObjectSubclass {
    fn parent_handle_elem_event(&self, card: &Self::Type, elem_id: &ElemId, events: ElemEventMask);
    fn parent_handle_disconnection(&self, card: &Self::Type);
}

impl<T: CardImpl> CardImplExt for T {
    fn parent_handle_elem_event(&self, card: &Self::Type, elem_id: &ElemId, events: ElemEventMask) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::ALSACtlCardClass;
            let f = (*parent_class)
                .handle_elem_event
                .expect("No parent class implementation for \"handle_elem_event\"");
            f(
                card.unsafe_cast_ref::<Card>().to_glib_none().0,
                elem_id.to_glib_none().0,
                events.into_glib(),
            )
        }
    }

    fn parent_handle_disconnection(&self, card: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::ALSACtlCardClass;
            let f = (*parent_class)
                .handle_disconnection
                .expect("No parent class implementation for \"handle_disconnection\"");
            f(card.unsafe_cast_ref::<Card>().to_glib_none().0)
        }
    }
}

unsafe impl<T: CardImpl> IsSubclassable<T> for Card {
    fn class_init(class: &mut Class<Self>) {
        Self::parent_class_init::<T>(class);

        let klass = class.as_mut();
        klass.handle_elem_event = Some(card_handle_elem_event::<T>);
        klass.handle_disconnection = Some(card_handle_disconnection::<T>);
    }
}

unsafe extern "C" fn card_handle_elem_event<T: CardImpl>(
    ptr: *mut ffi::ALSACtlCard,
    elem_id: *const ffi::ALSACtlElemId,
    events: ffi::ALSACtlElemEventMask,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<Card> = from_glib_borrow(ptr);

    imp.handle_elem_event(
        wrap.unsafe_cast_ref(),
        &from_glib_borrow(elem_id),
        from_glib(events),
    )
}

unsafe extern "C" fn card_handle_disconnection<T: CardImpl>(ptr: *mut ffi::ALSACtlCard) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<Card> = from_glib_borrow(ptr);

    imp.handle_disconnection(wrap.unsafe_cast_ref())
}
