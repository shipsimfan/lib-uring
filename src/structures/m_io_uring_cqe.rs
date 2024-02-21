/// I/O completion data structure (Completion Queue Entry)
#[repr(C)]
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct io_uring_cqe {
    /// `sqe.user_data` submission passed back
    pub user_data: u64,

    /// Result code for this event
    pub res: i32,

    #[allow(missing_docs)]
    pub flags: u32,

    /// If the ring is initialized with [`IORING_SETUP_CQE32`], then this field contains 16-bytes
    /// of padding, doubling the size of the CQE.
    pub big_cqe: [u64; 0],
}
