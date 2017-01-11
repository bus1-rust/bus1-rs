// Copyright (C) 2016-2017 Timothée Ravier
//
// This software may be modified and distributed under the terms
// of the MIT license.  See the LICENSE file for details.

#![allow(dead_code,
         non_camel_case_types,
         non_upper_case_globals,
         non_snake_case)]

use std::u64;

pub const BUS1_FD_MAX: u64 = 256;

pub const BUS1_IOCTL_MAGIC: u64 = 0x96;
pub const BUS1_HANDLE_INVALID: u64 = u64::MAX;
pub const BUS1_OFFSET_INVALID: u64 = u64::MAX;

pub const BUS1_DEFAULT_POOL_SIZE: u64 = 1024 * 1024 * 32;

#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum BUS1_HANDLE_FLAG {
    BUS1_HANDLE_FLAG_MANAGED = 1 << 0,
    BUS1_HANDLE_FLAG_REMOTE = 1 << 1,
}

#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum BUS1_PEER_FLAG {
    BUS1_PEER_FLAG_WANT_SECCTX = 1 << 0,
}

#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum BUS1_PEER_RESET_FLAG {
    BUS1_PEER_RESET_FLAG_FLUSH = 1 << 0,
    BUS1_PEER_RESET_FLAG_FLUSH_SEED = 1 << 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct bus1_cmd_peer_reset {
    pub flags: u64,
    pub peer_flags: u64,
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

#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum BUS1_NODES_DESTROY_FLAG {
    BUS1_NODES_DESTROY_FLAG_RELEASE_HANDLES = 1 << 0,
}

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct bus1_cmd_nodes_destroy {
    pub flags: u64,
    pub ptr_nodes: u64,
    pub n_nodes: u64,
}
// FIXME? __attribute__((__aligned__(8)));

impl ::std::default::Default for bus1_cmd_nodes_destroy {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum BUS1_SEND_FLAG {
    BUS1_SEND_FLAG_CONTINUE = 1 << 0,
    BUS1_SEND_FLAG_SEED = 1 << 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct bus1_cmd_send {
    pub flags: u64,
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

#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum EBUS1_RECV_FLAG {
    BUS1_RECV_FLAG_PEEK = 1 << 0,
    BUS1_RECV_FLAG_SEED = 1 << 1,
    BUS1_RECV_FLAG_INSTALL_FDS = 1 << 2,
}

#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum BUS1_MSG {
    BUS1_MSG_NONE = 0,
    BUS1_MSG_DATA = 1,
    BUS1_MSG_NODE_DESTROY = 2,
    BUS1_MSG_NODE_RELEASE = 3,
}

#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum BUS1_MSG_FLAG {
    BUS1_MSG_FLAG_HAS_SECCTX = 1,
    BUS1_MSG_FLAG_CONTINUE = 2,
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
    pub flags: u64,
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
