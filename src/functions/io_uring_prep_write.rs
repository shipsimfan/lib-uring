use crate::{io_uring_op, io_uring_prep_rw, io_uring_sqe};
use std::ffi::{c_int, c_uint, c_void};

// rustdoc imports
#[allow(unused_imports)]
use linux::unistd::write;

/// Prepare I/O write request
///
/// # Description
/// The [`io_uring_prep_write`] prepares an IO write request. The submission queue entry `sqe` is
/// setup to use the file descriptor `fd` to start writing `nbytes` from the buffer `buf` at the
/// specified offset.
///
/// On files that support seeking, if the `offset` is set to -1, the write operation commences at
/// the file offset, and the file offset is incremented by the number of bytes written. See
/// [`write`] for more details. Note that for an async API, reading and updating the current file
/// offset may result in unpredictable behavior, unless access to the file is serialized. It is not
/// encouraged to use this feature if it's possible to provide the desired IO offset from the
/// application or library.
///
/// On files that are not capable of seeking, the `offset` must be 0 or -1.
///
/// After the write has been prepared, it can be submitted with one of the submit functions.
pub unsafe fn io_uring_prep_write(
    sqe: *mut io_uring_sqe,
    fd: c_int,
    buf: *const c_void,
    nbytes: c_uint,
    offset: u64,
) {
    io_uring_prep_rw(io_uring_op::WRITE as _, sqe, fd, buf, nbytes, offset);
}
