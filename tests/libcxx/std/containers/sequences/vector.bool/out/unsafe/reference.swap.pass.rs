extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn tests_0() -> bool {
    let mut a: [bool; 4] = [false, true, false, true];
    let mut an: *mut bool = a.as_mut_ptr().offset(
        ((::std::mem::size_of::<[bool; 4]>() as usize)
            .wrapping_div((::std::mem::size_of::<bool>() as usize))) as isize,
    );
    let mut v: Vec<bool> = {
        let mut __v = Vec::new();
        let mut __it = a.as_mut_ptr();
        while __it != an {
            __v.push((*__it.iter_deref()).clone() != false);
            __it.iter_inc();
        }
        __v
    };
    let mut r1: *mut bool = (&mut v).as_mut_ptr().add(0_usize as usize);
    let mut r2: *mut bool = (&mut v).as_mut_ptr().add(3_usize as usize);
    const _: () = assert!(true, "(noexcept(v.swap(r1, r2)))");;
    assert!(!(*r1));
    assert!(*r2);
    unimplemented!();
    assert!(*r1);
    assert!(!(*r2));
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
    (unsafe { tests_0() });
    const _: () = assert!(true, "tests()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
