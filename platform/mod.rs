// Copyright 2015 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[cfg(target_os="linux")]
pub use platform::linux::channel;
#[cfg(target_os="linux")]
pub use platform::linux::UnixReceiver as OsIpcReceiver;
#[cfg(target_os="linux")]
pub use platform::linux::UnixSender as OsIpcSender;
#[cfg(target_os="linux")]
pub use platform::linux::UnixReceiverSet as OsIpcReceiverSet;
#[cfg(target_os="linux")]
pub use platform::linux::UnixSharedMemory as OsIpcSharedMemory;
#[cfg(target_os="linux")]
pub use platform::linux::UnixChannel as OsIpcChannel;
#[cfg(target_os="linux")]
pub use platform::linux::UnixSelectionResult as OsIpcSelectionResult;
#[cfg(target_os="linux")]
pub use platform::linux::OpaqueUnixChannel as OsOpaqueIpcChannel;
#[cfg(target_os="linux")]
pub use platform::linux::UnixOneShotServer as OsIpcOneShotServer;

#[cfg(target_os="macos")]
pub use platform::macos::channel;
#[cfg(target_os="macos")]
pub use platform::macos::MachReceiver as OsIpcReceiver;
#[cfg(target_os="macos")]
pub use platform::macos::MachSender as OsIpcSender;
#[cfg(target_os="macos")]
pub use platform::macos::MachReceiverSet as OsIpcReceiverSet;
#[cfg(target_os="macos")]
pub use platform::macos::MachSharedMemory as OsIpcSharedMemory;
#[cfg(target_os="macos")]
pub use platform::macos::MachChannel as OsIpcChannel;
#[cfg(target_os="macos")]
pub use platform::macos::MachSelectionResult as OsIpcSelectionResult;
#[cfg(target_os="macos")]
pub use platform::macos::OpaqueMachChannel as OsOpaqueIpcChannel;
#[cfg(target_os="macos")]
pub use platform::macos::MachOneShotServer as OsIpcOneShotServer;

// Windows and Android use in-process mpsc channels IPC for now
#[cfg(any(target_os="windows", target_os="android"))]
pub use platform::inprocess::channel;
#[cfg(any(target_os="windows", target_os="android"))]
pub use platform::inprocess::MpscReceiver as OsIpcReceiver;
#[cfg(any(target_os="windows", target_os="android"))]
pub use platform::inprocess::MpscSender as OsIpcSender;
#[cfg(any(target_os="windows", target_os="android"))]
pub use platform::inprocess::MpscReceiverSet as OsIpcReceiverSet;
#[cfg(any(target_os="windows", target_os="android"))]
pub use platform::inprocess::MpscSharedMemory as OsIpcSharedMemory;
#[cfg(any(target_os="windows", target_os="android"))]
pub use platform::inprocess::MpscChannel as OsIpcChannel;
#[cfg(any(target_os="windows", target_os="android"))]
pub use platform::inprocess::MpscSelectionResult as OsIpcSelectionResult;
#[cfg(any(target_os="windows", target_os="android"))]
pub use platform::inprocess::OpaqueMpscChannel as OsOpaqueIpcChannel;
#[cfg(any(target_os="windows", target_os="android"))]
pub use platform::inprocess::MpscOneShotServer as OsIpcOneShotServer;

#[cfg(target_os="linux")]
mod linux;
#[cfg(target_os="macos")]
mod macos;
#[cfg(any(target_os="windows", target_os="android"))]
mod inprocess;

#[cfg(test)]
mod test;

