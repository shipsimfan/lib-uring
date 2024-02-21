use crate::{io_uring_op, io_uring_prep_rw, io_uring_sqe};
use linux::time::__kernel_timespec;
use std::ffi::c_uint;

/// Prepare a timeout request
///
/// # Description
/// The [`io_uring_prep_timeout`] function prepares a timeout request. The submission queue entry
/// `sqe` is setup to arm a timeout specified by `ts` and with a timeout count of `count`
/// completion entries. The flags argument holds modifier flags for the request.
///
/// This request type can be used as a timeout waking anyone sleeping for events on the CQ ring.
/// The flags argument may contain:
///  * [`IORING_TIMEOUT_ABS`] - The value specified in `ts` is an absolute value rather than a
///                             relative one.
///  * [`IORING_TIMEOUT_BOOTTIME`] - The boottime clock source should be used.
///  * [`IORING_TIMEOUT_REALTIME`] - The realtime clock source should be used.
///  * [`IORING_TIMEOUT_ETIME_SUCCESS`] - Consider an expired timeout a success in terms of the
///                                       posted completion. Normally a timeout that triggers would
///                                       return in a `-ETIME` CQE `res` value.
///  * [`IORING_TIMEOUT_MULTISHOT`] - The request will return multiple timeout completions. The
///                                   completion flag [`IORING_CQE_F_MORE`] is set if more timeouts
///                                   are expected. The value specified in count is the number of
///                                   repeats. A value of 0 means the timeout is indefinite and can
///                                   only be stopped by a removal request.
///
/// The timeout completion event will trigger if either the specified timeout has occurred, or the
/// specified number of events to wait for have been posted to the CQ ring.
pub unsafe fn io_uring_prep_timeout(
    sqe: *mut io_uring_sqe,
    ts: *mut __kernel_timespec,
    count: c_uint,
    flags: c_uint,
) {
    io_uring_prep_rw(io_uring_op::TIMEOUT as _, sqe, -1, ts as _, 1, count as _);
    (*sqe).u3.timeout_flags = flags;
}
