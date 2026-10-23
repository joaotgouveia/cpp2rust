extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn testConstexprSpan_0(mut sp: (), mut sz: ()) -> () {
    const _: () = assert!(true, "");;
    return ((unimplemented!()) == ((sz).wrapping_mul((::std::mem::size_of::<()>() as ()))));
}
pub unsafe fn testConstexprSpan_1(mut sp: (), mut sz: ()) -> () {
    const _: () = assert!(true, "");;
    return ((unimplemented!()) == ((sz).wrapping_mul((::std::mem::size_of::<()>() as ()))));
}
pub unsafe fn testConstexprSpan_2(mut sp: (), mut sz: ()) -> () {
    const _: () = assert!(true, "");;
    return ((unimplemented!()) == ((sz).wrapping_mul((::std::mem::size_of::<()>() as ()))));
}
pub unsafe fn testConstexprSpan_3(mut sp: (), mut sz: ()) -> () {
    const _: () = assert!(true, "");;
    return ((unimplemented!()) == ((sz).wrapping_mul((::std::mem::size_of::<A>() as ()))));
}
pub unsafe fn testConstexprSpan_4(mut sp: (), mut sz: ()) -> () {
    const _: () = assert!(true, "");;
    return ((unimplemented!()) == ((sz).wrapping_mul((::std::mem::size_of::<()>() as ()))));
}
pub unsafe fn testConstexprSpan_5(mut sp: (), mut sz: ()) -> () {
    const _: () = assert!(true, "");;
    return ((unimplemented!()) == ((sz).wrapping_mul((::std::mem::size_of::<()>() as ()))));
}
pub unsafe fn testConstexprSpan_6(mut sp: (), mut sz: ()) -> () {
    const _: () = assert!(true, "");;
    return ((unimplemented!()) == ((sz).wrapping_mul((::std::mem::size_of::<()>() as ()))));
}
pub unsafe fn testConstexprSpan_7(mut sp: (), mut sz: ()) -> () {
    const _: () = assert!(true, "");;
    return ((unimplemented!()) == ((sz).wrapping_mul((::std::mem::size_of::<()>() as ()))));
}
pub unsafe fn testConstexprSpan_8(mut sp: (), mut sz: ()) -> () {
    const _: () = assert!(true, "");;
    return ((unimplemented!()) == ((sz).wrapping_mul((::std::mem::size_of::<A>() as ()))));
}
pub unsafe fn testConstexprSpan_9(mut sp: (), mut sz: ()) -> () {
    const _: () = assert!(true, "");;
    return ((unimplemented!()) == ((sz).wrapping_mul((::std::mem::size_of::<()>() as ()))));
}
pub unsafe fn testConstexprSpan_10(mut sp: (), mut sz: ()) -> () {
    const _: () = assert!(true, "");;
    return ((unimplemented!()) == ((sz).wrapping_mul((::std::mem::size_of::<()>() as ()))));
}
pub unsafe fn testRuntimeSpan_11(mut sp: (), mut sz: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
}
pub unsafe fn testRuntimeSpan_12(mut sp: (), mut sz: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
}
pub unsafe fn testRuntimeSpan_13(mut sp: (), mut sz: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
}
pub unsafe fn testRuntimeSpan_14(mut sp: (), mut sz: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
}
pub unsafe fn testRuntimeSpan_15(mut sp: (), mut sz: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
}
pub unsafe fn testRuntimeSpan_16(mut sp: (), mut sz: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
}
pub unsafe fn testRuntimeSpan_17(mut sp: (), mut sz: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
}
pub unsafe fn testRuntimeSpan_18(mut sp: (), mut sz: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
}
pub unsafe fn testRuntimeSpan_19(mut sp: (), mut sz: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
}
pub unsafe fn testRuntimeSpan_20(mut sp: (), mut sz: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
}
pub unsafe fn testRuntimeSpan_21(mut sp: (), mut sz: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
}
pub unsafe fn testRuntimeSpan_22(mut sp: (), mut sz: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
}
pub unsafe fn testRuntimeSpan_23(mut sp: (), mut sz: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
}
pub unsafe fn testRuntimeSpan_24(mut sp: (), mut sz: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
}
pub unsafe fn testRuntimeSpan_25(mut sp: (), mut sz: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct A {}
pub static mut iArr1_26: std::cell::LazyCell<[(); 10]> = std::cell::LazyCell::new(|| unsafe {
    [0_(), 1_(), 2_(), 3_(), 4_(), 5_(), 6_(), 7_(), 8_(), 9_()]
});
pub static mut iArr2_27: std::cell::LazyCell<[(); 10]> = std::cell::LazyCell::new(|| unsafe {
    [
        10_(),
        11_(),
        12_(),
        13_(),
        14_(),
        15_(),
        16_(),
        17_(),
        18_(),
        19_(),
    ]
});

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
    const _: () = assert!(true, "testConstexprSpan(std::span<int>(), 0)");;
    const _: () = assert!(true, "testConstexprSpan(std::span<long>(), 0)");;
    const _: () = assert!(true, "testConstexprSpan(std::span<double>(), 0)");;
    const _: () = assert!(true, "testConstexprSpan(std::span<A>(), 0)");;
    const _: () = assert!(true, "testConstexprSpan(std::span<std::string>(), 0)");;
    const _: () = assert!(true, "testConstexprSpan(std::span<int, 0>(), 0)");;
    const _: () = assert!(true, "testConstexprSpan(std::span<long, 0>(), 0)");;
    const _: () = assert!(true, "testConstexprSpan(std::span<double, 0>(), 0)");;
    const _: () = assert!(true, "testConstexprSpan(std::span<A, 0>(), 0)");;
    const _: () = assert!(true, "testConstexprSpan(std::span<std::string, 0>(), 0)");;
    const _: () = assert!(true, "testConstexprSpan(std::span<const int>(iArr1, 1), 1)");;
    const _: () = assert!(true, "testConstexprSpan(std::span<const int>(iArr1, 2), 2)");;
    const _: () = assert!(true, "testConstexprSpan(std::span<const int>(iArr1, 3), 3)");;
    const _: () = assert!(true, "testConstexprSpan(std::span<const int>(iArr1, 4), 4)");;
    const _: () = assert!(true, "testConstexprSpan(std::span<const int>(iArr1, 5), 5)");;
    (unsafe { testRuntimeSpan_11(unimplemented!(), 0_()) });
    (unsafe { testRuntimeSpan_12(unimplemented!(), 0_()) });
    (unsafe { testRuntimeSpan_13(unimplemented!(), 0_()) });
    (unsafe { testRuntimeSpan_14(unimplemented!(), 0_()) });
    (unsafe { testRuntimeSpan_15(unimplemented!(), 0_()) });
    (unsafe { testRuntimeSpan_16(unimplemented!(), 0_()) });
    (unsafe { testRuntimeSpan_17(unimplemented!(), 0_()) });
    (unsafe { testRuntimeSpan_18(unimplemented!(), 0_()) });
    (unsafe { testRuntimeSpan_19(unimplemented!(), 0_()) });
    (unsafe { testRuntimeSpan_20(unimplemented!(), 0_()) });
    (unsafe { testRuntimeSpan_11(unimplemented!(), 1_()) });
    (unsafe { testRuntimeSpan_11(unimplemented!(), 2_()) });
    (unsafe { testRuntimeSpan_11(unimplemented!(), 3_()) });
    (unsafe { testRuntimeSpan_11(unimplemented!(), 4_()) });
    (unsafe { testRuntimeSpan_11(unimplemented!(), 5_()) });
    (unsafe { testRuntimeSpan_21(unimplemented!(), 1_()) });
    (unsafe { testRuntimeSpan_22(unimplemented!(), 2_()) });
    (unsafe { testRuntimeSpan_23(unimplemented!(), 3_()) });
    (unsafe { testRuntimeSpan_24(unimplemented!(), 4_()) });
    (unsafe { testRuntimeSpan_25(unimplemented!(), 5_()) });
    let mut s: () = unimplemented!();
    (unsafe { testRuntimeSpan_15(unimplemented!(), 0_()) });
    (unsafe { testRuntimeSpan_15(unimplemented!(), 1_()) });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const iArr1_26);
    std::cell::LazyCell::force(&*&raw const iArr2_27);
}
