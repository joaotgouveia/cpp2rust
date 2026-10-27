extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_imp_0(mut _a0: ()) {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_imp_1(mut _a0: ()) {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_imp_2(mut _a0: ()) {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_imp_3(mut _a0: ()) {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_imp_4(mut _a0: ()) {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_imp_5(mut _a0: ()) {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_imp_6(mut _a0: ()) {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_imp_7(mut _a0: ()) {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_imp_8(mut _a0: ()) {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_imp_9(mut _a0: ()) {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_imp_10(mut _a0: ()) {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_imp_11(mut _a0: ()) {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_imp_12(mut _a0: ()) {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_imp_13(mut _a0: ()) {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_imp_14(mut _a0: ()) {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_imp_15(mut _a0: ()) {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_imp_16(mut _a0: ()) {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_imp_17(mut _a0: ()) {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_imp_18(mut _a0: ()) {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_imp_19(mut _a0: ()) {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_imp_20(mut _a0: ()) {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_21() {
    (unsafe { test_imp_3(unimplemented!()) });
}
pub unsafe fn test_22() {
    (unsafe { test_imp_4(unimplemented!()) });
}
pub unsafe fn test_23() {
    (unsafe { test_imp_5(unimplemented!()) });
}
pub unsafe fn test_24() {
    (unsafe { test_imp_6(unimplemented!()) });
}
pub unsafe fn test_25() {
    (unsafe { test_imp_7(unimplemented!()) });
}
pub unsafe fn test_26() {
    (unsafe { test_imp_8(unimplemented!()) });
}
pub unsafe fn test_27() {
    (unsafe { test_imp_9(unimplemented!()) });
}
pub unsafe fn test_28() {
    (unsafe { test_imp_10(unimplemented!()) });
}
pub unsafe fn test_29() {
    (unsafe { test_imp_11(unimplemented!()) });
}
pub unsafe fn test_30() {
    (unsafe { test_imp_12(unimplemented!()) });
}
pub unsafe fn test_31() {
    (unsafe { test_imp_13(unimplemented!()) });
}
pub unsafe fn test_32() {
    (unsafe { test_imp_14(unimplemented!()) });
}
pub unsafe fn test_33() {
    (unsafe { test_imp_15(unimplemented!()) });
}
pub unsafe fn test_34() {
    (unsafe { test_imp_16(unimplemented!()) });
}
pub unsafe fn test_35() {
    (unsafe { test_imp_17(unimplemented!()) });
}
pub unsafe fn test_36() {
    (unsafe { test_imp_18(unimplemented!()) });
}
pub unsafe fn test_37() {
    (unsafe { test_imp_19(unimplemented!()) });
}
pub unsafe fn test_38() {
    (unsafe { test_imp_20(unimplemented!()) });
}
pub unsafe fn test_39() {
    (unsafe { test_imp_0(unimplemented!()) });
}
pub unsafe fn test_40() {
    (unsafe { test_imp_1(unimplemented!()) });
}
pub unsafe fn test_41() {
    (unsafe { test_imp_2(unimplemented!()) });
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
    (unsafe { test_21() });
    (unsafe { test_22() });
    (unsafe { test_23() });
    (unsafe { test_24() });
    (unsafe { test_25() });
    (unsafe { test_26() });
    (unsafe { test_27() });
    (unsafe { test_28() });
    (unsafe { test_29() });
    (unsafe { test_30() });
    (unsafe { test_31() });
    (unsafe { test_32() });
    (unsafe { test_33() });
    (unsafe { test_34() });
    (unsafe { test_35() });
    (unsafe { test_36() });
    (unsafe { test_37() });
    (unsafe { test_38() });
    (unsafe { test_39() });
    (unsafe { test_40() });
    (unsafe { test_41() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
