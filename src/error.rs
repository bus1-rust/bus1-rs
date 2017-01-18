// Copyright 2016 Timothée Ravier
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use libc;
use std::borrow::Cow;
use std::fmt;
use std::io;
use std::result;

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind
}

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ErrorKind {
    NoMessageReady,
    InvalidFileDesc,
    ResQuotaExceeded,
    IoctlParamError,
    DestObjNotAvail,
    InvalidIoctlParam,
    MessageSizeTooBig,
    OutOfKernelMem,
    UnknownIoctl,
    UnknownObj,
    OperationNotSup,
    PermissionDenied,
    WouldExceedOffset,
    LocalPeerDisco,
    Invalid,
}

impl Error {
    pub fn new(k: ErrorKind) -> Error {
        Error { kind: k }
    }

    pub fn kind(&self) -> ErrorKind {
        self.kind
    }

    pub fn last_os_error() -> Error {
        // FIXME?
        Error::from_raw_os_error(io::Error::last_os_error().raw_os_error().unwrap())
    }

    pub fn from_raw_os_error(code: i32) -> Error {
        let k = match code {
            libc::EAGAIN => ErrorKind::NoMessageReady,
            libc::EBADF => ErrorKind::InvalidFileDesc,
            libc::EDQUOT => ErrorKind::ResQuotaExceeded,
            libc::EFAULT => ErrorKind::IoctlParamError,
            libc::EHOSTUNREACH => ErrorKind::DestObjNotAvail,
            libc::EINVAL => ErrorKind::InvalidIoctlParam,
            libc::EMSGSIZE => ErrorKind::MessageSizeTooBig,
            libc::ENOMEM => ErrorKind::OutOfKernelMem,
            libc::ENOTTY => ErrorKind::UnknownIoctl,
            libc::ENXIO => ErrorKind::UnknownObj,
            libc::EOPNOTSUPP => ErrorKind::OperationNotSup,
            libc::EPERM => ErrorKind::PermissionDenied,
            libc::ERANGE => ErrorKind::WouldExceedOffset,
            libc::ESHUTDOWN => ErrorKind::LocalPeerDisco,
            _ => ErrorKind::Invalid,
        };
        Error { kind: k }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        let desc: Cow<_> = match self.kind {
            ErrorKind::NoMessageReady => "No messages ready to be read.".into(),
            ErrorKind::InvalidFileDesc => "Invalid file descriptor.".into(),
            ErrorKind::ResQuotaExceeded => "Resource quota exceeded.".into(),
            ErrorKind::IoctlParamError => "Cannot read, or write, ioctl parameters.".into(),
            ErrorKind::DestObjNotAvail => "The destination object is no longer available.".into(),
            ErrorKind::InvalidIoctlParam => "Invalid ioctl parameters.".into(),
            ErrorKind::MessageSizeTooBig => "The message to be sent exceeds its allowed resource limits.".into(),
            ErrorKind::OutOfKernelMem => "Out of kernel memory.".into(),
            ErrorKind::UnknownIoctl => "Unknown ioctl.".into(),
            ErrorKind::UnknownObj => "Unknown object.".into(),
            ErrorKind::OperationNotSup => "Operation not supported.".into(),
            ErrorKind::PermissionDenied => "Permission denied.".into(),
            ErrorKind::WouldExceedOffset => "The message to be received would exceed the maximal offset.".into(),
            ErrorKind::LocalPeerDisco => "Local peer was already disconnected.".into(),
            ErrorKind::Invalid => "Invalid error code.".into(),
        };
        write!(out, "{}", desc)
    }
}
