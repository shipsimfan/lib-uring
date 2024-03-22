use crate::{io_uring_op, io_uring_prep_rw, io_uring_sqe};
use linux::sys::socket::{sockaddr, socklen_t};
use std::ffi::c_int;

/// Prepare an accept request
///
/// # Description
/// The [`io_uring_prep_accept`] function prepares an accept request similar to [`accept4`]. The
/// submission queue entry `sqe` is setup to use the file descriptor `sockfd` to start accepting a
/// connection request described by the socket address at `addr` and of structure length `addrlen`
/// and using modifier flags in `flags`.
pub unsafe fn io_uring_prep_accept(
    sqe: *mut io_uring_sqe,
    sockfd: c_int,
    addr: *mut sockaddr,
    addrlen: *mut socklen_t,
    flags: c_int,
) {
    io_uring_prep_rw(
        io_uring_op::ACCEPT as _,
        sqe,
        sockfd,
        addr.cast(),
        0,
        addrlen as _,
    );
    (*sqe).u3.accept_flags = flags as _;
}
