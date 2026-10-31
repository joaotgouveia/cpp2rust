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
pub struct test_macros_detail_is_same_std_chrono_weekday__std_chrono_weekday_ {}
pub unsafe fn euclidian_addition_1(mut rhs: u32, mut lhs: u32) -> u32 {
    let modulus: u32 = ((6_u32 as u32).wrapping_sub((0_u32 as u32)) as u32).wrapping_add(1_u32);
    let mut ret: u32 = (rhs).wrapping_add(lhs);
    if ((ret) > (6_u32)) {
        ret = (ret).wrapping_sub(modulus);
    }
    return ret;
}
pub unsafe fn test_2() -> bool {
    let mut i: u32 = 0_u32;
    'loop_: while ((i) <= (6_u32)) {
        let mut j: u32 = 0_u32;
        'loop_: while ((j) <= (6_u32)) {
            let mut wd1: () = unimplemented!();
            let mut wd2: () = unimplemented!();
            assert!(unimplemented!());
            assert!(((unimplemented!()) == (unsafe { euclidian_addition_1(i, j,) })));
            assert!(((unimplemented!()) == (unsafe { euclidian_addition_1(i, j,) })));
            j.prefix_inc();
        }
        i.prefix_inc();
    }
    assert!(unimplemented!());
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
