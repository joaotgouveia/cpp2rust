extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut a_0: std::cell::LazyCell<[(); 10]> =
    std::cell::LazyCell::new(|| unsafe { [unimplemented!(); 10] });
pub unsafe fn test_1() -> () {
    (unsafe {
        test_2(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut begin_3)),
            &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut a_0)),
        )
    });
    (unsafe {
        test_4(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut end_5)),
            &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut a_0)),
        )
    });
    (unsafe {
        test_6(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut cbegin_7)),
            &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut a_0)),
        )
    });
    (unsafe {
        test_8(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut cdata_9)),
            &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut a_0)),
        )
    });
    (unsafe {
        test_10(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut cend_11)),
            &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut a_0)),
        )
    });
    (unsafe {
        test_12(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut crbegin_13)),
            &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut a_0)),
        )
    });
    (unsafe {
        test_14(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut crend_15)),
            &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut a_0)),
        )
    });
    (unsafe {
        test_16(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut data_17)),
            &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut a_0)),
        )
    });
    (unsafe {
        test_18(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut empty_19)),
            &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut a_0)),
        )
    });
    (unsafe {
        test_20(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut rbegin_21)),
            &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut a_0)),
        )
    });
    (unsafe {
        test_22(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut rend_23)),
            &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut a_0)),
        )
    });
    (unsafe {
        test_24(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut size_25)),
            &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut a_0)),
        )
    });
    (unsafe {
        test_26(
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut ssize_27)),
            &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut a_0)),
        )
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
    (unsafe { test_1() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const a_0);
}
