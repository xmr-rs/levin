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

/// The start ID of levin commands and notifications.
pub const COMMAND_BASE_ID: u32 = 1000;

/// The id of a command.
pub type Id = u32;

/// A levin command.
///
/// [*See the* `Bucket` *type for more information.*](/bucket/struct.Bucket.html)
pub trait Command {
    /// The ID of this notification.
    ///
    /// Should be higher than [`COMMAND_BASE_ID`][1] and should
    /// be different to other commands/notifications IDs.
    ///
    /// [1]: const.COMMAND_BASE_ID.html
    const ID: Id;
}
