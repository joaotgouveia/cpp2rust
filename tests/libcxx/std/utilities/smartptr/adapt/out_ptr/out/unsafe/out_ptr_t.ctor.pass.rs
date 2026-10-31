extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_convertible_imp_0(mut _a0: i64) -> bool {
    return false;
}
pub unsafe fn test_convertible_imp_1(mut _a0: i64) -> bool {
    return false;
}
pub unsafe fn test_convertible_2() -> bool {
    return (unsafe { test_convertible_imp_0(0_i64) });
}
pub unsafe fn test_convertible_3() -> bool {
    return (unsafe { test_convertible_imp_1(0_i64) });
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe)]
pub struct MoveOnlyDeleter_int_ {
    pub wasMoveInitialized: bool,
}
impl MoveOnlyDeleter_int_ {
    pub unsafe fn move_from(_a0: *mut MoveOnlyDeleter_int_) -> Self {
        let mut this = Self {
            wasMoveInitialized: true,
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut MoveOnlyDeleter_int_,
    ) -> *mut MoveOnlyDeleter_int_ {
        self.wasMoveInitialized = (*_a0).wasMoveInitialized;
        return &mut (*(self as *mut MoveOnlyDeleter_int_));
    }
    pub unsafe fn operator_call(&self, mut p: *mut i32) {
        {
            let __p = p;
            if !__p.is_null() {
                ::std::mem::drop(Box::from_raw(__p))
            }
        };
    }
}
impl Default for MoveOnlyDeleter_int_ {
    fn default() -> Self {
        unsafe {
            MoveOnlyDeleter_int_ {
                wasMoveInitialized: false,
            }
        }
    }
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct ConstructiblePtr_int_ {
    pub ptr: Option<Box<i32>>,
}
impl ConstructiblePtr_int_ {
    pub unsafe fn move_from(_a0: *mut ConstructiblePtr_int_) -> Self {
        let mut this = Self {
            ptr: (*_a0).ptr.take(),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut ConstructiblePtr_int_,
    ) -> *mut ConstructiblePtr_int_ {
        self.ptr = (*_a0).ptr.take();
        return &mut (*(self as *mut ConstructiblePtr_int_));
    }
}
const _: () = assert!(true, "");
const _: () = assert!(
    true,
    "std::is_constructible_v< ConstructiblePtr<int>, int* >"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ResetArg {}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct ResettablePtr_int_ {
    pub ptr: Option<Box<i32>>,
}
impl ResettablePtr_int_ {
    pub unsafe fn move_from(_a0: *mut ResettablePtr_int_) -> Self {
        let mut this = Self {
            ptr: (*_a0).ptr.take(),
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut ResettablePtr_int_) -> *mut ResettablePtr_int_ {
        self.ptr = (*_a0).ptr.take();
        return &mut (*(self as *mut ResettablePtr_int_));
    }
}
const _: () = assert!(true, "");
const _: () = assert!(true, "std::is_constructible_v< ResettablePtr<int>, int* >");
#[repr(C)]
#[derive(FnPtrArg, Default)]
pub struct NonConstructiblePtr_int_ {}
impl NonConstructiblePtr_int_ {}
const _: () = assert!(true, "");
const _: () = assert!(
    true,
    "!std::is_constructible_v< NonConstructiblePtr<int>, int* >"
);
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
        let mut uPtr: Option<Box<i32>> = None;
        unimplemented!();
        const _: () = assert!(
            true,
            "!test_convertible<std::out_ptr_t<std::unique_ptr<int>, int*>>()"
        );;
        assert!(uPtr.is_none());
    }
    {
        let mut uPtr: Option<Box<i32>> = None;
        unimplemented!();
        const _: () = assert!(
            true,
            "!test_convertible<std::out_ptr_t<decltype(uPtr), int*, std::default_delete<int>>>()"
        );;
        assert!(uPtr.is_none());
    }
    {
        let mut uPtr: Option<Box<i32>> = None;
        unimplemented!();
        assert!(uPtr.is_none());
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
