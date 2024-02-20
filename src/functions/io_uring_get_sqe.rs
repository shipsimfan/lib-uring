use crate::{io_uring, io_uring_sqe};

// rustdoc imports
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "uring")]
extern "C" {
    /// Get the next available submission queue entry from the submission queue
    ///
    /// # Description
    /// The [`io_uring_get_sqe`] function gets the next available submission queue entry from the
    /// submission queue belonging to the `ring` parameter.
    ///
    /// On success [`io_uring_get_sqe`] returns a pointer to the submission queue entry. On failure
    /// [`null_mut`] is returned.
    ///
    /// If a submission queue entry is returned, it should be filled out via one of the prep
    /// functions such as [`io_uring_prep_read`] and submitted via [`io_uring_submit`].
    ///
    /// Note that neither [`io_uring_get_sqe`] nor the prep functions set (or clear) the
    /// `user_data` field of the SQE. If the caller expects [`io_uring_cqe_get_data`] or
    /// [`io_uring_cqe_get_data64`] to return valid data when reaping IO completions, either
    /// [`io_uring_sqe_set_data`] or [`io_uring_sqe_set_data64`] MUST have been called before
    /// submitting the request.
    ///
    /// # Return Value
    /// [`io_uring_get_sqe`] returns a pointer to the next submission queue event on success and
    /// [`null_mut`] on failure. If [`null_mut`] is returned, the SQ ring is currently full and
    /// entries  must be submitted for processing before new ones can get allocated.
    pub fn io_uring_get_sqe(ring: *mut io_uring) -> *mut io_uring_sqe;
}
