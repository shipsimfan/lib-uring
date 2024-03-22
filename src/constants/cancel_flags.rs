use std::ffi::c_int;

/// Cancel all requests that match the given key
pub const IORING_ASYNC_CANCEL_ALL: c_int = 1 << 0;

/// Key off 'fd' for cancelation rather than the request 'user_data'
pub const IORING_ASYNC_CANCEL_FD: c_int = 1 << 1;

/// Match any request
pub const IORING_ASYNC_CANCEL_ANY: c_int = 1 << 2;

/// 'fd' passed in is a fixed descriptor
pub const IORING_ASYNC_CANCEL_FD_FIXED: c_int = 1 << 3;
