extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_tie_0() -> bool {
    {
        let mut i: i32 = 42;
        let mut f: f64 = 1.1_f64;
        let mut res: () = unimplemented!();
        const _: () = assert!(true, "std::is_same<ExpectT, decltype(res)>::value");;
        assert!(((unimplemented!()) == (&mut i as *mut i32)));
        assert!(((unimplemented!()) == (&raw const (*unimplemented!()) as *const ())));
        assert!(((unimplemented!()) == (&mut f as *mut f64)));
        unimplemented!();
        assert!(((i) == (101)));
        assert!(((f) == (-1_f64)));
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
    (unsafe { test_tie_0() });
    const _: () = assert!(true, "test_tie()");;
    {
        let mut i: i32 = 0;
        let mut s: Vec<libc::c_char> = vec![libc::c_char::default()];
        unimplemented!();
        assert!(((i) == (42)));
        assert!(
            s == std::slice::from_raw_parts(
                c"C++".as_ptr(),
                libcc2rs::c_str_len(c"C++".as_ptr()) + 1
            )
            .to_vec()
        );
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
