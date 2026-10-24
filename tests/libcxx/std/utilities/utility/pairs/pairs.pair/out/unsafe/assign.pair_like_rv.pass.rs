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
            let mut a: () = unimplemented!();
            let mut p: () = unimplemented!();
            let result: *mut () = &mut (*unimplemented!());
            unimplemented!();
            unimplemented!();
            unimplemented!();
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
            let mut p: () = unimplemented!();
            let result: *mut () = &mut (*unimplemented!());
            unimplemented!();
            unimplemented!();
            unimplemented!();
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
            let mut data: [(); 5] = [1_(), 2_(), 3_(), 4_(), 5_()];
            let mut a: () = unimplemented!();
            let mut p: () = unimplemented!();
            let result: *mut () = &mut ({
                p = (unsafe { a() });
                (p).clone()
            });
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let mut a: () = unimplemented!();
        let mut p: () = unimplemented!();
        let result: *mut () = &mut (*unimplemented!());
        unimplemented!();
        unimplemented!();
        unimplemented!();
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
        let mut p: () = unimplemented!();
        unimplemented!();
    }
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Assignable_1 {
    pub ptr: *mut (),
}
impl Assignable_1 {
    pub unsafe fn new_1(mut p: *mut ()) -> Self {
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
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    (unsafe { test_0() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
