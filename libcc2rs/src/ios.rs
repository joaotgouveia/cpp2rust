// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use crate::Ptr;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Display, LowerHex, Octal};
use std::fs::File;
use std::io::Write;
use std::os::fd::{AsRawFd, RawFd};

// Integer base of each stream (std::ios_base::basefield), keyed by the
// stream's file descriptor; streams without an entry print in decimal.
thread_local! {
    static BASES: RefCell<HashMap<RawFd, u32>> = RefCell::new(HashMap::new());
}

fn set_base(stream: &File, base: u32) {
    BASES.with(|bases| bases.borrow_mut().insert(stream.as_raw_fd(), base));
}

fn base(stream: &File) -> u32 {
    BASES.with(|bases| *bases.borrow().get(&stream.as_raw_fd()).unwrap_or(&10))
}

/// Writes an integer to the stream in the stream's current base.
pub fn write_int<T: Display + LowerHex + Octal>(stream: &mut File, value: T) {
    let _ = match base(stream) {
        16 => write!(stream, "{value:x}"),
        8 => write!(stream, "{value:o}"),
        _ => write!(stream, "{value}"),
    };
}

/// Refcount counterpart of `write_int`.
pub fn write_int_refcount<T: Display + LowerHex + Octal>(stream: &Ptr<File>, value: T) {
    stream.with_mut(|stream| write_int(stream, value));
}

macro_rules! base_manipulator {
    ($unsafe_name:ident, $refcount_name:ident, $base:expr) => {
        /// # Safety
        ///
        /// `a0` must point to a live stream.
        pub unsafe fn $unsafe_name(a0: *mut File) -> *mut File {
            set_base(unsafe { &*a0 }, $base);
            a0
        }

        pub fn $refcount_name(a0: Ptr<File>) -> Ptr<File> {
            a0.with(|stream| set_base(stream, $base));
            a0
        }
    };
}

base_manipulator!(dec_unsafe, dec_refcount, 10);
base_manipulator!(hex_unsafe, hex_refcount, 16);
base_manipulator!(oct_unsafe, oct_refcount, 8);

macro_rules! stream_manipulator {
    ($unsafe_name:ident, $refcount_name:ident, $bytes:expr, $flush:expr) => {
        /// # Safety
        ///
        /// `a0` must point to a live stream.
        pub unsafe fn $unsafe_name(a0: *mut File) -> *mut File {
            let stream = unsafe { &mut *a0 };
            let _ = stream.write_all($bytes);
            if $flush {
                let _ = stream.flush();
            }
            a0
        }

        pub fn $refcount_name(a0: Ptr<File>) -> Ptr<File> {
            a0.with_mut(|stream| {
                let _ = stream.write_all($bytes);
                if $flush {
                    let _ = stream.flush();
                }
            });
            a0
        }
    };
}

stream_manipulator!(endl_unsafe, endl_refcount, b"\n", true);
stream_manipulator!(ends_unsafe, ends_refcount, b"\0", false);
stream_manipulator!(flush_unsafe, flush_refcount, b"", true);
