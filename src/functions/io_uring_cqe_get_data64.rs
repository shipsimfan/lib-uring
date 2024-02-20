use crate::io_uring_cqe;

// rustdoc imports
#[allow(unused_imports)]
use crate::{io_uring_sqe_set_data64, io_uring_wait_cqe};

#[link(name = "uring")]
extern "C" {
    /// Get user data for completion event
    ///
    /// # Description
    /// The [`io_uring_cqe_get_data64`] function returns the `user_data` with the completion queue
    /// entry `cqe` as a 64-bit data value.
    ///
    /// After the caller has received a completion queue entry (CQE) with [`io_uring_wait_cqe`],
    /// the application can call the [`io_uring_cqe_get_data64`] function to retrieve the
    /// `user_data` value. This requires that `user_data` has been set earlier with the
    /// [`io_uring_sqe_set_data64`] function.
    pub fn io_uring_cqe_get_data64(cqe: *mut io_uring_cqe) -> u64;
}
