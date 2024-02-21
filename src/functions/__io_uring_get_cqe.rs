use crate::{io_uring, io_uring_cqe};
use linux::signal::sigset_t;
use std::ffi::{c_int, c_uint};

#[link(name = "uring")]
extern "C" {
    pub fn __io_uring_get_cqe(
        ring: *mut io_uring,
        cqe_ptr: *mut *mut io_uring_cqe,
        submit: c_uint,
        wait_nr: c_uint,
        sigmask: *mut sigset_t,
    ) -> c_int;
}
