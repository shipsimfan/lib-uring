use crate::{__io_uring_get_cqe, io_uring, io_uring_cqe};
use std::{
    ffi::{c_int, c_uint},
    ptr::null_mut,
};

/// Return an IO completion, waiting for 'wait_nr' completions if one isn't readily available. Returns 0 with cqe_ptr filled in on success, -errno on failure.
pub unsafe fn io_uring_wait_cqe_nr(
    ring: *mut io_uring,
    cqe_ptr: *mut *mut io_uring_cqe,
    wait_nr: c_uint,
) -> c_int {
    return __io_uring_get_cqe(ring, cqe_ptr, 0, wait_nr, null_mut());
}
