extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut has_abs_v_0: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { true });
pub static mut has_abs_v_1: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { false });
const _: () = assert!(true, "has_abs_v<std::chrono::milliseconds>");
const _: () = assert!(true, "!has_abs_v<std::chrono::duration<unsigned>>");
pub unsafe fn test_2(f: *const (), d: *const ()) {
    {
        const _: () = assert!(true, "(std::is_same<R, Duration>::value)");;
        unimplemented!();
    }
}
pub unsafe fn test_3(f: *const (), d: *const ()) {
    {
        const _: () = assert!(true, "(std::is_same<R, Duration>::value)");;
        unimplemented!();
    }
}
pub unsafe fn test_4(f: *const (), d: *const ()) {
    {
        const _: () = assert!(true, "(std::is_same<R, Duration>::value)");;
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
    (unsafe {
        let mut _f: () = unimplemented!();
        let mut _d: () = unimplemented!();
        test_2(&mut _f, &mut _d)
    });
    (unsafe {
        let mut _f: () = unimplemented!();
        let mut _d: () = unimplemented!();
        test_2(&mut _f, &mut _d)
    });
    (unsafe {
        let mut _f: () = unimplemented!();
        let mut _d: () = unimplemented!();
        test_3(&mut _f, &mut _d)
    });
    (unsafe {
        let mut _f: () = unimplemented!();
        let mut _d: () = unimplemented!();
        test_3(&mut _f, &mut _d)
    });
    (unsafe {
        let mut _f: () = unimplemented!();
        let mut _d: () = unimplemented!();
        test_4(&mut _f, &mut _d)
    });
    {
        let h1: () = unimplemented!();
        const _: () = assert!(true, "h1.count() == 3");;
        let h2: () = unimplemented!();
        const _: () = assert!(true, "h2.count() == 3");;
    }
    {
        let d: () = unimplemented!();
        const _: () = assert!(true, "d.count() == 2");;
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const has_abs_v_0);
    std::cell::LazyCell::force(&*&raw const has_abs_v_1);
}
