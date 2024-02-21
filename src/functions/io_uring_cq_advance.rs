use crate::io_uring;
use std::{
    ffi::c_uint,
    sync::atomic::{AtomicU32, Ordering},
};

/// Must be called after [`io_uring_for_each_cqe`]
pub unsafe fn io_uring_cq_advance(ring: *mut io_uring, nr: c_uint) {
    if nr != 0 {
        let cq = &(*ring).cq;

        AtomicU32::from_ptr(cq.khead).store(*cq.khead + nr, Ordering::Release)
    }
}
