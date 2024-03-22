use crate::{io_uring_op, io_uring_prep_rw, io_uring_sqe};
use std::{ffi::c_int, ptr::null};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    IORING_ASYNC_CANCEL_ALL, IORING_ASYNC_CANCEL_ANY, IORING_ASYNC_CANCEL_FD,
    IORING_ASYNC_CANCEL_FD_FIXED,
};

/// Prepare a cancelation request
///
/// # Description
/// The [`io_uring_prep_cancel64`] function prepares a cancelation request. The submission queue
/// entry `sqe` is prepared to cancel an existing request identified by `user_data`.
///
/// The cancelation request will attempt to find the previously issued request identified by
/// `user_data` and cancel it. The identifier is what the previously issued request has in their
/// `user_data` field in the SQE.
///
/// By default, the first request matching the criteria given will be canceled. This can be
/// modified with any of the following flags passed in:
///  * [`IORING_ASYNC_CANCEL_ALL`] - Cancel all requests that match the given criteria, rather than
///                                  just canceling the first one found.
///  * [`IORING_ASYNC_CANCEL_FD`] - Match based on the file descriptor used in the original request
///                                 rather than the `user_data`.
///  * [`IORING_ASYNC_CANCEL_FD_FIXED`] - Set in conjunction with [`IORING_ASYNC_CANCEL_FD`],
///                                       indicating that the file descriptor given is a direct
///                                       descriptor rather than a normal file descriptor.
///  * [`IORING_ASYNC_CANCEL_ANY`] - Match any request in the ring, regardless of `user_data` or
///                                  file descriptor. Can be used to cancel any pending request in
///                                  the ring.
pub unsafe fn io_uring_prep_cancel64(sqe: *mut io_uring_sqe, user_data: u64, flags: c_int) {
    io_uring_prep_rw(io_uring_op::ASYNC_CANCEL as _, sqe, -1, null(), 0, 0);
    (*sqe).u2.addr = user_data;
    (*sqe).u3.cancel_flags = flags as _;
}
