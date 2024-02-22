use crate::{io_uring_op, io_uring_prep_rw, io_uring_sqe};
use std::{ffi::c_uint, ptr::null};

/// Prepare a request to update an existing timeout
///
/// # Description
/// This function modifies or cancels an existing timeout request. The submission queue entry `sqe`
/// is setup to arm a timeout removal specified by `user_data` and with modifier flags given by
/// `flags`.
///
/// The timeout remove command does not currently accept any flags.
pub unsafe fn io_uring_prep_timeout_remove(sqe: *mut io_uring_sqe, user_data: u64, flags: c_uint) {
    io_uring_prep_rw(io_uring_op::TIMEOUT_REMOVE as _, sqe, -1, null(), 0, 0);
    (*sqe).u2.addr = user_data;
    (*sqe).u3.timeout_flags = flags;
}
