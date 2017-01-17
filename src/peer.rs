// Copyright 2016 Timothée Ravier
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use libc::*;
use std::ffi::CString;
use std::fmt;
use std::fmt::Display;
use std::io;
use std::slice;

use error::{Error, ErrorKind, Result};
use ffi::*;
use handle::Handle;
use message::Message;
use messagebuilder::MessageBuilder;

static BUS1_PATH: &'static str = "/dev/bus1";

pub struct Peer {
    pool: *mut uint8_t,
    pool_size: size_t,
    fd: c_int,
    // FIXME: Keep track of handles allocated in 'send_handle_to_peer' ?
}

impl Peer {
    pub fn new() -> io::Result<Peer> {
        unsafe {
            // FIXME: unwrap?
            let fd = open(CString::new(BUS1_PATH).unwrap().as_ptr(),
                          O_RDWR | O_CLOEXEC | O_NOCTTY | O_NONBLOCK);
            if fd < 0 {
                return Err(io::Error::last_os_error())
            }
            let pool: *mut c_void = mmap(0 as *mut c_void,
                                         BUS1_DEFAULT_POOL_SIZE as usize,
                                         PROT_READ, MAP_SHARED, fd, 0);
            if pool == MAP_FAILED {
                return Err(io::Error::last_os_error())
            }
            debug!("New Peer({}) with pool: {:?}, pool size: {}", fd, pool, BUS1_DEFAULT_POOL_SIZE);
            Ok(Peer {
                pool: pool as *mut uint8_t,
                pool_size: BUS1_DEFAULT_POOL_SIZE as usize,
                fd: fd,
            })
        }
    }

    pub fn send_handle_to_peer(&self, handle: &Handle, dst_peer: &Peer) -> Result<Handle> {
        // Source handle values are self allocated but they must not have
        // BUS1_HANDLE_FLAG_MANAGED (and BUS1_HANDLE_FLAG_REMOTE?) set.
        // FIXME?
        // if handle.is_managed() || handle.is_remote() {
        //     return Err(Error::from_raw_os_error(EIO));
        // }
        let mut handle_transfer = bus1_cmd_handle_transfer {
            flags: 0,
            src_handle: handle.to_u64(),
            dst_fd: dst_peer.fd as u64,
            dst_handle: BUS1_HANDLE_INVALID as u64,
        };
        unsafe {
            debug!("Sending {} from {} to {}", handle, self, dst_peer);
            let r = bus1_cmd_handle_transfer(self.fd, &mut handle_transfer);
            if r < 0 {
                return Err(Error::last_os_error());
            }
            // FIXME
            assert!(handle_transfer.src_handle != BUS1_HANDLE_INVALID as u64);
            assert!(handle_transfer.dst_handle != BUS1_HANDLE_INVALID as u64);
            // assert!(handle_transfer.dst_handle & BUS1_HANDLE_FLAG::BUS1_HANDLE_FLAG_MANAGED as u64 > 0);
            // assert!(handle_transfer.dst_handle & BUS1_HANDLE_FLAG::BUS1_HANDLE_FLAG_REMOTE as u64 > 0);
            debug!("Sent Handle({}) from {} to {}: Handle({})", handle_transfer.src_handle, self, dst_peer, handle_transfer.dst_handle);
            return Ok(Handle::from(handle_transfer.dst_handle));
        }
    }

    pub fn send(&self, mut message: MessageBuilder) -> Result<()> {
        // FIXME: message.build should fail at compile time (see PhantomData)
        let mut cmd_send = message.build().unwrap();

        unsafe {
            debug!("Sending message from {} to handles {:?}", self, cmd_send.ptr_destinations);
            let r = bus1_cmd_send(self.fd, &mut cmd_send);
            if r < 0 {
                return Err(Error::last_os_error());
            }
            debug!("Sent message from {} to handles {:?}", self, cmd_send.ptr_destinations);
            return Ok(());
        }
    }

    pub fn recv(&self) -> Result<Message> {
        let mut recv = bus1_cmd_recv::default();

        recv.max_offset = BUS1_DEFAULT_POOL_SIZE;

        debug!("{}: receiving a message", self);
        unsafe {
            let r = bus1_cmd_recv(self.fd, &mut recv);
            if r < 0 {
                return Err(Error::last_os_error());
            }
            // FIXME? libbus1: if recv_msg.n_dropped ?
            if recv.msg.type_ != BUS1_MSG::BUS1_MSG_DATA as u64 &&
                recv.msg.type_ != BUS1_MSG::BUS1_MSG_NODE_DESTROY as u64 &&
                recv.msg.type_ != BUS1_MSG::BUS1_MSG_NODE_RELEASE as u64 {
                return Err(Error::new(ErrorKind::Invalid));
            }
            debug!("{}: received a message", self);
            return Ok(Message::new(
                    recv.msg,
                    self.fd,
                    slice::from_raw_parts(
                        (self.pool as usize + recv.msg.offset as usize + recv.msg.n_bytes as usize) as *const u64,
                        recv.msg.n_handles as usize
                    ),
                )
            );
        }
    }
}

impl Drop for Peer {
    fn drop(&mut self) {
        unsafe {
            munmap(self.pool as *mut c_void, self.pool_size);
            close(self.fd);
            debug!("Dropped {} with pool: {:?}, pool size: {}", self, self.pool, self.pool_size);
        }
    }
}

impl Display for Peer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Peer({})", self.fd)
    }
}
