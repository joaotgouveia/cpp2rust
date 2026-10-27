extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test1_0() {
    {
        let sv1: () = unimplemented!();
        const _: () = assert!(true, "sv1.size() == 0");;
        const _: () = assert!(true, "sv1.empty()");;
        const _: () = assert!(true, "sv1.size() == sv1.length()");;
        const _: () = assert!(true, "sv1.max_size() > sv1.size()");;
    }
    {
        let mut sv1: () = unimplemented!();
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut sv: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test1_1() {
    {
        let sv1: () = unimplemented!();
        const _: () = assert!(true, "sv1.size() == 0");;
        const _: () = assert!(true, "sv1.empty()");;
        const _: () = assert!(true, "sv1.size() == sv1.length()");;
        const _: () = assert!(true, "sv1.max_size() > sv1.size()");;
    }
    {
        let mut sv1: () = unimplemented!();
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut sv: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test1_2() {
    {
        let sv1: () = unimplemented!();
        const _: () = assert!(true, "sv1.size() == 0");;
        const _: () = assert!(true, "sv1.empty()");;
        const _: () = assert!(true, "sv1.size() == sv1.length()");;
        const _: () = assert!(true, "sv1.max_size() > sv1.size()");;
    }
    {
        let mut sv1: () = unimplemented!();
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut sv: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test1_3() {
    {
        let sv1: () = unimplemented!();
        const _: () = assert!(true, "sv1.size() == 0");;
        const _: () = assert!(true, "sv1.empty()");;
        const _: () = assert!(true, "sv1.size() == sv1.length()");;
        const _: () = assert!(true, "sv1.max_size() > sv1.size()");;
    }
    {
        let mut sv1: () = unimplemented!();
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut sv: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test1_4() {
    {
        let sv1: () = unimplemented!();
        const _: () = assert!(true, "sv1.size() == 0");;
        const _: () = assert!(true, "sv1.empty()");;
        const _: () = assert!(true, "sv1.size() == sv1.length()");;
        const _: () = assert!(true, "sv1.max_size() > sv1.size()");;
    }
    {
        let mut sv1: () = unimplemented!();
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut sv: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test2_5(mut s: *const (), mut len: ()) {
    {
        let mut sv1: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test2_6(mut s: *const (), mut len: ()) {
    {
        let mut sv1: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test2_7(mut s: *const (), mut len: ()) {
    {
        let mut sv1: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test2_8(mut s: *const (), mut len: ()) {
    {
        let mut sv1: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test2_9(mut s: *const (), mut len: ()) {
    {
        let mut sv1: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
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
    (unsafe { test1_0() });
    (unsafe { test1_1() });
    (unsafe { test1_2() });
    (unsafe { test1_3() });
    (unsafe { test1_4() });
    (unsafe {
        test2_5 ( c"ABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDE" .as_ptr() , 105_() , )
    });
    (unsafe { test2_5(c"ABCDE".as_ptr(), 5_()) });
    (unsafe { test2_5(c"a".as_ptr(), 1_()) });
    (unsafe { test2_5(c"".as_ptr(), 0_()) });
    (unsafe {
        test2_6(
            [
                65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 65 as (), 66 as (), 67 as (),
                68 as (), 69 as (), 65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 65 as (),
                66 as (), 67 as (), 68 as (), 69 as (), 65 as (), 66 as (), 67 as (), 68 as (),
                69 as (), 65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 65 as (), 66 as (),
                67 as (), 68 as (), 69 as (), 65 as (), 66 as (), 67 as (), 68 as (), 69 as (),
                65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 65 as (), 66 as (), 67 as (),
                68 as (), 69 as (), 65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 65 as (),
                66 as (), 67 as (), 68 as (), 69 as (), 65 as (), 66 as (), 67 as (), 68 as (),
                69 as (), 65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 65 as (), 66 as (),
                67 as (), 68 as (), 69 as (), 65 as (), 66 as (), 67 as (), 68 as (), 69 as (),
                65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 65 as (), 66 as (), 67 as (),
                68 as (), 69 as (), 65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 65 as (),
                66 as (), 67 as (), 68 as (), 69 as (), 65 as (), 66 as (), 67 as (), 68 as (),
                69 as (), 0 as (),
            ]
            .as_ptr(),
            105_(),
        )
    });
    (unsafe {
        test2_6(
            [65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 0 as ()].as_ptr(),
            5_(),
        )
    });
    (unsafe { test2_6([97 as (), 0 as ()].as_ptr(), 1_()) });
    (unsafe { test2_6([0 as ()].as_ptr(), 0_()) });
    (unsafe {
        test2_7(
            [
                65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 65 as (), 66 as (), 67 as (),
                68 as (), 69 as (), 65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 65 as (),
                66 as (), 67 as (), 68 as (), 69 as (), 65 as (), 66 as (), 67 as (), 68 as (),
                69 as (), 65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 65 as (), 66 as (),
                67 as (), 68 as (), 69 as (), 65 as (), 66 as (), 67 as (), 68 as (), 69 as (),
                65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 65 as (), 66 as (), 67 as (),
                68 as (), 69 as (), 65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 65 as (),
                66 as (), 67 as (), 68 as (), 69 as (), 65 as (), 66 as (), 67 as (), 68 as (),
                69 as (), 65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 65 as (), 66 as (),
                67 as (), 68 as (), 69 as (), 65 as (), 66 as (), 67 as (), 68 as (), 69 as (),
                65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 65 as (), 66 as (), 67 as (),
                68 as (), 69 as (), 65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 65 as (),
                66 as (), 67 as (), 68 as (), 69 as (), 65 as (), 66 as (), 67 as (), 68 as (),
                69 as (), 0 as (),
            ]
            .as_ptr(),
            105_(),
        )
    });
    (unsafe {
        test2_7(
            [65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 0 as ()].as_ptr(),
            5_(),
        )
    });
    (unsafe { test2_7([97 as (), 0 as ()].as_ptr(), 1_()) });
    (unsafe { test2_7([0 as ()].as_ptr(), 0_()) });
    (unsafe {
        test2_8(
            [
                65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 65 as (), 66 as (), 67 as (),
                68 as (), 69 as (), 65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 65 as (),
                66 as (), 67 as (), 68 as (), 69 as (), 65 as (), 66 as (), 67 as (), 68 as (),
                69 as (), 65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 65 as (), 66 as (),
                67 as (), 68 as (), 69 as (), 65 as (), 66 as (), 67 as (), 68 as (), 69 as (),
                65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 65 as (), 66 as (), 67 as (),
                68 as (), 69 as (), 65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 65 as (),
                66 as (), 67 as (), 68 as (), 69 as (), 65 as (), 66 as (), 67 as (), 68 as (),
                69 as (), 65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 65 as (), 66 as (),
                67 as (), 68 as (), 69 as (), 65 as (), 66 as (), 67 as (), 68 as (), 69 as (),
                65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 65 as (), 66 as (), 67 as (),
                68 as (), 69 as (), 65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 65 as (),
                66 as (), 67 as (), 68 as (), 69 as (), 65 as (), 66 as (), 67 as (), 68 as (),
                69 as (), 0 as (),
            ]
            .as_ptr(),
            105_(),
        )
    });
    (unsafe {
        test2_8(
            [65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 0 as ()].as_ptr(),
            5_(),
        )
    });
    (unsafe { test2_8([97 as (), 0 as ()].as_ptr(), 1_()) });
    (unsafe { test2_8([0 as ()].as_ptr(), 0_()) });
    (unsafe {
        test2_9(
            [
                65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 65 as (), 66 as (), 67 as (),
                68 as (), 69 as (), 65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 65 as (),
                66 as (), 67 as (), 68 as (), 69 as (), 65 as (), 66 as (), 67 as (), 68 as (),
                69 as (), 65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 65 as (), 66 as (),
                67 as (), 68 as (), 69 as (), 65 as (), 66 as (), 67 as (), 68 as (), 69 as (),
                65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 65 as (), 66 as (), 67 as (),
                68 as (), 69 as (), 65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 65 as (),
                66 as (), 67 as (), 68 as (), 69 as (), 65 as (), 66 as (), 67 as (), 68 as (),
                69 as (), 65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 65 as (), 66 as (),
                67 as (), 68 as (), 69 as (), 65 as (), 66 as (), 67 as (), 68 as (), 69 as (),
                65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 65 as (), 66 as (), 67 as (),
                68 as (), 69 as (), 65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 65 as (),
                66 as (), 67 as (), 68 as (), 69 as (), 65 as (), 66 as (), 67 as (), 68 as (),
                69 as (), 0 as (),
            ]
            .as_ptr(),
            105_(),
        )
    });
    (unsafe {
        test2_9(
            [65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 0 as ()].as_ptr(),
            5_(),
        )
    });
    (unsafe { test2_9([97 as (), 0 as ()].as_ptr(), 1_()) });
    (unsafe { test2_9([0 as ()].as_ptr(), 0_()) });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
