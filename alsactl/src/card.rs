// SPDX-License-Identifier: MIT
use super::*;

/// Trait containing the rest of [`struct@Card`] methods.
pub trait CardExtManual {
    /// Get the version of control protocol currently used. The version is expressed as the array with
    /// three elements; major, minor, and micro version in the order. The length of major version is
    /// 16 bit, the length of minor and micro version is 8 bit each.
    ///
    /// # Returns
    ///
    /// [`true`] when the overall operation finishes successfully, else [`false`].
    ///
    /// ## `proto_ver_triplet`
    /// The version of protocol currently used.
    #[doc(alias = "alsactl_card_get_protocol_version")]
    fn protocol_version(&self) -> Result<&[u16; 3], glib::Error>;

    /// Generate a list of [`ElemId`][crate::ElemId] for ALSA control character device associated
    /// to the sound card.
    ///
    /// The call of function executes several `ioctl(2)` system call with `SNDRV_CTL_IOCTL_ELEM_LIST`
    /// command for ALSA control character device.
    ///
    /// # Returns
    ///
    /// [`true`] when the overall operation finishes successfully, else [`false`].
    ///
    /// ## `entries`
    /// The list of entries for [`ElemId`][crate::ElemId].
    #[doc(alias = "alsactl_card_get_elem_id_list")]
    fn elem_id_list(&self) -> Result<Vec<ElemId>, glib::Error>;

    /// Get information of element corresponding to given id.
    ///
    /// The call of function executes `ioctl(2)` system call with `SNDRV_CTL_IOCTL_ELEM_INFO` command
    /// for ALSA control character device. For enumerated element, it executes the system call for
    /// several times to retrieve all of enumeration labels.
    /// ## `elem_id`
    /// A [`ElemId`][crate::ElemId].
    ///
    /// # Returns
    ///
    /// [`true`] when the overall operation finishes successfully, else [`false`].
    ///
    /// ## `elem_info`
    /// An instance of object which implements [`ElemInfoCommon`][crate::ElemInfoCommon].
    #[doc(alias = "alsactl_card_get_elem_info")]
    fn elem_info(&self, elem_id: &ElemId) -> Result<ElemInfo, glib::Error>;

    /// Command the given array of bytes as Type/Length/Value data for element pointed by the identifier.
    ///
    /// The call of function executes `ioctl(2)` system call with `SNDRV_CTL_IOCTL_TLV_COMMAND` command
    /// for ALSA control character device.
    /// ## `elem_id`
    /// A [`ElemId`][crate::ElemId].
    /// ## `container`
    /// The array with qudalets for Type-Length-Value data.
    ///
    /// # Returns
    ///
    /// [`true`] when the overall operation finishes successfully, else [`false`].
    #[doc(alias = "alsactl_card_command_elem_tlv")]
    fn command_elem_tlv(&self, elem_id: &ElemId, container: &mut [u32]) -> Result<(), glib::Error>;

    /// Read Type/Length/Value data from element pointed by the identifier and fulfil the given array of
    /// bytes with the data.
    ///
    /// The call of function executes `ioctl(2)` system call with `SNDRV_CTL_IOCTL_TLV_READ` command for
    /// ALSA control character device.
    /// ## `elem_id`
    /// A [`ElemId`][crate::ElemId].
    /// ## `container`
    /// The array with qudalets for Type-Length-Value data.
    ///
    /// # Returns
    ///
    /// [`true`] when the overall operation finishes successfully, else [`false`].
    #[doc(alias = "alsactl_card_read_elem_tlv")]
    fn read_elem_tlv(&self, elem_id: &ElemId, container: &mut [u32]) -> Result<(), glib::Error>;

