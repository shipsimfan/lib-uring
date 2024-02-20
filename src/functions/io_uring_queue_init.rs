use crate::io_uring;
use std::ffi::{c_int, c_uint};

// rustdoc imports
#[allow(unused_imports)]
use crate::io_uring_queue_exit;

#[link(name = "uring")]
extern "C" {
    /// Setup [`io_uring`] submission and completion queues
    ///
    /// # Description
    /// The [`io_uring_queue_init`] function executes the [`io_uring_setup`]) system call to
    /// initialize the submission and completion queues in the kernel with at least `entries`
    /// entries in the submission queue and then maps the resulting file descriptor to memory
    /// shared between the application and the kernel.
    ///
    /// By default, the CQ ring will have twice the number of entries as specified by entries for
    /// the SQ ring. This is adequate for regular file or storage workloads, but may be too small
    /// for networked workloads. The SQ ring entries do not impose a limit on the number of
    /// in-flight requests that the ring can support, it merely limits the number that can be
    /// submitted to the kernel in one go (batch). If the CQ ring overflows, e.g. more entries are
    /// generated than fits in the ring before the application can reap them, then if the kernel
    /// supports [`IORING_FEAT_NODROP`] the ring enters a CQ ring overflow state. Otherwise it
    /// drops the CQEs and increments `cq.koverflow` in [`io_uring`] with the number of CQEs
    /// dropped. The overflow state is indicated by [`IORING_SQ_CQ_OVERFLOW`] being set in the SQ
    /// ring flags. Unless the kernel runs out of available memory, entries are not dropped, but it
    /// is a much slower completion path and will slow down request processing. For that reason it
    /// should be avoided and the CQ ring sized appropriately for the workload. If the value isn't
    /// a power of 2, it will be rounded up to the nearest power of 2.
    ///
    /// On success, [`io_uring_queue_init`] returns 0 and ring will point to the shared memory
    /// containing the io_uring queues. On failure `-errno` is returned.
    ///
    /// `flags` will be passed through to the [`io_uring_setup`] syscall.
    ///
    /// On success, the resources held by ring should be released via a corresponding call to
    /// [`io_uring_queue_exit`].
    ///
    /// # Return Value
    /// [`io_uring_queue_init`] returns 0 on success and `-errno` on failure.
    pub fn io_uring_queue_init(entries: c_uint, ring: *mut io_uring, flags: c_uint) -> c_int;
}
