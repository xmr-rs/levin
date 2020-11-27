// Copyright 2018 Jean Pierre Dudey <jeandudey@hotmail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::result;
use thiserror::Error;

/// A result with a levin error.
pub type Result<T> = result::Result<T, Error>;

/// A levin error.
#[derive(Debug, Error)]
pub enum Error {
    /// An error when reading `BucketHead`.
    #[error("couldn't parse bucket head: {0}")]
    BucketHead(#[from] BucketHeadError),

    /// The command is invalid.
    #[error("the bucket command id ({0}) is invalid")]
    InvalidCommandId(u32),
}

/// An error returned when the data of `BucketHead` is invalid.
#[derive(Debug, Error)]
pub enum BucketHeadError {
    /// The version isn't supported.
    #[error("invalid levin protocol version (provided value: {0})")]
    InvalidProtocolVersion(u32),

    /// Invalid signature
    #[error("invalid bucket signature (bad signature: {:08X})", .0)]
    InvalidSignature(u64),

    /// An error code was returned.
    #[error("the bucket has an error number: {0}")]
    ReturnCode(i32),

    /// Packet too big. The maximum size of a levin bucket is [this][1]
    ///
    /// [1]: /bucket/constant.LEVIN_DEFAULT_MAX_PACKET_SIZE.html
    #[error("the bucket size is too big ({0} bytes)")]
    TooBig(u64),
}
