// Copyright 2018 Jean Pierre Dudey <jeandudey@hotmail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

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

//! Rust implementation of the levin protocol used in the Monero cryptocurrency.
//!

pub mod bucket;
pub mod net;

mod command;
mod error;

pub use command::{Command, Id, COMMAND_BASE_ID};
pub use error::{BucketHeadError, Error, Result};
