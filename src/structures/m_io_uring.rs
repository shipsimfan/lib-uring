use crate::{io_uring_cq, io_uring_sq};
use std::ffi::{c_int, c_uint};

#[repr(C)]
#[allow(non_camel_case_types)]
#[allow(missing_docs)]
pub struct io_uring {
    pub sq: io_uring_sq,
    pub cq: io_uring_cq,
    pub flags: c_uint,
    pub ring_fd: c_int,

    pub features: c_uint,
    pub enter_ring_fd: c_int,
    pub int_flags: u8,
    pub pad: [u8; 3],
    pub pad2: c_uint,
}

impl Default for io_uring {
    fn default() -> Self {
        io_uring {
            sq: io_uring_sq::default(),
            cq: io_uring_cq::default(),
            flags: 0,
            ring_fd: 0,
            features: 0,
            enter_ring_fd: 0,
            int_flags: 0,
            pad: [0; 3],
            pad2: 0,
        }
    }
}
