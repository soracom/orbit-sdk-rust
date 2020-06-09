#[macro_use]
extern crate serde_derive;

extern "C" {
    pub fn orbit_log(ptr: i32, size: i32);
    pub fn orbit_get_input_buffer(ptr: i32, len: i32) -> i32;
    pub fn orbit_get_input_buffer_len() -> i32;
    pub fn orbit_get_tag_value(name_ptr: i32, name_len: i32, value_ptr: i32, value_len: i32) -> i32;
    pub fn orbit_get_tag_value_len(name_ptr: i32, name_len: i32) -> i32;
    pub fn orbit_get_source_value(name_ptr: i32, name_len: i32, value_ptr: i32, value_len: i32) -> i32;
    pub fn orbit_get_source_value_len(name_ptr: i32, name_len: i32) -> i32;
    pub fn orbit_has_location() -> i32;
    pub fn orbit_get_location_lat() -> f64;
    pub fn orbit_get_location_lon() -> f64;
    pub fn orbit_get_timestamp() -> i64;
    pub fn orbit_set_output(ptr: i32, len: i32);
    pub fn orbit_set_output_content_type(ptr: i32, len: i32);
    pub fn orbit_get_userdata(ptr: i32, len: i32) -> i32;
    pub fn orbit_get_userdata_len() -> i32;
}

use std::mem::transmute;

#[derive(Serialize)]
pub struct Location {
    lat: f64,
    lon: f64,
}

pub fn log(message: &str) {
    unsafe {
        let ptr = message.as_bytes().as_ptr();
        orbit_log(transmute::<*const u8, i32>(ptr), message.len() as i32);
    }
}

pub fn get_input_buffer() -> Vec<u8> {
    unsafe {
        let len = orbit_get_input_buffer_len();
        let mut v = Vec::<u8>::with_capacity(len as usize);
        v.set_len(len as usize);
        let ptr = Box::into_raw(v.into_boxed_slice()) as *mut u8;
        let ptri = transmute::<*mut u8, i32>(ptr);
        let actual_len = orbit_get_input_buffer(ptri, len);

        let v = Vec::from_raw_parts(ptr, actual_len as usize, actual_len as usize);
        v.clone()
    }
}

pub fn get_tag_value(name: &str) -> String {
    unsafe {
        let name_ptr = name.as_bytes().as_ptr();
        let name_ptr = transmute::<*const u8, i32>(name_ptr);
        let name_len = name.len();
        let value_len = orbit_get_tag_value_len(name_ptr, name_len as i32);
        let mut v = Vec::<u8>::with_capacity(value_len as usize);
        v.set_len(value_len as usize);
        let value_ptr = Box::into_raw(v.into_boxed_slice()) as *mut u8;
        let value_ptri = transmute::<*mut u8, i32>(value_ptr);
        let actual_value_len =
            orbit_get_tag_value(name_ptr, name_len as i32, value_ptri, value_len);
        let v = Vec::from_raw_parts(
            value_ptr,
            actual_value_len as usize,
            actual_value_len as usize,
        );
        String::from_utf8(v).unwrap()
    }
}

pub fn get_source_value(name: &str) -> String {
    unsafe {
        let name_ptr = name.as_bytes().as_ptr();
        let name_ptr = transmute::<*const u8, i32>(name_ptr);
        let name_len = name.len();
        let value_len = orbit_get_source_value_len(name_ptr, name_len as i32);
        let mut v = Vec::<u8>::with_capacity(value_len as usize);
        v.set_len(value_len as usize);
        let value_ptr = Box::into_raw(v.into_boxed_slice()) as *mut u8;
        let value_ptri = transmute::<*mut u8, i32>(value_ptr);
        let actual_value_len =
            orbit_get_source_value(name_ptr, name_len as i32, value_ptri, value_len);
        let v = Vec::from_raw_parts(
            value_ptr,
            actual_value_len as usize,
            actual_value_len as usize,
        );
        String::from_utf8(v).unwrap()
    }
}

pub fn set_output_json(json_str: &str) {
    unsafe {
        let content_type = "application/json";
        let ptr = content_type.as_bytes().as_ptr();
        let ptr = transmute::<*const u8, i32>(ptr);
        let len = content_type.len();
        orbit_set_output_content_type(ptr, len as i32);

        let ptr = json_str.as_bytes().as_ptr();
        let ptr = transmute::<*const u8, i32>(ptr);
        let len = json_str.len();
        orbit_set_output(ptr, len as i32);
    }
}

pub fn get_location() -> Option<Location> {
    unsafe {
        if orbit_has_location() == 0 {
            return None;
        }

        let lat = orbit_get_location_lat();
        let lon = orbit_get_location_lon();

        Some(Location { lat, lon })
    }
}

pub fn get_timestamp() -> i64 {
    unsafe { orbit_get_timestamp() }
}

pub fn get_userdata() -> String {
    unsafe {
        let len = orbit_get_userdata_len();
        let mut v = Vec::<u8>::with_capacity(len as usize);
        v.set_len(len as usize);
        let ptr = Box::into_raw(v.into_boxed_slice()) as *mut u8;
        let ptri = transmute::<*mut u8, i32>(ptr);
        let actual_len = orbit_get_userdata(ptri, len);

        let v = Vec::from_raw_parts(ptr, actual_len as usize, actual_len as usize);
        match String::from_utf8(v) {
            Ok(str) => str,
            Err(_) => String::from(""),
        }
    }
}

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[no_mangle]
pub extern "C" fn malloc(size: usize) -> *mut u8 {
    let mut vec: Vec<u8> = Vec::with_capacity(size);
    unsafe {
        vec.set_len(size);
    }
    let slice = vec.into_boxed_slice();
    Box::into_raw(slice) as *mut u8
}
