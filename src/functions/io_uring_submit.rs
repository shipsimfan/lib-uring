use crate::io_uring;
use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::io_uring_get_sqe;

#[link(name = "uring")]
extern "C" {
    /// Submit requests to the submission queue
    ///
    /// # Description
    /// The [`io_uring_submit`] function submits the next events to the submission queue belonging
    /// to the ring.
    ///
    /// After the caller retrieves a submission queue entry (SQE) with [`io_uring_get_sqe`] and
    /// prepares the SQE using one of the provided helpers, it can be submitted with
    /// [`io_uring_submit`].
    ///
    /// # Return Value
    /// On success [`io_uring_submit`] returns the number of submitted submission queue entries, if
    /// [`SQPOLL`] is not used. If [`SQPOLL`] is used, the return value may report a higher number
    /// of submitted entries than actually submitted. If the the user requires accurate information
    /// about how many submission queue entries have been successfully submitted, while using
    /// [`SQPOLL`], the user must fall back to repeatedly submitting a single submission queue
    /// entry. On failure it returns `-errno`.
    pub fn io_uring_submit(ring: *mut io_uring) -> c_int;
}
