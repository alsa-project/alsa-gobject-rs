// SPDX-License-Identifier: MIT
use super::*;

pub trait CardExtManual {
    #[doc(alias = "alsactl_card_get_protocol_version")]
    fn protocol_version(&self) -> Result<&[u16; 3], glib::Error>;

    #[doc(alias = "alsactl_card_get_elem_id_list")]
    fn elem_id_list(&self) -> Result<Vec<ElemId>, glib::Error>;

    #[doc(alias = "alsactl_card_get_elem_info")]
    fn elem_info(&self, elem_id: &ElemId) -> Result<ElemInfo, glib::Error>;

    #[doc(alias = "alsactl_card_command_elem_tlv")]
    fn command_elem_tlv(&self, elem_id: &ElemId, container: &mut [u32]) -> Result<(), glib::Error>;

    #[doc(alias = "alsactl_card_read_elem_tlv")]
    fn read_elem_tlv(&self, elem_id: &ElemId, container: &mut [u32]) -> Result<(), glib::Error>;

    #[doc(alias = "alsactl_card_read_elem_value")]
    fn read_elem_value<P: IsA<ElemValue>>(
        &self,
        elem_id: &ElemId,
        elem_value: &mut P,
    ) -> Result<(), glib::Error>;

    #[doc(alias = "alsactl_card_add_elems")]
    fn add_elems<P: AsRef<ElemInfoCommon>>(
        &self,
        elem_id: &ElemId,
        elem_count: u32,
        elem_info: &P,
    ) -> Result<Vec<ElemId>, glib::Error>;

    #[doc(alias = "alsactl_card_replace_elems")]
    fn replace_elems<P: AsRef<ElemInfoCommon>>(
        &self,
        elem_id: &ElemId,
        elem_count: u32,
        elem_info: &P,
    ) -> Result<Vec<ElemId>, glib::Error>;
}

impl<O: IsA<Card>> CardExtManual for O {
    fn protocol_version(&self) -> Result<&[u16; 3], glib::Error> {
        unsafe {
            let mut triplet = std::ptr::null_mut() as *const [u16; 3];
            let mut error = std::ptr::null_mut();

            let _ = ffi::alsactl_card_get_protocol_version(
                self.as_ref().to_glib_none().0,
                &mut triplet as *mut *const [u16; 3],
                &mut error,
            );

            if error.is_null() {
                Ok(&*triplet)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn elem_id_list(&self) -> Result<Vec<ElemId>, glib::Error> {
        unsafe {
            let mut entries = std::ptr::null_mut();
            let mut error = std::ptr::null_mut();

            let _ = ffi::alsactl_card_get_elem_id_list(
                self.as_ref().to_glib_none().0,
                &mut entries,
                &mut error,
            );

            if error.is_null() {
                Ok(FromGlibPtrContainer::from_glib_full(entries))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn elem_info(&self, elem_id: &ElemId) -> Result<ElemInfo, glib::Error> {
        unsafe {
            let mut elem_info = std::ptr::null_mut();
            let mut error = std::ptr::null_mut();
            let _ = ffi::alsactl_card_get_elem_info(
                self.as_ref().to_glib_none().0,
                elem_id.to_glib_none().0,
                &mut elem_info,
                &mut error,
            );
            if error.is_null() {
                let obj = ElemInfoCommon::from_glib_full(elem_info);
                Ok(ElemInfo::from(obj))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn read_elem_value<Q: IsA<ElemValue>>(
        &self,
        elem_id: &ElemId,
        elem_value: &mut Q,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();

            let _ = ffi::alsactl_card_read_elem_value(
                self.as_ref().to_glib_none().0,
                elem_id.to_glib_none().0,
                &mut elem_value.as_ref().to_glib_none().0,
                &mut error,
            );

            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn command_elem_tlv(&self, elem_id: &ElemId, container: &mut [u32]) -> Result<(), glib::Error> {
        unsafe {
            let mut container_size = container.len();
            let mut error = std::ptr::null_mut();

            let _ = ffi::alsactl_card_command_elem_tlv(
                self.as_ref().to_glib_none().0,
                elem_id.to_glib_none().0,
                &container.as_mut_ptr(),
                &mut container_size,
                &mut error,
            );

            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn read_elem_tlv(&self, elem_id: &ElemId, container: &mut [u32]) -> Result<(), glib::Error> {
        unsafe {
            let mut container_size = container.len() as usize;
            let mut error = std::ptr::null_mut();

            let _ = ffi::alsactl_card_read_elem_tlv(
                self.as_ref().to_glib_none().0,
                elem_id.to_glib_none().0,
                &container.as_mut_ptr(),
                &mut container_size,
                &mut error,
            );

            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn add_elems<P: AsRef<ElemInfoCommon>>(
        &self,
        elem_id: &ElemId,
        elem_count: u32,
        elem_info: &P,
    ) -> Result<Vec<ElemId>, glib::Error> {
        unsafe {
            let mut entries = std::ptr::null_mut();
            let mut error = std::ptr::null_mut();

            let _ = ffi::alsactl_card_add_elems(
                self.as_ref().to_glib_none().0,
                elem_id.to_glib_none().0,
                elem_count,
                elem_info.as_ref().to_glib_none().0,
                &mut entries,
                &mut error,
            );

            if error.is_null() {
                Ok(FromGlibPtrContainer::from_glib_full(entries))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn replace_elems<P: AsRef<ElemInfoCommon>>(
        &self,
        elem_id: &ElemId,
        elem_count: u32,
        elem_info: &P,
    ) -> Result<Vec<ElemId>, glib::Error> {
        unsafe {
            let mut entries = std::ptr::null_mut();
            let mut error = std::ptr::null_mut();

            let _ = ffi::alsactl_card_replace_elems(
                self.as_ref().to_glib_none().0,
                elem_id.to_glib_none().0,
                elem_count,
                elem_info.as_ref().to_glib_none().0,
                &mut entries,
                &mut error,
            );

            if error.is_null() {
                Ok(FromGlibPtrContainer::from_glib_full(entries))
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}