    /// Read given value from element indicated by the given identifier.
    ///
    /// The call of function executes `ioctl(2)` system call with `SNDRV_CTL_IOCTL_ELEM_READ` command
    /// for ALSA control character device.
    /// ## `elem_id`
    /// A [`ElemId`][crate::ElemId].
    /// ## `elem_value`
    /// A derivative of #ALSACtlElemValue.
    ///
    /// # Returns
    ///
    /// [`true`] when the overall operation finishes successfully, else [`false`].
    #[doc(alias = "alsactl_card_read_elem_value")]
    fn read_elem_value<P: IsA<ElemValue>>(
        &self,
        elem_id: &ElemId,
        elem_value: &mut P,
    ) -> Result<(), glib::Error>;

    /// Add the user-defined elements and return the list of their identifier.
    ///
    /// The call of function executes `ioctl(2)` system call with `SNDRV_CTL_IOCTL_ELEM_ADD` command
    /// for ALSA control character device.
    /// ## `elem_id`
    /// A [`ElemId`][crate::ElemId].
    /// ## `elem_count`
    /// The number of elements going to be added.
    /// ## `elem_info`
    /// An instance of object which implements [`ElemInfoCommon`][crate::ElemInfoCommon].
    ///
    /// # Returns
    ///
    /// [`true`] when the overall operation finishes successfully, else [`false`].
    ///
    /// ## `entries`
    /// The list of added element identifiers.
    #[doc(alias = "alsactl_card_add_elems")]
    fn add_elems<P: AsRef<ElemInfoCommon>>(
        &self,
        elem_id: &ElemId,
        elem_count: u32,
        elem_info: &P,
    ) -> Result<Vec<ElemId>, glib::Error>;

    /// Add user-defined elements to replace the existent ones.
    ///
    /// The call of function executes `ioctl(2)` system call with `SNDRV_CTL_IOCTL_ELEM_REPLACE` command
    /// for ALSA control character device.
    /// ## `elem_id`
    /// A [`ElemId`][crate::ElemId].
    /// ## `elem_count`
    /// The number of elements going to be added.
    /// ## `elem_info`
    /// An instance of object which implements [`ElemInfoCommon`][crate::ElemInfoCommon].
    ///
    /// # Returns
    ///
    /// [`true`] when the overall operation finishes successfully, else [`false`].
    ///
    /// ## `entries`
    /// The list of renewed element identifiers.
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

            let is_ok = ffi::alsactl_card_get_protocol_version(
                self.as_ref().to_glib_none().0,
                &mut triplet as *mut *const [u16; 3],
                &mut error,
            );
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());

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

            let is_ok = ffi::alsactl_card_get_elem_id_list(
                self.as_ref().to_glib_none().0,
                &mut entries,
                &mut error,
            );
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());

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
            let is_ok = ffi::alsactl_card_get_elem_info(
                self.as_ref().to_glib_none().0,
                elem_id.to_glib_none().0,
                &mut elem_info,
                &mut error,
            );
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());

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

            let is_ok = ffi::alsactl_card_read_elem_value(
                self.as_ref().to_glib_none().0,
                elem_id.to_glib_none().0,
                &mut elem_value.as_ref().to_glib_none().0,
                &mut error,
            );
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());

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

            let is_ok = ffi::alsactl_card_command_elem_tlv(
                self.as_ref().to_glib_none().0,
                elem_id.to_glib_none().0,
                &container.as_mut_ptr(),
                &mut container_size,
                &mut error,
            );
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());

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

            let is_ok = ffi::alsactl_card_read_elem_tlv(
                self.as_ref().to_glib_none().0,
                elem_id.to_glib_none().0,
                &container.as_mut_ptr(),
                &mut container_size,
                &mut error,
            );
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());

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

            let is_ok = ffi::alsactl_card_add_elems(
                self.as_ref().to_glib_none().0,
                elem_id.to_glib_none().0,
                elem_count,
                elem_info.as_ref().to_glib_none().0,
                &mut entries,
                &mut error,
            );
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());

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

            let is_ok = ffi::alsactl_card_replace_elems(
                self.as_ref().to_glib_none().0,
                elem_id.to_glib_none().0,
                elem_count,
                elem_info.as_ref().to_glib_none().0,
                &mut entries,
                &mut error,
            );
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());

            if error.is_null() {
                Ok(FromGlibPtrContainer::from_glib_full(entries))
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}
