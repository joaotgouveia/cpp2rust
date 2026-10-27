extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn testConstexprSpan_0(mut sp: ()) -> () {
    unimplemented!();
    return ((unimplemented!())
        == (unimplemented!()
            .offset((unimplemented!()) as isize)
            .offset(-((1_()) as isize))));
}
pub unsafe fn testConstexprSpan_1(mut sp: ()) -> () {
    unimplemented!();
    return ((unimplemented!())
        == (unimplemented!()
            .offset((unimplemented!()) as isize)
            .offset(-((1_()) as isize))));
}
pub unsafe fn testConstexprSpan_2(mut sp: ()) -> () {
    unimplemented!();
    return ((unimplemented!())
        == (unimplemented!()
            .offset((unimplemented!()) as isize)
            .offset(-((1_()) as isize))));
}
pub unsafe fn testConstexprSpan_3(mut sp: ()) -> () {
    unimplemented!();
    return ((unimplemented!())
        == (unimplemented!()
            .offset((unimplemented!()) as isize)
            .offset(-((1_()) as isize))));
}
pub unsafe fn testConstexprSpan_4(mut sp: ()) -> () {
    unimplemented!();
    return ((unimplemented!())
        == (unimplemented!()
            .offset((unimplemented!()) as isize)
            .offset(-((1_()) as isize))));
}
pub unsafe fn testRuntimeSpan_5(mut sp: ()) {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn testRuntimeSpan_6(mut sp: ()) {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn testRuntimeSpan_7(mut sp: ()) {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn testRuntimeSpan_8(mut sp: ()) {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn testRuntimeSpan_9(mut sp: ()) {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn testRuntimeSpan_10(mut sp: ()) {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn testRuntimeSpan_11(mut sp: ()) {
    unimplemented!();
    unimplemented!();
}
pub unsafe fn testEmptySpan_12(mut sp: ()) {
    {}
}
pub static mut iArr1_13: std::cell::LazyCell<[(); 10]> = std::cell::LazyCell::new(|| unsafe {
    [0_(), 1_(), 2_(), 3_(), 4_(), 5_(), 6_(), 7_(), 8_(), 9_()]
});
pub static mut iArr2_14: std::cell::LazyCell<[(); 10]> = std::cell::LazyCell::new(|| unsafe {
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
    const _: () = assert!(true, "testConstexprSpan(std::span<const int>(iArr1, 1))");;
    const _: () = assert!(true, "testConstexprSpan(std::span<const int>(iArr1, 2))");;
    const _: () = assert!(true, "testConstexprSpan(std::span<const int>(iArr1, 3))");;
    const _: () = assert!(true, "testConstexprSpan(std::span<const int>(iArr1, 4))");;
    const _: () = assert!(true, "testConstexprSpan(std::span<const int, 1>(iArr1, 1))");;
    const _: () = assert!(true, "testConstexprSpan(std::span<const int, 2>(iArr1, 2))");;
    const _: () = assert!(true, "testConstexprSpan(std::span<const int, 3>(iArr1, 3))");;
    const _: () = assert!(true, "testConstexprSpan(std::span<const int, 4>(iArr1, 4))");;
    (unsafe { testRuntimeSpan_5(unimplemented!()) });
    (unsafe { testRuntimeSpan_5(unimplemented!()) });
    (unsafe { testRuntimeSpan_5(unimplemented!()) });
    (unsafe { testRuntimeSpan_5(unimplemented!()) });
    (unsafe { testRuntimeSpan_6(unimplemented!()) });
    (unsafe { testRuntimeSpan_7(unimplemented!()) });
    (unsafe { testRuntimeSpan_8(unimplemented!()) });
    (unsafe { testRuntimeSpan_9(unimplemented!()) });
    let mut s: () = unimplemented!();
    (unsafe { testRuntimeSpan_10(unimplemented!()) });
    (unsafe { testRuntimeSpan_11(unimplemented!()) });
    let mut sp: () = unimplemented!();
    (unsafe { testEmptySpan_12(unimplemented!()) });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const iArr1_13);
    std::cell::LazyCell::force(&*&raw const iArr2_14);
}
