extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn tests_0() -> bool {
    {
        let array: Vec<f64> = Vec::from([3.3_f64]);
        assert!(((*unimplemented!()) == (3.3_f64)));
    }
    {
        let array: Vec<f64> = Vec::from([3.3_f64, 4.4_f64]);
        assert!(((*unimplemented!()) == (3.3_f64)));
        assert!(((*unimplemented!()) == (4.4_f64)));
    }
    {
        let array: Vec<f64> = Vec::from([3.3_f64, 4.4_f64, 5.5_f64]);
        assert!(((*unimplemented!()) == (3.3_f64)));
        assert!(((*unimplemented!()) == (4.4_f64)));
        assert!(((*unimplemented!()) == (5.5_f64)));
    }
    {
        let array: Vec<f64> = Vec::from([3.3_f64]);
        const _: () = assert!(
            true,
            "std::is_same<double const&, decltype(std::get<0>(array))>::value"
        );;
    }
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
