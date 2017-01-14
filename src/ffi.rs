// Copyright 2016 Timothée Ravier
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(dead_code,
         non_upper_case_globals,
         non_snake_case)]

use std::u64;

pub const BUS1_FD_MAX: u64 = 256;

pub const BUS1_IOCTL_MAGIC: u64 = 0x96;
pub const BUS1_HANDLE_INVALID: u64 = u64::MAX;
pub const BUS1_OFFSET_INVALID: u64 = u64::MAX;

pub const BUS1_DEFAULT_POOL_SIZE: u64 = 1024 * 1024 * 32;

bitflags! {
    pub flags HandleFlags: u64 {
        const HANDLE_FLAG_MANAGED = 1 << 0,
        const HANDLE_FLAG_REMOTE = 1 << 1,
        const HANDLE_FLAG_ID = !(HANDLE_FLAG_MANAGED.bits & HANDLE_FLAG_REMOTE.bits),
    }
}

bitflags! {
    pub flags PeerFlags: u64 {
        const PEER_FLAG_WANT_SECCTX = 1 << 0,
    }
}

bitflags! {
    pub flags PeerResetFlags: u64 {
        const PEER_RESET_FLAG_FLUSH = 1 << 0,
        const PEER_RESET_FLAG_FLUSH_SEED = 1 << 1,
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct bus1_cmd_peer_reset {
    pub flags: PeerResetFlags,
    pub peer_flags: PeerFlags,
    pub max_slices: u32,
    pub max_handles: u32,
    pub max_inflight_bytes: u32,
    pub max_inflight_fds: u32,
}
// FIXME? __attribute__((__aligned__(8)));

impl ::std::default::Default for bus1_cmd_peer_reset {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct bus1_cmd_handle_transfer {
    pub flags: u64,
    pub src_handle: u64,
    pub dst_fd: u64,
    pub dst_handle: u64,
}
// FIXME? __attribute__((__aligned__(8)));

impl ::std::default::Default for bus1_cmd_handle_transfer {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

bitflags! {
    pub flags NodesDestroyFlags: u64 {
        const NODES_DESTROY_FLAG_RELEASE_HANDLES = 1 << 0,
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct bus1_cmd_nodes_destroy {
    pub flags: NodesDestroyFlags,
    pub ptr_nodes: u64,
    pub n_nodes: u64,
}
// FIXME? __attribute__((__aligned__(8)));

impl ::std::default::Default for bus1_cmd_nodes_destroy {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

bitflags! {
    pub flags SendFlags: u64 {
        const SEND_FLAG_CONTINUE = 1 << 0,
        const SEND_FLAG_SEED = 1 << 1,
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct bus1_cmd_send {
    pub flags: SendFlags,
    pub ptr_destinations: *mut u64,
    pub ptr_errors: *mut u64,
    pub n_destinations: u64,
    pub ptr_vecs: *mut u64,
    pub n_vecs: u64,
    pub ptr_handles: *mut u64,
    pub n_handles: u64,
    pub ptr_fds: *mut u64,
    pub n_fds: u64,
}
// FIXME? __attribute__((__aligned__(8)));

impl ::std::default::Default for bus1_cmd_send {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

bitflags! {
    pub flags RecvFlags: u64 {
        const RECV_FLAG_PEEK = 1 << 0,
        const RECV_FLAG_SEED = 1 << 1,
        const RECV_FLAG_INSTALL_FDS = 1 << 2,
    }
}

#[derive(Copy, Clone)]
#[derive(Debug)]
#[repr(u64)]
pub enum Msg {
    None = 0,
    Data = 1,
    NodeDestroy = 2,
    NodeRelease = 3,
}

bitflags! {
    pub flags MsgFlags: u64 {
        const MSG_FLAG_HAS_SECCTX = 1,
        const MSG_FLAG_CONTINUE = 2,
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct bus1_cmd_recv {
    pub flags: u64,
    pub max_offset: u64,
    pub msg: bus1_msg,
}
// FIXME? __attribute__((__aligned__(8)));

impl ::std::default::Default for bus1_cmd_recv {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct bus1_msg {
    // The underscore is required here as 'type' is a Rust keyword
    pub type_: u64,
    pub flags: MsgFlags,
    pub destination: u64,
    pub uid: u32,
    pub gid: u32,
    pub pid: u32,
    pub tid: u32,
    pub offset: u64,
    pub n_bytes: u64,
    pub n_handles: u64,
    pub n_fds: u64,
    pub n_secctx: u64,
}
// FIXME? __attribute__((__aligned__(8)));

impl ::std::default::Default for bus1_msg {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

ioctl!(readwrite bus1_cmd_peer_disconnect with BUS1_IOCTL_MAGIC, 0x00; u64);
ioctl!(readwrite bus1_cmd_peer_query      with BUS1_IOCTL_MAGIC, 0x01; bus1_cmd_peer_reset);
ioctl!(readwrite bus1_cmd_peer_reset      with BUS1_IOCTL_MAGIC, 0x02; bus1_cmd_peer_reset);
ioctl!(readwrite bus1_cmd_handle_release  with BUS1_IOCTL_MAGIC, 0x10; u64);
ioctl!(readwrite bus1_cmd_handle_transfer with BUS1_IOCTL_MAGIC, 0x11; bus1_cmd_handle_transfer);
ioctl!(readwrite bus1_cmd_nodes_destroy   with BUS1_IOCTL_MAGIC, 0x20; bus1_cmd_nodes_destroy);
ioctl!(readwrite bus1_cmd_slice_release   with BUS1_IOCTL_MAGIC, 0x30; u64);
ioctl!(readwrite bus1_cmd_send            with BUS1_IOCTL_MAGIC, 0x40; bus1_cmd_send);
ioctl!(readwrite bus1_cmd_recv            with BUS1_IOCTL_MAGIC, 0x50; bus1_cmd_recv);
