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
            let mut w: i32 = 262147_i32;
            let mut n: [libc::c_char; 4] = [
                (0 as libc::c_char),
                (0 as libc::c_char),
                (0 as libc::c_char),
                (0 as libc::c_char),
            ];
            let mut wp: *const i32 = std::ptr::null();
            let mut m: ::libc::mbstate_t = unsafe { std::mem::zeroed() };
            let mut np: *mut libc::c_char = std::ptr::null_mut();
            let mut r: () = unimplemented!();
            assert!(((r as i32) == (unimplemented!() as i32)));
            assert!(((wp) == (((&mut w as *mut i32).offset((1) as isize)).cast_const())));
            assert!(((np) == (n.as_mut_ptr().offset((4) as isize))));
            assert!(((n[(0) as usize] as i32) == ((-15 as libc::c_char) as i32)));
            assert!(((n[(1) as usize] as i32) == ((-128 as libc::c_char) as i32)));
            assert!(((n[(2) as usize] as i32) == ((-128 as libc::c_char) as i32)));
            assert!(((n[(3) as usize] as i32) == ((-125 as libc::c_char) as i32)));
            w = 4101_i32;
            r = unimplemented!();
            assert!(((r as i32) == (unimplemented!() as i32)));
            assert!(((wp) == (((&mut w as *mut i32).offset((1) as isize)).cast_const())));
            assert!(((np) == (n.as_mut_ptr().offset((3) as isize))));
            assert!(((n[(0) as usize] as i32) == ((-31 as libc::c_char) as i32)));
            assert!(((n[(1) as usize] as i32) == ((-128 as libc::c_char) as i32)));
            assert!(((n[(2) as usize] as i32) == ((-123 as libc::c_char) as i32)));
            assert!(((n[(3) as usize] as i32) == ((-125 as libc::c_char) as i32)));
            w = 1107_i32;
            r = unimplemented!();
            assert!(((r as i32) == (unimplemented!() as i32)));
            assert!(((wp) == (((&mut w as *mut i32).offset((1) as isize)).cast_const())));
            assert!(((np) == (n.as_mut_ptr().offset((2) as isize))));
            assert!(((n[(0) as usize] as i32) == ((-47 as libc::c_char) as i32)));
            assert!(((n[(1) as usize] as i32) == ((-109 as libc::c_char) as i32)));
            assert!(((n[(2) as usize] as i32) == ((-123 as libc::c_char) as i32)));
            assert!(((n[(3) as usize] as i32) == ((-125 as libc::c_char) as i32)));
            w = 86_i32;
            r = unimplemented!();
            assert!(((r as i32) == (unimplemented!() as i32)));
            assert!(((wp) == (((&mut w as *mut i32).offset((1) as isize)).cast_const())));
            assert!(((np) == (n.as_mut_ptr().offset((1) as isize))));
            assert!(((n[(0) as usize] as i32) == ((86 as libc::c_char) as i32)));
            assert!(((n[(1) as usize] as i32) == ((-109 as libc::c_char) as i32)));
            assert!(((n[(2) as usize] as i32) == ((-123 as libc::c_char) as i32)));
            assert!(((n[(3) as usize] as i32) == ((-125 as libc::c_char) as i32)));
        }
        {
            let mut c: () = unimplemented!();
            let mut w: i32 = 262147_i32;
            let mut n: [libc::c_char; 4] = [
                (0 as libc::c_char),
                (0 as libc::c_char),
                (0 as libc::c_char),
                (0 as libc::c_char),
            ];
            let mut wp: *const i32 = std::ptr::null();
            let mut m: ::libc::mbstate_t = unsafe { std::mem::zeroed() };
            let mut np: *mut libc::c_char = std::ptr::null_mut();
            let mut r: () = unimplemented!();
            assert!(((r as i32) == (unimplemented!() as i32)));
            assert!(((wp) == ((&mut w as *mut i32).cast_const())));
            assert!(((np) == (n.as_mut_ptr())));
            assert!(((n[(0) as usize] as i32) == ((0 as libc::c_char) as i32)));
            assert!(((n[(1) as usize] as i32) == ((0 as libc::c_char) as i32)));
            assert!(((n[(2) as usize] as i32) == ((0 as libc::c_char) as i32)));
            assert!(((n[(3) as usize] as i32) == ((0 as libc::c_char) as i32)));
            w = 4101_i32;
            r = unimplemented!();
            assert!(((r as i32) == (unimplemented!() as i32)));
            assert!(((wp) == ((&mut w as *mut i32).cast_const())));
            assert!(((np) == (n.as_mut_ptr())));
            assert!(((n[(0) as usize] as i32) == ((0 as libc::c_char) as i32)));
            assert!(((n[(1) as usize] as i32) == ((0 as libc::c_char) as i32)));
            assert!(((n[(2) as usize] as i32) == ((0 as libc::c_char) as i32)));
            assert!(((n[(3) as usize] as i32) == ((0 as libc::c_char) as i32)));
            w = 1107_i32;
            r = unimplemented!();
            assert!(((r as i32) == (unimplemented!() as i32)));
            assert!(((wp) == (((&mut w as *mut i32).offset((1) as isize)).cast_const())));
            assert!(((np) == (n.as_mut_ptr().offset((2) as isize))));
            assert!(((n[(0) as usize] as i32) == ((-47 as libc::c_char) as i32)));
            assert!(((n[(1) as usize] as i32) == ((-109 as libc::c_char) as i32)));
            assert!(((n[(2) as usize] as i32) == ((0 as libc::c_char) as i32)));
            assert!(((n[(3) as usize] as i32) == ((0 as libc::c_char) as i32)));
            w = 86_i32;
            r = unimplemented!();
            assert!(((r as i32) == (unimplemented!() as i32)));
            assert!(((wp) == (((&mut w as *mut i32).offset((1) as isize)).cast_const())));
            assert!(((np) == (n.as_mut_ptr().offset((1) as isize))));
            assert!(((n[(0) as usize] as i32) == ((86 as libc::c_char) as i32)));
            assert!(((n[(1) as usize] as i32) == ((-109 as libc::c_char) as i32)));
            assert!(((n[(2) as usize] as i32) == ((0 as libc::c_char) as i32)));
            assert!(((n[(3) as usize] as i32) == ((0 as libc::c_char) as i32)));
        }
        {
            let mut c: () = unimplemented!();
            let mut w: i32 = 262147_i32;
            let mut n: [libc::c_char; 7] = [
                (0 as libc::c_char),
                (0 as libc::c_char),
                (0 as libc::c_char),
                (0 as libc::c_char),
                (0 as libc::c_char),
                (0 as libc::c_char),
                (0 as libc::c_char),
            ];
            let mut wp: *const i32 = std::ptr::null();
            let mut m: ::libc::mbstate_t = unsafe { std::mem::zeroed() };
            let mut np: *mut libc::c_char = std::ptr::null_mut();
            let mut r: () = unimplemented!();
            assert!(((r as i32) == (unimplemented!() as i32)));
            assert!(((wp) == (((&mut w as *mut i32).offset((1) as isize)).cast_const())));
            assert!(((np) == (n.as_mut_ptr().offset((7) as isize))));
            assert!(((n[(0) as usize] as i32) == ((-17 as libc::c_char) as i32)));
            assert!(((n[(1) as usize] as i32) == ((-69 as libc::c_char) as i32)));
            assert!(((n[(2) as usize] as i32) == ((-65 as libc::c_char) as i32)));
            assert!(((n[(3) as usize] as i32) == ((-15 as libc::c_char) as i32)));
            assert!(((n[(4) as usize] as i32) == ((-128 as libc::c_char) as i32)));
            assert!(((n[(5) as usize] as i32) == ((-128 as libc::c_char) as i32)));
            assert!(((n[(6) as usize] as i32) == ((-125 as libc::c_char) as i32)));
            w = 4101_i32;
            r = unimplemented!();
            assert!(((r as i32) == (unimplemented!() as i32)));
            assert!(((wp) == (((&mut w as *mut i32).offset((1) as isize)).cast_const())));
            assert!(((np) == (n.as_mut_ptr().offset((6) as isize))));
            assert!(((n[(0) as usize] as i32) == ((-17 as libc::c_char) as i32)));
            assert!(((n[(1) as usize] as i32) == ((-69 as libc::c_char) as i32)));
            assert!(((n[(2) as usize] as i32) == ((-65 as libc::c_char) as i32)));
            assert!(((n[(3) as usize] as i32) == ((-31 as libc::c_char) as i32)));
            assert!(((n[(4) as usize] as i32) == ((-128 as libc::c_char) as i32)));
            assert!(((n[(5) as usize] as i32) == ((-123 as libc::c_char) as i32)));
            assert!(((n[(6) as usize] as i32) == ((-125 as libc::c_char) as i32)));
            w = 1107_i32;
            r = unimplemented!();
            assert!(((r as i32) == (unimplemented!() as i32)));
            assert!(((wp) == (((&mut w as *mut i32).offset((1) as isize)).cast_const())));
            assert!(((np) == (n.as_mut_ptr().offset((5) as isize))));
            assert!(((n[(0) as usize] as i32) == ((-17 as libc::c_char) as i32)));
            assert!(((n[(1) as usize] as i32) == ((-69 as libc::c_char) as i32)));
            assert!(((n[(2) as usize] as i32) == ((-65 as libc::c_char) as i32)));
            assert!(((n[(3) as usize] as i32) == ((-47 as libc::c_char) as i32)));
            assert!(((n[(4) as usize] as i32) == ((-109 as libc::c_char) as i32)));
            assert!(((n[(5) as usize] as i32) == ((-123 as libc::c_char) as i32)));
            assert!(((n[(6) as usize] as i32) == ((-125 as libc::c_char) as i32)));
            w = 86_i32;
            r = unimplemented!();
            assert!(((r as i32) == (unimplemented!() as i32)));
            assert!(((wp) == (((&mut w as *mut i32).offset((1) as isize)).cast_const())));
            assert!(((np) == (n.as_mut_ptr().offset((4) as isize))));
            assert!(((n[(0) as usize] as i32) == ((-17 as libc::c_char) as i32)));
            assert!(((n[(1) as usize] as i32) == ((-69 as libc::c_char) as i32)));
            assert!(((n[(2) as usize] as i32) == ((-65 as libc::c_char) as i32)));
            assert!(((n[(3) as usize] as i32) == ((86 as libc::c_char) as i32)));
            assert!(((n[(4) as usize] as i32) == ((-109 as libc::c_char) as i32)));
            assert!(((n[(5) as usize] as i32) == ((-123 as libc::c_char) as i32)));
            assert!(((n[(6) as usize] as i32) == ((-125 as libc::c_char) as i32)));
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
            let mut w: u32 = 262147_u32;
            let mut n: [libc::c_char; 4] = [
                (0 as libc::c_char),
                (0 as libc::c_char),
                (0 as libc::c_char),
                (0 as libc::c_char),
            ];
            let mut wp: *const u32 = std::ptr::null();
            let mut m: ::libc::mbstate_t = unsafe { std::mem::zeroed() };
            let mut np: *mut libc::c_char = std::ptr::null_mut();
            let mut r: () = unimplemented!();
            assert!(((r as i32) == (unimplemented!() as i32)));
            assert!(((wp) == (((&mut w as *mut u32).offset((1) as isize)).cast_const())));
            assert!(((np) == (n.as_mut_ptr().offset((4) as isize))));
            assert!(((n[(0) as usize] as i32) == ((-15 as libc::c_char) as i32)));
            assert!(((n[(1) as usize] as i32) == ((-128 as libc::c_char) as i32)));
            assert!(((n[(2) as usize] as i32) == ((-128 as libc::c_char) as i32)));
            assert!(((n[(3) as usize] as i32) == ((-125 as libc::c_char) as i32)));
            w = 4101_u32;
            r = unimplemented!();
            assert!(((r as i32) == (unimplemented!() as i32)));
            assert!(((wp) == (((&mut w as *mut u32).offset((1) as isize)).cast_const())));
            assert!(((np) == (n.as_mut_ptr().offset((3) as isize))));
            assert!(((n[(0) as usize] as i32) == ((-31 as libc::c_char) as i32)));
            assert!(((n[(1) as usize] as i32) == ((-128 as libc::c_char) as i32)));
            assert!(((n[(2) as usize] as i32) == ((-123 as libc::c_char) as i32)));
            assert!(((n[(3) as usize] as i32) == ((-125 as libc::c_char) as i32)));
            w = 1107_u32;
            r = unimplemented!();
            assert!(((r as i32) == (unimplemented!() as i32)));
            assert!(((wp) == (((&mut w as *mut u32).offset((1) as isize)).cast_const())));
            assert!(((np) == (n.as_mut_ptr().offset((2) as isize))));
            assert!(((n[(0) as usize] as i32) == ((-47 as libc::c_char) as i32)));
            assert!(((n[(1) as usize] as i32) == ((-109 as libc::c_char) as i32)));
            assert!(((n[(2) as usize] as i32) == ((-123 as libc::c_char) as i32)));
            assert!(((n[(3) as usize] as i32) == ((-125 as libc::c_char) as i32)));
            w = 86_u32;
            r = unimplemented!();
            assert!(((r as i32) == (unimplemented!() as i32)));
            assert!(((wp) == (((&mut w as *mut u32).offset((1) as isize)).cast_const())));
            assert!(((np) == (n.as_mut_ptr().offset((1) as isize))));
            assert!(((n[(0) as usize] as i32) == ((86 as libc::c_char) as i32)));
            assert!(((n[(1) as usize] as i32) == ((-109 as libc::c_char) as i32)));
            assert!(((n[(2) as usize] as i32) == ((-123 as libc::c_char) as i32)));
            assert!(((n[(3) as usize] as i32) == ((-125 as libc::c_char) as i32)));
        }
        {
            let mut c: () = unimplemented!();
            let mut w: u32 = 262147_u32;
            let mut n: [libc::c_char; 4] = [
                (0 as libc::c_char),
                (0 as libc::c_char),
                (0 as libc::c_char),
                (0 as libc::c_char),
            ];
            let mut wp: *const u32 = std::ptr::null();
            let mut m: ::libc::mbstate_t = unsafe { std::mem::zeroed() };
            let mut np: *mut libc::c_char = std::ptr::null_mut();
            let mut r: () = unimplemented!();
            assert!(((r as i32) == (unimplemented!() as i32)));
            assert!(((wp) == ((&mut w as *mut u32).cast_const())));
            assert!(((np) == (n.as_mut_ptr())));
            assert!(((n[(0) as usize] as i32) == ((0 as libc::c_char) as i32)));
            assert!(((n[(1) as usize] as i32) == ((0 as libc::c_char) as i32)));
            assert!(((n[(2) as usize] as i32) == ((0 as libc::c_char) as i32)));
            assert!(((n[(3) as usize] as i32) == ((0 as libc::c_char) as i32)));
            w = 4101_u32;
            r = unimplemented!();
            assert!(((r as i32) == (unimplemented!() as i32)));
            assert!(((wp) == ((&mut w as *mut u32).cast_const())));
            assert!(((np) == (n.as_mut_ptr())));
            assert!(((n[(0) as usize] as i32) == ((0 as libc::c_char) as i32)));
            assert!(((n[(1) as usize] as i32) == ((0 as libc::c_char) as i32)));
            assert!(((n[(2) as usize] as i32) == ((0 as libc::c_char) as i32)));
            assert!(((n[(3) as usize] as i32) == ((0 as libc::c_char) as i32)));
            w = 1107_u32;
            r = unimplemented!();
            assert!(((r as i32) == (unimplemented!() as i32)));
            assert!(((wp) == (((&mut w as *mut u32).offset((1) as isize)).cast_const())));
            assert!(((np) == (n.as_mut_ptr().offset((2) as isize))));
            assert!(((n[(0) as usize] as i32) == ((-47 as libc::c_char) as i32)));
            assert!(((n[(1) as usize] as i32) == ((-109 as libc::c_char) as i32)));
            assert!(((n[(2) as usize] as i32) == ((0 as libc::c_char) as i32)));
            assert!(((n[(3) as usize] as i32) == ((0 as libc::c_char) as i32)));
            w = 86_u32;
            r = unimplemented!();
            assert!(((r as i32) == (unimplemented!() as i32)));
            assert!(((wp) == (((&mut w as *mut u32).offset((1) as isize)).cast_const())));
            assert!(((np) == (n.as_mut_ptr().offset((1) as isize))));
            assert!(((n[(0) as usize] as i32) == ((86 as libc::c_char) as i32)));
            assert!(((n[(1) as usize] as i32) == ((-109 as libc::c_char) as i32)));
            assert!(((n[(2) as usize] as i32) == ((0 as libc::c_char) as i32)));
            assert!(((n[(3) as usize] as i32) == ((0 as libc::c_char) as i32)));
        }
        {
            let mut c: () = unimplemented!();
            let mut w: u32 = 262147_u32;
            let mut n: [libc::c_char; 7] = [
                (0 as libc::c_char),
                (0 as libc::c_char),
                (0 as libc::c_char),
                (0 as libc::c_char),
                (0 as libc::c_char),
                (0 as libc::c_char),
                (0 as libc::c_char),
            ];
            let mut wp: *const u32 = std::ptr::null();
            let mut m: ::libc::mbstate_t = unsafe { std::mem::zeroed() };
            let mut np: *mut libc::c_char = std::ptr::null_mut();
            let mut r: () = unimplemented!();
            assert!(((r as i32) == (unimplemented!() as i32)));
            assert!(((wp) == (((&mut w as *mut u32).offset((1) as isize)).cast_const())));
            assert!(((np) == (n.as_mut_ptr().offset((7) as isize))));
            assert!(((n[(0) as usize] as i32) == ((-17 as libc::c_char) as i32)));
            assert!(((n[(1) as usize] as i32) == ((-69 as libc::c_char) as i32)));
            assert!(((n[(2) as usize] as i32) == ((-65 as libc::c_char) as i32)));
            assert!(((n[(3) as usize] as i32) == ((-15 as libc::c_char) as i32)));
            assert!(((n[(4) as usize] as i32) == ((-128 as libc::c_char) as i32)));
            assert!(((n[(5) as usize] as i32) == ((-128 as libc::c_char) as i32)));
            assert!(((n[(6) as usize] as i32) == ((-125 as libc::c_char) as i32)));
            w = 4101_u32;
            r = unimplemented!();
            assert!(((r as i32) == (unimplemented!() as i32)));
            assert!(((wp) == (((&mut w as *mut u32).offset((1) as isize)).cast_const())));
            assert!(((np) == (n.as_mut_ptr().offset((6) as isize))));
            assert!(((n[(0) as usize] as i32) == ((-17 as libc::c_char) as i32)));
            assert!(((n[(1) as usize] as i32) == ((-69 as libc::c_char) as i32)));
            assert!(((n[(2) as usize] as i32) == ((-65 as libc::c_char) as i32)));
            assert!(((n[(3) as usize] as i32) == ((-31 as libc::c_char) as i32)));
            assert!(((n[(4) as usize] as i32) == ((-128 as libc::c_char) as i32)));
            assert!(((n[(5) as usize] as i32) == ((-123 as libc::c_char) as i32)));
            assert!(((n[(6) as usize] as i32) == ((-125 as libc::c_char) as i32)));
            w = 1107_u32;
            r = unimplemented!();
            assert!(((r as i32) == (unimplemented!() as i32)));
            assert!(((wp) == (((&mut w as *mut u32).offset((1) as isize)).cast_const())));
            assert!(((np) == (n.as_mut_ptr().offset((5) as isize))));
            assert!(((n[(0) as usize] as i32) == ((-17 as libc::c_char) as i32)));
            assert!(((n[(1) as usize] as i32) == ((-69 as libc::c_char) as i32)));
            assert!(((n[(2) as usize] as i32) == ((-65 as libc::c_char) as i32)));
            assert!(((n[(3) as usize] as i32) == ((-47 as libc::c_char) as i32)));
            assert!(((n[(4) as usize] as i32) == ((-109 as libc::c_char) as i32)));
            assert!(((n[(5) as usize] as i32) == ((-123 as libc::c_char) as i32)));
            assert!(((n[(6) as usize] as i32) == ((-125 as libc::c_char) as i32)));
            w = 86_u32;
            r = unimplemented!();
            assert!(((r as i32) == (unimplemented!() as i32)));
            assert!(((wp) == (((&mut w as *mut u32).offset((1) as isize)).cast_const())));
            assert!(((np) == (n.as_mut_ptr().offset((4) as isize))));
            assert!(((n[(0) as usize] as i32) == ((-17 as libc::c_char) as i32)));
            assert!(((n[(1) as usize] as i32) == ((-69 as libc::c_char) as i32)));
            assert!(((n[(2) as usize] as i32) == ((-65 as libc::c_char) as i32)));
            assert!(((n[(3) as usize] as i32) == ((86 as libc::c_char) as i32)));
            assert!(((n[(4) as usize] as i32) == ((-109 as libc::c_char) as i32)));
            assert!(((n[(5) as usize] as i32) == ((-123 as libc::c_char) as i32)));
            assert!(((n[(6) as usize] as i32) == ((-125 as libc::c_char) as i32)));
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
            let mut w: u16 = 4101_u16;
            let mut n: [libc::c_char; 4] = [
                (0 as libc::c_char),
                (0 as libc::c_char),
                (0 as libc::c_char),
                (0 as libc::c_char),
            ];
            let mut wp: *const u16 = std::ptr::null();
            let mut m: ::libc::mbstate_t = unsafe { std::mem::zeroed() };
            let mut np: *mut libc::c_char = std::ptr::null_mut();
            let mut r: () = unimplemented!();
            assert!(((r as i32) == (unimplemented!() as i32)));
            assert!(((wp) == (((&mut w as *mut u16).offset((1) as isize)).cast_const())));
            assert!(((np) == (n.as_mut_ptr().offset((3) as isize))));
            assert!(((n[(0) as usize] as i32) == ((-31 as libc::c_char) as i32)));
            assert!(((n[(1) as usize] as i32) == ((-128 as libc::c_char) as i32)));
            assert!(((n[(2) as usize] as i32) == ((-123 as libc::c_char) as i32)));
            assert!(((n[(3) as usize] as i32) == ((0 as libc::c_char) as i32)));
            w = 1107_u16;
            r = unimplemented!();
            assert!(((r as i32) == (unimplemented!() as i32)));
            assert!(((wp) == (((&mut w as *mut u16).offset((1) as isize)).cast_const())));
            assert!(((np) == (n.as_mut_ptr().offset((2) as isize))));
            assert!(((n[(0) as usize] as i32) == ((-47 as libc::c_char) as i32)));
            assert!(((n[(1) as usize] as i32) == ((-109 as libc::c_char) as i32)));
            assert!(((n[(2) as usize] as i32) == ((-123 as libc::c_char) as i32)));
            assert!(((n[(3) as usize] as i32) == ((0 as libc::c_char) as i32)));
            w = 86_u16;
            r = unimplemented!();
            assert!(((r as i32) == (unimplemented!() as i32)));
            assert!(((wp) == (((&mut w as *mut u16).offset((1) as isize)).cast_const())));
            assert!(((np) == (n.as_mut_ptr().offset((1) as isize))));
            assert!(((n[(0) as usize] as i32) == ((86 as libc::c_char) as i32)));
            assert!(((n[(1) as usize] as i32) == ((-109 as libc::c_char) as i32)));
            assert!(((n[(2) as usize] as i32) == ((-123 as libc::c_char) as i32)));
            assert!(((n[(3) as usize] as i32) == ((0 as libc::c_char) as i32)));
        }
        {
            let mut c: () = unimplemented!();
            let mut w: u16 = 4101_u16;
            let mut n: [libc::c_char; 4] = [
                (0 as libc::c_char),
                (0 as libc::c_char),
                (0 as libc::c_char),
                (0 as libc::c_char),
            ];
            let mut wp: *const u16 = std::ptr::null();
            let mut m: ::libc::mbstate_t = unsafe { std::mem::zeroed() };
            let mut np: *mut libc::c_char = std::ptr::null_mut();
            let mut r: () = unimplemented!();
            assert!(((r as i32) == (unimplemented!() as i32)));
            assert!(((wp) == ((&mut w as *mut u16).cast_const())));
            assert!(((np) == (n.as_mut_ptr())));
            assert!(((n[(0) as usize] as i32) == ((0 as libc::c_char) as i32)));
            assert!(((n[(1) as usize] as i32) == ((0 as libc::c_char) as i32)));
            assert!(((n[(2) as usize] as i32) == ((0 as libc::c_char) as i32)));
            assert!(((n[(3) as usize] as i32) == ((0 as libc::c_char) as i32)));
            w = 1107_u16;
            r = unimplemented!();
            assert!(((r as i32) == (unimplemented!() as i32)));
            assert!(((wp) == (((&mut w as *mut u16).offset((1) as isize)).cast_const())));
            assert!(((np) == (n.as_mut_ptr().offset((2) as isize))));
            assert!(((n[(0) as usize] as i32) == ((-47 as libc::c_char) as i32)));
            assert!(((n[(1) as usize] as i32) == ((-109 as libc::c_char) as i32)));
            assert!(((n[(2) as usize] as i32) == ((0 as libc::c_char) as i32)));
            assert!(((n[(3) as usize] as i32) == ((0 as libc::c_char) as i32)));
            w = 86_u16;
            r = unimplemented!();
            assert!(((r as i32) == (unimplemented!() as i32)));
            assert!(((wp) == (((&mut w as *mut u16).offset((1) as isize)).cast_const())));
            assert!(((np) == (n.as_mut_ptr().offset((1) as isize))));
            assert!(((n[(0) as usize] as i32) == ((86 as libc::c_char) as i32)));
            assert!(((n[(1) as usize] as i32) == ((-109 as libc::c_char) as i32)));
            assert!(((n[(2) as usize] as i32) == ((0 as libc::c_char) as i32)));
            assert!(((n[(3) as usize] as i32) == ((0 as libc::c_char) as i32)));
        }
        {
            let mut c: () = unimplemented!();
            let mut w: u16 = 4101_u16;
            let mut n: [libc::c_char; 7] = [
                (0 as libc::c_char),
                (0 as libc::c_char),
                (0 as libc::c_char),
                (0 as libc::c_char),
                (0 as libc::c_char),
                (0 as libc::c_char),
                (0 as libc::c_char),
            ];
            let mut wp: *const u16 = std::ptr::null();
            let mut m: ::libc::mbstate_t = unsafe { std::mem::zeroed() };
            let mut np: *mut libc::c_char = std::ptr::null_mut();
            let mut r: () = unimplemented!();
            assert!(((r as i32) == (unimplemented!() as i32)));
            assert!(((wp) == (((&mut w as *mut u16).offset((1) as isize)).cast_const())));
            assert!(((np) == (n.as_mut_ptr().offset((6) as isize))));
            assert!(((n[(0) as usize] as i32) == ((-17 as libc::c_char) as i32)));
            assert!(((n[(1) as usize] as i32) == ((-69 as libc::c_char) as i32)));
            assert!(((n[(2) as usize] as i32) == ((-65 as libc::c_char) as i32)));
            assert!(((n[(3) as usize] as i32) == ((-31 as libc::c_char) as i32)));
            assert!(((n[(4) as usize] as i32) == ((-128 as libc::c_char) as i32)));
            assert!(((n[(5) as usize] as i32) == ((-123 as libc::c_char) as i32)));
            assert!(((n[(6) as usize] as i32) == ((0 as libc::c_char) as i32)));
            w = 1107_u16;
            r = unimplemented!();
            assert!(((r as i32) == (unimplemented!() as i32)));
            assert!(((wp) == (((&mut w as *mut u16).offset((1) as isize)).cast_const())));
            assert!(((np) == (n.as_mut_ptr().offset((5) as isize))));
            assert!(((n[(0) as usize] as i32) == ((-17 as libc::c_char) as i32)));
            assert!(((n[(1) as usize] as i32) == ((-69 as libc::c_char) as i32)));
            assert!(((n[(2) as usize] as i32) == ((-65 as libc::c_char) as i32)));
            assert!(((n[(3) as usize] as i32) == ((-47 as libc::c_char) as i32)));
            assert!(((n[(4) as usize] as i32) == ((-109 as libc::c_char) as i32)));
            assert!(((n[(5) as usize] as i32) == ((-123 as libc::c_char) as i32)));
            assert!(((n[(6) as usize] as i32) == ((0 as libc::c_char) as i32)));
            w = 86_u16;
            r = unimplemented!();
            assert!(((r as i32) == (unimplemented!() as i32)));
            assert!(((wp) == (((&mut w as *mut u16).offset((1) as isize)).cast_const())));
            assert!(((np) == (n.as_mut_ptr().offset((4) as isize))));
            assert!(((n[(0) as usize] as i32) == ((-17 as libc::c_char) as i32)));
            assert!(((n[(1) as usize] as i32) == ((-69 as libc::c_char) as i32)));
            assert!(((n[(2) as usize] as i32) == ((-65 as libc::c_char) as i32)));
            assert!(((n[(3) as usize] as i32) == ((86 as libc::c_char) as i32)));
            assert!(((n[(4) as usize] as i32) == ((-109 as libc::c_char) as i32)));
            assert!(((n[(5) as usize] as i32) == ((-123 as libc::c_char) as i32)));
            assert!(((n[(6) as usize] as i32) == ((0 as libc::c_char) as i32)));
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
