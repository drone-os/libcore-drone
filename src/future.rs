//! Asynchronous values.

pub use core::future::*;

use core::{ops::Generator, pin::Pin, task::Poll};

/// Wrap a generator in a future.
#[doc(hidden)]
pub fn from_generator<T: Generator<Yield = ()>>(x: T) -> impl Future<Output = T::Return> {
    drone_core::future::from_generator(x)
}

#[doc(hidden)]
/// Polls a future in the current task context.
pub fn poll_with_tls_context<F>(f: Pin<&mut F>) -> Poll<F::Output>
where
    F: Future,
{
    drone_core::future::poll_with_context(f)
}
