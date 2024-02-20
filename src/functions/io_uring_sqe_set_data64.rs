use crate::io_uring_sqe;

// rustdoc imports
#[allow(unused_imports)]
use crate::io_uring_get_sqe;

#[link(name = "uring")]
extern "C" {
    /// Set user data for submission queue event
    ///
    /// # Description
    /// The [`io_uring_sqe_set_data64`] function stores a 64-bit data value with the submission
    /// queue entry sqe.
    ///
    /// After the caller has requested a submission queue entry (SQE) with [`io_uring_get_sqe`],
    /// [`io_uring_sqe_set_data64`] can associate a value the SQE. Once the completion arrives, the
    /// function [`io_uring_cqe_get_data64`] can be called to retrieve the data pointer associated
    /// with the submitted request.
    pub fn io_uring_sqe_set_data64(sqe: *mut io_uring_sqe, data: u64);
}
