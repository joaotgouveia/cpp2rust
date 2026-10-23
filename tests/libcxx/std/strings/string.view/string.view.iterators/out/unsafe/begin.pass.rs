extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_0(mut s: ()) {
    let cs: *const () = &s;
    let mut b: () = unimplemented!();
    let mut cb1: () = unimplemented!();
    let mut cb2: () = unimplemented!();
    if !(unimplemented!()) {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_1(mut s: ()) {
    let cs: *const () = &s;
    let mut b: () = unimplemented!();
    let mut cb1: () = unimplemented!();
    let mut cb2: () = unimplemented!();
    if !(unimplemented!()) {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_2(mut s: ()) {
    let cs: *const () = &s;
    let mut b: () = unimplemented!();
    let mut cb1: () = unimplemented!();
    let mut cb2: () = unimplemented!();
    if !(unimplemented!()) {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_3(mut s: ()) {
    let cs: *const () = &s;
    let mut b: () = unimplemented!();
    let mut cb1: () = unimplemented!();
    let mut cb2: () = unimplemented!();
    if !(unimplemented!()) {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_4(mut s: ()) {
    let cs: *const () = &s;
    let mut b: () = unimplemented!();
    let mut cb1: () = unimplemented!();
    let mut cb2: () = unimplemented!();
    if !(unimplemented!()) {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
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
    (unsafe { test_0(unimplemented!()) });
    (unsafe { test_1(unimplemented!()) });
    (unsafe { test_2(unimplemented!()) });
    (unsafe { test_0(unimplemented!()) });
    (unsafe { test_3(unimplemented!()) });
    (unsafe { test_1(unimplemented!()) });
    (unsafe { test_2(unimplemented!()) });
    (unsafe { test_4(unimplemented!()) });
    (unsafe { test_4(unimplemented!()) });
    {
        let sv: () = unimplemented!();
        let u8sv: () = unimplemented!();
        let u16sv: () = unimplemented!();
        let u32sv: () = unimplemented!();
        const _: () = assert!(true, "*sv.begin() == sv[0]");;
        const _: () = assert!(true, "*u8sv.begin() == u8sv[0]");;
        const _: () = assert!(true, "*u16sv.begin() == u16sv[0]");;
        const _: () = assert!(true, "*u32sv.begin() == u32sv[0]");;
        const _: () = assert!(true, "*sv.cbegin() == sv[0]");;
        const _: () = assert!(true, "*u8sv.cbegin() == u8sv[0]");;
        const _: () = assert!(true, "*u16sv.cbegin() == u16sv[0]");;
        const _: () = assert!(true, "*u32sv.cbegin() == u32sv[0]");;
        {
            let wsv: () = unimplemented!();
            const _: () = assert!(true, "*wsv.begin() == wsv[0]");;
            const _: () = assert!(true, "*wsv.cbegin() == wsv[0]");;
        }
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
