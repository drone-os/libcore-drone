[![crates.io](https://img.shields.io/crates/v/libcore-drone.svg)](https://crates.io/crates/libcore-drone)
![maintenance](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)

# libcore-drone

A libcore wrapper to enable native `async`/`await` syntax for [Drone]
applications.

## Usage

Place the following to the Cargo.toml:

```toml
[dependencies]
core = { package = "libcore-drone", version = "0.10.0" }
```

## Description

This crate re-exports contents of [`core`] and defines two new functions
with the following paths:

* `core::future::from_generator`
* `core::future::poll_with_tls_context`

These two functions are absent from original libcore, but defined in libstd.
This is the reason for the following errors when you attempt to use `.await`
in `no_std` context:

```
error[E0433]: failed to resolve: could not find `poll_with_tls_context` in `future`
error[E0433]: failed to resolve: could not find `from_generator` in `future`
```

[Drone]: https://github.com/drone-os/drone-core

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
