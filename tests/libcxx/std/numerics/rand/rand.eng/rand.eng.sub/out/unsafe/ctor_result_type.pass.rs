extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn make_implicit_0() -> () {
    return unimplemented!();
}
pub unsafe fn make_implicit_1() -> () {
    return unimplemented!();
}
pub unsafe fn test_convertible_imp_2(mut _a0: ()) -> () {
    return true;
}
pub unsafe fn test_convertible_imp_3(mut _a0: ()) -> () {
    return true;
}
pub unsafe fn test_convertible_imp_4(mut _a0: ()) -> () {
    return false;
}
pub unsafe fn test_convertible_imp_5(mut _a0: ()) -> () {
    return false;
}
pub unsafe fn test_convertible_6() -> () {
    return (unsafe { test_convertible_imp_2(0_()) });
}
pub unsafe fn test_convertible_7() -> () {
    return (unsafe { test_convertible_imp_3(0_()) });
}
pub unsafe fn test_convertible_8() -> () {
    return (unsafe { test_convertible_imp_4(0_()) });
}
pub unsafe fn test_convertible_9() -> () {
    return (unsafe { test_convertible_imp_5(0_()) });
}
pub unsafe fn to_string_10(e: *const ()) -> () {
    let mut os: () = unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn to_string_11(e: *const ()) -> () {
    let mut os: () = unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn test1_12() {
    let mut a : *const () = c"15136306 8587749 2346244 16479026 15515802 9510553 16090340 14501685 13839944 10789678 11581259 9590790 5840316 5953700 13398366 8134459 16629731 6851902 15583892 1317475 4231148 9092691 5707268 2355175 0" .as_ptr() ;
    let mut e1: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test2_13() {
    let mut a : *const () = c"10880375256626 126660097854724 33643165434010 78293780235492 179418984296008 96783156950859 238199764491708 34339434557790 155299155394531 29014415493780 209265474179052 263777435457028 0" .as_ptr() ;
    let mut e1: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_implicit_ctor_14() {
    unimplemented!();
}
pub unsafe fn test_implicit_ctor_15() {
    unimplemented!();
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
    (unsafe { test1_12() });
    (unsafe { test2_13() });
    const _: () = assert!(true, "test_convertible<std::ranlux24_base>()");;
    const _: () = assert!(true, "test_convertible<std::ranlux48_base>()");;
    (unsafe { test_implicit_ctor_14() });
    (unsafe { test_implicit_ctor_15() });
    const _: () = assert!(
        true,
        "!test_convertible<std::ranlux24_base, std::uint_fast32_t>()"
    );;
    const _: () = assert!(
        true,
        "!test_convertible<std::ranlux48_base, std::uint_fast64_t>()"
    );;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
