// Copyright 2016 Timothée Ravier
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::io::{Error, ErrorKind, Result};

use ffi::*;
use handle::Handle;

pub struct MessageBuilder {
    destination_handles: Vec<Handle>,
    handles: Vec<Handle>,
    fds: Vec<u64>,
    data: Vec<Vec<u8>>,
}

impl MessageBuilder {
    pub fn new() -> MessageBuilder {
        MessageBuilder {
            destination_handles: vec![],
            handles: vec![],
            fds: vec![],
            data: vec![],
        }
    }

    pub fn add_destinations(&mut self, handles: &mut Vec<Handle>)  {
        if handles.is_empty() {
            return
        }
        debug!("Adding destinations: {:?}", handles);
        self.destination_handles.append(handles);
    }

    pub fn add_handles(&mut self, handles: &mut Vec<Handle>) {
        if handles.is_empty() {
            return
        }
        debug!("Adding handles: {:?}", handles);
        self.handles.append(handles);
    }

    pub fn add_data(&mut self, data: Vec<u8>) {
        if data.is_empty() {
            return
        }
        self.data.push(data);
    }

    pub fn add_fds(&mut self, fds: &mut Vec<u64>) {
        if fds.is_empty() {
            return
        }
        self.fds.append(fds);
    }

    // TODO: Add phantomdata?
    // pub fn build(self) -> Result<bus1_cmd_send> {
    pub fn build(&mut self) -> Result<bus1_cmd_send> {
        let mut cmd_send: bus1_cmd_send = Default::default();

        if self.destination_handles.len() == 0 {
            return Err(Error::new(ErrorKind::Other, "No destination handles"));
        }
        cmd_send.ptr_destinations = self.destination_handles.as_mut_ptr() as *mut u64;
        cmd_send.n_destinations = self.destination_handles.len() as u64;

        cmd_send.ptr_handles = self.handles.as_mut_ptr() as *mut u64;
        cmd_send.n_handles = self.handles.len() as u64;

        // TODO: fds

        // TODO: data
        // if data.len() != 0 {
        //     cmd_send.ptr_vecs = vec![
        //         iovec {
        //             iov_base: data.as_mut_ptr() as *mut c_void,
        //             iov_len: data.len() as size_t
        //         }
        //     ].as_mut_ptr() as *mut u64;
        //     cmd_send.n_vecs = 1;
        // }

        Ok(cmd_send)
    }
}
