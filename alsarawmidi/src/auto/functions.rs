// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use alsarawmidi_sys;
use glib;
use glib::translate::*;
use std::mem;
use std::ptr;
use StreamDirection;
use SubstreamInfo;

use glib::GString;

pub fn get_device_id_list(card_id: u32) -> Result<Vec<u32>, glib::Error> {
    unsafe {
        let mut entries = ptr::null_mut();
        let mut entry_count = mem::MaybeUninit::uninit();
        let mut error = ptr::null_mut();
        let _ = alsarawmidi_sys::alsarawmidi_get_device_id_list(card_id, &mut entries, entry_count.as_mut_ptr(), &mut error);
        if error.is_null() { Ok(FromGlibContainer::from_glib_full_num(entries, entry_count.assume_init() as usize)) } else { Err(from_glib_full(error)) }
    }
}

pub fn get_rawmidi_devnode(card_id: u32, device_id: u32) -> Result<GString, glib::Error> {
    unsafe {
        let mut devnode = ptr::null_mut();
        let mut error = ptr::null_mut();
        let _ = alsarawmidi_sys::alsarawmidi_get_rawmidi_devnode(card_id, device_id, &mut devnode, &mut error);
        if error.is_null() { Ok(from_glib_full(devnode)) } else { Err(from_glib_full(error)) }
    }
}

pub fn get_rawmidi_sysname(card_id: u32, device_id: u32) -> Result<GString, glib::Error> {
    unsafe {
        let mut sysname = ptr::null_mut();
        let mut error = ptr::null_mut();
        let _ = alsarawmidi_sys::alsarawmidi_get_rawmidi_sysname(card_id, device_id, &mut sysname, &mut error);
        if error.is_null() { Ok(from_glib_full(sysname)) } else { Err(from_glib_full(error)) }
    }
}

pub fn get_subdevice_id_list(card: u32, device: u32, direction: StreamDirection) -> Result<Vec<u32>, glib::Error> {
    unsafe {
        let mut entries = ptr::null_mut();
        let mut entry_count = mem::MaybeUninit::uninit();
        let mut error = ptr::null_mut();
        let _ = alsarawmidi_sys::alsarawmidi_get_subdevice_id_list(card, device, direction.to_glib(), &mut entries, entry_count.as_mut_ptr(), &mut error);
        if error.is_null() { Ok(FromGlibContainer::from_glib_full_num(entries, entry_count.assume_init() as usize)) } else { Err(from_glib_full(error)) }
    }
}

pub fn get_substream_info(card_id: u32, device_id: u32, direction: StreamDirection, subdevice_id: u32) -> Result<SubstreamInfo, glib::Error> {
    unsafe {
        let mut substream_info = ptr::null_mut();
        let mut error = ptr::null_mut();
        let _ = alsarawmidi_sys::alsarawmidi_get_substream_info(card_id, device_id, direction.to_glib(), subdevice_id, &mut substream_info, &mut error);
        if error.is_null() { Ok(from_glib_full(substream_info)) } else { Err(from_glib_full(error)) }
    }
}
