use std::io;

mod completion;
mod io_uring;
mod metrics;

pub use self::{
    completion::Completion,
    io_uring::{Config, Ordering, Uring},
};

use {
    completion::{pair, Filler},
    metrics::{Measure, M},
};

/// Create a new IO system.
pub fn new() -> io::Result<Uring> {
    Config::default().start()
}

/// Encompasses various types of IO structures that
/// can be operated on as if they were a libc::iovec
pub trait AsIoVec {
    /// Returns the address of this object.
    fn into_new_iovec(&self) -> libc::iovec;
}

impl<A: ?Sized + AsRef<[u8]>> AsIoVec for A {
    fn into_new_iovec(&self) -> libc::iovec {
        let self_ref: &[u8] = self.as_ref();
        let self_ptr: *const [u8] = self_ref;
        libc::iovec {
            iov_base: self_ptr as *mut _,
            iov_len: self_ref.len(),
        }
    }
}

/// We use this internally as a way of communicating
/// that for certain operations, we cannot accept a
/// reference into read-only memory, like for reads.
///
/// If your compilation fails because of something
/// related to this, it's because you are trying
/// to use memory as a destination for a read
/// that could never actually be written to anyway,
/// which the compiler may place in read-only
/// memory in your process that cannot be written
/// to by anybody.
///
/// # Examples
///
/// This will cause the following code to break,
/// which would have caused an IO error anyway
/// due to trying to write to static read-only
/// memory:
///
/// ```compile_fail
/// let ring = rio::new().unwrap();
/// let file = std::fs::File::open("failure").unwrap();
///
/// // the following buffer is placed in
/// // static, read-only memory and would
/// // never be valid to write to
/// let buffer: &[u8] = b"this is read-only";
///
/// // this fails to compile, because &[u8]
/// // does not implement `AsIoVecMut`:
/// ring.read_at(&file, &buffer, 0).unwrap();
/// ```
///
/// which can be fixed by making it a mutable
/// slice:
///
/// ```no_run
/// let ring = rio::new().unwrap();
/// let file = std::fs::File::open("failure").unwrap();
///
/// // the following buffer is placed in
/// // readable and writable memory, due to
/// // its mutability
/// let buffer: &mut [u8] = &mut [0; 42];
///
/// // this now works
/// ring.read_at(&file, &buffer, 0).wait();
/// ```
pub trait AsIoVecMut {}

impl<A: ?Sized + AsMut<[u8]>> AsIoVecMut for A {}

/// A trait for describing transformations from the
/// `io_uring_cqe` type into an expected meaningful
/// high-level result.
pub trait FromCqe {
    /// Describes a conversion from a successful
    /// `io_uring_cqe` to a desired output type.
    fn from_cqe(cqe: io_uring::io_uring_cqe) -> Self;
}

impl FromCqe for usize {
    fn from_cqe(cqe: io_uring::io_uring_cqe) -> usize {
        use std::convert::TryFrom;
        usize::try_from(cqe.res).unwrap()
    }
}

impl FromCqe for () {
    fn from_cqe(_: io_uring::io_uring_cqe) {}
}
