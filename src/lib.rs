//! Raw bindings for `liburing`

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]
#![feature(c_size_t)]

mod constants;
mod enumerations;
mod functions;
mod structures;

pub use constants::*;
pub use enumerations::io_uring_op;
pub use functions::{
    io_uring_cq_advance, io_uring_cq_ready, io_uring_cqe_get_data, io_uring_cqe_get_data64,
    io_uring_get_sqe, io_uring_prep_read, io_uring_prep_rw, io_uring_prep_timeout,
    io_uring_queue_exit, io_uring_queue_init, io_uring_sqe_set_data, io_uring_sqe_set_data64,
    io_uring_submit, io_uring_wait_cqe, io_uring_wait_cqe_nr,
};
pub use structures::{
    io_uring, io_uring_cq, io_uring_cqe, io_uring_sq, io_uring_sqe, io_uring_sqe_u1,
    io_uring_sqe_u1_s, io_uring_sqe_u2, io_uring_sqe_u3, io_uring_sqe_u4, io_uring_sqe_u5,
    io_uring_sqe_u5_s, io_uring_sqe_u6, io_uring_sqe_u6_s,
};

use functions::{__io_uring_get_cqe, __io_uring_peek_cqe};
