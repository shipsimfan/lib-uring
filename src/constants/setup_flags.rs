use std::ffi::c_uint;

/// io_context is polled
pub const IORING_SETUP_IOPOLL: c_uint = 1 << 0;

/// SQ poll thread
pub const IORING_SETUP_SQPOLL: c_uint = 1 << 1;

/// sq_thread_cpu is valid
pub const IORING_SETUP_SQ_AFF: c_uint = 1 << 2;

/// App defines CQ size
pub const IORING_SETUP_CQSIZE: c_uint = 1 << 3;

/// Clamp SQ/CQ ring sizes
pub const IORING_SETUP_CLAMP: c_uint = 1 << 4;

/// Attach to existing wq
pub const IORING_SETUP_ATTACH_WQ: c_uint = 1 << 5;

/// Start with ring disabled
pub const IORING_SETUP_R_DISABLED: c_uint = 1 << 6;

/// Continue submit on error
pub const IORING_SETUP_SUBMIT_ALL: c_uint = 1 << 7;

/// Cooperative task running. When requests complete, they often require forcing the submitter to
/// transition to the kernel to complete. If this flag is set, work will be done when the task
/// transitions anyway, rather than force an inter-processor interrupt reschedule. This avoids
/// interrupting a task running in userspace, and saves an IPI.
pub const IORING_SETUP_COOP_TASKRUN: c_uint = 1 << 8;

/// If [`IORING_SETUP_COOP_TASKRUN`] is set, get notified if task work is available for running and
/// a kernel transition would be needed to run it. This sets [`IORING_SQ_TASKRUN`] in the sq ring
/// flags. Not valid with [`IORING_SETUP_COOP_TASKRUN`].
pub const IORING_SETUP_TASKRUN_FLAG: c_uint = 1 << 9;

/// SQEs are 128 byte
pub const IORING_SETUP_SQE128: c_uint = 1 << 10;

/// CQEs are 32 byte
pub const IORING_SETUP_CQE32: c_uint = 1 << 11;

/// Only one task is allowed to submit requests
pub const IORING_SETUP_SINGLE_ISSUER: c_uint = 1 << 12;

/// Defer running task work to get events. Rather than running bits of task work whenever the task
/// transitions try to do it just before it is needed.
pub const IORING_SETUP_DEFER_TASKRUN: c_uint = 1 << 13;

/// Application provides ring memory
pub const IORING_SETUP_NO_MMAP: c_uint = 1 << 14;

/// Register the ring fd in itself for use with [`IORING_REGISTER_USE_REGISTERED_RING`]; return a
/// registered fd index rather than an fd.
pub const IORING_SETUP_REGISTERED_FD_ONLY: c_uint = 1 << 15;
