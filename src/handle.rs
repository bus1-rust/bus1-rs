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
    handle: HandleFlags,
}

impl Handle {
    pub fn new(h: u64) -> Handle {
        h.into()
    }

    pub fn to_u64(&self) -> u64 {
        self.handle.bits()
    }

    pub fn is_valid(&self) -> bool {
        ! self.handle.is_all()
    }

    pub fn is_remote(&self) -> bool {
        self.handle.contains(HANDLE_FLAG_REMOTE)
    }

    pub fn is_managed(&self) -> bool {
        self.handle.contains(HANDLE_FLAG_MANAGED)
    }
}

impl From<u64> for Handle {
    fn from(h: u64) -> Handle {
        debug!("New Handle({}) created from u64", h);
        Handle { handle: HandleFlags::from_bits_truncate(h) }
    }
}

impl Display for Handle {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "Handle({})", self.to_u64())
    }
}
