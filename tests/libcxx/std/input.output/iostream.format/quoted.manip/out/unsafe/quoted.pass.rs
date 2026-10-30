extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn is_skipws_0(is: *const ()) -> () {
    return ((unimplemented!()) != (0_()));
}
pub unsafe fn is_skipws_1(is: *const ()) -> () {
    return ((unimplemented!()) != (0_()));
}
pub unsafe fn both_ways_2(mut p: *const ()) {
    let mut str: () = unimplemented!();
    let mut q: () = unimplemented!();
    let mut ss: () = unimplemented!();
    let mut skippingws: () = (unsafe { is_skipws_0(&ss) });
    (&(skippingws));
    unimplemented!();
    unimplemented!();
}
pub unsafe fn round_trip_3(mut p: *const ()) {
    let mut ss: () = unimplemented!();
    let mut skippingws: () = (unsafe { is_skipws_0(&ss) });
    unimplemented!();
    let mut s: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn round_trip_4(mut p: *const ()) {
    let mut ss: () = unimplemented!();
    let mut skippingws: () = (unsafe { is_skipws_1(&ss) });
    unimplemented!();
    let mut s: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn round_trip_ws_5(mut p: *const ()) {
    let mut ss: () = unimplemented!();
    unimplemented!();
    let mut skippingws: () = (unsafe { is_skipws_0(&ss) });
    unimplemented!();
    let mut s: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn round_trip_ws_6(mut p: *const ()) {
    let mut ss: () = unimplemented!();
    unimplemented!();
    let mut skippingws: () = (unsafe { is_skipws_1(&ss) });
    unimplemented!();
    let mut s: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn round_trip_d_7(mut p: *const (), mut delim: ()) {
    let mut ss: () = unimplemented!();
    let mut d: () = delim;
    unimplemented!();
    let mut s: () = unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn round_trip_d_8(mut p: *const (), mut delim: ()) {
    let mut ss: () = unimplemented!();
    let mut d: () = delim;
    unimplemented!();
    let mut s: () = unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn round_trip_e_9(mut p: *const (), mut escape: ()) {
    let mut ss: () = unimplemented!();
    let mut e: () = escape;
    unimplemented!();
    let mut s: () = unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn round_trip_e_10(mut p: *const (), mut escape: ()) {
    let mut ss: () = unimplemented!();
    let mut e: () = escape;
    unimplemented!();
    let mut s: () = unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn quote_11(mut p: *const (), mut delim: Option<()>, mut escape: Option<()>) -> () {
    let mut delim: () = delim.unwrap_or_else(|| unsafe { ('\"' as ()) });
    let mut escape: () = escape.unwrap_or_else(|| unsafe { ('\\' as ()) });
    let mut ss: () = unimplemented!();
    let mut d: () = delim;
    let mut e: () = escape;
    unimplemented!();
    let mut s: () = unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn quote_12(mut p: *const (), mut delim: Option<()>, mut escape: Option<()>) -> () {
    let mut delim: () = delim.unwrap_or_else(|| unsafe { ('\"' as ()) });
    let mut escape: () = escape.unwrap_or_else(|| unsafe { ('\\' as ()) });
    let mut ss: () = unimplemented!();
    let mut d: () = delim;
    let mut e: () = escape;
    unimplemented!();
    let mut s: () = unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn unquote_13(mut p: *const (), mut delim: Option<()>, mut escape: Option<()>) -> () {
    let mut delim: () = delim.unwrap_or_else(|| unsafe { ('\"' as ()) });
    let mut escape: () = escape.unwrap_or_else(|| unsafe { ('\\' as ()) });
    let mut ss: () = unimplemented!();
    unimplemented!();
    let mut d: () = delim;
    let mut e: () = escape;
    let mut s: () = unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn unquote_14(mut p: *const (), mut delim: Option<()>, mut escape: Option<()>) -> () {
    let mut delim: () = delim.unwrap_or_else(|| unsafe { ('\"' as ()) });
    let mut escape: () = escape.unwrap_or_else(|| unsafe { ('\\' as ()) });
    let mut ss: () = unimplemented!();
    unimplemented!();
    let mut d: () = delim;
    let mut e: () = escape;
    let mut s: () = unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn test_padding_15() {
    {
        let mut ss: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut ss: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
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
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    (unsafe { both_ways_2(c"".as_ptr()) });
    (unsafe { round_trip_3(c"".as_ptr()) });
    (unsafe { round_trip_ws_5(c"".as_ptr()) });
    (unsafe { round_trip_d_7(c"".as_ptr(), ('q' as ())) });
    (unsafe { round_trip_e_9(c"".as_ptr(), ('q' as ())) });
    (unsafe { round_trip_4([0 as ()].as_ptr()) });
    (unsafe { round_trip_ws_6([0 as ()].as_ptr()) });
    (unsafe { round_trip_d_8([0 as ()].as_ptr(), ('q' as ())) });
    (unsafe { round_trip_e_10([0 as ()].as_ptr(), ('q' as ())) });
    (unsafe { round_trip_3(c"Hi".as_ptr()) });
    (unsafe { round_trip_ws_5(c"Hi".as_ptr()) });
    (unsafe { round_trip_d_7(c"Hi".as_ptr(), ('!' as ())) });
    (unsafe { round_trip_e_9(c"Hi".as_ptr(), ('!' as ())) });
    unimplemented!();
    unimplemented!();
    (unsafe { round_trip_4([72 as (), 105 as (), 0 as ()].as_ptr()) });
    (unsafe { round_trip_ws_6([72 as (), 105 as (), 0 as ()].as_ptr()) });
    (unsafe { round_trip_d_8([72 as (), 105 as (), 0 as ()].as_ptr(), ('!' as ())) });
    (unsafe { round_trip_e_10([72 as (), 105 as (), 0 as ()].as_ptr(), ('!' as ())) });
    unimplemented!();
    unimplemented!();
    (unsafe { round_trip_3(c"Hi Mom".as_ptr()) });
    (unsafe { round_trip_ws_5(c"Hi Mom".as_ptr()) });
    (unsafe {
        round_trip_4(
            [
                72 as (), 105 as (), 32 as (), 77 as (), 111 as (), 109 as (), 0 as (),
            ]
            .as_ptr(),
        )
    });
    (unsafe {
        round_trip_ws_6(
            [
                72 as (), 105 as (), 32 as (), 77 as (), 111 as (), 109 as (), 0 as (),
            ]
            .as_ptr(),
        )
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (unsafe { test_padding_15() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
