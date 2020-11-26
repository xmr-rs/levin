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

use failure_derive::Fail;
use std::result;

/// A result with a levin error.
pub type Result<T> = result::Result<T, Error>;

/// A levin error.
#[derive(Debug, Fail)]
pub enum Error {
    /// An error when reading `BucketHead`.
    #[fail(display = "couldn't parse bucket head: {}", _0)]
    BucketHead(#[cause] BucketHeadError),

    /// The command is invalid.
    #[fail(display = "the bucket command id ({}) is invalid", _0)]
    InvalidCommandId(u32),
}

impl From<BucketHeadError> for Error {
    fn from(e: BucketHeadError) -> Error {
        Error::BucketHead(e)
    }
}

/// An error returned when the data of `BucketHead` is invalid.
#[derive(Debug, Fail)]
pub enum BucketHeadError {
    /// The version isn't supported.
    #[fail(display = "invalid levin protocol version (provided value: {})", _0)]
    InvalidProtocolVersion(u32),

    /// Invalid signature
    #[fail(display = "invalid bucket signature (bad signature: {:08X})", _0)]
    InvalidSignature(u64),

    /// An error code was returned.
    #[fail(display = "the bucket has an error number: {}", _0)]
    ReturnCode(i32),

    /// Packet too big. The maximum size of a levin bucket is [this][1]
    ///
    /// [1]: /bucket/constant.LEVIN_DEFAULT_MAX_PACKET_SIZE.html
    #[fail(display = "the bucket size is too big ({} bytes)", _0)]
    TooBig(u64),
}
