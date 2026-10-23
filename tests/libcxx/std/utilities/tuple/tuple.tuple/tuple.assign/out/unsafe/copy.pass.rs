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
pub struct NonAssignable {}
#[repr(C)]
#[derive(FnPtrArg, Default)]
pub struct CopyAssignable {}
const _: () = assert!(true, "std::is_copy_assignable<CopyAssignable>::value");
#[repr(C)]
#[derive(FnPtrArg, MoveAssignUnsafe, Default)]
pub struct MoveAssignable {}
impl MoveAssignable {
    pub unsafe fn move_assign(&mut self, _a0: *mut MoveAssignable) -> *mut MoveAssignable {
        return &mut (*(self as *mut MoveAssignable));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CopyAssignUnsafe, Default)]
pub struct NothrowCopyAssignable {}
impl NothrowCopyAssignable {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const NothrowCopyAssignable,
    ) -> *mut NothrowCopyAssignable {
        return &mut (*(self as *mut NothrowCopyAssignable));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CopyAssignUnsafe, Default)]
pub struct PotentiallyThrowingCopyAssignable {}
impl PotentiallyThrowingCopyAssignable {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const PotentiallyThrowingCopyAssignable,
    ) -> *mut PotentiallyThrowingCopyAssignable {
        return &mut (*(self as *mut PotentiallyThrowingCopyAssignable));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct CopyAssignableInt {}
impl CopyAssignableInt {
    pub unsafe fn operator_assign_1(&mut self, _a0: *mut ()) -> *mut CopyAssignableInt {
        return &mut (*(self as *mut CopyAssignableInt));
    }
}
pub unsafe fn test_0() -> () {
    {
        let mut t0: () = unimplemented!();
        let mut t: () = unimplemented!();
        t = (t0).clone();
    }
    {
        let mut t0: () = unimplemented!();
        let mut t: () = unimplemented!();
        t = (t0).clone();
        unimplemented!();
    }
    {
        let mut t0: () = unimplemented!();
        let mut t: () = unimplemented!();
        t = (t0).clone();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut x: () = 42_();
        let mut y: () = 100_();
        let mut x2: () = -1_();
        let mut y2: () = 500_();
        let mut t: () = unimplemented!();
        let mut t2: () = unimplemented!();
        t = (t2).clone();
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
    (unsafe { test_0() });
    const _: () = assert!(true, "test()");;
    {
        let t0: () = unimplemented!();
        let mut t: () = unimplemented!();
        t = (t0).clone();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        const _: () = assert!(true, "!std::is_copy_assignable<T>::value");;
    }
    {
        const _: () = assert!(true, "!std::is_copy_assignable<T>::value");;
    }
    {
        const _: () = assert!(true, "std::is_copy_assignable<T>::value");;
    }
    {
        const _: () = assert!(true, "!std::is_copy_assignable<T>::value");;
    }
    {
        const _: () = assert!(true, "!std::is_assignable<T&, P>::value");;
    }
    {
        const _: () = assert!(true, "!std::is_assignable<T&, P const>::value");;
    }
    {
        const _: () = assert!(true, "!std::is_assignable<T&, P&>::value");;
    }
    {
        const _: () = assert!(true, "std::is_nothrow_copy_assignable<T>::value");;
    }
    {
        const _: () = assert!(true, "std::is_copy_assignable<T>::value");;
        const _: () = assert!(true, "!std::is_nothrow_copy_assignable<T>::value");;
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
