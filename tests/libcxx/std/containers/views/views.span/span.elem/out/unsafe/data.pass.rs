extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn testConstexprSpan_0(mut sp: (), mut ptr: ()) -> () {
    const _: () = assert!(true, "");;
    return ((unimplemented!()) == (ptr));
}
pub unsafe fn testConstexprSpan_1(mut sp: (), mut ptr: ()) -> () {
    const _: () = assert!(true, "");;
    return ((unimplemented!()) == (ptr));
}
pub unsafe fn testConstexprSpan_2(mut sp: (), mut ptr: ()) -> () {
    const _: () = assert!(true, "");;
    return ((unimplemented!()) == (ptr));
}
pub unsafe fn testConstexprSpan_3(mut sp: (), mut ptr: ()) -> () {
    const _: () = assert!(true, "");;
    return ((unimplemented!()) == (ptr));
}
pub unsafe fn testConstexprSpan_4(mut sp: (), mut ptr: ()) -> () {
    const _: () = assert!(true, "");;
    return ((unimplemented!()) == (ptr));
}
pub unsafe fn testConstexprSpan_5(mut sp: (), mut ptr: ()) -> () {
    const _: () = assert!(true, "");;
    return ((unimplemented!()) == (ptr));
}
pub unsafe fn testConstexprSpan_6(mut sp: (), mut ptr: ()) -> () {
    const _: () = assert!(true, "");;
    return ((unimplemented!()) == (ptr));
}
pub unsafe fn testConstexprSpan_7(mut sp: (), mut ptr: ()) -> () {
    const _: () = assert!(true, "");;
    return ((unimplemented!()) == (ptr));
}
pub unsafe fn testConstexprSpan_8(mut sp: (), mut ptr: ()) -> () {
    const _: () = assert!(true, "");;
    return ((unimplemented!()) == (ptr));
}
pub unsafe fn testConstexprSpan_9(mut sp: (), mut ptr: ()) -> () {
    const _: () = assert!(true, "");;
    return ((unimplemented!()) == (ptr));
}
pub unsafe fn testConstexprSpan_10(mut sp: (), mut ptr: ()) -> () {
    const _: () = assert!(true, "");;
    return ((unimplemented!()) == (ptr));
}
pub unsafe fn testConstexprSpan_11(mut sp: (), mut ptr: ()) -> () {
    const _: () = assert!(true, "");;
    return ((unimplemented!()) == (ptr));
}
pub unsafe fn testConstexprSpan_12(mut sp: (), mut ptr: ()) -> () {
    const _: () = assert!(true, "");;
    return ((unimplemented!()) == (ptr));
}
pub unsafe fn testConstexprSpan_13(mut sp: (), mut ptr: ()) -> () {
    const _: () = assert!(true, "");;
    return ((unimplemented!()) == (ptr));
}
pub unsafe fn testConstexprSpan_14(mut sp: (), mut ptr: ()) -> () {
    const _: () = assert!(true, "");;
    return ((unimplemented!()) == (ptr));
}
pub unsafe fn testRuntimeSpan_15(mut sp: (), mut ptr: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
}
pub unsafe fn testRuntimeSpan_16(mut sp: (), mut ptr: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
}
pub unsafe fn testRuntimeSpan_17(mut sp: (), mut ptr: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
}
pub unsafe fn testRuntimeSpan_18(mut sp: (), mut ptr: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
}
pub unsafe fn testRuntimeSpan_19(mut sp: (), mut ptr: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
}
pub unsafe fn testRuntimeSpan_20(mut sp: (), mut ptr: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
}
pub unsafe fn testRuntimeSpan_21(mut sp: (), mut ptr: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
}
pub unsafe fn testRuntimeSpan_22(mut sp: (), mut ptr: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
}
pub unsafe fn testRuntimeSpan_23(mut sp: (), mut ptr: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
}
pub unsafe fn testRuntimeSpan_24(mut sp: (), mut ptr: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
}
pub unsafe fn testRuntimeSpan_25(mut sp: (), mut ptr: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
}
pub unsafe fn testRuntimeSpan_26(mut sp: (), mut ptr: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
}
pub unsafe fn testRuntimeSpan_27(mut sp: (), mut ptr: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
}
pub unsafe fn testRuntimeSpan_28(mut sp: (), mut ptr: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
}
pub unsafe fn testRuntimeSpan_29(mut sp: (), mut ptr: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct A {}
pub static mut iArr1_30: std::cell::LazyCell<[(); 10]> = std::cell::LazyCell::new(|| unsafe {
    [0_(), 1_(), 2_(), 3_(), 4_(), 5_(), 6_(), 7_(), 8_(), 9_()]
});
pub static mut iArr2_31: std::cell::LazyCell<[(); 10]> = std::cell::LazyCell::new(|| unsafe {
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
    const _: () = assert!(true, "testConstexprSpan(std::span<int>(), nullptr)");;
    const _: () = assert!(true, "testConstexprSpan(std::span<long>(), nullptr)");;
    const _: () = assert!(true, "testConstexprSpan(std::span<double>(), nullptr)");;
    const _: () = assert!(true, "testConstexprSpan(std::span<A>(), nullptr)");;
    const _: () = assert!(true, "testConstexprSpan(std::span<std::string>(), nullptr)");;
    const _: () = assert!(
        true,
        "testConstexprSpan(std::span<const int>(iArr1, 1), iArr1)"
    );;
    const _: () = assert!(
        true,
        "testConstexprSpan(std::span<const int>(iArr1, 2), iArr1)"
    );;
    const _: () = assert!(
        true,
        "testConstexprSpan(std::span<const int>(iArr1, 3), iArr1)"
    );;
    const _: () = assert!(
        true,
        "testConstexprSpan(std::span<const int>(iArr1, 4), iArr1)"
    );;
    const _: () = assert!(
        true,
        "testConstexprSpan(std::span<const int>(iArr1 + 1, 1), iArr1 + 1)"
    );;
    const _: () = assert!(
        true,
        "testConstexprSpan(std::span<const int>(iArr1 + 2, 2), iArr1 + 2)"
    );;
    const _: () = assert!(
        true,
        "testConstexprSpan(std::span<const int>(iArr1 + 3, 3), iArr1 + 3)"
    );;
    const _: () = assert!(
        true,
        "testConstexprSpan(std::span<const int>(iArr1 + 4, 4), iArr1 + 4)"
    );;
    const _: () = assert!(true, "testConstexprSpan(std::span<int, 0>(), nullptr)");;
    const _: () = assert!(true, "testConstexprSpan(std::span<long, 0>(), nullptr)");;
    const _: () = assert!(true, "testConstexprSpan(std::span<double, 0>(), nullptr)");;
    const _: () = assert!(true, "testConstexprSpan(std::span<A, 0>(), nullptr)");;
    const _: () = assert!(
        true,
        "testConstexprSpan(std::span<std::string, 0>(), nullptr)"
    );;
    const _: () = assert!(
        true,
        "testConstexprSpan(std::span<const int, 1>(iArr1, 1), iArr1)"
    );;
    const _: () = assert!(
        true,
        "testConstexprSpan(std::span<const int, 2>(iArr1, 2), iArr1)"
    );;
    const _: () = assert!(
        true,
        "testConstexprSpan(std::span<const int, 3>(iArr1, 3), iArr1)"
    );;
    const _: () = assert!(
        true,
        "testConstexprSpan(std::span<const int, 4>(iArr1, 4), iArr1)"
    );;
    const _: () = assert!(
        true,
        "testConstexprSpan(std::span<const int, 1>(iArr1 + 1, 1), iArr1 + 1)"
    );;
    const _: () = assert!(
        true,
        "testConstexprSpan(std::span<const int, 2>(iArr1 + 2, 2), iArr1 + 2)"
    );;
    const _: () = assert!(
        true,
        "testConstexprSpan(std::span<const int, 3>(iArr1 + 3, 3), iArr1 + 3)"
    );;
    const _: () = assert!(
        true,
        "testConstexprSpan(std::span<const int, 4>(iArr1 + 4, 4), iArr1 + 4)"
    );;
    (unsafe { testRuntimeSpan_15(unimplemented!(), unimplemented!()) });
    (unsafe { testRuntimeSpan_16(unimplemented!(), unimplemented!()) });
    (unsafe { testRuntimeSpan_17(unimplemented!(), unimplemented!()) });
    (unsafe { testRuntimeSpan_18(unimplemented!(), unimplemented!()) });
    (unsafe { testRuntimeSpan_19(unimplemented!(), unimplemented!()) });
    (unsafe {
        let _sp: () = unimplemented!();
        let _ptr: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut iArr2_31)).as_mut_ptr();
        testRuntimeSpan_15(_sp, _ptr)
    });
    (unsafe {
        let _sp: () = unimplemented!();
        let _ptr: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut iArr2_31)).as_mut_ptr();
        testRuntimeSpan_15(_sp, _ptr)
    });
    (unsafe {
        let _sp: () = unimplemented!();
        let _ptr: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut iArr2_31)).as_mut_ptr();
        testRuntimeSpan_15(_sp, _ptr)
    });
    (unsafe {
        let _sp: () = unimplemented!();
        let _ptr: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut iArr2_31)).as_mut_ptr();
        testRuntimeSpan_15(_sp, _ptr)
    });
    (unsafe {
        let _sp: () = unimplemented!();
        let _ptr: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut iArr2_31))
            .as_mut_ptr()
            .offset((1_()) as isize);
        testRuntimeSpan_15(_sp, _ptr)
    });
    (unsafe {
        let _sp: () = unimplemented!();
        let _ptr: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut iArr2_31))
            .as_mut_ptr()
            .offset((2_()) as isize);
        testRuntimeSpan_15(_sp, _ptr)
    });
    (unsafe {
        let _sp: () = unimplemented!();
        let _ptr: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut iArr2_31))
            .as_mut_ptr()
            .offset((3_()) as isize);
        testRuntimeSpan_15(_sp, _ptr)
    });
    (unsafe {
        let _sp: () = unimplemented!();
        let _ptr: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut iArr2_31))
            .as_mut_ptr()
            .offset((4_()) as isize);
        testRuntimeSpan_15(_sp, _ptr)
    });
    (unsafe { testRuntimeSpan_20(unimplemented!(), unimplemented!()) });
    (unsafe { testRuntimeSpan_21(unimplemented!(), unimplemented!()) });
    (unsafe { testRuntimeSpan_22(unimplemented!(), unimplemented!()) });
    (unsafe { testRuntimeSpan_23(unimplemented!(), unimplemented!()) });
    (unsafe { testRuntimeSpan_24(unimplemented!(), unimplemented!()) });
    (unsafe {
        let _sp: () = unimplemented!();
        let _ptr: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut iArr2_31)).as_mut_ptr();
        testRuntimeSpan_25(_sp, _ptr)
    });
    (unsafe {
        let _sp: () = unimplemented!();
        let _ptr: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut iArr2_31)).as_mut_ptr();
        testRuntimeSpan_26(_sp, _ptr)
    });
    (unsafe {
        let _sp: () = unimplemented!();
        let _ptr: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut iArr2_31)).as_mut_ptr();
        testRuntimeSpan_27(_sp, _ptr)
    });
    (unsafe {
        let _sp: () = unimplemented!();
        let _ptr: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut iArr2_31)).as_mut_ptr();
        testRuntimeSpan_28(_sp, _ptr)
    });
    (unsafe {
        let _sp: () = unimplemented!();
        let _ptr: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut iArr2_31))
            .as_mut_ptr()
            .offset((1_()) as isize);
        testRuntimeSpan_25(_sp, _ptr)
    });
    (unsafe {
        let _sp: () = unimplemented!();
        let _ptr: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut iArr2_31))
            .as_mut_ptr()
            .offset((2_()) as isize);
        testRuntimeSpan_26(_sp, _ptr)
    });
    (unsafe {
        let _sp: () = unimplemented!();
        let _ptr: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut iArr2_31))
            .as_mut_ptr()
            .offset((3_()) as isize);
        testRuntimeSpan_27(_sp, _ptr)
    });
    (unsafe {
        let _sp: () = unimplemented!();
        let _ptr: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut iArr2_31))
            .as_mut_ptr()
            .offset((4_()) as isize);
        testRuntimeSpan_28(_sp, _ptr)
    });
    let mut s: () = unimplemented!();
    (unsafe {
        let _sp: () = unimplemented!();
        let _ptr: () = (&mut s as *mut ());
        testRuntimeSpan_19(_sp, _ptr)
    });
    (unsafe {
        let _sp: () = unimplemented!();
        let _ptr: () = (&mut s as *mut ());
        testRuntimeSpan_29(_sp, _ptr)
    });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const iArr1_30);
    std::cell::LazyCell::force(&*&raw const iArr2_31);
}
