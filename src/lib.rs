// Copyright 2016 Timothée Ravier
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Wrapper around the bus1 IPC for Linux.
//!
//! This project is unstable and will change to match upstream bus1 changes.
//!
//! The currently exposed API is incomplete and probably not safe.
//!
//! # Example
//!
//! ```
//! #[macro_use] extern crate log;
//! extern crate env_logger;
//! extern crate bus1;
//!
//! use bus1::*;
//!
//! fn main() {
//!     env_logger::init().unwrap();
//!
//!     let peer3 = Peer::new().unwrap();
//!     let peer4 = Peer::new().unwrap();
//!     let peer5 = Peer::new().unwrap();
//!
//!     let node1 = Handle::new(4);
//!
//!     let peer4_node1 = peer3.send_handle_to_peer(&node1, &peer4).unwrap();
//!
//!     let mut msg = MessageBuilder::new();
//!     msg.add_destinations(&mut vec![peer4_node1]);
//!
//!     peer4.send(msg).unwrap();
//!
//!     let message = peer3.recv().unwrap();
//!
//!     Message::release_slice(message);
//!
//!     let node2 = Handle::new(8);
//!
//!     let peer5_node2 = peer3.send_handle_to_peer(&node2, &peer5).unwrap();
//!
//!     let mut msg = MessageBuilder::new();
//!     msg.add_destinations(&mut vec![peer5_node2]);
//!
//!     peer5.send(msg).unwrap();
//!
//!     let message = peer3.recv().unwrap();
//!
//!     Message::release_slice(message);
//! }
//! ```
//!
//! To run this example with debug output:
//!
//! ```shell
//! $ RUST_LOG="bus1=debug" cargo run
//! ```

extern crate libc;
#[macro_use] extern crate log;
#[macro_use] extern crate ioctl;

pub use error::*;
pub use handle::*;
pub use message::*;
pub use messagebuilder::*;
pub use peer::*;

mod error;
mod ffi;
mod handle;
mod message;
mod messagebuilder;
mod peer;
