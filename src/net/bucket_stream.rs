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

use crate::{
    bucket::{Bucket, Receive},
    error::Result,
};
use futures::{stream::Stream, try_ready, Async, Future, Poll};
use std::{io, mem::replace};
use tokio_io::AsyncRead;

/// Creates the bucket stream.
pub fn bucket_stream<A>(a: A) -> BucketStream<A>
where
    A: AsyncRead,
{
    BucketStream {
        future: Bucket::receive_future(a),
    }
}

/// A stream of buckets.
#[derive(Debug)]
pub struct BucketStream<A: AsyncRead> {
    future: Receive<A>,
}

impl<A> Stream for BucketStream<A>
where
    A: AsyncRead,
{
    type Item = Result<Bucket>;
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, io::Error> {
        let (stream, result) = try_ready!(self.future.poll());

        replace(&mut self.future, Bucket::receive_future(stream));

        Ok(Async::Ready(Some(result)))
    }
}
