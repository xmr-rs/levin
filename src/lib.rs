// Copyright 2018 Jean Pierre Dudey <jeandudey@hotmail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![deny(anonymous_parameters, missing_debug_implementations, missing_docs, trivial_casts, trivial_numeric_casts, unreachable_pub, unsafe_code, unstable_features, unused_extern_crates, unused_import_braces, unused_qualifications)]

//! # xmr-levin
//!
//! Rust implementation of the levin protocol used in the Monero cryptocurrency.
//!
//! # Usage:
//!
//! Add it to the dependencies in your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! levin = { git = "https://www.github.com/xmr-rs/levin", version = "0.1.0" }
//! ```
//!
//! And import it in your crate like this:
//!
//! ```rust
//! // or if you're using it inside a crate of the xmr project:
//! extern crate levin;
//! ```
//!

#[macro_use]
extern crate futures;
extern crate tokio_core;
extern crate tokio_io;

extern crate crossbeam;
extern crate parking_lot;

extern crate bytes;

#[macro_use]
extern crate failure_derive;
extern crate failure;

#[macro_use]
extern crate log;

extern crate portable_storage;

pub mod bucket;
pub mod net;

mod command;
mod error;

pub use command::{COMMAND_BASE_ID, Command, Id};
pub use error::{BucketHeadError, Error, Result};
