use crate::io_uring;

// rustdoc imports
#[allow(unused_imports)]
use crate::io_uring_queue_init;

#[link(name = "uring")]
extern "system" {
    /// Tear down [`io_uring`] submission and completion queues
    ///
    /// # Description
    /// [`io_uring_queue_exit`] will release all resources acquired and initialized by
    /// [`io_uring_queue_init`]. It first unmaps the memory shared between the application and the
    /// kernel and then closes the [`io_uring`] file descriptor.
    pub fn io_uring_queue_exit(ring: *mut io_uring);
}
