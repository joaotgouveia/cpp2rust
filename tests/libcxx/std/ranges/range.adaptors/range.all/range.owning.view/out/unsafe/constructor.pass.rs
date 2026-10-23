extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct DefaultConstructible {
    pub i: (),
}
impl DefaultConstructible {
    pub unsafe fn new(mut j: Option<()>) -> Self {
        let mut j: () = j.unwrap_or_else(|| unsafe { 42_() });
        let mut this = Self { i: j };
        this
    }
}
impl Default for DefaultConstructible {
    fn default() -> Self {
        unsafe { DefaultConstructible::new(None) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NotDefaultConstructible {
    pub i: (),
}
impl NotDefaultConstructible {
    pub unsafe fn new(mut j: ()) -> Self {
        let mut this = Self { i: j };
        this
    }
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, Default)]
pub struct MoveChecker {
    pub i: (),
}
impl MoveChecker {
    pub unsafe fn new(mut j: ()) -> Self {
        let mut this = Self { i: j };
        this
    }
    pub unsafe fn move_from(v: *mut MoveChecker) -> Self {
        let mut this = Self {
            i: unimplemented!(),
        };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NoexceptChecker {}
pub unsafe fn test_0() -> () {
    {
        const _: () = assert!(true, "std::is_constructible_v<OwningView>");;
        const _: () = assert!(true, "std::default_initializable<OwningView>");;
        const _: () = assert!(true, "std::movable<OwningView>");;
        const _: () = assert!(true, "std::is_trivially_move_constructible_v<OwningView>");;
        const _: () = assert!(true, "std::is_trivially_move_assignable_v<OwningView>");;
        const _: () = assert!(true, "!std::is_copy_constructible_v<OwningView>");;
        const _: () = assert!(true, "!std::is_copy_assignable_v<OwningView>");;
        const _: () = assert!(true, "!std::is_constructible_v<OwningView, int>");;
        const _: () = assert!(
            true,
            "!std::is_constructible_v<OwningView, DefaultConstructible&>"
        );;
        const _: () = assert!(
            true,
            "std::is_constructible_v<OwningView, DefaultConstructible&&>"
        );;
        const _: () = assert!(true, "!std::is_convertible_v<int, OwningView>");;
        const _: () = assert!(
            true,
            "std::is_convertible_v<DefaultConstructible&&, OwningView>"
        );;
        {
            let mut ov: () = unimplemented!();
            unimplemented!();
        }
        {
            let mut ov: () = unimplemented!();
            unimplemented!();
        }
    }
    {
        const _: () = assert!(true, "!std::is_constructible_v<OwningView>");;
        const _: () = assert!(!(false), "!std::default_initializable<OwningView>");;
        const _: () = assert!(true, "std::movable<OwningView>");;
        const _: () = assert!(true, "std::is_trivially_move_constructible_v<OwningView>");;
        const _: () = assert!(true, "std::is_trivially_move_assignable_v<OwningView>");;
        const _: () = assert!(true, "!std::is_copy_constructible_v<OwningView>");;
        const _: () = assert!(true, "!std::is_copy_assignable_v<OwningView>");;
        const _: () = assert!(true, "!std::is_constructible_v<OwningView, int>");;
        const _: () = assert!(
            true,
            "!std::is_constructible_v<OwningView, NotDefaultConstructible&>"
        );;
        const _: () = assert!(
            true,
            "std::is_constructible_v<OwningView, NotDefaultConstructible&&>"
        );;
        const _: () = assert!(true, "!std::is_convertible_v<int, OwningView>");;
        const _: () = assert!(
            true,
            "std::is_convertible_v<NotDefaultConstructible&&, OwningView>"
        );;
        {
            let mut ov: () = unimplemented!();
            unimplemented!();
        }
    }
    {
        const _: () = assert!(true, "!std::is_constructible_v<OwningView>");;
        const _: () = assert!(!(false), "!std::default_initializable<OwningView>");;
        const _: () = assert!(true, "std::movable<OwningView>");;
        const _: () = assert!(true, "!std::is_trivially_move_constructible_v<OwningView>");;
        const _: () = assert!(true, "!std::is_trivially_move_assignable_v<OwningView>");;
        const _: () = assert!(true, "!std::is_copy_constructible_v<OwningView>");;
        const _: () = assert!(true, "!std::is_copy_assignable_v<OwningView>");;
        const _: () = assert!(true, "!std::is_constructible_v<OwningView, int>");;
        const _: () = assert!(true, "!std::is_constructible_v<OwningView, MoveChecker&>");;
        const _: () = assert!(true, "std::is_constructible_v<OwningView, MoveChecker&&>");;
        const _: () = assert!(true, "!std::is_convertible_v<int, OwningView>");;
        const _: () = assert!(true, "std::is_convertible_v<MoveChecker&&, OwningView>");;
        {
            let mut m: MoveChecker = MoveChecker::new({ 42_() });
            let mut ov: () = unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        const _: () = assert!(true, "std::is_nothrow_constructible_v<NoexceptChecker>");;
        const _: () = assert!(
            true,
            "std::is_nothrow_constructible_v<std::ranges::owning_view<NoexceptChecker>>"
        );;
        const _: () = assert!(
            true,
            "!std::is_nothrow_constructible_v<DefaultConstructible>"
        );;
        const _: () = assert!(
            true,
            "!std::is_nothrow_constructible_v<std::ranges::owning_view<DefaultConstructible>>"
        );;
        const _: () = assert!(
            true,
            "std::is_nothrow_move_constructible_v<NoexceptChecker>"
        );;
        const _: () = assert!(
            true,
            "std::is_nothrow_move_constructible_v<std::ranges::owning_view<NoexceptChecker>>"
        );;
        const _: () = assert!(true, "!std::is_nothrow_move_constructible_v<MoveChecker>");;
        const _: () = assert!(
            true,
            "!std::is_nothrow_move_constructible_v<std::ranges::owning_view<MoveChecker>>"
        );;
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
    (unsafe { test_0() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
