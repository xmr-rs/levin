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

use crate::error::{BucketHeadError, Result};
use bytes::{Buf, BufMut, BytesMut, LittleEndian};

/// BucketHead signature.
pub const LEVIN_SIGNATURE: u64 = 0x0101010101012101;

/// Identifies a command as a request.
pub const LEVIN_PACKET_REQUEST: u32 = 0x00000001;
/// Identifies a command as a response.
pub const LEVIN_PACKET_RESPONSE: u32 = 0x00000002;

/// Levin maximum packet size. It's default is 100 *MB*.
pub const LEVIN_DEFAULT_MAX_PACKET_SIZE: u64 = 100000000;

/// Current levin protocol version.
pub const LEVIN_PROTOCOL_VER_1: u32 = 1;

/// Size in bytes of `BucketHead`.
pub const BUCKET_HEAD_LENGTH: usize = 33;

/// Ok return code.
pub const LEVIN_OK: i32 = 0;

/// Header of a levin bucket.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BucketHead {
    /// This identifies the stream as a valid header.
    pub signature: u64,

    /// Size in bytes of the rest.
    pub cb: u64,

    /// Specifies if a command has to return a response.
    pub have_to_return_data: bool,

    /// The command ID.
    pub command: u32,

    /// The return code.
    pub return_code: i32,

    /// Flags of this header.
    pub flags: u32,

    /// The levin protocol version.
    pub protocol_version: u32,
}

impl BucketHead {
    /// Read a `BucketHead` from a buffer.
    ///
    /// # Panics
    ///
    /// This function panics if the parameter `buf` isn't greater than
    /// or equal to [`BUCKET_HEAD_LENGTH`][1].
    ///
    /// [1]: constant.BUCKET_HEAD_LENGTH.html
    pub fn read<B: Buf>(buf: &mut B) -> Result<Self> {
        assert!(buf.remaining() >= BUCKET_HEAD_LENGTH);

        let bucket_head = BucketHead {
            signature: buf.get_u64::<LittleEndian>(),
            cb: buf.get_u64::<LittleEndian>(),
            have_to_return_data: buf.get_u8() != 0,
            command: buf.get_u32::<LittleEndian>(),
            return_code: buf.get_i32::<LittleEndian>(),
            flags: buf.get_u32::<LittleEndian>(),
            protocol_version: buf.get_u32::<LittleEndian>(),
        };

        if bucket_head.signature != LEVIN_SIGNATURE {
            return Err(BucketHeadError::InvalidSignature(bucket_head.signature).into());
        }

        if bucket_head.protocol_version != LEVIN_PROTOCOL_VER_1 {
            return Err(
                BucketHeadError::InvalidProtocolVersion(bucket_head.protocol_version).into(),
            );
        }

        if bucket_head.cb > LEVIN_DEFAULT_MAX_PACKET_SIZE {
            return Err(BucketHeadError::TooBig(bucket_head.cb).into());
        }

        if bucket_head.return_code < 0 {
            return Err(BucketHeadError::ReturnCode(bucket_head.return_code).into());
        }

        Ok(bucket_head)
    }

    /// Write a `BucketHead` to a buffer.
    pub fn write(buf: &mut BytesMut, bucket_head: &BucketHead) {
        buf.reserve(BUCKET_HEAD_LENGTH);

        let have_to_return_data = if bucket_head.have_to_return_data {
            1u8
        } else {
            0u8
        };

        buf.put_u64::<LittleEndian>(bucket_head.signature);
        buf.put_u64::<LittleEndian>(bucket_head.cb);
        buf.put_u8(have_to_return_data);
        buf.put_u32::<LittleEndian>(bucket_head.command);
        buf.put_i32::<LittleEndian>(bucket_head.return_code);
        buf.put_u32::<LittleEndian>(bucket_head.flags);
        buf.put_u32::<LittleEndian>(bucket_head.protocol_version);
    }

    /// Checks if this bucket is a request, returns `true` if it is.
    pub fn is_request(&self) -> bool {
        self.flags & LEVIN_PACKET_REQUEST == LEVIN_PACKET_REQUEST
    }
}
