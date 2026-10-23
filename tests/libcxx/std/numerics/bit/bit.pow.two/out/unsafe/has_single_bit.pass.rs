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
pub struct test_macros_detail_is_same_bool__bool_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct A {}
pub type E1 = ();
pub const E1_rEd: E1 = 0;
pub type E2 = ();
pub const E2_red: E2 = 0;
pub unsafe fn test_1() -> () {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut max: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {}
    return true;
}
pub unsafe fn test_2() -> () {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut max: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {}
    return true;
}
pub unsafe fn test_3() -> () {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut max: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {}
    return true;
}
pub unsafe fn test_4() -> () {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut max: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {}
    return true;
}
pub unsafe fn test_5() -> () {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut max: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {}
    return true;
}
pub unsafe fn test_6() -> () {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut max: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        let mut val: () = ((1_()) << (32_()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        val <<= 60_();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut x: () = ((1_()) << (63_()));
        let mut y: () = ((1_()) << (64_()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    return true;
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
    {
        let mut lambda: FnPtr<Generic> = lambda_unsafe!(Generic, {});
        const _: () = assert!(true, "!std::is_invocable_v<L, signed char>");;
        const _: () = assert!(true, "!std::is_invocable_v<L, short>");;
        const _: () = assert!(true, "!std::is_invocable_v<L, int>");;
        const _: () = assert!(true, "!std::is_invocable_v<L, long>");;
        const _: () = assert!(true, "!std::is_invocable_v<L, long long>");;
        const _: () = assert!(true, "!std::is_invocable_v<L, __int128_t>");;
        const _: () = assert!(true, "!std::is_invocable_v<L, std::int8_t>");;
        const _: () = assert!(true, "!std::is_invocable_v<L, std::int16_t>");;
        const _: () = assert!(true, "!std::is_invocable_v<L, std::int32_t>");;
        const _: () = assert!(true, "!std::is_invocable_v<L, std::int64_t>");;
        const _: () = assert!(true, "!std::is_invocable_v<L, std::intmax_t>");;
        const _: () = assert!(true, "!std::is_invocable_v<L, std::intptr_t>");;
        const _: () = assert!(true, "!std::is_invocable_v<L, std::ptrdiff_t>");;
        const _: () = assert!(true, "!std::is_invocable_v<L, bool>");;
        const _: () = assert!(true, "!std::is_invocable_v<L, char>");;
        const _: () = assert!(true, "!std::is_invocable_v<L, wchar_t>");;
        const _: () = assert!(true, "!std::is_invocable_v<L, char8_t>");;
        const _: () = assert!(true, "!std::is_invocable_v<L, char16_t>");;
        const _: () = assert!(true, "!std::is_invocable_v<L, char32_t>");;
        const _: () = assert!(true, "!std::is_invocable_v<L, A>");;
        const _: () = assert!(true, "!std::is_invocable_v<L, A*>");;
        const _: () = assert!(true, "!std::is_invocable_v<L, E1>");;
        const _: () = assert!(true, "!std::is_invocable_v<L, E2>");;
    }
    const _: () = assert!(true, "test<unsigned char>()");;
    const _: () = assert!(true, "test<unsigned short>()");;
    const _: () = assert!(true, "test<unsigned int>()");;
    const _: () = assert!(true, "test<unsigned long>()");;
    const _: () = assert!(true, "test<unsigned long long>()");;
    const _: () = assert!(true, "test<__uint128_t>()");;
    const _: () = assert!(true, "test<std::uint8_t>()");;
    const _: () = assert!(true, "test<std::uint16_t>()");;
    const _: () = assert!(true, "test<std::uint32_t>()");;
    const _: () = assert!(true, "test<std::uint64_t>()");;
    const _: () = assert!(true, "test<std::uintmax_t>()");;
    const _: () = assert!(true, "test<std::uintptr_t>()");;
    const _: () = assert!(true, "test<std::size_t>()");;
    (unsafe { test_1() });
    (unsafe { test_2() });
    (unsafe { test_3() });
    (unsafe { test_4() });
    (unsafe { test_5() });
    (unsafe { test_6() });
    (unsafe { test_1() });
    (unsafe { test_2() });
    (unsafe { test_3() });
    (unsafe { test_4() });
    (unsafe { test_4() });
    (unsafe { test_4() });
    (unsafe { test_4() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
