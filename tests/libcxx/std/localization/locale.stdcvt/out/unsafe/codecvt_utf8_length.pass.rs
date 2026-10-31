extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestHelper_wchar_t__4_ {}
impl TestHelper_wchar_t__4_ {
    pub unsafe fn test() {
        {
            let mut c: () = unimplemented!();
            let mut n: [libc::c_char; 4] = [
                (-15 as libc::c_char),
                (-128 as libc::c_char),
                (-128 as libc::c_char),
                (-125 as libc::c_char),
            ];
            let mut m: ::libc::mbstate_t = unsafe { std::mem::zeroed() };
            let mut r: i32 = unimplemented!();
            assert!(((r) == (4)));
            n[(0) as usize] = (-31 as libc::c_char);
            n[(1) as usize] = (-128 as libc::c_char);
            n[(2) as usize] = (-123 as libc::c_char);
            r = unimplemented!();
            assert!(((r) == (3)));
            n[(0) as usize] = (-47 as libc::c_char);
            n[(1) as usize] = (-109 as libc::c_char);
            r = unimplemented!();
            assert!(((r) == (2)));
            n[(0) as usize] = (86 as libc::c_char);
            r = unimplemented!();
            assert!(((r) == (1)));
        }
        {
            let mut c: () = unimplemented!();
            let mut n: [libc::c_char; 4] = [
                (-15 as libc::c_char),
                (-128 as libc::c_char),
                (-128 as libc::c_char),
                (-125 as libc::c_char),
            ];
            let mut m: ::libc::mbstate_t = unsafe { std::mem::zeroed() };
            let mut r: i32 = unimplemented!();
            assert!(((r) == (0)));
            n[(0) as usize] = (-31 as libc::c_char);
            n[(1) as usize] = (-128 as libc::c_char);
            n[(2) as usize] = (-123 as libc::c_char);
            r = unimplemented!();
            assert!(((r) == (0)));
            n[(0) as usize] = (-47 as libc::c_char);
            n[(1) as usize] = (-109 as libc::c_char);
            r = unimplemented!();
            assert!(((r) == (2)));
            n[(0) as usize] = (86 as libc::c_char);
            r = unimplemented!();
            assert!(((r) == (1)));
        }
        {
            let mut c: () = unimplemented!();
            let mut n: [libc::c_char; 7] = [
                (-17 as libc::c_char),
                (-69 as libc::c_char),
                (-65 as libc::c_char),
                (-15 as libc::c_char),
                (-128 as libc::c_char),
                (-128 as libc::c_char),
                (-125 as libc::c_char),
            ];
            let mut m: ::libc::mbstate_t = unsafe { std::mem::zeroed() };
            let mut r: i32 = unimplemented!();
            assert!(((r) == (7)));
            n[(0) as usize] = (-31 as libc::c_char);
            n[(1) as usize] = (-128 as libc::c_char);
            n[(2) as usize] = (-123 as libc::c_char);
            r = unimplemented!();
            assert!(((r) == (3)));
            n[(0) as usize] = (-17 as libc::c_char);
            n[(1) as usize] = (-69 as libc::c_char);
            n[(2) as usize] = (-65 as libc::c_char);
            n[(3) as usize] = (-47 as libc::c_char);
            n[(4) as usize] = (-109 as libc::c_char);
            r = unimplemented!();
            assert!(((r) == (5)));
            n[(0) as usize] = (86 as libc::c_char);
            r = unimplemented!();
            assert!(((r) == (1)));
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestHelper_char32_t__4_ {}
impl TestHelper_char32_t__4_ {
    pub unsafe fn test() {
        {
            let mut c: () = unimplemented!();
            let mut n: [libc::c_char; 4] = [
                (-15 as libc::c_char),
                (-128 as libc::c_char),
                (-128 as libc::c_char),
                (-125 as libc::c_char),
            ];
            let mut m: ::libc::mbstate_t = unsafe { std::mem::zeroed() };
            let mut r: i32 = unimplemented!();
            assert!(((r) == (4)));
            n[(0) as usize] = (-31 as libc::c_char);
            n[(1) as usize] = (-128 as libc::c_char);
            n[(2) as usize] = (-123 as libc::c_char);
            r = unimplemented!();
            assert!(((r) == (3)));
            n[(0) as usize] = (-47 as libc::c_char);
            n[(1) as usize] = (-109 as libc::c_char);
            r = unimplemented!();
            assert!(((r) == (2)));
            n[(0) as usize] = (86 as libc::c_char);
            r = unimplemented!();
            assert!(((r) == (1)));
        }
        {
            let mut c: () = unimplemented!();
            let mut n: [libc::c_char; 4] = [
                (-15 as libc::c_char),
                (-128 as libc::c_char),
                (-128 as libc::c_char),
                (-125 as libc::c_char),
            ];
            let mut m: ::libc::mbstate_t = unsafe { std::mem::zeroed() };
            let mut r: i32 = unimplemented!();
            assert!(((r) == (0)));
            n[(0) as usize] = (-31 as libc::c_char);
            n[(1) as usize] = (-128 as libc::c_char);
            n[(2) as usize] = (-123 as libc::c_char);
            r = unimplemented!();
            assert!(((r) == (0)));
            n[(0) as usize] = (-47 as libc::c_char);
            n[(1) as usize] = (-109 as libc::c_char);
            r = unimplemented!();
            assert!(((r) == (2)));
            n[(0) as usize] = (86 as libc::c_char);
            r = unimplemented!();
            assert!(((r) == (1)));
        }
        {
            let mut c: () = unimplemented!();
            let mut n: [libc::c_char; 7] = [
                (-17 as libc::c_char),
                (-69 as libc::c_char),
                (-65 as libc::c_char),
                (-15 as libc::c_char),
                (-128 as libc::c_char),
                (-128 as libc::c_char),
                (-125 as libc::c_char),
            ];
            let mut m: ::libc::mbstate_t = unsafe { std::mem::zeroed() };
            let mut r: i32 = unimplemented!();
            assert!(((r) == (7)));
            n[(0) as usize] = (-31 as libc::c_char);
            n[(1) as usize] = (-128 as libc::c_char);
            n[(2) as usize] = (-123 as libc::c_char);
            r = unimplemented!();
            assert!(((r) == (3)));
            n[(0) as usize] = (-17 as libc::c_char);
            n[(1) as usize] = (-69 as libc::c_char);
            n[(2) as usize] = (-65 as libc::c_char);
            n[(3) as usize] = (-47 as libc::c_char);
            n[(4) as usize] = (-109 as libc::c_char);
            r = unimplemented!();
            assert!(((r) == (5)));
            n[(0) as usize] = (86 as libc::c_char);
            r = unimplemented!();
            assert!(((r) == (1)));
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestHelper_char16_t__2_ {}
impl TestHelper_char16_t__2_ {
    pub unsafe fn test() {
        {
            let mut c: () = unimplemented!();
            let mut n: [libc::c_char; 4] = [
                (-15 as libc::c_char),
                (-128 as libc::c_char),
                (-128 as libc::c_char),
                (-125 as libc::c_char),
            ];
            let mut m: ::libc::mbstate_t = unsafe { std::mem::zeroed() };
            let mut r: i32 = unimplemented!();
            assert!(((r) == (0)));
            n[(0) as usize] = (-31 as libc::c_char);
            n[(1) as usize] = (-128 as libc::c_char);
            n[(2) as usize] = (-123 as libc::c_char);
            r = unimplemented!();
            assert!(((r) == (3)));
            n[(0) as usize] = (-47 as libc::c_char);
            n[(1) as usize] = (-109 as libc::c_char);
            r = unimplemented!();
            assert!(((r) == (2)));
            n[(0) as usize] = (86 as libc::c_char);
            r = unimplemented!();
            assert!(((r) == (1)));
        }
        {
            let mut c: () = unimplemented!();
            let mut n: [libc::c_char; 4] = [
                (-15 as libc::c_char),
                (-128 as libc::c_char),
                (-128 as libc::c_char),
                (-125 as libc::c_char),
            ];
            let mut m: ::libc::mbstate_t = unsafe { std::mem::zeroed() };
            let mut r: i32 = unimplemented!();
            assert!(((r) == (0)));
            n[(0) as usize] = (-31 as libc::c_char);
            n[(1) as usize] = (-128 as libc::c_char);
            n[(2) as usize] = (-123 as libc::c_char);
            r = unimplemented!();
            assert!(((r) == (0)));
            n[(0) as usize] = (-47 as libc::c_char);
            n[(1) as usize] = (-109 as libc::c_char);
            r = unimplemented!();
            assert!(((r) == (2)));
            n[(0) as usize] = (86 as libc::c_char);
            r = unimplemented!();
            assert!(((r) == (1)));
        }
        {
            let mut c: () = unimplemented!();
            let mut n: [libc::c_char; 7] = [
                (-17 as libc::c_char),
                (-69 as libc::c_char),
                (-65 as libc::c_char),
                (-15 as libc::c_char),
                (-128 as libc::c_char),
                (-128 as libc::c_char),
                (-125 as libc::c_char),
            ];
            let mut m: ::libc::mbstate_t = unsafe { std::mem::zeroed() };
            let mut r: i32 = unimplemented!();
            assert!(((r) == (3)));
            n[(0) as usize] = (-31 as libc::c_char);
            n[(1) as usize] = (-128 as libc::c_char);
            n[(2) as usize] = (-123 as libc::c_char);
            r = unimplemented!();
            assert!(((r) == (3)));
            n[(0) as usize] = (-17 as libc::c_char);
            n[(1) as usize] = (-69 as libc::c_char);
            n[(2) as usize] = (-65 as libc::c_char);
            n[(3) as usize] = (-47 as libc::c_char);
            n[(4) as usize] = (-109 as libc::c_char);
            r = unimplemented!();
            assert!(((r) == (5)));
            n[(0) as usize] = (86 as libc::c_char);
            r = unimplemented!();
            assert!(((r) == (1)));
        }
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
    (unsafe { TestHelper_wchar_t__4_::test() });
    (unsafe { TestHelper_char32_t__4_::test() });
    (unsafe { TestHelper_char16_t__2_::test() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
