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
            let mut a: Vec<i32> = Vec::from([1, 2]);
            let mut p: (i32, i32) = unimplemented!();
            let result: *mut (i32, i32) = &mut (*unimplemented!());
            assert!(((result) == (&mut p as *mut (i32, i32))));
            assert!(((p.0) == (1)));
            assert!(((p.1) == (2)));
            const _: () = assert!(
                true,
                "!std::is_assignable_v<std::pair<int, int>&, std::array<int, 1>>"
            );;
            const _: () = assert!(
                true,
                "std::is_assignable_v<std::pair<int, int>&, std::array<int, 2>>"
            );;
            const _: () = assert!(
                true,
                "!std::is_assignable_v<std::pair<int, int>&, std::array<int, 3>>"
            );;
        }
        {
            let mut a: () = unimplemented!();
            let mut p: (i32, i32) = unimplemented!();
            let result: *mut (i32, i32) = &mut (*unimplemented!());
            assert!(((result) == (&mut p as *mut (i32, i32))));
            assert!(((p.0) == (1)));
            assert!(((p.1) == (2)));
            const _: () = assert!(
                true,
                "!std::is_assignable_v<std::pair<int, int>&, std::tuple<int>>"
            );;
            const _: () = assert!(
                true,
                "std::is_assignable_v<std::pair<int, int>&, std::tuple<int, int>>"
            );;
            const _: () = assert!(
                true,
                "!std::is_assignable_v<std::pair<int, int>&, std::tuple<int, int, int>>"
            );;
        }
        {
            let mut data: [i32; 5] = [1, 2, 3, 4, 5];
            let mut a: () = unimplemented!();
            let mut p: (Assignable_1, Assignable_1) = unimplemented!();
            let result: *mut (Assignable_1, Assignable_1) = &mut (*p = (unsafe { a() }));
            assert!(((result) == (&mut p as *mut (Assignable_1, Assignable_1))));
            assert!(((p.0.ptr) == (data.as_mut_ptr())));
            assert!(((p.1.ptr) == (data.as_mut_ptr().offset((5) as isize))));
        }
    }
    {
        let mut a: () = unimplemented!();
        let mut p: (i64, Vec<libc::c_char>) = unimplemented!();
        let result: *mut (i64, Vec<libc::c_char>) = &mut (*unimplemented!());
        assert!(((result) == (&mut p as *mut (i64, Vec<libc::c_char>))));
        assert!(((p.0) == (34_i64)));
        assert!(unimplemented!());
        const _: () = assert!(
            true,
            "!std::is_assignable_v<std::pair<long, std::string>&, std::tuple<char*, std::string>>"
        );;
        const _: () = assert!(
            true,
            "!std::is_assignable_v<std::pair<long, std::string>&, std::tuple<long, void*>>"
        );;
        const _: () = assert!(
            true,
            "std::is_assignable_v<std::pair<long, std::string>&, std::tuple<long, std::string>>"
        );;
    }
    {
        let mut a: () = unimplemented!();
        let mut p: (NoCopy_2, NoCopy_2) = unimplemented!();
        unimplemented!();
    }
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe)]
#[cpp_from(*mut i32 , new_1, value)]
pub struct Assignable_1 {
    pub ptr: *mut i32,
}
impl Assignable_1 {
    pub unsafe fn new_1(mut p: *mut i32) -> Self {
        let mut this = Self { ptr: p };
        this
    }
}
impl Default for Assignable_1 {
    fn default() -> Self {
        unsafe {
            Assignable_1 {
                ptr: std::ptr::null_mut(),
            }
        }
    }
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct NoCopy_2 {}
impl NoCopy_2 {
    pub unsafe fn move_from(_a0: *mut NoCopy_2) -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut NoCopy_2) -> *mut NoCopy_2 {
        return &mut (*(self as *mut NoCopy_2));
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
