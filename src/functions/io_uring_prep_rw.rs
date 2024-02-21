use crate::io_uring_sqe;
use std::ffi::{c_int, c_uint, c_void};

#[allow(missing_docs)]
pub unsafe fn io_uring_prep_rw(
    op: c_int,
    sqe: *mut io_uring_sqe,
    fd: c_int,
    addr: *const c_void,
    len: c_uint,
    offset: u64,
) {
    (*sqe).opcode = op as _;
    (*sqe).flags = 0;
    (*sqe).ioprio = 0;
    (*sqe).fd = fd;
    (*sqe).u1.off = offset;
    (*sqe).u2.addr = addr as _;
    (*sqe).len = len;
    (*sqe).u3.rw_flags = 0;
    (*sqe).u4.buf_index = 0;
    (*sqe).personality = 0;
    (*sqe).u5.file_index = 0;
    (*sqe).u6.s.addr3 = 0;
    (*sqe).u6.s._pad = [0];
}
