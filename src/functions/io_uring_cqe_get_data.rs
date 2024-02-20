use crate::io_uring_cqe;
use std::ffi::c_void;

// rustdoc imports
#[allow(unused_imports)]
use crate::{io_uring_sqe_set_data, io_uring_wait_cqe};

#[link(name = "uring")]
extern "C" {
    /// Get user data for completion event
    ///
    /// # Description
    /// The [`io_uring_cqe_get_data`] function returns the `user_data` with the completion queue
    /// entry `cqe` as a data pointer.
    ///
    /// After the caller has received a completion queue entry (CQE) with [`io_uring_wait_cqe`],
    /// the application can call the [`io_uring_cqe_get_data`] function to retrieve the `user_data`
    /// value. This requires that `user_data` has been set earlier with the
    /// [`io_uring_sqe_set_data`] function.
    pub fn io_uring_cqe_get_data(cqe: *mut io_uring_cqe) -> *mut c_void;
}
