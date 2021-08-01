#![doc = include_str!("../README.md")]
#![warn(clippy::pedantic)]
use std::os::raw::{c_char, c_int};

#[repr(C)]
pub struct dbm {
    _private: [u8; 0],
}

#[link(name = "gdbm_compat")]
extern "C" {
    pub fn dbm_open(filename: *const c_char, flags: c_int, mode: u32) -> *mut dbm;
    pub fn dbm_close(dbm_ptr: *mut dbm);
    /// store_mode either DBM_INSERT or DBM_REPLACE
    pub fn dbm_store(
        dbm_ptr: *mut dbm,
        key_datum: datum,
        value_datum: datum,
        store_mode: c_int,
    ) -> c_int;
    pub fn dbm_fetch(dbm_ptr: *mut dbm, key_datum: datum) -> datum;
    pub fn dbm_delete(dbm_ptr: *mut dbm, key_datum: datum) -> c_int;
    /// reset database's cursor to first key and return first value_datum
    pub fn dbm_firstkey(dbm_ptr: *mut dbm) -> datum;
    /// move database's cursor to next and return value_datum
    /// if return value datum.dptr is null, means all keys had been traverse
    pub fn dbm_nextkey(dbm_ptr: *mut dbm) -> datum;
    pub fn dbm_error(dbm_ptr: *mut dbm) -> c_int;
    pub fn dbm_clearerr(dbm_ptr: *mut dbm) -> c_int;
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct datum {
    pub dptr: *mut c_char,
    pub dsize: c_int,
}

/// `store_mode` argument of [`dbm_store`]
pub struct StoreMode;

impl StoreMode {
    pub const DBM_INSERT: c_int = 0;
    pub const DBM_REPLACE: c_int = 1;
}
