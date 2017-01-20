// Copyright 2016 Timothée Ravier
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use libc::*;
use std::ops::Drop;

use ffi::*;
use handle::Handle;
use error::Error;

pub struct Message<'a> {
    msg: bus1_msg,

    peer_fd: c_int,
    handles: &'a [u64],

    // type: BUS1_MSG,
    // destination: u64,
    // uid: uid_t,
    // gid: gid_t,
    // pid: pid_t,
    // tid: tid_t,
    // struct iovecs *vecs,
    // size_t n_vecs,
    // B1Handle **handles,
    // size_t n_handles,
    // int *fds,
    // size_t n_fds
}

impl<'a> Message<'a> {
    pub fn new(msg: bus1_msg, fd: c_int, handles: &'a [u64],) -> Message<'a> {
        debug!("New message received on handle: {}, from {}({}) {}:{}, offset: {}, n_bytes: {}, n_handles: {}, n_fds: {}",
               msg.destination, msg.pid, msg.tid, msg.uid, msg.gid, msg.offset, msg.n_bytes, msg.n_handles, msg.n_fds);
        Message {
            msg: msg,
            peer_fd: fd,
            handles: handles,
        }
    }

    pub fn type_(&self) -> u64 {
        self.msg.type_
    }

    pub fn flags(&self) -> MsgFlags {
        self.msg.flags
    }

    pub fn destination(&self) -> Handle {
        Handle::new(self.msg.destination)
    }

    pub fn release_slice(self) {
        ()
    }

    fn _release_slice(&self) -> Option<Error> {
        unsafe {
            // FIXME: bus1 ioctl is readwrite but does not use this arguement
            let mut fixme_offset = self.msg.offset;
            let r = bus1_cmd_slice_release(self.peer_fd, &mut fixme_offset);
            if r < 0 {
                debug!("Could not drop slice (offset: {}, n_bytes: {})", self.msg.offset, self.msg.n_bytes);
                return Some(Error::last_os_error());
            }
        }
        debug!("Dropped slice (offset: {}, n_bytes: {})", self.msg.offset, self.msg.n_bytes);
        None
    }

    pub fn get_handles(&self) -> &'a [u64] {
        self.handles
    }
}

impl<'a> Drop for Message<'a> {
    fn drop(&mut self) {
        match self._release_slice() {
            Some(e) => panic!(e), // FIXME: pragma/feature debug
            None => ()
        }
    }
}
