// Copyright 2018 Jean Pierre Dudey <jeandudey@hotmail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::{
    bucket::{Bucket, Receive},
    error::Result,
};
use futures::{stream::Stream, try_ready, Async, Future, Poll};
use std::io;
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

        self.future = Bucket::receive_future(stream);

        Ok(Async::Ready(Some(result)))
    }
}
