// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use crate::Ptr;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Display, LowerHex, Octal, UpperHex};
use std::fs::File;
use std::io::Write;
use std::os::fd::{AsRawFd, RawFd};

// Format state of each stream (std::ios_base::fmtflags), keyed by the
// stream's file descriptor; streams without an entry use the defaults.
// Fields without a reader yet are kept so manipulators record the full state.
#[allow(dead_code)]
#[derive(Clone, Copy)]
struct Flags {
    base: u32,
    boolalpha: bool,
    showbase: bool,
    showpoint: bool,
    showpos: bool,
    skipws: bool,
    unitbuf: bool,
    uppercase: bool,
    adjust: Adjust,
    float: FloatField,
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
enum Adjust {
    Default,
    Left,
    Right,
    Internal,
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
enum FloatField {
    Default,
    Fixed,
    Scientific,
    Hex,
}

impl Default for Flags {
    fn default() -> Self {
        Flags {
            base: 10,
            boolalpha: false,
            showbase: false,
            showpoint: false,
            showpos: false,
            skipws: true,
            unitbuf: false,
            uppercase: false,
            adjust: Adjust::Default,
            float: FloatField::Default,
        }
    }
}

thread_local! {
    static FLAGS: RefCell<HashMap<RawFd, Flags>> = RefCell::new(HashMap::new());
}

fn flags(stream: &File) -> Flags {
    FLAGS.with(|flags| {
        flags
            .borrow()
            .get(&stream.as_raw_fd())
            .copied()
            .unwrap_or_default()
    })
}

fn update(stream: &File, f: impl FnOnce(&mut Flags)) {
    FLAGS.with(|flags| f(flags.borrow_mut().entry(stream.as_raw_fd()).or_default()));
}

/// An integer as `num_put` prints it: base, showbase, uppercase, showpos.
pub trait StreamInt: Display + LowerHex + UpperHex + Octal + Copy {
    fn is_signed_positive(self) -> bool;
    fn is_zero(self) -> bool;
}

macro_rules! stream_int {
    ($($t:ty),*) => {$(
        impl StreamInt for $t {
            #[allow(unused_comparisons)]
            fn is_signed_positive(self) -> bool {
                <$t>::MIN < 0 && self >= 0
            }
            fn is_zero(self) -> bool {
                self == 0
            }
        }
    )*};
}

stream_int!(i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize);

fn format_int<T: StreamInt>(flags: Flags, value: T) -> String {
    match flags.base {
        16 => {
            let digits = if flags.uppercase {
                format!("{value:X}")
            } else {
                format!("{value:x}")
            };
            if flags.showbase && !value.is_zero() {
                format!("{}{digits}", if flags.uppercase { "0X" } else { "0x" })
            } else {
                digits
            }
        }
        8 => {
            if flags.showbase && !value.is_zero() {
                format!("0{value:o}")
            } else {
                format!("{value:o}")
            }
        }
        _ => {
            if flags.showpos && value.is_signed_positive() {
                format!("+{value}")
            } else {
                format!("{value}")
            }
        }
    }
}

/// Writes an integer to the stream in the stream's current format.
pub fn write_int<T: StreamInt>(stream: &mut File, value: T) {
    let text = format_int(flags(stream), value);
    let _ = stream.write_all(text.as_bytes());
}

/// Refcount counterpart of `write_int`.
pub fn write_int_refcount<T: StreamInt>(stream: &Ptr<File>, value: T) {
    stream.with_mut(|stream| write_int(stream, value));
}

/// Writes a bool to the stream: `true`/`false` with boolalpha, else as an int.
pub fn write_bool(stream: &mut File, value: bool) {
    if flags(stream).boolalpha {
        let _ = stream.write_all(if value { b"true" } else { b"false" });
    } else {
        write_int(stream, value as i64);
    }
}

/// Refcount counterpart of `write_bool`.
pub fn write_bool_refcount(stream: &Ptr<File>, value: bool) {
    stream.with_mut(|stream| write_bool(stream, value));
}

macro_rules! flag_manipulator {
    ($unsafe_name:ident, $refcount_name:ident, |$f:ident| $set:expr) => {
        /// # Safety
        ///
        /// `a0` must point to a live stream.
        pub unsafe fn $unsafe_name(a0: *mut File) -> *mut File {
            update(unsafe { &*a0 }, |$f| $set);
            a0
        }

        pub fn $refcount_name(a0: Ptr<File>) -> Ptr<File> {
            a0.with(|stream| update(stream, |$f| $set));
            a0
        }
    };
}

flag_manipulator!(dec_unsafe, dec_refcount, |f| f.base = 10);
flag_manipulator!(hex_unsafe, hex_refcount, |f| f.base = 16);
flag_manipulator!(oct_unsafe, oct_refcount, |f| f.base = 8);
flag_manipulator!(boolalpha_unsafe, boolalpha_refcount, |f| f.boolalpha = true);
flag_manipulator!(noboolalpha_unsafe, noboolalpha_refcount, |f| f.boolalpha = false);
flag_manipulator!(showbase_unsafe, showbase_refcount, |f| f.showbase = true);
flag_manipulator!(noshowbase_unsafe, noshowbase_refcount, |f| f.showbase = false);
flag_manipulator!(showpoint_unsafe, showpoint_refcount, |f| f.showpoint = true);
flag_manipulator!(noshowpoint_unsafe, noshowpoint_refcount, |f| f.showpoint = false);
flag_manipulator!(showpos_unsafe, showpos_refcount, |f| f.showpos = true);
flag_manipulator!(noshowpos_unsafe, noshowpos_refcount, |f| f.showpos = false);
flag_manipulator!(skipws_unsafe, skipws_refcount, |f| f.skipws = true);
flag_manipulator!(noskipws_unsafe, noskipws_refcount, |f| f.skipws = false);
flag_manipulator!(unitbuf_unsafe, unitbuf_refcount, |f| f.unitbuf = true);
flag_manipulator!(nounitbuf_unsafe, nounitbuf_refcount, |f| f.unitbuf = false);
flag_manipulator!(uppercase_unsafe, uppercase_refcount, |f| f.uppercase = true);
flag_manipulator!(nouppercase_unsafe, nouppercase_refcount, |f| f.uppercase = false);
flag_manipulator!(left_unsafe, left_refcount, |f| f.adjust = Adjust::Left);
flag_manipulator!(right_unsafe, right_refcount, |f| f.adjust = Adjust::Right);
flag_manipulator!(internal_unsafe, internal_refcount, |f| f.adjust = Adjust::Internal);
flag_manipulator!(fixed_unsafe, fixed_refcount, |f| f.float = FloatField::Fixed);
flag_manipulator!(scientific_unsafe, scientific_refcount, |f| f.float = FloatField::Scientific);
flag_manipulator!(hexfloat_unsafe, hexfloat_refcount, |f| f.float = FloatField::Hex);
flag_manipulator!(defaultfloat_unsafe, defaultfloat_refcount, |f| f.float = FloatField::Default);

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
