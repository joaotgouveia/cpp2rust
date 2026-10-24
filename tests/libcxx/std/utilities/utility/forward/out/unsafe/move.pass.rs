extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe)]
pub struct move_only {}
impl move_only {
    pub unsafe fn move_from(_a0: *mut move_only) -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut move_only) -> *mut move_only {
        return &mut (*(self as *mut move_only));
    }
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        this
    }
}
impl Default for move_only {
    fn default() -> Self {
        unsafe { move_only::new() }
    }
}
pub unsafe fn source_0() -> move_only {
    return move_only::new();
}
pub unsafe fn csource_1() -> move_only {
    return move_only::new();
}
pub unsafe fn test_2(mut _a0: move_only) {}
pub static mut x_3: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 42_() });
pub static mut cx_4: std::cell::LazyCell<*const ()> =
    std::cell::LazyCell::new(|| unsafe { &(*std::cell::LazyCell::force_mut(&mut *&raw mut x_3)) });
pub static mut copy_ctor_5: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut move_ctor_6: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe)]
pub struct A {}
impl A {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn copy_from(_a0: *const A) -> Self {
        let mut this = Self {};
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copy_ctor_5)).prefix_inc();
        this
    }
    pub unsafe fn move_from(_a0: *mut A) -> Self {
        let mut this = Self {};
        (*std::cell::LazyCell::force_mut(&mut *&raw mut move_ctor_6)).prefix_inc();
        this
    }
}
impl Clone for A {
    fn clone(&self) -> Self {
        unsafe { A::copy_from(self as *const A) }
    }
}
impl Default for A {
    fn default() -> Self {
        unsafe { A::new() }
    }
}
pub unsafe fn test_constexpr_move_7() -> () {
    let mut y: () = 42_();
    let cy: () = y;
    return ((((*unimplemented!()) == (42_())) && ((*unimplemented!()) == (42_())))
        && ((*unimplemented!()) == (42_())))
        && ((*unimplemented!()) == (42_()));
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
        const _: () = assert!(true, "std::is_same<decltype(std::move(x)), int&&>::value");;
        const _: () = assert!(true, "");;
        const _: () = assert!(
            true,
            "std::is_same<decltype(std::move(cx)), const int&&>::value"
        );;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "std::is_same<decltype(std::move(42)), int&&>::value");;
        const _: () = assert!(true, "");;
        const _: () = assert!(
            true,
            "std::is_same<decltype(std::move(get<const int&&>())), const int&&>::value"
        );;
        const _: () = assert!(true, "");;
    }
    {
        let mut a: A = A::new();
        let ca: A = A::new();
        unimplemented!();
        unimplemented!();
        let mut a2: A = A::copy_from({ &a });
        &(a2);
        unimplemented!();
        unimplemented!();
        let mut a3: A = A::move_from({ &mut (*unimplemented!()) });
        &(a3);
        unimplemented!();
        unimplemented!();
        let mut a4: A = A::copy_from({ &ca });
        &(a4);
        unimplemented!();
        unimplemented!();
        let mut a5: A = A::copy_from({ &(*unimplemented!()) });
        &(a5);
        unimplemented!();
        unimplemented!();
    }
    {
        let mut mo: move_only = move_only::new();
        (unsafe { test_2(move_only::move_from({ &mut (*unimplemented!()) })) });
        (unsafe { test_2((unsafe { source_0() })) });
    }
    {
        let y: () = 42_();
        const _: () = assert!(true, "std::move(y) == 42");;
        const _: () = assert!(true, "test_constexpr_move()");;
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const x_3);
    std::cell::LazyCell::force(&*&raw const cx_4);
    std::cell::LazyCell::force(&*&raw const copy_ctor_5);
    std::cell::LazyCell::force(&*&raw const move_ctor_6);
}
