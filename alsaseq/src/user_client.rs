// SPDX-License-Identifier: MIT
use crate::*;

pub trait UserClientExtManual {
    fn get_protocol_version(&self) -> Result<&[u16;3], glib::Error>;

    fn create_port<P: IsA<PortInfo>>(
        &self,
        port_info: &mut P,
    ) -> Result<(), glib::Error>;

    fn create_port_at<P: IsA<PortInfo>>(
        &self,
        port_info: &mut P,
        port_id: u8,
    ) -> Result<(), glib::Error>;

    fn create_queue<P: IsA<QueueInfo>>(
        &self,
        queue_info: &mut P,
    ) -> Result<(), glib::Error>;

    fn get_info<P: IsA<ClientInfo>>(&self, client_info: &mut P) -> Result<(), glib::Error>;

    fn get_pool<P: IsA<ClientPool>>(&self, client_pool: &mut P) -> Result<(), glib::Error>;
}

impl<O: IsA<UserClient>> UserClientExtManual for O {
    fn get_protocol_version(&self) -> Result<&[u16;3], glib::Error> {
        unsafe {
            let mut triplet = std::ptr::null_mut() as *const [u16;3];
            let mut error = std::ptr::null_mut();

            alsaseq_sys::alsaseq_user_client_get_protocol_version(
                self.as_ref().to_glib_none().0,
                &mut triplet as *mut *const [u16;3],
                &mut error,
            );

            if error.is_null() {
                Ok(&*triplet)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn create_port<P: IsA<PortInfo>>(
        &self,
        port_info: &mut P,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            alsaseq_sys::alsaseq_user_client_create_port(
                self.as_ref().to_glib_none().0,
                &mut port_info.as_ref().to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn create_port_at<P: IsA<PortInfo>>(
        &self,
        port_info: &mut P,
        port_id: u8,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            alsaseq_sys::alsaseq_user_client_create_port_at(
                self.as_ref().to_glib_none().0,
                &mut port_info.as_ref().to_glib_none().0,
                port_id,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn create_queue<P: IsA<QueueInfo>>(
        &self,
        queue_info: &mut P,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();

            alsaseq_sys::alsaseq_user_client_create_queue(
                self.as_ref().to_glib_none().0,
                &mut queue_info.as_ref().to_glib_none().0,
                &mut error,
            );

            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn get_info<P: IsA<ClientInfo>>(&self, client_info: &mut P) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();

            alsaseq_sys::alsaseq_user_client_get_info(
                self.as_ref().to_glib_none().0,
                &mut client_info.as_ref().to_glib_none().0,
                &mut error,
            );

            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn get_pool<P: IsA<ClientPool>>(&self, client_pool: &mut P) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();

            alsaseq_sys::alsaseq_user_client_get_pool(
                self.as_ref().to_glib_none().0,
                &mut client_pool.as_ref().to_glib_none().0,
                &mut error,
            );

            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}
