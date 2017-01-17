// Copyright 2016 Timothée Ravier
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

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
