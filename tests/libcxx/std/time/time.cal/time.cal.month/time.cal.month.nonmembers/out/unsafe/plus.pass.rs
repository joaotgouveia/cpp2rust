extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type anon_0 = u32;
pub const anon_0_value: anon_0 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_chrono_month__std_chrono_month_ {}
pub unsafe fn test_1() -> bool {
    let mut my: () = unimplemented!();
    let mut i: u32 = 0_u32;
    'loop_: while ((i) <= (15_u32)) {
        let mut m1: () = unimplemented!();
        let mut m2: () = unimplemented!();
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(unimplemented!());
        let mut exp: u32 = (i).wrapping_add(2_u32);
        'loop_: while ((exp) > (12_u32)) {
            exp = (exp).wrapping_sub(12_u32);
        }
        assert!(((unimplemented!()) == (exp)));
        assert!(((unimplemented!()) == (exp)));
        i.prefix_inc();
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
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    (unsafe { test_1() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
