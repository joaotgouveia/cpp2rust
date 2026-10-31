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
pub struct test_macros_detail_is_same_float__float_ {}
pub type anon_1 = u32;
pub const anon_1_value: anon_1 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_double__double_ {}
pub type anon_2 = u32;
pub const anon_2_value: anon_2 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_long_double__long_double_ {}
pub unsafe fn constexpr_test_3() -> bool {
    return (((unimplemented!()) == (0_f32)) && ((unimplemented!()) == (6_f32)))
        && ((unimplemented!()) == (24_f32));
}
pub unsafe fn constexpr_test_4() -> bool {
    return (((unimplemented!()) == (0_f64)) && ((unimplemented!()) == (6_f64)))
        && ((unimplemented!()) == (24_f64));
}
pub unsafe fn constexpr_test_5() -> bool {
    return (((unimplemented!()) == (0_f64)) && ((unimplemented!()) == (6_f64)))
        && ((unimplemented!()) == (24_f64));
}
pub unsafe fn test_6() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let maxV: f32 = f32::MAX;
    let inf: f32 = unimplemented!();
    assert!(((unimplemented!()) == (0_f32)));
    assert!(((unimplemented!()) == (12_f32)));
    assert!(((unimplemented!()) == (12_f32)));
    assert!(((unimplemented!()) == (0_f32)));
    assert!(((unimplemented!()) == (6_f32)));
    assert!(((unimplemented!()) == (6_f32)));
    assert!(((unimplemented!()) == (24_f32)));
    assert!(((unimplemented!()) == (-12_i32 as f32)));
    assert!(((unimplemented!()) == (maxV)));
    assert!(((unimplemented!()) == (maxV)));
    assert!(((unimplemented!()) == (2.3_f64 as f32)));
    assert!(((unimplemented!()) == (0_f32)));
    assert!(unimplemented!());
}
pub unsafe fn test_7() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let maxV: f64 = f64::MAX;
    let inf: f64 = unimplemented!();
    assert!(((unimplemented!()) == (0_f64)));
    assert!(((unimplemented!()) == (12_f64)));
    assert!(((unimplemented!()) == (12_f64)));
    assert!(((unimplemented!()) == (0_f64)));
    assert!(((unimplemented!()) == (6_f64)));
    assert!(((unimplemented!()) == (6_f64)));
    assert!(((unimplemented!()) == (24_f64)));
    assert!(((unimplemented!()) == (-12_i32 as f64)));
    assert!(((unimplemented!()) == (maxV)));
    assert!(((unimplemented!()) == (maxV)));
    assert!(((unimplemented!()) == (2.3_f64)));
    assert!(((unimplemented!()) == (0_f64)));
    assert!(unimplemented!());
}
pub unsafe fn test_8() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let maxV: f64 = f64::MAX;
    let inf: f64 = unimplemented!();
    assert!(((unimplemented!()) == (0_f64)));
    assert!(((unimplemented!()) == (12_f64)));
    assert!(((unimplemented!()) == (12_f64)));
    assert!(((unimplemented!()) == (0_f64)));
    assert!(((unimplemented!()) == (6_f64)));
    assert!(((unimplemented!()) == (6_f64)));
    assert!(((unimplemented!()) == (24_f64)));
    assert!(((unimplemented!()) == (-12_i32 as f64)));
    assert!(((unimplemented!()) == (maxV)));
    assert!(((unimplemented!()) == (maxV)));
    assert!(((unimplemented!()) == (2.3_f64)));
    assert!(((unimplemented!()) == (0_f64)));
    assert!(unimplemented!());
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
    const _: () = assert!(true, "constexpr_test<float>()");;
    const _: () = assert!(true, "constexpr_test<double>()");;
    const _: () = assert!(true, "constexpr_test<long double>()");;
    (unsafe { test_6() });
    (unsafe { test_7() });
    (unsafe { test_8() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
