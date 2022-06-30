// SPDX-License-Identifier: MIT

use super::*;

pub trait CardImpl: ObjectImpl + CardImplExt {
    fn handle_elem_event(&self, card: &Card, elem_id: &ElemId, events: ElemEventMask) {
        self.parent_handle_elem_event(card, elem_id, events)
    }
    fn handle_disconnection(&self, card: &Card) {
        self.parent_handle_disconnection(card)
    }
}

pub trait CardImplExt: ObjectSubclass {
    fn parent_handle_elem_event(&self, card: &Card, elem_id: &ElemId, events: ElemEventMask);
    fn parent_handle_disconnection(&self, card: &Card);
}

impl<T: CardImpl> CardImplExt for T {
    fn parent_handle_elem_event(&self, card: &Card, elem_id: &ElemId, events: ElemEventMask) {
        unsafe {
            let data = T::type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut alsactl_sys::ALSACtlCardClass;
            let f = (*parent_class)
                .handle_elem_event
                .expect("No parent class implementation for \"handle_elem_event\"");
            f(
                card.to_glib_none().0,
                elem_id.to_glib_none().0,
                events.to_glib(),
            )
        }
    }

    fn parent_handle_disconnection(&self, card: &Card) {
        unsafe {
            let data = T::type_data();
            let parent_class =
                data.as_ref().get_parent_class() as *mut alsactl_sys::ALSACtlCardClass;
            let f = (*parent_class)
                .handle_disconnection
                .expect("No parent class implementation for \"handle_disconnection\"");
            f(card.to_glib_none().0)
        }
    }
}

unsafe impl<T: CardImpl> IsSubclassable<T> for CardClass {
    fn override_vfuncs(&mut self) {
        <ObjectClass as IsSubclassable<T>>::override_vfuncs(self);
        unsafe {
            let klass = &mut *(self as *mut Self as *mut alsactl_sys::ALSACtlCardClass);
            klass.handle_elem_event = Some(card_handle_elem_event::<T>);
            klass.handle_disconnection = Some(card_handle_disconnection::<T>);
        }
    }
}

unsafe extern "C" fn card_handle_elem_event<T: CardImpl>(
    ptr: *mut alsactl_sys::ALSACtlCard,
    elem_id: *const alsactl_sys::ALSACtlElemId,
    events: alsactl_sys::ALSACtlElemEventMask,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Card> = from_glib_borrow(ptr);

    imp.handle_elem_event(&wrap, &from_glib_borrow(elem_id), from_glib(events))
}

unsafe extern "C" fn card_handle_disconnection<T: CardImpl>(ptr: *mut alsactl_sys::ALSACtlCard) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Card> = from_glib_borrow(ptr);

    imp.handle_disconnection(&wrap)
}
