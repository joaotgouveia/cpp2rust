extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn assert_contiguous_0(c: *const ()) {
    let mut i: () = 0_();
    'loop_: while ((i) < (unimplemented!())) {
        unimplemented!();
        i.prefix_inc();
    }
}
pub unsafe fn assert_contiguous_1(c: *const ()) {
    let mut i: () = 0_();
    'loop_: while ((i) < (unimplemented!())) {
        unimplemented!();
        i.prefix_inc();
    }
}
pub unsafe fn assert_contiguous_2(c: *const ()) {
    let mut i: () = 0_();
    'loop_: while ((i) < (unimplemented!())) {
        unimplemented!();
        i.prefix_inc();
    }
}
pub unsafe fn assert_contiguous_3(c: *const ()) {
    let mut i: () = 0_();
    'loop_: while ((i) < (unimplemented!())) {
        unimplemented!();
        i.prefix_inc();
    }
}
pub unsafe fn assert_contiguous_4(c: *const ()) {
    let mut i: () = 0_();
    'loop_: while ((i) < (unimplemented!())) {
        unimplemented!();
        i.prefix_inc();
    }
}
pub unsafe fn assert_contiguous_5(c: *const ()) {
    let mut i: () = 0_();
    'loop_: while ((i) < (unimplemented!())) {
        unimplemented!();
        i.prefix_inc();
    }
}
pub unsafe fn assert_contiguous_6(c: *const ()) {
    let mut i: () = 0_();
    'loop_: while ((i) < (unimplemented!())) {
        unimplemented!();
        i.prefix_inc();
    }
}
pub unsafe fn assert_contiguous_7(c: *const ()) {
    let mut i: () = 0_();
    'loop_: while ((i) < (unimplemented!())) {
        unimplemented!();
        i.prefix_inc();
    }
}
pub unsafe fn tests_8() -> () {
    (unsafe {
        let mut _c: () = unimplemented!();
        assert_contiguous_0(&mut _c)
    });
    (unsafe {
        let mut _c: () = unimplemented!();
        assert_contiguous_1(&mut _c)
    });
    (unsafe {
        let mut _c: () = unimplemented!();
        assert_contiguous_2(&mut _c)
    });
    (unsafe {
        let mut _c: () = unimplemented!();
        assert_contiguous_3(&mut _c)
    });
    (unsafe {
        let mut _c: () = unimplemented!();
        assert_contiguous_4(&mut _c)
    });
    (unsafe {
        let mut _c: () = unimplemented!();
        assert_contiguous_5(&mut _c)
    });
    (unsafe {
        let mut _c: () = unimplemented!();
        assert_contiguous_6(&mut _c)
    });
    (unsafe {
        let mut _c: () = unimplemented!();
        assert_contiguous_7(&mut _c)
    });
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
    (unsafe { tests_8() });
    const _: () = assert!(true, "tests()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
