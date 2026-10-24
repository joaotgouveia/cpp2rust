extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NonConstSwappable {}
pub unsafe fn swap_0(lhs: *const ConstSwappable, rhs: *const ConstSwappable) {
    unimplemented!();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ConstSwappable {
    pub i: (),
}
const _: () = assert!(
    true,
    "std::is_swappable_v<const std::pair<ConstSwappable, ConstSwappable>>"
);
const _: () = assert!(
    true,
    "!std::is_swappable_v<const std::pair<NonConstSwappable, ConstSwappable>>"
);
const _: () = assert!(
    true,
    "!std::is_swappable_v<const std::pair<ConstSwappable, NonConstSwappable>>"
);
const _: () = assert!(
    true,
    "!std::is_swappable_v<const std::pair<NonConstSwappable, NonConstSwappable>>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct SwapMayThrow_false_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct SwapMayThrow_true_ {}
const _: () = assert!(
    true,
    "NonMemberSwapNoexcept<const std::pair<SwapMayThrow<false>, SwapMayThrow<false>>>"
);
const _: () = assert!(
    !(false),
    "!NonMemberSwapNoexcept<const std::pair<SwapMayThrow<true>, SwapMayThrow<false>>>"
);
const _: () = assert!(
    !(false),
    "!NonMemberSwapNoexcept<const std::pair<SwapMayThrow<false>, SwapMayThrow<true>>>"
);
const _: () = assert!(
    !(false),
    "!NonMemberSwapNoexcept<const std::pair<SwapMayThrow<true>, SwapMayThrow<true>>>"
);
pub unsafe fn test_1() -> () {
    {
        let p1: () = unimplemented!();
        let p2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut i1: () = 0_();
        let mut i2: () = 1_();
        let mut i3: () = 2_();
        let mut i4: () = 3_();
        let p1: () = std_pair_int_ref__int_ref_::new_1({ &mut i1 }, { &mut i2 });
        let p2: () = std_pair_int_ref__int_ref_::new_1({ &mut i3 }, { &mut i4 });
        unimplemented!();
        unimplemented!();
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
    (unsafe { test_1() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
