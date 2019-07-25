//! A libcore wrapper to enable async/await for Drone OS.

#![feature(generator_trait)]
#![deny(bare_trait_objects)]
#![deny(elided_lifetimes_in_paths)]
#![warn(missing_docs)]
#![warn(clippy::pedantic)]
#![no_std]

pub mod future;

pub use core::*;
