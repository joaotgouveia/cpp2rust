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
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "std::is_trivially_copyable<T1>::value");;
        const _: () = assert!(true, "std::is_trivially_default_constructible<T1>::value");;
        const _: () = assert!(true, "std::is_standard_layout<T1>::value");;
        const _: () = assert!(true, "std::alignment_of<T1>::value == 1");;
        const _: () = assert!(
            ((::std::mem::size_of::<()>()) == (10_())),
            "sizeof(T1) == 10"
        );;
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "std::is_trivially_copyable<T1>::value");;
        const _: () = assert!(true, "std::is_trivially_default_constructible<T1>::value");;
        const _: () = assert!(true, "std::is_standard_layout<T1>::value");;
        const _: () = assert!(true, "std::alignment_of<T1>::value == 2");;
        const _: () = assert!(
            ((::std::mem::size_of::<()>()) == (10_())),
            "sizeof(T1) == 10"
        );;
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "std::is_trivially_copyable<T1>::value");;
        const _: () = assert!(true, "std::is_trivially_default_constructible<T1>::value");;
        const _: () = assert!(true, "std::is_standard_layout<T1>::value");;
        const _: () = assert!(true, "std::alignment_of<T1>::value == 4");;
        const _: () = assert!(
            ((::std::mem::size_of::<()>()) == (12_())),
            "sizeof(T1) == 12"
        );;
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "std::is_trivially_copyable<T1>::value");;
        const _: () = assert!(true, "std::is_trivially_default_constructible<T1>::value");;
        const _: () = assert!(true, "std::is_standard_layout<T1>::value");;
        const _: () = assert!(true, "std::alignment_of<T1>::value == 8");;
        const _: () = assert!(
            ((::std::mem::size_of::<()>()) == (16_())),
            "sizeof(T1) == 16"
        );;
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "std::is_trivially_copyable<T1>::value");;
        const _: () = assert!(true, "std::is_trivially_default_constructible<T1>::value");;
        const _: () = assert!(true, "std::is_standard_layout<T1>::value");;
        const _: () = assert!(true, "std::alignment_of<T1>::value == 2");;
        const _: () = assert!(
            ((::std::mem::size_of::<()>()) == (10_())),
            "sizeof(T1) == 10"
        );;
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "std::is_trivially_copyable<T1>::value");;
        const _: () = assert!(true, "std::is_trivially_default_constructible<T1>::value");;
        const _: () = assert!(true, "std::is_standard_layout<T1>::value");;
        const _: () = assert!(true, "std::alignment_of<T1>::value == 2");;
        const _: () = assert!(
            ((::std::mem::size_of::<()>()) == (10_())),
            "sizeof(T1) == 10"
        );;
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "std::is_trivially_copyable<T1>::value");;
        const _: () = assert!(true, "std::is_trivially_default_constructible<T1>::value");;
        const _: () = assert!(true, "std::is_standard_layout<T1>::value");;
        const _: () = assert!(true, "std::alignment_of<T1>::value == 4");;
        const _: () = assert!(((::std::mem::size_of::<()>()) == (4_())), "sizeof(T1) == 4");;
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "std::is_trivially_copyable<T1>::value");;
        const _: () = assert!(true, "std::is_trivially_default_constructible<T1>::value");;
        const _: () = assert!(true, "std::is_standard_layout<T1>::value");;
        const _: () = assert!(true, "std::alignment_of<T1>::value == 4");;
        const _: () = assert!(((::std::mem::size_of::<()>()) == (4_())), "sizeof(T1) == 4");;
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "std::is_trivially_copyable<T1>::value");;
        const _: () = assert!(true, "std::is_trivially_default_constructible<T1>::value");;
        const _: () = assert!(true, "std::is_standard_layout<T1>::value");;
        const _: () = assert!(true, "std::alignment_of<T1>::value == 4");;
        const _: () = assert!(((::std::mem::size_of::<()>()) == (4_())), "sizeof(T1) == 4");;
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
