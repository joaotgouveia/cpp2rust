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
    cpp_swap_unsafe!(
        (&raw const (*lhs).i).cast_mut(),
        (&raw const (*rhs).i).cast_mut()
    );
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ConstSwappable {
    pub i: i32,
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
pub unsafe fn test_1() -> bool {
    {
        let p1: (ConstSwappable, ConstSwappable) =
            (ConstSwappable { i: 0 }, ConstSwappable { i: 1 });
        let p2: (ConstSwappable, ConstSwappable) =
            (ConstSwappable { i: 2 }, ConstSwappable { i: 3 });
        unimplemented!();
        assert!(((p1.0.i) == (2)));
        assert!(((p1.1.i) == (3)));
        assert!(((p2.0.i) == (0)));
        assert!(((p2.1.i) == (1)));
    }
    {
        let mut i1: i32 = 0;
        let mut i2: i32 = 1;
        let mut i3: i32 = 2;
        let mut i4: i32 = 3;
        let p1: (*mut i32, *mut i32) = unimplemented!();
        let p2: (*mut i32, *mut i32) = unimplemented!();
        unimplemented!();
        assert!(((*p1.0) == (2)));
        assert!(((*p1.1) == (3)));
        assert!(((*p2.0) == (0)));
        assert!(((*p2.1) == (1)));
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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    (unsafe { test_1() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
