use crate::{io_uring_sqe, io_uring_sqe_set_data64};
use std::ffi::c_void;

// rustdoc imports
#[allow(unused_imports)]
use crate::{io_uring_cqe_get_data, io_uring_get_sqe};

/// Set user data for submission queue event
///
/// # Description
/// The [`io_uring_sqe_set_data`] function stores a `user_data` pointer with the submission
/// queue entry `sqe`.
///
/// After the caller has requested a submission queue entry (SQE) with [`io_uring_get_sqe`],
/// [`io_uring_sqe_set_data`] can associate a data pointer the SQE. Once the completion
/// arrives, the function [`io_uring_cqe_get_data`] can be called to retrieve the data pointer
/// associated with the submitted request.
pub unsafe fn io_uring_sqe_set_data(sqe: *mut io_uring_sqe, user_data: *mut c_void) {
    io_uring_sqe_set_data64(sqe, user_data as _);
}
