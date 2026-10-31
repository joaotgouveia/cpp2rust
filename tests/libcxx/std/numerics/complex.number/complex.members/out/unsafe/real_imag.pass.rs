extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_constexpr_0() {
    let c1: () = unimplemented!();
    const _: () = assert!(true, "c1.real() == 0");;
    const _: () = assert!(true, "c1.imag() == 0");;
    let c2: () = unimplemented!();
    const _: () = assert!(true, "c2.real() == 3");;
    const _: () = assert!(true, "c2.imag() == 0");;
    let c3: () = unimplemented!();
    const _: () = assert!(true, "c3.real() == 3");;
    const _: () = assert!(true, "c3.imag() == 4");;
}
pub unsafe fn test_constexpr_1() {
    let c1: () = unimplemented!();
    const _: () = assert!(true, "c1.real() == 0");;
    const _: () = assert!(true, "c1.imag() == 0");;
    let c2: () = unimplemented!();
    const _: () = assert!(true, "c2.real() == 3");;
    const _: () = assert!(true, "c2.imag() == 0");;
    let c3: () = unimplemented!();
    const _: () = assert!(true, "c3.real() == 3");;
    const _: () = assert!(true, "c3.imag() == 4");;
}
pub unsafe fn test_constexpr_2() {
    let c1: () = unimplemented!();
    const _: () = assert!(true, "c1.real() == 0");;
    const _: () = assert!(true, "c1.imag() == 0");;
    let c2: () = unimplemented!();
    const _: () = assert!(true, "c2.real() == 3");;
    const _: () = assert!(true, "c2.imag() == 0");;
    let c3: () = unimplemented!();
    const _: () = assert!(true, "c3.real() == 3");;
    const _: () = assert!(true, "c3.imag() == 4");;
}
pub unsafe fn test_constexpr_3() {
    let c1: () = unimplemented!();
    const _: () = assert!(true, "c1.real() == 0");;
    const _: () = assert!(true, "c1.imag() == 0");;
    let c2: () = unimplemented!();
    const _: () = assert!(true, "c2.real() == 3");;
    const _: () = assert!(true, "c2.imag() == 0");;
    let c3: () = unimplemented!();
    const _: () = assert!(true, "c3.real() == 3");;
    const _: () = assert!(true, "c3.imag() == 4");;
}
pub unsafe fn test_4() -> bool {
    let mut c: () = unimplemented!();
    assert!(((unimplemented!()) == (0_f32)));
    assert!(((unimplemented!()) == (0_f32)));
    unimplemented!();
    assert!(((unimplemented!() as f64) == (3.5_f64)));
    assert!(((unimplemented!()) == (0_f32)));
    unimplemented!();
    assert!(((unimplemented!() as f64) == (3.5_f64)));
    assert!(((unimplemented!() as f64) == (4.5_f64)));
    unimplemented!();
    assert!(((unimplemented!() as f64) == (-4.5_f64)));
    assert!(((unimplemented!() as f64) == (4.5_f64)));
    unimplemented!();
    assert!(((unimplemented!() as f64) == (-4.5_f64)));
    assert!(((unimplemented!() as f64) == (-5.5_f64)));
    (unsafe { test_constexpr_0() });
    return true;
}
pub unsafe fn test_5() -> bool {
    let mut c: () = unimplemented!();
    assert!(((unimplemented!()) == (0_f64)));
    assert!(((unimplemented!()) == (0_f64)));
    unimplemented!();
    assert!(((unimplemented!()) == (3.5_f64)));
    assert!(((unimplemented!()) == (0_f64)));
    unimplemented!();
    assert!(((unimplemented!()) == (3.5_f64)));
    assert!(((unimplemented!()) == (4.5_f64)));
    unimplemented!();
    assert!(((unimplemented!()) == (-4.5_f64)));
    assert!(((unimplemented!()) == (4.5_f64)));
    unimplemented!();
    assert!(((unimplemented!()) == (-4.5_f64)));
    assert!(((unimplemented!()) == (-5.5_f64)));
    (unsafe { test_constexpr_1() });
    return true;
}
pub unsafe fn test_6() -> bool {
    let mut c: () = unimplemented!();
    assert!(((unimplemented!()) == (0_f64)));
    assert!(((unimplemented!()) == (0_f64)));
    unimplemented!();
    assert!(((unimplemented!()) == (3.5_f64)));
    assert!(((unimplemented!()) == (0_f64)));
    unimplemented!();
    assert!(((unimplemented!()) == (3.5_f64)));
    assert!(((unimplemented!()) == (4.5_f64)));
    unimplemented!();
    assert!(((unimplemented!()) == (-4.5_f64)));
    assert!(((unimplemented!()) == (4.5_f64)));
    unimplemented!();
    assert!(((unimplemented!()) == (-4.5_f64)));
    assert!(((unimplemented!()) == (-5.5_f64)));
    (unsafe { test_constexpr_2() });
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
    (unsafe { test_4() });
    (unsafe { test_5() });
    (unsafe { test_6() });
    (unsafe { test_constexpr_3() });
    const _: () = assert!(true, "test<float>()");;
    const _: () = assert!(true, "test<double>()");;
    const _: () = assert!(true, "test<long double>()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
