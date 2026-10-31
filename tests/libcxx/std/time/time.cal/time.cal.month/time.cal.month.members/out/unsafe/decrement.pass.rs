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
pub type anon_1 = u32;
pub const anon_1_value: anon_1 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_chrono_month_ref__std_chrono_month_ref_ {}
pub unsafe fn test_2() -> bool {
    let mut i: u32 = 0_u32;
    'loop_: while ((i) <= (15_u32)) {
        let mut m1: () = unimplemented!();
        let mut m2: () = unimplemented!();
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        let mut exp: u32 = if ((i) == (0_u32)) {
            11_u32
        } else {
            if ((i) == (1_u32)) {
                12_u32
            } else {
                (i).wrapping_sub(1_u32)
            }
        };
        'loop_: while ((exp) > (12_u32)) {
            exp = (exp).wrapping_sub(12_u32);
        }
        assert!(((unimplemented!()) == (exp)));
        i.prefix_inc();
    }
    let mut i: u32 = 0_u32;
    'loop_: while ((i) <= (15_u32)) {
        let mut m1: () = unimplemented!();
        let mut m2: () = unimplemented!();
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(unimplemented!());
        let mut exp: u32 = if ((i) == (0_u32)) {
            11_u32
        } else {
            if ((i) == (1_u32)) {
                12_u32
            } else {
                (i).wrapping_sub(1_u32)
            }
        };
        'loop_: while ((exp) > (12_u32)) {
            exp = (exp).wrapping_sub(12_u32);
        }
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
    (unsafe { test_2() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
