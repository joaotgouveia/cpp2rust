extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn CreateTuple_0(mut _a0: ()) {
    let mut tuple: () = unimplemented!();
    let mut e1: () = (*(unsafe { get_1(&mut tuple) })).clone();
    unimplemented!();
    let mut e2: () = (*(unsafe { get_2(&mut tuple) })).clone();
    unimplemented!();
    let mut t1: () = unimplemented!();
    let mut t2: () = unimplemented!();
    let mut t3: () = unimplemented!();
    let mut t4: () = unimplemented!();
    let mut t5: () = unimplemented!();
    &(t1);
    &(t2);
    &(t3);
    &(t4);
    &(t5);
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_3() -> () {
    (unsafe { CreateTuple_0(unsafe { std::mem::zeroed::<()>() }) });
    return true;
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
    (unsafe { test_3() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
