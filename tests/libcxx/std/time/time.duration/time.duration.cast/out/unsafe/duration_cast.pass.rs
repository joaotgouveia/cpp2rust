extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_0(f: *const (), d: *const ()) {
    {
        const _: () = assert!(true, "(std::is_same<R, ToDuration>::value)");;
        unimplemented!();
    }
}
pub unsafe fn test_1(f: *const (), d: *const ()) {
    {
        const _: () = assert!(true, "(std::is_same<R, ToDuration>::value)");;
        unimplemented!();
    }
}
pub unsafe fn test_2(f: *const (), d: *const ()) {
    {
        const _: () = assert!(true, "(std::is_same<R, ToDuration>::value)");;
        unimplemented!();
    }
}
pub unsafe fn test_3(f: *const (), d: *const ()) {
    {
        const _: () = assert!(true, "(std::is_same<R, ToDuration>::value)");;
        unimplemented!();
    }
}
pub unsafe fn test_4(f: *const (), d: *const ()) {
    {
        const _: () = assert!(true, "(std::is_same<R, ToDuration>::value)");;
        unimplemented!();
    }
}
pub unsafe fn test_5(f: *const (), d: *const ()) {
    {
        const _: () = assert!(true, "(std::is_same<R, ToDuration>::value)");;
        unimplemented!();
    }
}
pub unsafe fn test_6(f: *const (), d: *const ()) {
    {
        const _: () = assert!(true, "(std::is_same<R, ToDuration>::value)");;
        unimplemented!();
    }
}
pub unsafe fn test_7(f: *const (), d: *const ()) {
    {
        const _: () = assert!(true, "(std::is_same<R, ToDuration>::value)");;
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
        test_0(&mut _f, &mut _d)
    });
    (unsafe {
        let mut _f: () = unimplemented!();
        let mut _d: () = unimplemented!();
        test_1(&mut _f, &mut _d)
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
        test_4(&mut _f, &mut _d)
    });
    (unsafe {
        let mut _f: () = unimplemented!();
        let mut _d: () = unimplemented!();
        test_5(&mut _f, &mut _d)
    });
    (unsafe {
        let mut _f: () = unimplemented!();
        let mut _d: () = unimplemented!();
        test_6(&mut _f, &mut _d)
    });
    (unsafe {
        let mut _f: () = unimplemented!();
        let mut _d: () = unimplemented!();
        test_7(&mut _f, &mut _d)
    });
    {
        let h: () = unimplemented!();
        const _: () = assert!(true, "h.count() == 2");;
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
