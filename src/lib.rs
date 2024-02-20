//! Raw bindings for `liburing`

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]
#![feature(c_size_t)]

mod functions;
mod structures;
mod types;

pub use functions::{
    io_uring_cq_ready, io_uring_cqe_get_data, io_uring_cqe_get_data64, io_uring_get_sqe,
    io_uring_prep_read, io_uring_queue_exit, io_uring_queue_init, io_uring_sqe_set_data,
    io_uring_sqe_set_data64, io_uring_submit, io_uring_wait_cqe,
};
pub use structures::{io_uring, io_uring_cq, io_uring_sq};
pub use types::{io_uring_cqe, io_uring_sqe};
