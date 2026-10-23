extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn testConstexprSpan_0(mut sp: (), mut idx: ()) -> () {
    unimplemented!();
    let r1: () = &(*unimplemented!());
    let r2: () = &(*(unimplemented!().offset((idx) as isize)));
    return ((*r1) == (*r2));
}
pub unsafe fn testConstexprSpan_1(mut sp: (), mut idx: ()) -> () {
    unimplemented!();
    let r1: () = &(*unimplemented!());
    let r2: () = &(*(unimplemented!().offset((idx) as isize)));
    return ((*r1) == (*r2));
}
pub unsafe fn testConstexprSpan_2(mut sp: (), mut idx: ()) -> () {
    unimplemented!();
    let r1: () = &(*unimplemented!());
    let r2: () = &(*(unimplemented!().offset((idx) as isize)));
    return ((*r1) == (*r2));
}
pub unsafe fn testConstexprSpan_3(mut sp: (), mut idx: ()) -> () {
    unimplemented!();
    let r1: () = &(*unimplemented!());
    let r2: () = &(*(unimplemented!().offset((idx) as isize)));
    return ((*r1) == (*r2));
}
pub unsafe fn testConstexprSpan_4(mut sp: (), mut idx: ()) -> () {
    unimplemented!();
    let r1: () = &(*unimplemented!());
    let r2: () = &(*(unimplemented!().offset((idx) as isize)));
    return ((*r1) == (*r2));
}
pub unsafe fn testRuntimeSpan_5(mut sp: (), mut idx: ()) {
    unimplemented!();
    let r1: () = &mut (*unimplemented!());
    let r2: () = &mut (*(unimplemented!().offset((idx) as isize)));
    unimplemented!();
}
pub unsafe fn testRuntimeSpan_6(mut sp: (), mut idx: ()) {
    unimplemented!();
    let r1: () = &mut (*unimplemented!());
    let r2: () = &mut (*(unimplemented!().offset((idx) as isize)));
    unimplemented!();
}
pub unsafe fn testRuntimeSpan_7(mut sp: (), mut idx: ()) {
    unimplemented!();
    let r1: () = &mut (*unimplemented!());
    let r2: () = &mut (*(unimplemented!().offset((idx) as isize)));
    unimplemented!();
}
pub unsafe fn testRuntimeSpan_8(mut sp: (), mut idx: ()) {
    unimplemented!();
    let r1: () = &mut (*unimplemented!());
    let r2: () = &mut (*(unimplemented!().offset((idx) as isize)));
    unimplemented!();
}
pub unsafe fn testRuntimeSpan_9(mut sp: (), mut idx: ()) {
    unimplemented!();
    let r1: () = &mut (*unimplemented!());
    let r2: () = &mut (*(unimplemented!().offset((idx) as isize)));
    unimplemented!();
}
pub unsafe fn testRuntimeSpan_10(mut sp: (), mut idx: ()) {
    unimplemented!();
    let r1: () = &mut (*unimplemented!());
    let r2: () = &mut (*(unimplemented!().offset((idx) as isize)));
    unimplemented!();
}
pub unsafe fn testRuntimeSpan_11(mut sp: (), mut idx: ()) {
    unimplemented!();
    let r1: () = &mut (*unimplemented!());
    let r2: () = &mut (*(unimplemented!().offset((idx) as isize)));
    unimplemented!();
}
pub static mut iArr1_12: std::cell::LazyCell<[(); 10]> = std::cell::LazyCell::new(|| unsafe {
    [0_(), 1_(), 2_(), 3_(), 4_(), 5_(), 6_(), 7_(), 8_(), 9_()]
});
pub static mut iArr2_13: std::cell::LazyCell<[(); 10]> = std::cell::LazyCell::new(|| unsafe {
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
    const _: () = assert!(true, "testConstexprSpan(std::span<const int>(iArr1, 1), 0)");;
    const _: () = assert!(true, "testConstexprSpan(std::span<const int>(iArr1, 2), 0)");;
    const _: () = assert!(true, "testConstexprSpan(std::span<const int>(iArr1, 2), 1)");;
    const _: () = assert!(true, "testConstexprSpan(std::span<const int>(iArr1, 3), 0)");;
    const _: () = assert!(true, "testConstexprSpan(std::span<const int>(iArr1, 3), 1)");;
    const _: () = assert!(true, "testConstexprSpan(std::span<const int>(iArr1, 3), 2)");;
    const _: () = assert!(true, "testConstexprSpan(std::span<const int>(iArr1, 4), 0)");;
    const _: () = assert!(true, "testConstexprSpan(std::span<const int>(iArr1, 4), 1)");;
    const _: () = assert!(true, "testConstexprSpan(std::span<const int>(iArr1, 4), 2)");;
    const _: () = assert!(true, "testConstexprSpan(std::span<const int>(iArr1, 4), 3)");;
    const _: () = assert!(
        true,
        "testConstexprSpan(std::span<const int, 1>(iArr1, 1), 0)"
    );;
    const _: () = assert!(
        true,
        "testConstexprSpan(std::span<const int, 2>(iArr1, 2), 0)"
    );;
    const _: () = assert!(
        true,
        "testConstexprSpan(std::span<const int, 2>(iArr1, 2), 1)"
    );;
    const _: () = assert!(
        true,
        "testConstexprSpan(std::span<const int, 3>(iArr1, 3), 0)"
    );;
    const _: () = assert!(
        true,
        "testConstexprSpan(std::span<const int, 3>(iArr1, 3), 1)"
    );;
    const _: () = assert!(
        true,
        "testConstexprSpan(std::span<const int, 3>(iArr1, 3), 2)"
    );;
    const _: () = assert!(
        true,
        "testConstexprSpan(std::span<const int, 4>(iArr1, 4), 0)"
    );;
    const _: () = assert!(
        true,
        "testConstexprSpan(std::span<const int, 4>(iArr1, 4), 1)"
    );;
    const _: () = assert!(
        true,
        "testConstexprSpan(std::span<const int, 4>(iArr1, 4), 2)"
    );;
    const _: () = assert!(
        true,
        "testConstexprSpan(std::span<const int, 4>(iArr1, 4), 3)"
    );;
    (unsafe { testRuntimeSpan_5(unimplemented!(), 0_()) });
    (unsafe { testRuntimeSpan_5(unimplemented!(), 0_()) });
    (unsafe { testRuntimeSpan_5(unimplemented!(), 1_()) });
    (unsafe { testRuntimeSpan_5(unimplemented!(), 0_()) });
    (unsafe { testRuntimeSpan_5(unimplemented!(), 1_()) });
    (unsafe { testRuntimeSpan_5(unimplemented!(), 2_()) });
    (unsafe { testRuntimeSpan_5(unimplemented!(), 0_()) });
    (unsafe { testRuntimeSpan_5(unimplemented!(), 1_()) });
    (unsafe { testRuntimeSpan_5(unimplemented!(), 2_()) });
    (unsafe { testRuntimeSpan_5(unimplemented!(), 3_()) });
    (unsafe { testRuntimeSpan_6(unimplemented!(), 0_()) });
    (unsafe { testRuntimeSpan_7(unimplemented!(), 0_()) });
    (unsafe { testRuntimeSpan_7(unimplemented!(), 1_()) });
    (unsafe { testRuntimeSpan_8(unimplemented!(), 0_()) });
    (unsafe { testRuntimeSpan_8(unimplemented!(), 1_()) });
    (unsafe { testRuntimeSpan_8(unimplemented!(), 2_()) });
    (unsafe { testRuntimeSpan_9(unimplemented!(), 0_()) });
    (unsafe { testRuntimeSpan_9(unimplemented!(), 1_()) });
    (unsafe { testRuntimeSpan_9(unimplemented!(), 2_()) });
    (unsafe { testRuntimeSpan_9(unimplemented!(), 3_()) });
    let mut s: () = unimplemented!();
    (unsafe { testRuntimeSpan_10(unimplemented!(), 0_()) });
    (unsafe { testRuntimeSpan_11(unimplemented!(), 0_()) });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const iArr1_12);
    std::cell::LazyCell::force(&*&raw const iArr2_13);
}
