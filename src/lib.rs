//! A libcore wrapper to enable native `async`/`await` syntax for [Drone]
//! applications.
//!
//! # Usage
//!
//! Place the following to the Cargo.toml:
//!
//! ```toml
//! [dependencies]
//! core = { package = "libcore-drone", version = "0.10.0" }
//! ```
//!
//! # Description
//!
//! This crate re-exports contents of [`core`] and defines two new functions
//! with the following paths:
//!
//! * `core::future::from_generator`
//! * `core::future::poll_with_tls_context`
//!
//! These two functions are absent from original libcore, but defined in libstd.
//! This is the reason for the following errors when you attempt to use `.await`
//! in `no_std` context:
//!
//! ```text
//! error[E0433]: failed to resolve: could not find `poll_with_tls_context` in `future`
//! error[E0433]: failed to resolve: could not find `from_generator` in `future`
//! ```
//!
//! [Drone]: https://github.com/drone-os/drone-core

#![feature(generator_trait)]
#![deny(elided_lifetimes_in_paths)]
#![warn(missing_docs)]
#![warn(clippy::pedantic)]
#![no_std]

pub mod future;

pub use core::*;
