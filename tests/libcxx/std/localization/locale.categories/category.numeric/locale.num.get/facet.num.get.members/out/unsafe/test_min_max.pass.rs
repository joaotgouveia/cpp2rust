extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn check_stream_failed_0(val: *const Vec<libc::c_char>) -> bool {
    let mut ss: () = unimplemented!();
    let mut result: i16 = 0_i16;
    return unimplemented!();
}
pub unsafe fn check_stream_failed_1(val: *const Vec<libc::c_char>) -> bool {
    let mut ss: () = unimplemented!();
    let mut result: u16 = 0_u16;
    return unimplemented!();
}
pub unsafe fn check_stream_failed_2(val: *const Vec<libc::c_char>) -> bool {
    let mut ss: () = unimplemented!();
    let mut result: i32 = 0_i32;
    return unimplemented!();
}
pub unsafe fn check_stream_failed_3(val: *const Vec<libc::c_char>) -> bool {
    let mut ss: () = unimplemented!();
    let mut result: u32 = 0_u32;
    return unimplemented!();
}
pub unsafe fn check_stream_failed_4(val: *const Vec<libc::c_char>) -> bool {
    let mut ss: () = unimplemented!();
    let mut result: i64 = 0_i64;
    return unimplemented!();
}
pub unsafe fn check_stream_failed_5(val: *const Vec<libc::c_char>) -> bool {
    let mut ss: () = unimplemented!();
    let mut result: u64 = 0_u64;
    return unimplemented!();
}
pub unsafe fn check_stream_failed_6(val: *const Vec<libc::c_char>) -> bool {
    let mut ss: () = unimplemented!();
    let mut result: i64 = 0_i64;
    return unimplemented!();
}
pub unsafe fn check_stream_failed_7(val: *const Vec<libc::c_char>) -> bool {
    let mut ss: () = unimplemented!();
    let mut result: u64 = 0_u64;
    return unimplemented!();
}
pub unsafe fn check_limits_8() {
    let is_unsigned: bool = unimplemented!();
    let mut minv: i16 = i16::MIN;
    let mut maxv: i16 = i16::MAX;
    let mut miniss: () = unimplemented!();
    let mut maxiss: () = unimplemented!();
    assert!(unimplemented!());
    assert!(unimplemented!());
    let mut mins: Vec<libc::c_char> = unimplemented!();
    let mut maxs: Vec<libc::c_char> = unimplemented!();
    let mut maxoss: () = unimplemented!();
    let mut minoss: () = unimplemented!();
    let mut new_minv: i16 = 0_i16;
    let mut new_maxv: i16 = 0_i16;
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(((new_minv as i32) == (minv as i32)));
    assert!(((new_maxv as i32) == (maxv as i32)));
    (*(&mut maxs)
        .as_mut_ptr()
        .add((maxs.len() - 1).wrapping_sub(1_usize) as usize))
    .postfix_inc();
    assert!((unsafe { check_stream_failed_0(&maxs,) }));
    {
        (*(&mut mins)
            .as_mut_ptr()
            .add((mins.len() - 1).wrapping_sub(1_usize) as usize))
        .postfix_inc();
        assert!((unsafe { check_stream_failed_0(&mins,) }));
    }
}
pub unsafe fn check_limits_9() {
    let is_unsigned: bool = unimplemented!();
    let mut minv: u16 = u16::MIN;
    let mut maxv: u16 = u16::MAX;
    let mut miniss: () = unimplemented!();
    let mut maxiss: () = unimplemented!();
    assert!(unimplemented!());
    assert!(unimplemented!());
    let mut mins: Vec<libc::c_char> = unimplemented!();
    let mut maxs: Vec<libc::c_char> = unimplemented!();
    let mut maxoss: () = unimplemented!();
    let mut minoss: () = unimplemented!();
    let mut new_minv: u16 = 0_u16;
    let mut new_maxv: u16 = 0_u16;
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(((new_minv as i32) == (minv as i32)));
    assert!(((new_maxv as i32) == (maxv as i32)));
    (*(&mut maxs)
        .as_mut_ptr()
        .add((maxs.len() - 1).wrapping_sub(1_usize) as usize))
    .postfix_inc();
    assert!((unsafe { check_stream_failed_1(&maxs,) }));
    {}
}
pub unsafe fn check_limits_10() {
    let is_unsigned: bool = unimplemented!();
    let mut minv: i32 = i32::MIN;
    let mut maxv: i32 = i32::MAX;
    let mut miniss: () = unimplemented!();
    let mut maxiss: () = unimplemented!();
    assert!(unimplemented!());
    assert!(unimplemented!());
    let mut mins: Vec<libc::c_char> = unimplemented!();
    let mut maxs: Vec<libc::c_char> = unimplemented!();
    let mut maxoss: () = unimplemented!();
    let mut minoss: () = unimplemented!();
    let mut new_minv: i32 = 0_i32;
    let mut new_maxv: i32 = 0_i32;
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(((new_minv) == (minv)));
    assert!(((new_maxv) == (maxv)));
    (*(&mut maxs)
        .as_mut_ptr()
        .add((maxs.len() - 1).wrapping_sub(1_usize) as usize))
    .postfix_inc();
    assert!((unsafe { check_stream_failed_2(&maxs,) }));
    {
        (*(&mut mins)
            .as_mut_ptr()
            .add((mins.len() - 1).wrapping_sub(1_usize) as usize))
        .postfix_inc();
        assert!((unsafe { check_stream_failed_2(&mins,) }));
    }
}
pub unsafe fn check_limits_11() {
    let is_unsigned: bool = unimplemented!();
    let mut minv: u32 = u32::MIN;
    let mut maxv: u32 = u32::MAX;
    let mut miniss: () = unimplemented!();
    let mut maxiss: () = unimplemented!();
    assert!(unimplemented!());
    assert!(unimplemented!());
    let mut mins: Vec<libc::c_char> = unimplemented!();
    let mut maxs: Vec<libc::c_char> = unimplemented!();
    let mut maxoss: () = unimplemented!();
    let mut minoss: () = unimplemented!();
    let mut new_minv: u32 = 0_u32;
    let mut new_maxv: u32 = 0_u32;
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(((new_minv) == (minv)));
    assert!(((new_maxv) == (maxv)));
    (*(&mut maxs)
        .as_mut_ptr()
        .add((maxs.len() - 1).wrapping_sub(1_usize) as usize))
    .postfix_inc();
    assert!((unsafe { check_stream_failed_3(&maxs,) }));
    {}
}
pub unsafe fn check_limits_12() {
    let is_unsigned: bool = unimplemented!();
    let mut minv: i64 = i64::MIN;
    let mut maxv: i64 = i64::MAX;
    let mut miniss: () = unimplemented!();
    let mut maxiss: () = unimplemented!();
    assert!(unimplemented!());
    assert!(unimplemented!());
    let mut mins: Vec<libc::c_char> = unimplemented!();
    let mut maxs: Vec<libc::c_char> = unimplemented!();
    let mut maxoss: () = unimplemented!();
    let mut minoss: () = unimplemented!();
    let mut new_minv: i64 = 0_i64;
    let mut new_maxv: i64 = 0_i64;
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(((new_minv) == (minv)));
    assert!(((new_maxv) == (maxv)));
    (*(&mut maxs)
        .as_mut_ptr()
        .add((maxs.len() - 1).wrapping_sub(1_usize) as usize))
    .postfix_inc();
    assert!((unsafe { check_stream_failed_4(&maxs,) }));
    {
        (*(&mut mins)
            .as_mut_ptr()
            .add((mins.len() - 1).wrapping_sub(1_usize) as usize))
        .postfix_inc();
        assert!((unsafe { check_stream_failed_4(&mins,) }));
    }
}
pub unsafe fn check_limits_13() {
    let is_unsigned: bool = unimplemented!();
    let mut minv: u64 = u64::MIN;
    let mut maxv: u64 = u64::MAX;
    let mut miniss: () = unimplemented!();
    let mut maxiss: () = unimplemented!();
    assert!(unimplemented!());
    assert!(unimplemented!());
    let mut mins: Vec<libc::c_char> = unimplemented!();
    let mut maxs: Vec<libc::c_char> = unimplemented!();
    let mut maxoss: () = unimplemented!();
    let mut minoss: () = unimplemented!();
    let mut new_minv: u64 = 0_u64;
    let mut new_maxv: u64 = 0_u64;
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(((new_minv) == (minv)));
    assert!(((new_maxv) == (maxv)));
    (*(&mut maxs)
        .as_mut_ptr()
        .add((maxs.len() - 1).wrapping_sub(1_usize) as usize))
    .postfix_inc();
    assert!((unsafe { check_stream_failed_5(&maxs,) }));
    {}
}
pub unsafe fn check_limits_14() {
    let is_unsigned: bool = unimplemented!();
    let mut minv: i64 = i64::MIN;
    let mut maxv: i64 = i64::MAX;
    let mut miniss: () = unimplemented!();
    let mut maxiss: () = unimplemented!();
    assert!(unimplemented!());
    assert!(unimplemented!());
    let mut mins: Vec<libc::c_char> = unimplemented!();
    let mut maxs: Vec<libc::c_char> = unimplemented!();
    let mut maxoss: () = unimplemented!();
    let mut minoss: () = unimplemented!();
    let mut new_minv: i64 = 0_i64;
    let mut new_maxv: i64 = 0_i64;
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(((new_minv) == (minv)));
    assert!(((new_maxv) == (maxv)));
    (*(&mut maxs)
        .as_mut_ptr()
        .add((maxs.len() - 1).wrapping_sub(1_usize) as usize))
    .postfix_inc();
    assert!((unsafe { check_stream_failed_6(&maxs,) }));
    {
        (*(&mut mins)
            .as_mut_ptr()
            .add((mins.len() - 1).wrapping_sub(1_usize) as usize))
        .postfix_inc();
        assert!((unsafe { check_stream_failed_6(&mins,) }));
    }
}
pub unsafe fn check_limits_15() {
    let is_unsigned: bool = unimplemented!();
    let mut minv: u64 = unimplemented!();
    let mut maxv: u64 = unimplemented!();
    let mut miniss: () = unimplemented!();
    let mut maxiss: () = unimplemented!();
    assert!(unimplemented!());
    assert!(unimplemented!());
    let mut mins: Vec<libc::c_char> = unimplemented!();
    let mut maxs: Vec<libc::c_char> = unimplemented!();
    let mut maxoss: () = unimplemented!();
    let mut minoss: () = unimplemented!();
    let mut new_minv: u64 = 0_u64;
    let mut new_maxv: u64 = 0_u64;
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(((new_minv) == (minv)));
    assert!(((new_maxv) == (maxv)));
    (*(&mut maxs)
        .as_mut_ptr()
        .add((maxs.len() - 1).wrapping_sub(1_usize) as usize))
    .postfix_inc();
    assert!((unsafe { check_stream_failed_7(&maxs,) }));
    {}
}
pub fn main() {
    let mut args: Vec<Vec<u8>> = std::env::args()
        .map(|arg| arg.as_bytes().to_vec())
        .collect();
    args.iter_mut().for_each(|v| v.push(0));
    let mut argv: Vec<*mut libc::c_char> = args
        .iter()
        .map(|arg| arg.as_ptr() as *mut libc::c_char)
        .collect();
    argv.push(::std::ptr::null_mut());
    unsafe {
        __cpp2rust_init_globals();
        ::std::process::exit(main_0((argv.len() - 1) as i32, argv.as_mut_ptr()) as i32)
    }
}
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    (unsafe { check_limits_8() });
    (unsafe { check_limits_9() });
    (unsafe { check_limits_10() });
    (unsafe { check_limits_11() });
    (unsafe { check_limits_12() });
    (unsafe { check_limits_13() });
    (unsafe { check_limits_14() });
    (unsafe { check_limits_15() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
