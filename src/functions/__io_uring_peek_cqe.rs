use crate::{io_uring, io_uring_cq_advance, io_uring_cqe, IORING_FEAT_EXT_ARG, IORING_SETUP_CQE32};
use std::{
    ffi::{c_int, c_uint},
    ptr::null_mut,
    sync::atomic::{AtomicU32, Ordering},
};

// rustdoc imports
#[allow(unused_imports)]
use crate::io_uring_wait_cqe;

/// Internal helper, don't use directly in applications. Use one of the "official" versions of
/// this, [`io_uring_peek_cqe`], [`io_uring_wait_cqe`], or [`io_uring_wait_cqes`].
pub(crate) unsafe fn __io_uring_peek_cqe(
    ring: *mut io_uring,
    cqe_ptr: *mut *mut io_uring_cqe,
    nr_available: *mut c_uint,
) -> c_int {
    let mask = (*ring).cq.ring_mask;
    let shift = if (*ring).flags & IORING_SETUP_CQE32 != 0 {
        1
    } else {
        0
    };

    let mut err = 0;
    let mut cqe;
    let mut available;

    loop {
        let tail = AtomicU32::from_ptr((*ring).cq.ktail).load(Ordering::Acquire);
        let head = *(*ring).cq.khead;

        cqe = null_mut();
        available = tail - head;
        if available == 0 {
            break;
        }

        cqe = (*ring).cq.cqes.add(((head & mask) << shift) as usize);
        if (*ring).features & IORING_FEAT_EXT_ARG == 0 && (*cqe).user_data == u64::MAX {
            if (*cqe).res < 0 {
                err = (*cqe).res;
            }

            io_uring_cq_advance(ring, 1);

            if err == 0 {
                continue;
            }

            cqe = null_mut();
        }

        break;
    }

    *cqe_ptr = cqe;
    if nr_available != null_mut() {
        *nr_available = available;
    }

    err
}
