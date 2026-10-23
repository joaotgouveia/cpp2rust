extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type anon_0 = ();
pub const anon_0_value: anon_0 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_byte__std_byte_ {}
pub unsafe fn testRuntimeSpan_1(mut sp: ()) {
    const _: () = assert!(true, "");;
    let mut spBytes: () = unimplemented!();
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    unimplemented!();
    unimplemented!();
}
pub unsafe fn testRuntimeSpan_2(mut sp: ()) {
    const _: () = assert!(true, "");;
    let mut spBytes: () = unimplemented!();
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    unimplemented!();
    unimplemented!();
}
pub unsafe fn testRuntimeSpan_3(mut sp: ()) {
    const _: () = assert!(true, "");;
    let mut spBytes: () = unimplemented!();
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    unimplemented!();
    unimplemented!();
}
pub unsafe fn testRuntimeSpan_4(mut sp: ()) {
    const _: () = assert!(true, "");;
    let mut spBytes: () = unimplemented!();
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    unimplemented!();
    unimplemented!();
}
pub unsafe fn testRuntimeSpan_5(mut sp: ()) {
    const _: () = assert!(true, "");;
    let mut spBytes: () = unimplemented!();
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    unimplemented!();
    unimplemented!();
}
pub unsafe fn testRuntimeSpan_6(mut sp: ()) {
    const _: () = assert!(true, "");;
    let mut spBytes: () = unimplemented!();
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    unimplemented!();
    unimplemented!();
}
pub unsafe fn testRuntimeSpan_7(mut sp: ()) {
    const _: () = assert!(true, "");;
    let mut spBytes: () = unimplemented!();
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    unimplemented!();
    unimplemented!();
}
pub unsafe fn testRuntimeSpan_8(mut sp: ()) {
    const _: () = assert!(true, "");;
    let mut spBytes: () = unimplemented!();
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    unimplemented!();
    unimplemented!();
}
pub unsafe fn testRuntimeSpan_9(mut sp: ()) {
    const _: () = assert!(true, "");;
    let mut spBytes: () = unimplemented!();
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    unimplemented!();
    unimplemented!();
}
pub unsafe fn testRuntimeSpan_10(mut sp: ()) {
    const _: () = assert!(true, "");;
    let mut spBytes: () = unimplemented!();
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    unimplemented!();
    unimplemented!();
}
pub unsafe fn testRuntimeSpan_11(mut sp: ()) {
    const _: () = assert!(true, "");;
    let mut spBytes: () = unimplemented!();
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    unimplemented!();
    unimplemented!();
}
pub unsafe fn testRuntimeSpan_12(mut sp: ()) {
    const _: () = assert!(true, "");;
    let mut spBytes: () = unimplemented!();
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    unimplemented!();
    unimplemented!();
}
pub unsafe fn testRuntimeSpan_13(mut sp: ()) {
    const _: () = assert!(true, "");;
    let mut spBytes: () = unimplemented!();
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    unimplemented!();
    unimplemented!();
}
pub unsafe fn testRuntimeSpan_14(mut sp: ()) {
    const _: () = assert!(true, "");;
    let mut spBytes: () = unimplemented!();
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    unimplemented!();
    unimplemented!();
}
pub unsafe fn testRuntimeSpan_15(mut sp: ()) {
    const _: () = assert!(true, "");;
    let mut spBytes: () = unimplemented!();
    const _: () = assert!(true, "");;
    {
        unimplemented!();
    }
    unimplemented!();
    unimplemented!();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct A {}
pub static mut iArr2_16: std::cell::LazyCell<[(); 10]> = std::cell::LazyCell::new(|| unsafe {
    [0_(), 1_(), 2_(), 3_(), 4_(), 5_(), 6_(), 7_(), 8_(), 9_()]
});
pub unsafe fn test_constraints_17() {
    const _: () = assert!(true, "hasAsWritableBytes<int>");;
    const _: () = assert!(true, "hasAsWritableBytes<long>");;
    const _: () = assert!(true, "hasAsWritableBytes<double>");;
    const _: () = assert!(true, "hasAsWritableBytes<A>");;
    const _: () = assert!(true, "hasAsWritableBytes<std::string>");;
    const _: () = assert!(!(false), "!hasAsWritableBytes<const int>");;
    const _: () = assert!(!(false), "!hasAsWritableBytes<const long>");;
    const _: () = assert!(!(false), "!hasAsWritableBytes<const double>");;
    const _: () = assert!(!(false), "!hasAsWritableBytes<const A>");;
    const _: () = assert!(!(false), "!hasAsWritableBytes<const std::string>");;
    const _: () = assert!(!(false), "!hasAsWritableBytes<volatile int>");;
    const _: () = assert!(!(false), "!hasAsWritableBytes<volatile long>");;
    const _: () = assert!(!(false), "!hasAsWritableBytes<volatile double>");;
    const _: () = assert!(!(false), "!hasAsWritableBytes<volatile A>");;
    const _: () = assert!(!(false), "!hasAsWritableBytes<volatile std::string>");;
    const _: () = assert!(!(false), "!hasAsWritableBytes<const volatile int>");;
    const _: () = assert!(!(false), "!hasAsWritableBytes<const volatile long>");;
    const _: () = assert!(!(false), "!hasAsWritableBytes<const volatile double>");;
    const _: () = assert!(!(false), "!hasAsWritableBytes<const volatile A>");;
    const _: () = assert!(!(false), "!hasAsWritableBytes<const volatile std::string>");;
    const _: () = assert!(true, "hasAsWritableBytes<int, 0>");;
    const _: () = assert!(true, "hasAsWritableBytes<long, 0>");;
    const _: () = assert!(true, "hasAsWritableBytes<double, 0>");;
    const _: () = assert!(true, "hasAsWritableBytes<A, 0>");;
    const _: () = assert!(true, "hasAsWritableBytes<std::string, 0>");;
    const _: () = assert!(!(false), "!hasAsWritableBytes<const int, 0>");;
    const _: () = assert!(!(false), "!hasAsWritableBytes<const long, 0>");;
    const _: () = assert!(!(false), "!hasAsWritableBytes<const double, 0>");;
    const _: () = assert!(!(false), "!hasAsWritableBytes<const A, 0>");;
    const _: () = assert!(!(false), "!hasAsWritableBytes<const std::string, 0>");;
    const _: () = assert!(!(false), "!hasAsWritableBytes<volatile int, 0>");;
    const _: () = assert!(!(false), "!hasAsWritableBytes<volatile long, 0>");;
    const _: () = assert!(!(false), "!hasAsWritableBytes<volatile double, 0>");;
    const _: () = assert!(!(false), "!hasAsWritableBytes<volatile A, 0>");;
    const _: () = assert!(!(false), "!hasAsWritableBytes<volatile std::string, 0>");;
    const _: () = assert!(!(false), "!hasAsWritableBytes<const volatile int, 0>");;
    const _: () = assert!(!(false), "!hasAsWritableBytes<const volatile long, 0>");;
    const _: () = assert!(!(false), "!hasAsWritableBytes<const volatile double, 0>");;
    const _: () = assert!(!(false), "!hasAsWritableBytes<const volatile A, 0>");;
    const _: () = assert!(
        !(false),
        "!hasAsWritableBytes<const volatile std::string, 0>"
    );;
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
    (unsafe { test_constraints_17() });
    (unsafe { testRuntimeSpan_1(unimplemented!()) });
    (unsafe { testRuntimeSpan_2(unimplemented!()) });
    (unsafe { testRuntimeSpan_3(unimplemented!()) });
    (unsafe { testRuntimeSpan_4(unimplemented!()) });
    (unsafe { testRuntimeSpan_5(unimplemented!()) });
    (unsafe { testRuntimeSpan_6(unimplemented!()) });
    (unsafe { testRuntimeSpan_7(unimplemented!()) });
    (unsafe { testRuntimeSpan_8(unimplemented!()) });
    (unsafe { testRuntimeSpan_9(unimplemented!()) });
    (unsafe { testRuntimeSpan_10(unimplemented!()) });
    (unsafe { testRuntimeSpan_1(unimplemented!()) });
    (unsafe { testRuntimeSpan_1(unimplemented!()) });
    (unsafe { testRuntimeSpan_1(unimplemented!()) });
    (unsafe { testRuntimeSpan_1(unimplemented!()) });
    (unsafe { testRuntimeSpan_1(unimplemented!()) });
    (unsafe { testRuntimeSpan_11(unimplemented!()) });
    (unsafe { testRuntimeSpan_12(unimplemented!()) });
    (unsafe { testRuntimeSpan_13(unimplemented!()) });
    (unsafe { testRuntimeSpan_14(unimplemented!()) });
    (unsafe { testRuntimeSpan_15(unimplemented!()) });
    let mut s: () = unimplemented!();
    (unsafe { testRuntimeSpan_5(unimplemented!()) });
    (unsafe { testRuntimeSpan_5(unimplemented!()) });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const iArr2_16);
}
