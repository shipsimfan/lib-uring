use crate::io_uring_sqe;
use std::ffi::{c_int, c_uint, c_void};

#[link(name = "uring")]
extern "C" {
    /// Prepare I/O read request
    ///
    /// # Description
    /// The [`io_uring_prep_read`] prepares an IO read request. The submission queue entry `sqe` is
    /// setup to use the file descriptor `fd` to start reading `bytes` into the buffer `buf` at the
    /// specified `offset`.
    ///
    /// On files that support seeking, if the `offset` is set to -1, the read operation commences
    /// at the file offset, and the file offset is incremented by the number of bytes read. Note
    /// that for an async API, reading and updating the current file offset may result in
    /// unpredictable behavior, unless access to the file is serialized. It is not encouraged to
    /// use this feature, if it's possible to provide the desired IO offset from the application or
    /// library.
    ///
    /// On files that are not capable of seeking, the offset must be 0 or -1.
    ///
    /// After the read has been prepared it can be submitted with one of the submit functions.
    pub fn io_uring_prep_read(
        sqe: *mut io_uring_sqe,
        fd: c_int,
        buf: *mut c_void,
        bytes: c_uint,
        offset: u64,
    );
}
