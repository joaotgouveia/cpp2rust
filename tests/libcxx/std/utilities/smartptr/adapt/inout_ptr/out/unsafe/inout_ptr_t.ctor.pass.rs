extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_convertible_imp_0(mut _a0: ()) -> () {
    return false;
}
pub unsafe fn test_convertible_imp_1(mut _a0: ()) -> () {
    return false;
}
pub unsafe fn test_convertible_2() -> () {
    return (unsafe { test_convertible_imp_0(0_()) });
}
pub unsafe fn test_convertible_3() -> () {
    return (unsafe { test_convertible_imp_1(0_()) });
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe)]
pub struct MoveOnlyDeleter_int_ {
    pub wasMoveInitialized: (),
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
    pub unsafe fn operator_call(&self, mut p: *mut ()) {
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
    pub ptr: (),
}
impl ConstructiblePtr_int_ {
    pub unsafe fn move_from(_a0: *mut ConstructiblePtr_int_) -> Self {
        let mut this = Self {
            ptr: unimplemented!(),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut ConstructiblePtr_int_,
    ) -> *mut ConstructiblePtr_int_ {
        unimplemented!();
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
    pub ptr: (),
}
impl ResettablePtr_int_ {
    pub unsafe fn move_from(_a0: *mut ResettablePtr_int_) -> Self {
        let mut this = Self {
            ptr: unimplemented!(),
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut ResettablePtr_int_) -> *mut ResettablePtr_int_ {
        unimplemented!();
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
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    {
        let mut uPtr: () = unimplemented!();
        unimplemented!();
        const _: () = assert!(
            true,
            "!test_convertible<std::inout_ptr_t<std::unique_ptr<int>, int*>>()"
        );;
        unimplemented!();
    }
    {
        let mut deleter: FnPtr<Generic> = lambda_unsafe!(Generic, {}, |mut p: *mut ()| {
            {
                let __p = p;
                if !__p.is_null() {
                    ::std::mem::drop(Box::from_raw(__p))
                }
            };
        });
        let mut uPtr: () = unimplemented!();
        unimplemented!();
        const _: () = assert!(
            true,
            "!test_convertible<std::inout_ptr_t<std::unique_ptr<int, decltype(deleter)>, int*>>()"
        );;
        unimplemented!();
    }
    {
        let mut uPtr: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
