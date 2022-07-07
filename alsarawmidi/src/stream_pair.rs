// SPDX-License-Identifier: MIT
use super::*;

/// Trait containing the rest of[`struct@StreamPair`] methods.
pub trait StreamPairExtManual {
    /// Get the version of rawmidi protocol currently used. The version is expressed as the array with
    /// three elements; major, minor, and micro version in the order. The length of major version is
    /// 16 bit, the length of minor and micro version is 8 bit each.
    ///
    /// # Returns
    ///
    /// [`true`] when the overall operation finishes successfully, else [`false`].
    ///
    /// ## `proto_ver_triplet`
    /// The version of protocol currently used.
    #[doc(alias = "alsarawmidi_stream_pair_get_protocol_version")]
    #[doc(alias = "get_protocol_version")]
    fn protocol_version(&self) -> Result<&[u16; 3], glib::Error>;

    /// Retrieve status of substream for given direction, which is attached to the pair of streams.
    ///
    /// The call of function executes `ioctl(2)` system call with `SNDRV_RAWMIDI_IOCTL_STATUS` command
    /// for ALSA rawmidi character device.
    /// ## `direction`
    /// The direction of substream attached to the stream pair.
    /// ## `substream_status`
    /// The status of substream.
    ///
    /// # Returns
    ///
    /// [`true`] when the overall operation finishes successfully, else [`false`].
    #[doc(alias = "alsarawmidi_stream_pair_get_substream_status")]
    fn substream_status<P: IsA<SubstreamStatus>>(
        &self,
        direction: StreamDirection,
        substream_status: &mut P,
    ) -> Result<(), glib::Error>;

    /// Copy data from intermediate buffer to given buffer for substream attached to the pair of
    /// streams. In a case that the instance is opened without `O_NONBLOCK` flag and the intermediate
    /// buffer has no data, call of the API is blocked till any data is available.
    ///
    /// The call of function executes `read(2)` system for ALSA rawmidi character device.
    /// ## `buf`
    /// The buffer to copy data.
    ///
    /// # Returns
    ///
    /// [`true`] when the overall operation finishes successfully, else [`false`].
    #[doc(alias = "alsarawmidi_stream_pair_read_from_substream")]
    fn read_from_substream(&self, buf: &mut [u8]) -> Result<usize, glib::Error>;
}

impl<O: IsA<StreamPair>> StreamPairExtManual for O {
    fn protocol_version(&self) -> Result<&[u16; 3], glib::Error> {
        unsafe {
            let mut triplet = std::ptr::null_mut() as *const [u16; 3];
            let mut error = std::ptr::null_mut();

            let _ = ffi::alsarawmidi_stream_pair_get_protocol_version(
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

    fn substream_status<P: IsA<SubstreamStatus>>(
        &self,
        direction: StreamDirection,
        substream_status: &mut P,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();

            let _ = ffi::alsarawmidi_stream_pair_get_substream_status(
                self.as_ref().to_glib_none().0,
                direction.into_glib(),
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

            let _ = ffi::alsarawmidi_stream_pair_read_from_substream(
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
