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
    command::{Command, Id},
    net::handlers::{InvokationHandler, NotificationHandler, RemoteHandler},
};
use log::*;
use std::{collections::HashMap, sync::Arc};

/// A reference to an `IoHandler`.
pub type IoHandlerRef = Arc<IoHandler>;

/// Handles external IO.
#[allow(missing_debug_implementations)]
#[derive(Clone)]
pub struct IoHandler {
    handlers: HashMap<Id, RemoteHandler>,
}

impl IoHandler {
    /// Creates an empty `IoHandler`.
    pub fn new() -> IoHandler {
        IoHandler {
            handlers: HashMap::new(),
        }
    }

    /// Creates an `IoHandler` with the given capacity.
    pub fn with_capacity(cap: usize) -> IoHandler {
        IoHandler {
            handlers: HashMap::with_capacity(cap),
        }
    }

    /// Add a notification to this handler.
    pub fn add_notification<C, F>(&mut self, handler: F)
    where
        C: Command,
        F: NotificationHandler + 'static,
    {
        let result = self
            .handlers
            .insert(C::ID, RemoteHandler::Notification(Arc::new(handler)));
        if result.is_some() {
            warn!("Command #{} was previosly added.", C::ID);
        }
        trace!("Adding notification #{}", C::ID);
    }

    /// Add a notification to this handler.
    pub fn add_invokation<C, F>(&mut self, handler: F)
    where
        C: Command,
        F: InvokationHandler + 'static,
    {
        let result = self
            .handlers
            .insert(C::ID, RemoteHandler::Invokation(Arc::new(handler)));
        if result.is_some() {
            warn!("Command #{} was previosly added.", C::ID);
        }
        trace!("Adding invokation #{}", C::ID);
    }

    /// Get a handler.
    pub(crate) fn get(&self, id: Id) -> Option<RemoteHandler> {
        self.handlers.get(&id).cloned()
    }

    /// Converts this `IoHandler` to an `IoHandlerRef`
    pub fn to_ref(self) -> IoHandlerRef {
        Arc::new(self)
    }
}
