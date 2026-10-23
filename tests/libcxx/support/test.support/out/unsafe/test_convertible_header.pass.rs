extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_convertible_imp_0(mut _a0: ()) -> () {
    return true;
}
pub unsafe fn test_convertible_imp_1(mut _a0: ()) -> () {
    return true;
}
pub unsafe fn test_convertible_imp_2(mut _a0: ()) -> () {
    return true;
}
pub unsafe fn test_convertible_imp_3(mut _a0: ()) -> () {
    return true;
}
pub unsafe fn test_convertible_imp_4(mut _a0: ()) -> () {
    return true;
}
pub unsafe fn test_convertible_imp_5(mut _a0: ()) -> () {
    return false;
}
pub unsafe fn test_convertible_imp_6(mut _a0: ()) -> () {
    return false;
}
pub unsafe fn test_convertible_imp_7(mut _a0: ()) -> () {
    return false;
}
pub unsafe fn test_convertible_imp_8(mut _a0: ()) -> () {
    return false;
}
pub unsafe fn test_convertible_imp_9(mut _a0: ()) -> () {
    return false;
}
pub unsafe fn test_convertible_10() -> () {
    return (unsafe { test_convertible_imp_0(0_()) });
}
pub unsafe fn test_convertible_11() -> () {
    return (unsafe { test_convertible_imp_5(0_()) });
}
pub unsafe fn test_convertible_12() -> () {
    return (unsafe { test_convertible_imp_1(0_()) });
}
pub unsafe fn test_convertible_13() -> () {
    return (unsafe { test_convertible_imp_6(0_()) });
}
pub unsafe fn test_convertible_14() -> () {
    return (unsafe { test_convertible_imp_2(0_()) });
}
pub unsafe fn test_convertible_15() -> () {
    return (unsafe { test_convertible_imp_7(0_()) });
}
pub unsafe fn test_convertible_16() -> () {
    return (unsafe { test_convertible_imp_3(0_()) });
}
pub unsafe fn test_convertible_17() -> () {
    return (unsafe { test_convertible_imp_8(0_()) });
}
pub unsafe fn test_convertible_18() -> () {
    return (unsafe { test_convertible_imp_4(0_()) });
}
pub unsafe fn test_convertible_19() -> () {
    return (unsafe { test_convertible_imp_9(0_()) });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct ImplicitDefault {}
impl ImplicitDefault {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        this
    }
}
impl Default for ImplicitDefault {
    fn default() -> Self {
        unsafe { ImplicitDefault::new() }
    }
}
const _: () = assert!(true, "test_convertible<ImplicitDefault>()");
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct ExplicitDefault {}
impl ExplicitDefault {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        this
    }
}
impl Default for ExplicitDefault {
    fn default() -> Self {
        unsafe { ExplicitDefault::new() }
    }
}
const _: () = assert!(true, "!test_convertible<ExplicitDefault>()");
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ImplicitInt {}
impl ImplicitInt {
    pub unsafe fn new(mut _a0: ()) -> Self {
        let mut this = Self {};
        this
    }
}
const _: () = assert!(true, "test_convertible<ImplicitInt, int>()");
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ExplicitInt {}
impl ExplicitInt {
    pub unsafe fn new(mut _a0: ()) -> Self {
        let mut this = Self {};
        this
    }
}
const _: () = assert!(true, "!test_convertible<ExplicitInt, int>()");
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe, Default)]
pub struct ImplicitCopy {}
impl ImplicitCopy {
    pub unsafe fn copy_from(_a0: *const ImplicitCopy) -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn copy_assign(&mut self, _a0: *const ImplicitCopy) -> *mut ImplicitCopy {
        return &mut (*(self as *mut ImplicitCopy));
    }
}
impl Clone for ImplicitCopy {
    fn clone(&self) -> Self {
        unsafe { ImplicitCopy::copy_from(self as *const ImplicitCopy) }
    }
}
const _: () = assert!(true, "test_convertible<ImplicitCopy, ImplicitCopy>()");
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe, Default)]
pub struct ExplicitCopy {}
impl ExplicitCopy {
    pub unsafe fn copy_from(_a0: *const ExplicitCopy) -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn copy_assign(&mut self, _a0: *const ExplicitCopy) -> *mut ExplicitCopy {
        return &mut (*(self as *mut ExplicitCopy));
    }
}
impl Clone for ExplicitCopy {
    fn clone(&self) -> Self {
        unsafe { ExplicitCopy::copy_from(self as *const ExplicitCopy) }
    }
}
const _: () = assert!(true, "!test_convertible<ExplicitCopy, ExplicitCopy>()");
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, Default)]
pub struct ImplicitMove {}
impl ImplicitMove {
    pub unsafe fn move_from(_a0: *mut ImplicitMove) -> Self {
        let mut this = Self {};
        this
    }
}
const _: () = assert!(true, "test_convertible<ImplicitMove, ImplicitMove>()");
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, Default)]
pub struct ExplicitMove {}
impl ExplicitMove {
    pub unsafe fn move_from(_a0: *mut ExplicitMove) -> Self {
        let mut this = Self {};
        this
    }
}
const _: () = assert!(true, "!test_convertible<ExplicitMove, ExplicitMove>()");
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ImplicitArgs {}
impl ImplicitArgs {
    pub unsafe fn new(mut _a0: (), mut _a1: (), mut _a2: ()) -> Self {
        let mut this = Self {};
        this
    }
}
const _: () = assert!(true, "test_convertible<ImplicitArgs, int, int, int>()");
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ExplicitArgs {}
impl ExplicitArgs {
    pub unsafe fn new(mut _a0: (), mut _a1: (), mut _a2: ()) -> Self {
        let mut this = Self {};
        this
    }
}
const _: () = assert!(true, "!test_convertible<ExplicitArgs, int, int, int>()");
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
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
