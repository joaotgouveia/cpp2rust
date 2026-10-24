extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_padding_0() {
    {
        let mut ss: () = unimplemented!();
        write!(
            ss,
            "{:}{:unimplemented!()}{:}",
            Some(libcc2rs::left_unsafe),
            unimplemented!(),
            unimplemented!(),
        );
        unimplemented!();
    }
    {
        let mut ss: () = unimplemented!();
        write!(
            ss,
            "{:}{:unimplemented!()}{:}",
            Some(libcc2rs::right_unsafe),
            unimplemented!(),
            unimplemented!(),
        );
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
    (unsafe { both_ways_3(c"".as_ptr()) });
    (unsafe { round_trip_4(c"".as_ptr()) });
    (unsafe { round_trip_ws_5(c"".as_ptr()) });
    (unsafe { round_trip_d_6(c"".as_ptr(), ('q' as ())) });
    (unsafe { round_trip_e_7(c"".as_ptr(), ('q' as ())) });
    (unsafe { round_trip_8([0 as ()].as_ptr()) });
    (unsafe { round_trip_ws_9([0 as ()].as_ptr()) });
    (unsafe { round_trip_d_10([0 as ()].as_ptr(), ('q' as ())) });
    (unsafe { round_trip_e_11([0 as ()].as_ptr(), ('q' as ())) });
    (unsafe { round_trip_4(c"Hi".as_ptr()) });
    (unsafe { round_trip_ws_5(c"Hi".as_ptr()) });
    (unsafe { round_trip_d_6(c"Hi".as_ptr(), ('!' as ())) });
    (unsafe { round_trip_e_7(c"Hi".as_ptr(), ('!' as ())) });
    unimplemented!();
    unimplemented!();
    (unsafe { round_trip_8([72 as (), 105 as (), 0 as ()].as_ptr()) });
    (unsafe { round_trip_ws_9([72 as (), 105 as (), 0 as ()].as_ptr()) });
    (unsafe { round_trip_d_10([72 as (), 105 as (), 0 as ()].as_ptr(), ('!' as ())) });
    (unsafe { round_trip_e_11([72 as (), 105 as (), 0 as ()].as_ptr(), ('!' as ())) });
    unimplemented!();
    unimplemented!();
    (unsafe { round_trip_4(c"Hi Mom".as_ptr()) });
    (unsafe { round_trip_ws_5(c"Hi Mom".as_ptr()) });
    (unsafe {
        round_trip_8(
            [
                72 as (), 105 as (), 32 as (), 77 as (), 111 as (), 109 as (), 0 as (),
            ]
            .as_ptr(),
        )
    });
    (unsafe {
        round_trip_ws_9(
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
    (unsafe { test_padding_0() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
