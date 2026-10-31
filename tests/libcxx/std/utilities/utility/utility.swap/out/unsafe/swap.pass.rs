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
pub unsafe fn can_swap_0() -> bool {
    return unimplemented!();
}
pub unsafe fn can_swap_1() -> bool {
    return unimplemented!();
}
pub unsafe fn can_swap_2() -> bool {
    return unimplemented!();
}
pub unsafe fn can_swap_3() -> bool {
    return unimplemented!();
}
pub unsafe fn can_swap_4() -> bool {
    return unimplemented!();
}
pub unsafe fn test_swap_constexpr_5() -> bool {
    let mut i: i32 = 1;
    let mut j: i32 = 2;
    cpp_swap_unsafe!(&mut i, &mut j);
    return ((i) == (2)) && ((j) == (1));
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
    {
        let mut i: i32 = 1;
        let mut j: i32 = 2;
        cpp_swap_unsafe!(&mut i, &mut j);
        assert!(((i) == (2)));
        assert!(((j) == (1)));
    }
    {
        let mut i: Option<Box<i32>> = Some(Box::from_raw((Box::leak(Box::new(1)) as *mut i32)));
        let mut j: Option<Box<i32>> = Some(Box::from_raw((Box::leak(Box::new(2)) as *mut i32)));
        unimplemented!();
        assert!(
            ((*i.as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (2))
        );
        assert!(
            ((*j.as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
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
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
