extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn checkCV_0() {
    let mut arr: [(); 3] = [1_(), 2_(), 3_()];
    let carr: [(); 3] = [4_(), 5_(), 6_()];
    let mut varr: [(); 3] = [7_(), 8_(), 9_()];
    let cvarr: [(); 3] = [1_(), 3_(), 5_()];
    {
        let mut s1: () = unimplemented!();
        let mut s2: () = unimplemented!();
        let mut s3: () = unimplemented!();
        let mut s4: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut s1: () = unimplemented!();
        let mut s2: () = unimplemented!();
        let mut s3: () = unimplemented!();
        let mut s4: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut s1: () = unimplemented!();
        let mut s2: () = unimplemented!();
        let mut s3: () = unimplemented!();
        let mut s4: () = unimplemented!();
        let mut s5: () = unimplemented!();
        let mut s6: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut s1: () = unimplemented!();
        let mut s2: () = unimplemented!();
        let mut s3: () = unimplemented!();
        let mut s4: () = unimplemented!();
        let mut s5: () = unimplemented!();
        let mut s6: () = unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn testSpan_1() -> () {
    let mut val: [(); 2] = [unimplemented!(), unimplemented!()];
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut s1: () = unimplemented!();
    let mut s2: () = unimplemented!();
    let mut s3: () = unimplemented!();
    let mut s4: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut s5: () = unimplemented!();
    let mut s6: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub unsafe fn testSpan_2() -> () {
    let mut val: [(); 2] = [unimplemented!(), unimplemented!()];
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut s1: () = unimplemented!();
    let mut s2: () = unimplemented!();
    let mut s3: () = unimplemented!();
    let mut s4: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut s5: () = unimplemented!();
    let mut s6: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub unsafe fn testSpan_3() -> () {
    let mut val: [A; 2] = [<A>::default(), <A>::default()];
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut s1: () = unimplemented!();
    let mut s2: () = unimplemented!();
    let mut s3: () = unimplemented!();
    let mut s4: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut s5: () = unimplemented!();
    let mut s6: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub unsafe fn testSpan_4() -> () {
    let mut val: [(); 2] = [unimplemented!(), unimplemented!()];
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut s1: () = unimplemented!();
    let mut s2: () = unimplemented!();
    let mut s3: () = unimplemented!();
    let mut s4: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut s5: () = unimplemented!();
    let mut s6: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct A {}
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
    (unsafe { testSpan_1() });
    (unsafe { testSpan_2() });
    (unsafe { testSpan_3() });
    (unsafe { testSpan_4() });
    const _: () = assert!(true, "testSpan<int>()");;
    const _: () = assert!(true, "testSpan<double>()");;
    const _: () = assert!(true, "testSpan<A>()");;
    (unsafe { checkCV_0() });
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<std::span<int, 2>, int (&)[3]>"
        );;
    }
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<std::span<float>, int (&)[3]>"
        );;
        const _: () = assert!(
            true,
            "!std::is_constructible_v<std::span<float, 3>, int (&)[3]>"
        );;
    }
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<std::span<int>, const int (&)[3]>"
        );;
        const _: () = assert!(
            true,
            "!std::is_constructible_v<std::span<int>, volatile int (&)[3]>"
        );;
        const _: () = assert!(
            true,
            "!std::is_constructible_v<std::span<int>, const volatile int (&)[3]>"
        );;
        const _: () = assert!(
            true,
            "!std::is_constructible_v<std::span<const int>, volatile int (&)[3]>"
        );;
        const _: () = assert!(
            true,
            "!std::is_constructible_v<std::span<const int>, const volatile int (&)[3]>"
        );;
        const _: () = assert!(
            true,
            "!std::is_constructible_v<std::span<volatile int>, const int (&)[3]>"
        );;
        const _: () = assert!(
            true,
            "!std::is_constructible_v<std::span<volatile int>, const volatile int (&)[3]>"
        );;
    }
    {
        const _: () = assert!(
            true,
            "!std::is_constructible_v<std::span<int, 3>, const int (&)[3]>"
        );;
        const _: () = assert!(
            true,
            "!std::is_constructible_v<std::span<int, 3>, volatile int (&)[3]>"
        );;
        const _: () = assert!(
            true,
            "!std::is_constructible_v<std::span<int, 3>, const volatile int (&)[3]>"
        );;
        const _: () = assert!(
            true,
            "!std::is_constructible_v<std::span<const int, 3>, volatile int (&)[3]>"
        );;
        const _: () = assert!(
            true,
            "!std::is_constructible_v<std::span<const int, 3>, const volatile int (&)[3]>"
        );;
        const _: () = assert!(
            true,
            "!std::is_constructible_v<std::span<volatile int, 3>, const int (&)[3]>"
        );;
        const _: () = assert!(
            true,
            "!std::is_constructible_v<std::span<volatile int, 3>, const volatile int (&)[3]>"
        );;
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
