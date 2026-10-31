extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_0() -> bool {
    let mut vec_: Vec<bool> = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut ref_: *mut bool = (&mut vec_).as_mut_ptr().add(0_usize as usize);
    let cref: *mut bool = (&mut vec_).as_mut_ptr().add(0_usize as usize);
    assert!(*ref_);
    {
        *ref_ = false;
        ref_
    };
    assert!(!(*(&mut vec_).as_mut_ptr().add(0_usize as usize)));
    assert!(!(*(&mut vec_).as_mut_ptr().add(1_usize as usize)));
    {
        *ref_ = true;
        ref_
    };
    assert!(*(&mut vec_).as_mut_ptr().add(0_usize as usize));
    assert!(!(*(&mut vec_).as_mut_ptr().add(1_usize as usize)));
    assert!(*cref);
    {
        *cref = false;
        cref
    };
    assert!(!(*(&mut vec_).as_mut_ptr().add(0_usize as usize)));
    assert!(!(*(&mut vec_).as_mut_ptr().add(1_usize as usize)));
    {
        *cref = true;
        cref
    };
    assert!(*(&mut vec_).as_mut_ptr().add(0_usize as usize));
    assert!(!(*(&mut vec_).as_mut_ptr().add(1_usize as usize)));
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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    (unsafe { test_0() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
