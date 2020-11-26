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

use portable_storage::Section;
use std::{net::SocketAddr, sync::Arc};

/// A trait that handles notifications.
pub trait NotificationHandler: Send + Sync + 'static {
    /// This is the function that handles the notification.
    fn call(&self, addr: SocketAddr, request: Section);
}

impl<F> NotificationHandler for F
where
    F: Send + Sync + 'static + Fn(SocketAddr, Section),
{
    fn call(&self, addr: SocketAddr, request: Section) {
        self(addr, request)
    }
}

/// A trait that handles invokations.
pub trait InvokationHandler: Send + Sync + 'static {
    /// This handles the invokation.
    fn call(&self, addr: SocketAddr, request: Section) -> Result<Option<Section>, i32>;
}

impl<F> InvokationHandler for F
where
    F: Send + Sync + 'static + Fn(SocketAddr, Section) -> Result<Option<Section>, i32>,
{
    fn call(&self, addr: SocketAddr, request: Section) -> Result<Option<Section>, i32> {
        self(addr, request)
    }
}

/// A handler for a invokation/notification.
#[allow(missing_debug_implementations)]
#[derive(Clone)]
pub enum RemoteHandler {
    /// A notification.
    Notification(Arc<NotificationHandler>),

    /// An invokation.
    Invokation(Arc<InvokationHandler>),
}
