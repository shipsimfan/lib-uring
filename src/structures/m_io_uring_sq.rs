use crate::io_uring_sqe;
use core::ffi::{c_size_t, c_uint, c_void};
use std::ptr::null_mut;

#[repr(C)]
#[allow(non_camel_case_types)]
#[allow(missing_docs)]
pub struct io_uring_sq {
    pub khead: *mut c_uint,
    pub ktail: *mut c_uint,
    pub kring_mask: *mut c_uint,
    pub kring_entries: *mut c_uint,
    pub kflags: *mut c_uint,
    pub kdropped: *mut c_uint,
    pub array: *mut c_uint,
    pub sqes: *mut io_uring_sqe,

    pub sqe_head: c_uint,
    pub sqe_tail: c_uint,

    pub ring_sz: c_size_t,
    pub ring_ptr: *mut c_void,

    pub ring_mask: c_uint,
    pub ring_entries: c_uint,

    pub pad: [c_uint; 2],
}

impl Default for io_uring_sq {
    fn default() -> Self {
        io_uring_sq {
            khead: null_mut(),
            ktail: null_mut(),
            kring_mask: null_mut(),
            kring_entries: null_mut(),
            kflags: null_mut(),
            kdropped: null_mut(),
            array: null_mut(),
            sqes: null_mut(),
            sqe_head: 0,
            sqe_tail: 0,
            ring_sz: 0,
            ring_ptr: null_mut(),
            ring_mask: 0,
            ring_entries: 0,
            pad: [0; 2],
        }
    }
}
