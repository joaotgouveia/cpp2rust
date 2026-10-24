extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_0() -> () {
    {
        {
            let mut x: () = 91_();
            let mut y: () = 92_();
            let mut a: () = unimplemented!();
            let p: () = std_pair_int_ref__int_ref_::new_1({ &mut x }, { &mut y });
            let result: *const () = &(*unimplemented!());
            unimplemented!();
            unimplemented!();
            unimplemented!();
            const _: () = assert!(
                true,
                "!std::is_assignable_v<std::pair<int&, int&> const&, std::array<int, 1>>"
            );;
            const _: () = assert!(
                true,
                "std::is_assignable_v<std::pair<int&, int&> const&, std::array<int, 2>>"
            );;
            const _: () = assert!(
                true,
                "!std::is_assignable_v<std::pair<int&, int&> const&, std::array<int, 3>>"
            );;
        }
        {
            let mut x: () = 91_();
            let mut y: () = 92_();
            let mut a: () = unimplemented!();
            let p: () = std_pair_int_ref__int_ref_::new_1({ &mut x }, { &mut y });
            let result: *const () = &(*unimplemented!());
            unimplemented!();
            unimplemented!();
            unimplemented!();
            const _: () = assert!(
                true,
                "!std::is_assignable_v<std::pair<int&, int&> const&, std::tuple<int>>"
            );;
            const _: () = assert!(
                true,
                "std::is_assignable_v<std::pair<int&, int&> const&, std::tuple<int, int>>"
            );;
            const _: () = assert!(
                true,
                "!std::is_assignable_v<std::pair<int&, int&> const&, std::tuple<int, int, int>>"
            );;
        }
        {
            let mut data: [(); 5] = [1_(), 2_(), 3_(), 4_(), 5_()];
            let mut a: () = unimplemented!();
            let p: () = unimplemented!();
            let result: *const () = &(*unimplemented!());
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let mut a: () = unimplemented!();
        let p: () = unimplemented!();
        let result: *const () = &(*unimplemented!());
        unimplemented!();
        unimplemented!();
        unimplemented!();
        const _: () = assert!(
            true,
            "!std::is_assignable_v<std::pair<ConstAssignable, ConstAssignable> const&, std::tuple<void*, int>>"
        );;
        const _: () = assert!(
            true,
            "!std::is_assignable_v<std::pair<ConstAssignable, ConstAssignable> const&, std::tuple<int, void*>>"
        );;
        const _: () = assert!(
            true,
            "std::is_assignable_v<std::pair<ConstAssignable, ConstAssignable> const&, std::tuple<int, int>>"
        );;
    }
    {
        let mut a: () = unimplemented!();
        let p: () = unimplemented!();
        unimplemented!();
    }
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct ConstAssignable_1 {
    pub ptr: *mut (),
}
impl ConstAssignable_1 {
    pub unsafe fn new_1(mut p: *mut ()) -> Self {
        let mut this = Self { ptr: p };
        this
    }
    pub unsafe fn operator_assign_3(
        &self,
        other: *const ConstAssignable_1,
    ) -> *const ConstAssignable_1 {
        (*(&raw const self.ptr).cast_mut()) = (*other).ptr;
        return &(*(self as *const ConstAssignable_1));
    }
}
impl Default for ConstAssignable_1 {
    fn default() -> Self {
        unsafe {
            ConstAssignable_1 {
                ptr: std::ptr::null_mut(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct ConstAssignable_2 {
    pub val: (),
}
impl ConstAssignable_2 {
    pub unsafe fn operator_assign_2(&self, mut v: ()) -> *const ConstAssignable_2 {
        (*(&raw const self.val).cast_mut()) = v;
        return &(*(self as *const ConstAssignable_2));
    }
}
impl Default for ConstAssignable_2 {
    fn default() -> Self {
        unsafe { ConstAssignable_2 { val: 0_() } }
    }
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct NoCopy_3 {}
impl NoCopy_3 {
    pub unsafe fn move_from(_a0: *mut NoCopy_3) -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn move_assign(&self, _a0: *mut NoCopy_3) -> *const NoCopy_3 {
        return &(*(self as *const NoCopy_3));
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
    (unsafe { test_0() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
