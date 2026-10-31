extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_0() {
    {
        let c: () = unimplemented!();
        assert!(((unimplemented!()) == (0_f32)));
        assert!(((unimplemented!()) == (0_f32)));
    }
    {
        let c: () = unimplemented!();
        assert!(((unimplemented!() as f64) == (7.5_f64)));
        assert!(((unimplemented!()) == (0_f32)));
    }
    {
        let c: () = unimplemented!();
        assert!(((unimplemented!() as f64) == (8.5_f64)));
        assert!(((unimplemented!()) == (0_f32)));
    }
    {
        let c: () = unimplemented!();
        assert!(((unimplemented!() as f64) == (10.5_f64)));
        assert!(((unimplemented!() as f64) == (-9.5_f64)));
    }
    {
        let c: () = unimplemented!();
        const _: () = assert!(true, "c.real() == 0");;
        const _: () = assert!(true, "c.imag() == 0");;
    }
    {
        let c: () = unimplemented!();
        const _: () = assert!(true, "c.real() == 7.5");;
        const _: () = assert!(true, "c.imag() == 0");;
    }
    {
        let c: () = unimplemented!();
        const _: () = assert!(true, "c.real() == 8.5");;
        const _: () = assert!(true, "c.imag() == 0");;
    }
    {
        let c: () = unimplemented!();
        const _: () = assert!(true, "c.real() == 10.5");;
        const _: () = assert!(true, "c.imag() == -9.5");;
    }
}
pub unsafe fn test_1() {
    {
        let c: () = unimplemented!();
        assert!(((unimplemented!()) == (0_f64)));
        assert!(((unimplemented!()) == (0_f64)));
    }
    {
        let c: () = unimplemented!();
        assert!(((unimplemented!()) == (7.5_f64)));
        assert!(((unimplemented!()) == (0_f64)));
    }
    {
        let c: () = unimplemented!();
        assert!(((unimplemented!()) == (8.5_f64)));
        assert!(((unimplemented!()) == (0_f64)));
    }
    {
        let c: () = unimplemented!();
        assert!(((unimplemented!()) == (10.5_f64)));
        assert!(((unimplemented!()) == (-9.5_f64)));
    }
    {
        let c: () = unimplemented!();
        const _: () = assert!(true, "c.real() == 0");;
        const _: () = assert!(true, "c.imag() == 0");;
    }
    {
        let c: () = unimplemented!();
        const _: () = assert!(true, "c.real() == 7.5");;
        const _: () = assert!(true, "c.imag() == 0");;
    }
    {
        let c: () = unimplemented!();
        const _: () = assert!(true, "c.real() == 8.5");;
        const _: () = assert!(true, "c.imag() == 0");;
    }
    {
        let c: () = unimplemented!();
        const _: () = assert!(true, "c.real() == 10.5");;
        const _: () = assert!(true, "c.imag() == -9.5");;
    }
}
pub unsafe fn test_2() {
    {
        let c: () = unimplemented!();
        assert!(((unimplemented!()) == (0_f64)));
        assert!(((unimplemented!()) == (0_f64)));
    }
    {
        let c: () = unimplemented!();
        assert!(((unimplemented!()) == (7.5_f64)));
        assert!(((unimplemented!()) == (0_f64)));
    }
    {
        let c: () = unimplemented!();
        assert!(((unimplemented!()) == (8.5_f64)));
        assert!(((unimplemented!()) == (0_f64)));
    }
    {
        let c: () = unimplemented!();
        assert!(((unimplemented!()) == (10.5_f64)));
        assert!(((unimplemented!()) == (-9.5_f64)));
    }
    {
        let c: () = unimplemented!();
        const _: () = assert!(true, "c.real() == 0");;
        const _: () = assert!(true, "c.imag() == 0");;
    }
    {
        let c: () = unimplemented!();
        const _: () = assert!(true, "c.real() == 7.5");;
        const _: () = assert!(true, "c.imag() == 0");;
    }
    {
        let c: () = unimplemented!();
        const _: () = assert!(true, "c.real() == 8.5");;
        const _: () = assert!(true, "c.imag() == 0");;
    }
    {
        let c: () = unimplemented!();
        const _: () = assert!(true, "c.real() == 10.5");;
        const _: () = assert!(true, "c.imag() == -9.5");;
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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    (unsafe { test_0() });
    (unsafe { test_1() });
    (unsafe { test_2() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
