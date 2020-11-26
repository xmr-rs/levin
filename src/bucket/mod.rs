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

//! Buckets
//!
//! Buckets are the packet of information that the levin protocol use
//! to send and receive commands.

mod bucket;
mod bucket_head;

pub use self::{
    bucket::{Bucket, Receive},
    bucket_head::{
        BucketHead, BUCKET_HEAD_LENGTH, LEVIN_DEFAULT_MAX_PACKET_SIZE, LEVIN_OK,
        LEVIN_PACKET_REQUEST, LEVIN_PACKET_RESPONSE, LEVIN_PROTOCOL_VER_1, LEVIN_SIGNATURE,
    },
};
