extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;

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
    const _: () = assert!(
        true,
        "std::is_same<decltype( 3h   ), std::chrono::hours>::value"
    );;
    const _: () = assert!(
        true,
        "std::is_same<decltype( 3min ), std::chrono::minutes>::value"
    );;
    const _: () = assert!(
        true,
        "std::is_same<decltype( 3s   ), std::chrono::seconds>::value"
    );;
    const _: () = assert!(
        true,
        "std::is_same<decltype( 3ms  ), std::chrono::milliseconds>::value"
    );;
    const _: () = assert!(
        true,
        "std::is_same<decltype( 3us  ), std::chrono::microseconds>::value"
    );;
    const _: () = assert!(
        true,
        "std::is_same<decltype( 3ns  ), std::chrono::nanoseconds>::value"
    );;
    let mut h: () = (unsafe { unimplemented!()() });
    unimplemented!();
    let mut h2: () = unimplemented!();
    unimplemented!();
    let mut min: () = (unsafe { unimplemented!()() });
    unimplemented!();
    let mut min2: () = unimplemented!();
    unimplemented!();
    let mut s: () = (unsafe { unimplemented!()() });
    unimplemented!();
    let mut s2: () = unimplemented!();
    unimplemented!();
    let mut ms: () = (unsafe { unimplemented!()() });
    unimplemented!();
    let mut ms2: () = unimplemented!();
    unimplemented!();
    let mut us: () = (unsafe { unimplemented!()() });
    unimplemented!();
    let mut us2: () = unimplemented!();
    unimplemented!();
    let mut ns: () = (unsafe { unimplemented!()() });
    unimplemented!();
    let mut ns2: () = unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
