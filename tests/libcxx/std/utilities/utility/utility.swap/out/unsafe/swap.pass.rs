extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe)]
pub struct CopyOnly {}
impl CopyOnly {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn copy_from(_a0: *const CopyOnly) -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn copy_assign(&mut self, _a0: *const CopyOnly) -> *mut CopyOnly {
        return &mut (*(self as *mut CopyOnly));
    }
}
impl Clone for CopyOnly {
    fn clone(&self) -> Self {
        unsafe { CopyOnly::copy_from(self as *const CopyOnly) }
    }
}
impl Default for CopyOnly {
    fn default() -> Self {
        unsafe { CopyOnly::new() }
    }
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe)]
pub struct MoveOnly {}
impl MoveOnly {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn move_from(_a0: *mut MoveOnly) -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut MoveOnly) -> *mut MoveOnly {
        return &mut (*(self as *mut MoveOnly));
    }
}
impl Default for MoveOnly {
    fn default() -> Self {
        unsafe { MoveOnly::new() }
    }
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe)]
pub struct NoexceptMoveOnly {}
impl NoexceptMoveOnly {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn move_from(_a0: *mut NoexceptMoveOnly) -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut NoexceptMoveOnly) -> *mut NoexceptMoveOnly {
        return &mut (*(self as *mut NoexceptMoveOnly));
    }
}
impl Default for NoexceptMoveOnly {
    fn default() -> Self {
        unsafe { NoexceptMoveOnly::new() }
    }
}
#[repr(C)]
#[derive(FnPtrArg, MoveAssignUnsafe, Default)]
pub struct NotMoveConstructible {}
impl NotMoveConstructible {
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut NotMoveConstructible,
    ) -> *mut NotMoveConstructible {
        return &mut (*(self as *mut NotMoveConstructible));
    }
}
#[repr(C)]
#[derive(FnPtrArg, Default)]
pub struct NotMoveAssignable {}
impl NotMoveAssignable {}
pub unsafe fn can_swap_0() -> () {
    return unimplemented!();
}
pub unsafe fn can_swap_1() -> () {
    return unimplemented!();
}
pub unsafe fn can_swap_2() -> () {
    return unimplemented!();
}
pub unsafe fn can_swap_3() -> () {
    return unimplemented!();
}
pub unsafe fn can_swap_4() -> () {
    return unimplemented!();
}
pub unsafe fn test_swap_constexpr_5() -> () {
    let mut i: () = 1_();
    let mut j: () = 2_();
    unimplemented!();
    return ((i) == (2_())) && ((j) == (1_()));
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
        let mut i: () = 1_();
        let mut j: () = 2_();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut i: () = unimplemented!();
        let mut j: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        const _: () = assert!(true, "can_swap<CopyOnly&>()");;
        const _: () = assert!(true, "can_swap<MoveOnly&>()");;
        const _: () = assert!(true, "can_swap<NoexceptMoveOnly&>()");;
        const _: () = assert!(true, "!can_swap<NotMoveConstructible&>()");;
        const _: () = assert!(true, "!can_swap<NotMoveAssignable&>()");;
        let mut c: CopyOnly = CopyOnly::new();
        let mut m: MoveOnly = MoveOnly::new();
        let mut nm: NoexceptMoveOnly = NoexceptMoveOnly::new();
        const _: () = assert!(true, "!noexcept(std::swap(c, c))");;
        const _: () = assert!(true, "!noexcept(std::swap(m, m))");;
        const _: () = assert!(true, "noexcept(std::swap(nm, nm))");;
    }
    const _: () = assert!(true, "test_swap_constexpr()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
