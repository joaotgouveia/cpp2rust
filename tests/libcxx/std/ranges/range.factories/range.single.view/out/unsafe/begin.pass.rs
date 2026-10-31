extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type anon_0 = u32;
pub const anon_0_value: anon_0 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_int_ptr__int_ptr_ {}
pub type anon_1 = u32;
pub const anon_1_value: anon_1 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_int_ptr__const_int_ptr_ {}
pub type anon_2 = u32;
pub const anon_2_value: anon_2 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_Empty_ptr__Empty_ptr_ {}
pub type anon_3 = u32;
pub const anon_3_value: anon_3 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_Empty_ptr__const_Empty_ptr_ {}
pub type anon_4 = u32;
pub const anon_4_value: anon_4 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_BigType_ptr__BigType_ptr_ {}
pub type anon_5 = u32;
pub const anon_5_value: anon_5 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_BigType_ptr__const_BigType_ptr_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Empty {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct BigType {
    pub buffer: [libc::c_char; 64],
}
impl Default for BigType {
    fn default() -> Self {
        unsafe {
            BigType {
                buffer: [
                    (10 as libc::c_char),
                    (0 as libc::c_char),
                    (0 as libc::c_char),
                    (0 as libc::c_char),
                    (0 as libc::c_char),
                    (0 as libc::c_char),
                    (0 as libc::c_char),
                    (0 as libc::c_char),
                    (0 as libc::c_char),
                    (0 as libc::c_char),
                    (0 as libc::c_char),
                    (0 as libc::c_char),
                    (0 as libc::c_char),
                    (0 as libc::c_char),
                    (0 as libc::c_char),
                    (0 as libc::c_char),
                    (0 as libc::c_char),
                    (0 as libc::c_char),
                    (0 as libc::c_char),
                    (0 as libc::c_char),
                    (0 as libc::c_char),
                    (0 as libc::c_char),
                    (0 as libc::c_char),
                    (0 as libc::c_char),
                    (0 as libc::c_char),
                    (0 as libc::c_char),
                    (0 as libc::c_char),
                    (0 as libc::c_char),
                    (0 as libc::c_char),
                    (0 as libc::c_char),
                    (0 as libc::c_char),
                    (0 as libc::c_char),
                    (0 as libc::c_char),
                    (0 as libc::c_char),
                    (0 as libc::c_char),
                    (0 as libc::c_char),
                    (0 as libc::c_char),
                    (0 as libc::c_char),
                    (0 as libc::c_char),
                    (0 as libc::c_char),
                    (0 as libc::c_char),
                    (0 as libc::c_char),
                    (0 as libc::c_char),
                    (0 as libc::c_char),
                    (0 as libc::c_char),
                    (0 as libc::c_char),
                    (0 as libc::c_char),
                    (0 as libc::c_char),
                    (0 as libc::c_char),
                    (0 as libc::c_char),
                    (0 as libc::c_char),
                    (0 as libc::c_char),
                    (0 as libc::c_char),
                    (0 as libc::c_char),
                    (0 as libc::c_char),
                    (0 as libc::c_char),
                    (0 as libc::c_char),
                    (0 as libc::c_char),
                    (0 as libc::c_char),
                    (0 as libc::c_char),
                    (0 as libc::c_char),
                    (0 as libc::c_char),
                    (0 as libc::c_char),
                    (0 as libc::c_char),
                ],
            }
        }
    }
}
pub unsafe fn test_6() -> bool {
    {
        let mut sv: () = unimplemented!();
        assert!(((*unimplemented!()) == (42)));
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "noexcept(sv.begin())");;
    }
    {
        let sv: () = unimplemented!();
        assert!(((*unimplemented!()) == (42)));
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "noexcept(sv.begin())");;
    }
    {
        let mut sv: () = unimplemented!();
        assert!(!((unimplemented!()).is_null()));
        const _: () = assert!(true, "");;
    }
    {
        let sv: () = unimplemented!();
        assert!(!((unimplemented!()).is_null()));
        const _: () = assert!(true, "");;
    }
    {
        let mut sv: () = unimplemented!();
        assert!((((*unimplemented!()).buffer[(0) as usize] as i32) == (10)));
        const _: () = assert!(true, "");;
    }
    {
        let sv: () = unimplemented!();
        assert!((((*unimplemented!()).buffer[(0) as usize] as i32) == (10)));
        const _: () = assert!(true, "");;
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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    (unsafe { test_6() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
