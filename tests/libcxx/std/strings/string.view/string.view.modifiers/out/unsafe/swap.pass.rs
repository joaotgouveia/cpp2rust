extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_0(mut s: *const (), mut len: ()) {
    {
        let mut sv1: () = unimplemented!();
        let mut sv2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_1(mut s: *const (), mut len: ()) {
    {
        let mut sv1: () = unimplemented!();
        let mut sv2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_2(mut s: *const (), mut len: ()) {
    {
        let mut sv1: () = unimplemented!();
        let mut sv2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_3(mut s: *const (), mut len: ()) {
    {
        let mut sv1: () = unimplemented!();
        let mut sv2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_ce_4(mut n: (), mut k: ()) -> () {
    let mut sv1: () = unimplemented!();
    let mut sv2: () = unimplemented!();
    unimplemented!();
    return unimplemented!();
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
    (unsafe { test_0(c"ABCDE".as_ptr(), 5_()) });
    (unsafe { test_0(c"a".as_ptr(), 1_()) });
    (unsafe { test_0(c"".as_ptr(), 0_()) });
    (unsafe {
        test_1(
            [65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 0 as ()].as_ptr(),
            5_(),
        )
    });
    (unsafe { test_1([97 as (), 0 as ()].as_ptr(), 1_()) });
    (unsafe { test_1([0 as ()].as_ptr(), 0_()) });
    (unsafe {
        test_2(
            [65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 0 as ()].as_ptr(),
            5_(),
        )
    });
    (unsafe { test_2([97 as (), 0 as ()].as_ptr(), 1_()) });
    (unsafe { test_2([0 as ()].as_ptr(), 0_()) });
    (unsafe {
        test_3(
            [65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 0 as ()].as_ptr(),
            5_(),
        )
    });
    (unsafe { test_3([97 as (), 0 as ()].as_ptr(), 1_()) });
    (unsafe { test_3([0 as ()].as_ptr(), 0_()) });
    {
        const _: () = assert!(true, "test_ce(2, 3) == 3");;
        const _: () = assert!(true, "test_ce(5, 3) == 3");;
        const _: () = assert!(true, "test_ce(0, 1) == 1");;
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
