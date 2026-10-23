extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn testConstexprSpan_0(mut s: ()) -> () {
    let mut ret: () = true;
    let mut b: () = unimplemented!();
    if unimplemented!() {
        ret = (ret) && (unimplemented!());
    } else {
        ret = (ret) && ((*unimplemented!()) == (*unimplemented!()));
        ret = (ret)
            && ((&mut (*unimplemented!()) as *mut ()) == (&mut (*unimplemented!()) as *mut ()));
    }
    return ret;
}
pub unsafe fn testConstexprSpan_1(mut s: ()) -> () {
    let mut ret: () = true;
    let mut b: () = unimplemented!();
    if unimplemented!() {
        ret = (ret) && (unimplemented!());
    } else {
        ret = (ret) && ((*unimplemented!()) == (*unimplemented!()));
        ret = (ret)
            && ((&mut (*unimplemented!()) as *mut ()) == (&mut (*unimplemented!()) as *mut ()));
    }
    return ret;
}
pub unsafe fn testConstexprSpan_2(mut s: ()) -> () {
    let mut ret: () = true;
    let mut b: () = unimplemented!();
    if unimplemented!() {
        ret = (ret) && (unimplemented!());
    } else {
        ret = (ret) && ((*unimplemented!()) == (*unimplemented!()));
        ret = (ret)
            && ((&mut (*unimplemented!()) as *mut ()) == (&mut (*unimplemented!()) as *mut ()));
    }
    return ret;
}
pub unsafe fn testConstexprSpan_3(mut s: ()) -> () {
    let mut ret: () = true;
    let mut b: () = unimplemented!();
    if unimplemented!() {
        ret = (ret) && (unimplemented!());
    } else {
        ret = (ret)
            && (unsafe {
                let _arg0: A = (*unimplemented!());
                operator_eq_4(_arg0, (*unimplemented!()))
            });
        ret =
            (ret) && ((&mut (*unimplemented!()) as *mut A) == (&mut (*unimplemented!()) as *mut A));
    }
    return ret;
}
pub unsafe fn testConstexprSpan_5(mut s: ()) -> () {
    let mut ret: () = true;
    let mut b: () = unimplemented!();
    if unimplemented!() {
        ret = (ret) && (unimplemented!());
    } else {
        ret = (ret) && (unimplemented!());
        ret = (ret)
            && ((&mut (*unimplemented!()) as *mut ()) == (&mut (*unimplemented!()) as *mut ()));
    }
    return ret;
}
pub unsafe fn testConstexprSpan_6(mut s: ()) -> () {
    let mut ret: () = true;
    let mut b: () = unimplemented!();
    {
        ret = (ret) && (unimplemented!());
    }
    return ret;
}
pub unsafe fn testConstexprSpan_7(mut s: ()) -> () {
    let mut ret: () = true;
    let mut b: () = unimplemented!();
    {
        ret = (ret) && (unimplemented!());
    }
    return ret;
}
pub unsafe fn testConstexprSpan_8(mut s: ()) -> () {
    let mut ret: () = true;
    let mut b: () = unimplemented!();
    {
        ret = (ret) && (unimplemented!());
    }
    return ret;
}
pub unsafe fn testConstexprSpan_9(mut s: ()) -> () {
    let mut ret: () = true;
    let mut b: () = unimplemented!();
    {
        ret = (ret) && (unimplemented!());
    }
    return ret;
}
pub unsafe fn testConstexprSpan_10(mut s: ()) -> () {
    let mut ret: () = true;
    let mut b: () = unimplemented!();
    {
        ret = (ret) && (unimplemented!());
    }
    return ret;
}
pub unsafe fn testConstexprSpan_11(mut s: ()) -> () {
    let mut ret: () = true;
    let mut b: () = unimplemented!();
    if unimplemented!() {
        ret = (ret) && (unimplemented!());
    } else {
        ret = (ret) && ((*unimplemented!()) == (*unimplemented!()));
        ret = (ret) && ((&(*unimplemented!()) as *const ()) == (&(*unimplemented!()) as *const ()));
    }
    return ret;
}
pub unsafe fn testRuntimeSpan_12(mut s: ()) {
    let mut b: () = unimplemented!();
    if unimplemented!() {
        unimplemented!();
    } else {
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn testRuntimeSpan_13(mut s: ()) {
    let mut b: () = unimplemented!();
    if unimplemented!() {
        unimplemented!();
    } else {
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn testRuntimeSpan_14(mut s: ()) {
    let mut b: () = unimplemented!();
    if unimplemented!() {
        unimplemented!();
    } else {
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn testRuntimeSpan_15(mut s: ()) {
    let mut b: () = unimplemented!();
    if unimplemented!() {
        unimplemented!();
    } else {
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn testRuntimeSpan_16(mut s: ()) {
    let mut b: () = unimplemented!();
    if unimplemented!() {
        unimplemented!();
    } else {
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn testRuntimeSpan_17(mut s: ()) {
    let mut b: () = unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn testRuntimeSpan_18(mut s: ()) {
    let mut b: () = unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn testRuntimeSpan_19(mut s: ()) {
    let mut b: () = unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn testRuntimeSpan_20(mut s: ()) {
    let mut b: () = unimplemented!();
    {
        unimplemented!();
    }
}
pub unsafe fn testRuntimeSpan_21(mut s: ()) {
    let mut b: () = unimplemented!();
    {
        unimplemented!();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct A {}
impl std::cmp::PartialEq for A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_4(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for A {}
pub unsafe fn operator_eq_4(mut _a0: A, mut _a1: A) -> () {
    return true;
}
pub static mut iArr1_22: std::cell::LazyCell<[(); 10]> = std::cell::LazyCell::new(|| unsafe {
    [0_(), 1_(), 2_(), 3_(), 4_(), 5_(), 6_(), 7_(), 8_(), 9_()]
});
pub static mut iArr2_23: std::cell::LazyCell<[(); 10]> = std::cell::LazyCell::new(|| unsafe {
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
    const _: () = assert!(true, "testConstexprSpan(std::span<int>())");;
    const _: () = assert!(true, "testConstexprSpan(std::span<long>())");;
    const _: () = assert!(true, "testConstexprSpan(std::span<double>())");;
    const _: () = assert!(true, "testConstexprSpan(std::span<A>())");;
    const _: () = assert!(true, "testConstexprSpan(std::span<std::string>())");;
    const _: () = assert!(true, "testConstexprSpan(std::span<int, 0>())");;
    const _: () = assert!(true, "testConstexprSpan(std::span<long, 0>())");;
    const _: () = assert!(true, "testConstexprSpan(std::span<double, 0>())");;
    const _: () = assert!(true, "testConstexprSpan(std::span<A, 0>())");;
    const _: () = assert!(true, "testConstexprSpan(std::span<std::string, 0>())");;
    const _: () = assert!(true, "testConstexprSpan(std::span<const int>(iArr1, 1))");;
    const _: () = assert!(true, "testConstexprSpan(std::span<const int>(iArr1, 2))");;
    const _: () = assert!(true, "testConstexprSpan(std::span<const int>(iArr1, 3))");;
    const _: () = assert!(true, "testConstexprSpan(std::span<const int>(iArr1, 4))");;
    const _: () = assert!(true, "testConstexprSpan(std::span<const int>(iArr1, 5))");;
    (unsafe { testRuntimeSpan_12(unimplemented!()) });
    (unsafe { testRuntimeSpan_13(unimplemented!()) });
    (unsafe { testRuntimeSpan_14(unimplemented!()) });
    (unsafe { testRuntimeSpan_15(unimplemented!()) });
    (unsafe { testRuntimeSpan_16(unimplemented!()) });
    (unsafe { testRuntimeSpan_17(unimplemented!()) });
    (unsafe { testRuntimeSpan_18(unimplemented!()) });
    (unsafe { testRuntimeSpan_19(unimplemented!()) });
    (unsafe { testRuntimeSpan_20(unimplemented!()) });
    (unsafe { testRuntimeSpan_21(unimplemented!()) });
    (unsafe { testRuntimeSpan_12(unimplemented!()) });
    (unsafe { testRuntimeSpan_12(unimplemented!()) });
    (unsafe { testRuntimeSpan_12(unimplemented!()) });
    (unsafe { testRuntimeSpan_12(unimplemented!()) });
    (unsafe { testRuntimeSpan_12(unimplemented!()) });
    let mut s: () = unimplemented!();
    (unsafe { testRuntimeSpan_16(unimplemented!()) });
    (unsafe { testRuntimeSpan_16(unimplemented!()) });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const iArr1_22);
    std::cell::LazyCell::force(&*&raw const iArr2_23);
}
