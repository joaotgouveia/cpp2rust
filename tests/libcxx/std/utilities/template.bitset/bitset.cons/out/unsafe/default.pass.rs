extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_default_ctor_0() {
    {
        let v1: () = unimplemented!();
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) < (unimplemented!())) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let v1: () = unimplemented!();
        const _: () = assert!(true, "v1.size() == N");;
    }
}
pub unsafe fn test_default_ctor_1() {
    {
        let v1: () = unimplemented!();
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) < (unimplemented!())) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let v1: () = unimplemented!();
        const _: () = assert!(true, "v1.size() == N");;
    }
}
pub unsafe fn test_default_ctor_2() {
    {
        let v1: () = unimplemented!();
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) < (unimplemented!())) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let v1: () = unimplemented!();
        const _: () = assert!(true, "v1.size() == N");;
    }
}
pub unsafe fn test_default_ctor_3() {
    {
        let v1: () = unimplemented!();
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) < (unimplemented!())) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let v1: () = unimplemented!();
        const _: () = assert!(true, "v1.size() == N");;
    }
}
pub unsafe fn test_default_ctor_4() {
    {
        let v1: () = unimplemented!();
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) < (unimplemented!())) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let v1: () = unimplemented!();
        const _: () = assert!(true, "v1.size() == N");;
    }
}
pub unsafe fn test_default_ctor_5() {
    {
        let v1: () = unimplemented!();
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) < (unimplemented!())) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let v1: () = unimplemented!();
        const _: () = assert!(true, "v1.size() == N");;
    }
}
pub unsafe fn test_default_ctor_6() {
    {
        let v1: () = unimplemented!();
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) < (unimplemented!())) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let v1: () = unimplemented!();
        const _: () = assert!(true, "v1.size() == N");;
    }
}
pub unsafe fn test_default_ctor_7() {
    {
        let v1: () = unimplemented!();
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) < (unimplemented!())) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let v1: () = unimplemented!();
        const _: () = assert!(true, "v1.size() == N");;
    }
}
pub unsafe fn test_default_ctor_8() {
    {
        let v1: () = unimplemented!();
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) < (unimplemented!())) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let v1: () = unimplemented!();
        const _: () = assert!(true, "v1.size() == N");;
    }
}
pub unsafe fn test_9() -> () {
    (unsafe { test_default_ctor_0() });
    (unsafe { test_default_ctor_1() });
    (unsafe { test_default_ctor_2() });
    (unsafe { test_default_ctor_3() });
    (unsafe { test_default_ctor_4() });
    (unsafe { test_default_ctor_5() });
    (unsafe { test_default_ctor_6() });
    (unsafe { test_default_ctor_7() });
    (unsafe { test_default_ctor_8() });
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
    (unsafe { test_9() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
