//! Asynchronous values.

pub use core::future::*;

use core::{ops::Generator, pin::Pin, task::Poll};
use drone_core::asnc::{asnc, poll_with_context};

/// Wrap a generator in a future.
#[doc(hidden)]
pub fn from_generator<T: Generator<Yield = ()>>(x: T) -> impl Future<Output = T::Return> {
    asnc(x)
}

#[doc(hidden)]
/// Polls a future in the current thread-local task waker.
pub fn poll_with_tls_context<F>(f: Pin<&mut F>) -> Poll<F::Output>
where
    F: Future,
{
    poll_with_context(f)
}
