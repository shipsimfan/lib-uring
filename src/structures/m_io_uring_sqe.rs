use std::ffi::c_int;

/// I/O submission data structure (Submission Queue Entry)
#[repr(C)]
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct io_uring_sqe {
    /// Type of operation for this sqe
    pub opcode: u8,

    /// IOSQE_ flags
    pub flags: u8,

    /// `ioprio` for the request
    pub ioprio: u16,

    /// File descriptor to do I/O on
    pub fd: i32,

    #[allow(missing_docs)]
    pub u1: io_uring_sqe_u1,

    #[allow(missing_docs)]
    pub u2: io_uring_sqe_u2,

    /// Buffer size or number of iovecs
    pub len: u32,

    #[allow(missing_docs)]
    pub u3: io_uring_sqe_u3,

    /// Data to be passed back at completion time
    pub user_data: u64,

    #[allow(missing_docs)]
    pub u4: io_uring_sqe_u4,

    /// Personality to use, if used
    pub personality: u16,

    #[allow(missing_docs)]
    pub u5: io_uring_sqe_u5,

    #[allow(missing_docs)]
    pub u6: io_uring_sqe_u6,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[allow(non_camel_case_types)]
#[allow(missing_docs)]
pub union io_uring_sqe_u1 {
    /// Offset into file
    pub off: u64,

    pub addr2: u64,
    pub s: io_uring_sqe_u1_s,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[allow(non_camel_case_types)]
#[allow(missing_docs)]
pub struct io_uring_sqe_u1_s {
    pub cmd_op: u32,
    pub _pad: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[allow(non_camel_case_types)]
#[allow(missing_docs)]
pub union io_uring_sqe_u2 {
    /// Pointer to buffer or iovecs
    pub addr: u64,
    pub splice_off_in: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[allow(non_camel_case_types)]
#[allow(missing_docs)]
pub union io_uring_sqe_u3 {
    pub rw_flags: c_int,
    pub fsync_flags: u32,
    pub poll_events: u16,
    pub poll32_events: u32,
    pub sync_range_flags: u32,
    pub msg_flags: u32,
    pub timeout_flags: u32,
    pub accept_flags: u32,
    pub cancel_flags: u32,
    pub open_flags: u32,
    pub statx_flags: u32,
    pub fadvise_advice: u32,
    pub splice_flags: u32,
    pub rename_flags: u32,
    pub unlink_flags: u32,
    pub hardlink_flags: u32,
    pub xattr_flags: u32,
    pub msg_ring_flags: u32,
    pub uring_cmd_flags: u32,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
#[allow(non_camel_case_types)]
#[allow(missing_docs)]
pub union io_uring_sqe_u4 {
    /// Index into fixed buffers, if used
    pub buf_index: u16,

    /// For grouped buffer selection
    pub buf_group: u16,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[allow(non_camel_case_types)]
#[allow(missing_docs)]
pub union io_uring_sqe_u5 {
    pub splice_fd_in: i32,
    pub file_index: u32,
    pub s: io_uring_sqe_u5_s,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[allow(non_camel_case_types)]
#[allow(missing_docs)]
pub struct io_uring_sqe_u5_s {
    pub addr_len: u16,
    pub _pad: [u16; 1],
}

#[repr(C)]
#[derive(Clone, Copy)]
#[allow(non_camel_case_types)]
#[allow(missing_docs)]
pub union io_uring_sqe_u6 {
    pub s: io_uring_sqe_u6_s,
    pub cmd: [u8; 0],
}

#[repr(C)]
#[derive(Clone, Copy)]
#[allow(non_camel_case_types)]
#[allow(missing_docs)]
pub struct io_uring_sqe_u6_s {
    pub addr3: u64,
    pub _pad: [u64; 1],
}
