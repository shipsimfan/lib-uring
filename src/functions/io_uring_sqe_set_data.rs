use crate::io_uring_sqe;
use std::ffi::c_void;

// rustdoc imports
#[allow(unused_imports)]
use crate::io_uring_get_sqe;

#[link(name = "uring")]
extern "C" {
    /// Set user data for submission queue event
    ///
    /// # Description
    /// The [`io_uring_sqe_set_data`] function stores a `user_data` pointer with the submission
    /// queue entry `sqe`.
    ///
    /// After the caller has requested a submission queue entry (SQE) with [`io_uring_get_sqe`],
    /// they can associate a data pointer or value with the SQE. Once the completion arrives, the
    /// function [`io_uring_cqe_get_data`] or [`io_uring_cqe_get_data64`] can be called to retrieve
    /// the data pointer associated with the submitted request.
    pub fn io_uring_sqe_set_data(sqe: *mut io_uring_sqe, user_data: *mut c_void);
}
