use crate::{io_uring, io_uring_cqe};
use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::io_uring_submit;

#[link(name = "uring")]
extern "C" {
    /// Wait for one `io_uring` completion event
    ///
    /// # Description
    /// The [`io_uring_wait_cqe`] function waits for an IO completion from the queue belonging to
    /// the ring param, waiting for it if necessary. If an event is already available in the ring
    /// when invoked, no waiting will occur. The `cqe` param is filled in on success.
    ///
    /// After the caller has submitted a request with [`io_uring_submit`], the application can
    /// retrieve the completion with [`io_uring_wait_cqe`].
    ///
    /// # Return Value
    /// On success [`io_uring_wait_cqe`] returns 0 and the `cqe` param is filled in. On failure it
    /// returns `-errno`. The return value indicates the result of waiting for a CQE, and it has no
    /// relation to the CQE result itself.
    pub fn io_uring_wait_cqe(ring: *mut io_uring, cqe: *mut *mut io_uring_cqe) -> c_int;
}
