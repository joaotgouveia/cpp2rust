extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_0() -> bool {
    {
        {
            let mut x: i32 = 91;
            let mut y: i32 = 92;
            let mut a: Vec<i32> = Vec::from([1, 2]);
            let p: (*mut i32, *mut i32) = unimplemented!();
            let result: *const (*mut i32, *mut i32) = &(*unimplemented!());
            assert!(((result) == (&p as *const (*mut i32, *mut i32))));
            assert!(((x) == (1)));
            assert!(((y) == (2)));
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
            let mut x: i32 = 91;
            let mut y: i32 = 92;
            let mut a: () = unimplemented!();
            let p: (*mut i32, *mut i32) = unimplemented!();
            let result: *const (*mut i32, *mut i32) = &(*unimplemented!());
            assert!(((result) == (&p as *const (*mut i32, *mut i32))));
            assert!(((x) == (1)));
            assert!(((y) == (2)));
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
            let mut data: [i32; 5] = [1, 2, 3, 4, 5];
            let mut a: () = unimplemented!();
            let p: (ConstAssignable_1, ConstAssignable_1) = unimplemented!();
            let result: *const (ConstAssignable_1, ConstAssignable_1) = &(*unimplemented!());
            assert!(((result) == (&p as *const (ConstAssignable_1, ConstAssignable_1))));
            assert!(((p.0.ptr) == (data.as_mut_ptr())));
            assert!(((p.1.ptr) == (data.as_mut_ptr().offset((5) as isize))));
        }
    }
    {
        let mut a: () = unimplemented!();
        let p: (ConstAssignable_2, ConstAssignable_2) = unimplemented!();
        let result: *const (ConstAssignable_2, ConstAssignable_2) = &(*unimplemented!());
        assert!(((result) == (&p as *const (ConstAssignable_2, ConstAssignable_2))));
        assert!(((p.0.val) == (1)));
        assert!(((p.1.val) == (2)));
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
        let p: (NoCopy_3, NoCopy_3) = unimplemented!();
        unimplemented!();
    }
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe)]
#[cpp_from(*mut i32 , new_1, value)]
pub struct ConstAssignable_1 {
    pub ptr: *mut i32,
}
impl ConstAssignable_1 {
    pub unsafe fn new_1(mut p: *mut i32) -> Self {
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
    pub val: i32,
}
impl ConstAssignable_2 {
    pub unsafe fn operator_assign_2(&self, mut v: i32) -> *const ConstAssignable_2 {
        (*(&raw const self.val).cast_mut()) = v;
        return &(*(self as *const ConstAssignable_2));
    }
}
impl Default for ConstAssignable_2 {
    fn default() -> Self {
        unsafe { ConstAssignable_2 { val: 0 } }
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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    (unsafe { test_0() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
