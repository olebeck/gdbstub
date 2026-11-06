use crate::target::{Target, TargetResult};

/// Target Extension - List a target's threads.
pub trait Threads: Target {
    /// Get Thread list XML for this target.
    ///
    /// See the [GDB Documentation] for a description of the format.
    ///
    /// [GDB Documentation]: https://sourceware.org/gdb/current/onlinedocs/gdb.html/Thread-List-Format.html#
    ///
    /// Return the number of bytes written into `buf` (which may be less than
    /// `length`).
    ///
    /// If `offset` is greater than the length of the underlying data, return
    /// `Ok(0)`.
    fn get_threads(&self, offset: u64, length: usize, buf: &mut [u8]) -> TargetResult<usize, Self>;
}

define_ext!(ThreadsOps, Threads);
