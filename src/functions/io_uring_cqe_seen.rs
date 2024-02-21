use crate::{io_uring, io_uring_cq_advance, io_uring_cqe};
use std::ptr::null_mut;

// rustdoc imports
#[allow(unused_imports)]
use crate::{io_uring_submit, io_uring_wait_cqe};

/// Mark `io_uring` completion event as consumed
///
/// # Description
/// The [`io_uring_cqe_seen`] function marks the IO completion cqe belonging to the `ring`
/// parameter as consumed.
///
/// After the caller has submitted a request with [`io_uring_submit`], the application can retrieve
/// the completion with [`io_uring_wait_cqe`], [`io_uring_peek_cqe`], or any of the other CQE
/// retrieval helpers, and mark it as consumed with [`io_uring_cqe_seen`].
///
/// Completions must be marked as completed so their slot can get reused.
pub unsafe fn io_uring_cqe_seen(ring: *mut io_uring, cqe: *mut io_uring_cqe) {
    if cqe != null_mut() {
        io_uring_cq_advance(ring, 1);
    }
}
