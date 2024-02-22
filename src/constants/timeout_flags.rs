use std::ffi::c_uint;

/// The value specified in `ts` is an absolute value rather than a relative one.
pub const IORING_TIMEOUT_ABS: c_uint = 1 << 0;

/// The boottime clock source should be used.
pub const IORING_TIMEOUT_BOOTTIME: c_uint = 1 << 2;

/// The realtime clock source should be used.
pub const IORING_TIMEOUT_REALTIME: c_uint = 1 << 3;

/// Consider an expired timeout a success in terms of the posted completion. Normally a timeout
/// that triggers would return in a `-ETIME` CQE `res` value.
pub const IORING_TIMEOUT_ETIME_SUCCESS: c_uint = 1 << 5;

/// The request will return multiple timeout completions. The completion flag [`IORING_CQE_F_MORE`]
/// is set if more timeouts are expected. The value specified in count is the number of repeats. A
/// value of 0 means the timeout is indefinite and can only be stopped by a removal request.
pub const IORING_TIMEOUT_MULTISHOT: c_uint = 1 << 6;
