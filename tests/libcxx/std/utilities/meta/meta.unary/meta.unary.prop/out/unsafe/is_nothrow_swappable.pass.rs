extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(FnPtrArg, Default)]
pub struct MyNS_A {}
#[repr(C)]
#[derive(FnPtrArg, Default)]
pub struct MyNS_B {}
pub unsafe fn swap_0(_a0: *mut MyNS_A, _a1: *mut MyNS_A) {}
pub unsafe fn swap_1(_a0: *mut MyNS_B, _a1: *mut MyNS_B) {}
#[repr(C)]
#[derive(FnPtrArg, Default)]
pub struct MyNS_M {}
pub unsafe fn swap_2(_a0: *mut MyNS_M, _a1: *mut MyNS_M) {}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct MyNS_ThrowingMove {}
impl MyNS_ThrowingMove {
    pub unsafe fn move_from(_a0: *mut MyNS_ThrowingMove) -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut MyNS_ThrowingMove) -> *mut MyNS_ThrowingMove {
        return &mut (*(self as *mut MyNS_ThrowingMove));
    }
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
        const _: () = assert!(true, "std::is_nothrow_swappable<int>::value");;
        const _: () = assert!(true, "std::is_nothrow_swappable<int&>::value");;
        const _: () = assert!(true, "!std::is_nothrow_swappable<M>::value");;
        const _: () = assert!(true, "!std::is_nothrow_swappable<M&&>::value");;
    }
    {
        const _: () = assert!(true, "std::is_nothrow_swappable<A>::value");;
        const _: () = assert!(
            true,
            "!std::is_nothrow_swappable<B>::value\n                      && std::is_swappable<B>::value"
        );;
        const _: () = assert!(
            true,
            "!std::is_nothrow_swappable<ThrowingMove>::value\n                      && std::is_swappable<ThrowingMove>::value"
        );;
    }
    {
        const _: () = assert!(true, "!std::is_nothrow_swappable<const A>::value");;
    }
    {
        const _: () = assert!(true, "!std::is_nothrow_swappable<void>::value");;
        const _: () = assert!(true, "!std::is_nothrow_swappable<int() const>::value");;
        const _: () = assert!(
            true,
            "!std::is_nothrow_swappable<int(int, ...) const &>::value"
        );;
    }
    {
        const _: () = assert!(true, "std::is_nothrow_swappable_v<int>");;
        const _: () = assert!(true, "!std::is_nothrow_swappable_v<void>");;
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
