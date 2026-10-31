extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn testConstexprSpan_0(mut sp: ()) -> bool {
    const _: () = assert!(true, "");;
    return ((unimplemented!()) == (unimplemented!()));
}
pub unsafe fn testConstexprSpan_1(mut sp: ()) -> bool {
    const _: () = assert!(true, "");;
    return ((unimplemented!()) == (unimplemented!()));
}
pub unsafe fn testConstexprSpan_2(mut sp: ()) -> bool {
    const _: () = assert!(true, "");;
    return ((unimplemented!()) == (unimplemented!()));
}
pub unsafe fn testConstexprSpan_3(mut sp: ()) -> bool {
    const _: () = assert!(true, "");;
    return ((unimplemented!()) == (unimplemented!()));
}
pub unsafe fn testConstexprSpan_4(mut sp: ()) -> bool {
    const _: () = assert!(true, "");;
    return ((unimplemented!()) == (unimplemented!()));
}
pub unsafe fn testRuntimeSpan_5(mut sp: ()) {
    const _: () = assert!(true, "");;
    assert!(((unimplemented!()) == (unimplemented!())));
}
pub unsafe fn testRuntimeSpan_6(mut sp: ()) {
    const _: () = assert!(true, "");;
    assert!(((unimplemented!()) == (unimplemented!())));
}
pub unsafe fn testRuntimeSpan_7(mut sp: ()) {
    const _: () = assert!(true, "");;
    assert!(((unimplemented!()) == (unimplemented!())));
}
pub unsafe fn testRuntimeSpan_8(mut sp: ()) {
    const _: () = assert!(true, "");;
    assert!(((unimplemented!()) == (unimplemented!())));
}
pub unsafe fn testRuntimeSpan_9(mut sp: ()) {
    const _: () = assert!(true, "");;
    assert!(((unimplemented!()) == (unimplemented!())));
}
pub unsafe fn testRuntimeSpan_10(mut sp: ()) {
    const _: () = assert!(true, "");;
    assert!(((unimplemented!()) == (unimplemented!())));
}
pub unsafe fn testRuntimeSpan_11(mut sp: ()) {
    const _: () = assert!(true, "");;
    assert!(((unimplemented!()) == (unimplemented!())));
}
pub unsafe fn testEmptySpan_12(mut sp: ()) {
    {}
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct A {}
pub static mut iArr1_13: std::cell::LazyCell<[i32; 10]> =
    std::cell::LazyCell::new(|| unsafe { [0, 1, 2, 3, 4, 5, 6, 7, 8, 9] });
pub static mut iArr2_14: std::cell::LazyCell<[i32; 10]> =
    std::cell::LazyCell::new(|| unsafe { [10, 11, 12, 13, 14, 15, 16, 17, 18, 19] });

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
    let mut s: Vec<libc::c_char> = vec![libc::c_char::default()];
    (unsafe { testRuntimeSpan_10(unimplemented!()) });
    (unsafe { testRuntimeSpan_11(unimplemented!()) });
    let mut sp: () = unimplemented!();
    (unsafe { testEmptySpan_12(unimplemented!()) });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const iArr1_13);
    std::cell::LazyCell::force(&*&raw const iArr2_14);
}
