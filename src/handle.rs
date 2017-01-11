// Copyright (C) 2016-2017 Timothée Ravier
//
// This software may be modified and distributed under the terms
// of the MIT license.  See the LICENSE file for details.

use std::convert::From;
use std::fmt::{Display, Formatter, Error};

use ffi::*;

#[repr(C)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Handle {
    handle: u64,
}

impl Handle {
    pub fn new(h: u64) -> Handle {
        debug!("New Handle({}) created from u64", h);
        Handle { handle: h }
    }

    pub fn to_u64(&self) -> u64 {
        self.handle
    }

    pub fn is_valid(&self) -> bool {
        self.handle != BUS1_HANDLE_INVALID
    }

    pub fn is_remote(&self) -> bool {
        self.handle & BUS1_HANDLE_FLAG::BUS1_HANDLE_FLAG_REMOTE as u64 != 0
    }

    pub fn is_managed(&self) -> bool {
        self.handle & BUS1_HANDLE_FLAG::BUS1_HANDLE_FLAG_MANAGED as u64 != 0
    }
}

impl From<u64> for Handle {
    fn from (h: u64) -> Handle {
        debug!("New Handle({}) created from u64", h);
        Handle { handle: h }
    }
}

impl Display for Handle {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "Handle({})", self.handle)
    }
}
