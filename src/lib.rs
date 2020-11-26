// Copyright 2020 Jean Pierre Dudey <me@jeandudey.tech>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![deny(
    anonymous_parameters,
    missing_debug_implementations,
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unsafe_code,
    unstable_features,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications
)]

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

pub mod bucket;
pub mod net;

mod command;
mod error;

pub use command::{Command, Id, COMMAND_BASE_ID};
pub use error::{BucketHeadError, Error, Result};
