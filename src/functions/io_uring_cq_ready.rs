use crate::io_uring;
use std::{
    ffi::c_uint,
    sync::atomic::{AtomicU32, Ordering},
};

/// Returns number of unconsumed ready entries in the CQ ring
///
/// # Description
/// The [`io_uring_cq_ready`] function returns the number of unconsumed entries that are ready
/// belonging to the `ring` parameter.
///
/// # Return Value
/// Returns the number of unconsumed ready entries in the CQ ring.
pub unsafe fn io_uring_cq_ready(ring: *const io_uring) -> c_uint {
    AtomicU32::from_ptr((*ring).cq.ktail).load(Ordering::Acquire) - *(*ring).cq.khead
}
