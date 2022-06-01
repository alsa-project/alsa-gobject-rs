// SPDX-License-Identifier: MIT
use crate::*;

pub trait StreamPairExtManual {
    fn get_protocol_version(&self) -> Result<&[u16;3], glib::Error>;

    fn get_substream_status<P: IsA<SubstreamStatus>>(
        &self,
        direction: StreamDirection,
        substream_status: &mut P,
    ) -> Result<(), glib::Error>;

    fn read_from_substream(&self, buf: &mut [u8]) -> Result<usize, glib::Error>;
}

impl<O: IsA<StreamPair>> StreamPairExtManual for O {
    fn get_protocol_version(&self) -> Result<&[u16;3], glib::Error> {
        unsafe {
            let mut triplet = std::ptr::null_mut() as *const [u16;3];
            let mut error = std::ptr::null_mut();

            alsarawmidi_sys::alsarawmidi_stream_pair_get_protocol_version(
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

    fn get_substream_status<P: IsA<SubstreamStatus>>(
        &self,
        direction: StreamDirection,
        substream_status: &mut P,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();

            alsarawmidi_sys::alsarawmidi_stream_pair_get_substream_status(
                self.as_ref().to_glib_none().0,
                direction.to_glib(),
                &mut substream_status.as_ref().to_glib_none().0,
                &mut error,
            );

            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn read_from_substream(&self, buf: &mut [u8]) -> Result<usize, glib::Error> {
        unsafe {
            let mut len = buf.len();
            let mut error = std::ptr::null_mut();

            alsarawmidi_sys::alsarawmidi_stream_pair_read_from_substream(
                self.as_ref().to_glib_none().0,
                &mut buf.as_mut_ptr(),
                &mut len,
                &mut error,
            );

            if error.is_null() {
                Ok(len)
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}
