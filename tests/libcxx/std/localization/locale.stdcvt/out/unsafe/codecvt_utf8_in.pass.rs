extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;

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
        let mut c: () = unimplemented!();
        let mut w: u32 = 0_u32;
        let mut n: [libc::c_char; 4] = [
            (-15 as libc::c_char),
            (-128 as libc::c_char),
            (-128 as libc::c_char),
            (-125 as libc::c_char),
        ];
        let mut wp: *mut u32 = std::ptr::null_mut();
        let mut m: ::libc::mbstate_t = unsafe { std::mem::zeroed() };
        let mut np: *const libc::c_char = std::ptr::null();
        let mut r: () = unimplemented!();
        assert!(((r as i32) == (unimplemented!() as i32)));
        assert!(((wp) == ((&mut w as *mut u32).offset((1) as isize))));
        assert!(((np) == ((n.as_mut_ptr().offset((4) as isize)).cast_const())));
        assert!(((w) == (262147_u32)));
        n[(0) as usize] = (-31 as libc::c_char);
        n[(1) as usize] = (-128 as libc::c_char);
        n[(2) as usize] = (-123 as libc::c_char);
        r = unimplemented!();
        assert!(((r as i32) == (unimplemented!() as i32)));
        assert!(((wp) == ((&mut w as *mut u32).offset((1) as isize))));
        assert!(((np) == ((n.as_mut_ptr().offset((3) as isize)).cast_const())));
        assert!(((w) == (4101_u32)));
        n[(0) as usize] = (-47 as libc::c_char);
        n[(1) as usize] = (-109 as libc::c_char);
        r = unimplemented!();
        assert!(((r as i32) == (unimplemented!() as i32)));
        assert!(((wp) == ((&mut w as *mut u32).offset((1) as isize))));
        assert!(((np) == ((n.as_mut_ptr().offset((2) as isize)).cast_const())));
        assert!(((w) == (1107_u32)));
        w = 86_u32;
        n[(0) as usize] = (86 as libc::c_char);
        r = unimplemented!();
        assert!(((r as i32) == (unimplemented!() as i32)));
        assert!(((wp) == ((&mut w as *mut u32).offset((1) as isize))));
        assert!(((np) == ((n.as_mut_ptr().offset((1) as isize)).cast_const())));
        assert!(((w) == (86_u32)));
    }
    {
        let mut c: () = unimplemented!();
        let mut w: u32 = 0_u32;
        let mut n: [libc::c_char; 4] = [
            (-15 as libc::c_char),
            (-128 as libc::c_char),
            (-128 as libc::c_char),
            (-125 as libc::c_char),
        ];
        let mut wp: *mut u32 = std::ptr::null_mut();
        let mut m: ::libc::mbstate_t = unsafe { std::mem::zeroed() };
        let mut np: *const libc::c_char = std::ptr::null();
        let mut r: () = unimplemented!();
        assert!(((r as i32) == (unimplemented!() as i32)));
        assert!(((wp) == (&mut w as *mut u32)));
        assert!(((np) == ((n.as_mut_ptr()).cast_const())));
        assert!(((w) == (0_u32)));
        n[(0) as usize] = (-31 as libc::c_char);
        n[(1) as usize] = (-128 as libc::c_char);
        n[(2) as usize] = (-123 as libc::c_char);
        r = unimplemented!();
        assert!(((r as i32) == (unimplemented!() as i32)));
        assert!(((wp) == (&mut w as *mut u32)));
        assert!(((np) == ((n.as_mut_ptr()).cast_const())));
        assert!(((w) == (0_u32)));
        n[(0) as usize] = (-47 as libc::c_char);
        n[(1) as usize] = (-109 as libc::c_char);
        r = unimplemented!();
        assert!(((r as i32) == (unimplemented!() as i32)));
        assert!(((wp) == ((&mut w as *mut u32).offset((1) as isize))));
        assert!(((np) == ((n.as_mut_ptr().offset((2) as isize)).cast_const())));
        assert!(((w) == (1107_u32)));
        w = 86_u32;
        n[(0) as usize] = (86 as libc::c_char);
        r = unimplemented!();
        assert!(((r as i32) == (unimplemented!() as i32)));
        assert!(((wp) == ((&mut w as *mut u32).offset((1) as isize))));
        assert!(((np) == ((n.as_mut_ptr().offset((1) as isize)).cast_const())));
        assert!(((w) == (86_u32)));
    }
    {
        let mut c: () = unimplemented!();
        let mut w: u32 = 0_u32;
        let mut n: [libc::c_char; 7] = [
            (-17 as libc::c_char),
            (-69 as libc::c_char),
            (-65 as libc::c_char),
            (-15 as libc::c_char),
            (-128 as libc::c_char),
            (-128 as libc::c_char),
            (-125 as libc::c_char),
        ];
        let mut wp: *mut u32 = std::ptr::null_mut();
        let mut m: ::libc::mbstate_t = unsafe { std::mem::zeroed() };
        let mut np: *const libc::c_char = std::ptr::null();
        let mut r: () = unimplemented!();
        assert!(((r as i32) == (unimplemented!() as i32)));
        assert!(((wp) == ((&mut w as *mut u32).offset((1) as isize))));
        assert!(((np) == ((n.as_mut_ptr().offset((7) as isize)).cast_const())));
        assert!(((w) == (262147_u32)));
        n[(0) as usize] = (-31 as libc::c_char);
        n[(1) as usize] = (-128 as libc::c_char);
        n[(2) as usize] = (-123 as libc::c_char);
        r = unimplemented!();
        assert!(((r as i32) == (unimplemented!() as i32)));
        assert!(((wp) == ((&mut w as *mut u32).offset((1) as isize))));
        assert!(((np) == ((n.as_mut_ptr().offset((3) as isize)).cast_const())));
        assert!(((w) == (4101_u32)));
        n[(0) as usize] = (-17 as libc::c_char);
        n[(1) as usize] = (-69 as libc::c_char);
        n[(2) as usize] = (-65 as libc::c_char);
        n[(3) as usize] = (-47 as libc::c_char);
        n[(4) as usize] = (-109 as libc::c_char);
        r = unimplemented!();
        assert!(((r as i32) == (unimplemented!() as i32)));
        assert!(((wp) == ((&mut w as *mut u32).offset((1) as isize))));
        assert!(((np) == ((n.as_mut_ptr().offset((5) as isize)).cast_const())));
        assert!(((w) == (1107_u32)));
        w = 86_u32;
        n[(0) as usize] = (86 as libc::c_char);
        r = unimplemented!();
        assert!(((r as i32) == (unimplemented!() as i32)));
        assert!(((wp) == ((&mut w as *mut u32).offset((1) as isize))));
        assert!(((np) == ((n.as_mut_ptr().offset((1) as isize)).cast_const())));
        assert!(((w) == (86_u32)));
    }
    {
        let mut c: () = unimplemented!();
        let mut w: u32 = 0_u32;
        let mut n: [libc::c_char; 4] = [
            (-15 as libc::c_char),
            (-128 as libc::c_char),
            (-128 as libc::c_char),
            (-125 as libc::c_char),
        ];
        let mut wp: *mut u32 = std::ptr::null_mut();
        let mut m: ::libc::mbstate_t = unsafe { std::mem::zeroed() };
        let mut np: *const libc::c_char = std::ptr::null();
        let mut r: () = unimplemented!();
        assert!(((r as i32) == (unimplemented!() as i32)));
        assert!(((wp) == ((&mut w as *mut u32).offset((1) as isize))));
        assert!(((np) == ((n.as_mut_ptr().offset((4) as isize)).cast_const())));
        assert!(((w) == (262147_u32)));
        n[(0) as usize] = (-31 as libc::c_char);
        n[(1) as usize] = (-128 as libc::c_char);
        n[(2) as usize] = (-123 as libc::c_char);
        r = unimplemented!();
        assert!(((r as i32) == (unimplemented!() as i32)));
        assert!(((wp) == ((&mut w as *mut u32).offset((1) as isize))));
        assert!(((np) == ((n.as_mut_ptr().offset((3) as isize)).cast_const())));
        assert!(((w) == (4101_u32)));
        n[(0) as usize] = (-47 as libc::c_char);
        n[(1) as usize] = (-109 as libc::c_char);
        r = unimplemented!();
        assert!(((r as i32) == (unimplemented!() as i32)));
        assert!(((wp) == ((&mut w as *mut u32).offset((1) as isize))));
        assert!(((np) == ((n.as_mut_ptr().offset((2) as isize)).cast_const())));
        assert!(((w) == (1107_u32)));
        w = 86_u32;
        n[(0) as usize] = (86 as libc::c_char);
        r = unimplemented!();
        assert!(((r as i32) == (unimplemented!() as i32)));
        assert!(((wp) == ((&mut w as *mut u32).offset((1) as isize))));
        assert!(((np) == ((n.as_mut_ptr().offset((1) as isize)).cast_const())));
        assert!(((w) == (86_u32)));
    }
    {
        let mut c: () = unimplemented!();
        let mut w: u32 = 0_u32;
        let mut n: [libc::c_char; 4] = [
            (-15 as libc::c_char),
            (-128 as libc::c_char),
            (-128 as libc::c_char),
            (-125 as libc::c_char),
        ];
        let mut wp: *mut u32 = std::ptr::null_mut();
        let mut m: ::libc::mbstate_t = unsafe { std::mem::zeroed() };
        let mut np: *const libc::c_char = std::ptr::null();
        let mut r: () = unimplemented!();
        assert!(((r as i32) == (unimplemented!() as i32)));
        assert!(((wp) == (&mut w as *mut u32)));
        assert!(((np) == ((n.as_mut_ptr()).cast_const())));
        assert!(((w) == (0_u32)));
        n[(0) as usize] = (-31 as libc::c_char);
        n[(1) as usize] = (-128 as libc::c_char);
        n[(2) as usize] = (-123 as libc::c_char);
        r = unimplemented!();
        assert!(((r as i32) == (unimplemented!() as i32)));
        assert!(((wp) == (&mut w as *mut u32)));
        assert!(((np) == ((n.as_mut_ptr()).cast_const())));
        assert!(((w) == (0_u32)));
        n[(0) as usize] = (-47 as libc::c_char);
        n[(1) as usize] = (-109 as libc::c_char);
        r = unimplemented!();
        assert!(((r as i32) == (unimplemented!() as i32)));
        assert!(((wp) == ((&mut w as *mut u32).offset((1) as isize))));
        assert!(((np) == ((n.as_mut_ptr().offset((2) as isize)).cast_const())));
        assert!(((w) == (1107_u32)));
        w = 86_u32;
        n[(0) as usize] = (86 as libc::c_char);
        r = unimplemented!();
        assert!(((r as i32) == (unimplemented!() as i32)));
        assert!(((wp) == ((&mut w as *mut u32).offset((1) as isize))));
        assert!(((np) == ((n.as_mut_ptr().offset((1) as isize)).cast_const())));
        assert!(((w) == (86_u32)));
    }
    {
        let mut c: () = unimplemented!();
        let mut w: u32 = 0_u32;
        let mut n: [libc::c_char; 7] = [
            (-17 as libc::c_char),
            (-69 as libc::c_char),
            (-65 as libc::c_char),
            (-15 as libc::c_char),
            (-128 as libc::c_char),
            (-128 as libc::c_char),
            (-125 as libc::c_char),
        ];
        let mut wp: *mut u32 = std::ptr::null_mut();
        let mut m: ::libc::mbstate_t = unsafe { std::mem::zeroed() };
        let mut np: *const libc::c_char = std::ptr::null();
        let mut r: () = unimplemented!();
        assert!(((r as i32) == (unimplemented!() as i32)));
        assert!(((wp) == ((&mut w as *mut u32).offset((1) as isize))));
        assert!(((np) == ((n.as_mut_ptr().offset((7) as isize)).cast_const())));
        assert!(((w) == (262147_u32)));
        n[(0) as usize] = (-31 as libc::c_char);
        n[(1) as usize] = (-128 as libc::c_char);
        n[(2) as usize] = (-123 as libc::c_char);
        r = unimplemented!();
        assert!(((r as i32) == (unimplemented!() as i32)));
        assert!(((wp) == ((&mut w as *mut u32).offset((1) as isize))));
        assert!(((np) == ((n.as_mut_ptr().offset((3) as isize)).cast_const())));
        assert!(((w) == (4101_u32)));
        n[(0) as usize] = (-17 as libc::c_char);
        n[(1) as usize] = (-69 as libc::c_char);
        n[(2) as usize] = (-65 as libc::c_char);
        n[(3) as usize] = (-47 as libc::c_char);
        n[(4) as usize] = (-109 as libc::c_char);
        r = unimplemented!();
        assert!(((r as i32) == (unimplemented!() as i32)));
        assert!(((wp) == ((&mut w as *mut u32).offset((1) as isize))));
        assert!(((np) == ((n.as_mut_ptr().offset((5) as isize)).cast_const())));
        assert!(((w) == (1107_u32)));
        w = 86_u32;
        n[(0) as usize] = (86 as libc::c_char);
        r = unimplemented!();
        assert!(((r as i32) == (unimplemented!() as i32)));
        assert!(((wp) == ((&mut w as *mut u32).offset((1) as isize))));
        assert!(((np) == ((n.as_mut_ptr().offset((1) as isize)).cast_const())));
        assert!(((w) == (86_u32)));
    }
    {
        let mut c: () = unimplemented!();
        let mut w: u16 = 0_u16;
        let mut n: [libc::c_char; 3] = [
            (-31 as libc::c_char),
            (-128 as libc::c_char),
            (-123 as libc::c_char),
        ];
        let mut wp: *mut u16 = std::ptr::null_mut();
        let mut m: ::libc::mbstate_t = unsafe { std::mem::zeroed() };
        let mut np: *const libc::c_char = std::ptr::null();
        let mut r: () = unimplemented!();
        assert!(((r as i32) == (unimplemented!() as i32)));
        assert!(((wp) == ((&mut w as *mut u16).offset((1) as isize))));
        assert!(((np) == ((n.as_mut_ptr().offset((3) as isize)).cast_const())));
        assert!(((w as i32) == (4101)));
        n[(0) as usize] = (-47 as libc::c_char);
        n[(1) as usize] = (-109 as libc::c_char);
        r = unimplemented!();
        assert!(((r as i32) == (unimplemented!() as i32)));
        assert!(((wp) == ((&mut w as *mut u16).offset((1) as isize))));
        assert!(((np) == ((n.as_mut_ptr().offset((2) as isize)).cast_const())));
        assert!(((w as i32) == (1107)));
        w = 86_u16;
        n[(0) as usize] = (86 as libc::c_char);
        r = unimplemented!();
        assert!(((r as i32) == (unimplemented!() as i32)));
        assert!(((wp) == ((&mut w as *mut u16).offset((1) as isize))));
        assert!(((np) == ((n.as_mut_ptr().offset((1) as isize)).cast_const())));
        assert!(((w as i32) == (86)));
    }
    {
        let mut c: () = unimplemented!();
        let mut w: u16 = 0_u16;
        let mut n: [libc::c_char; 3] = [
            (-31 as libc::c_char),
            (-128 as libc::c_char),
            (-123 as libc::c_char),
        ];
        let mut wp: *mut u16 = std::ptr::null_mut();
        let mut m: ::libc::mbstate_t = unsafe { std::mem::zeroed() };
        let mut np: *const libc::c_char = std::ptr::null();
        let mut r: () = unimplemented!();
        assert!(((r as i32) == (unimplemented!() as i32)));
        assert!(((wp) == (&mut w as *mut u16)));
        assert!(((np) == ((n.as_mut_ptr()).cast_const())));
        assert!(((w as i32) == (0)));
        n[(0) as usize] = (-47 as libc::c_char);
        n[(1) as usize] = (-109 as libc::c_char);
        r = unimplemented!();
        assert!(((r as i32) == (unimplemented!() as i32)));
        assert!(((wp) == ((&mut w as *mut u16).offset((1) as isize))));
        assert!(((np) == ((n.as_mut_ptr().offset((2) as isize)).cast_const())));
        assert!(((w as i32) == (1107)));
        w = 86_u16;
        n[(0) as usize] = (86 as libc::c_char);
        r = unimplemented!();
        assert!(((r as i32) == (unimplemented!() as i32)));
        assert!(((wp) == ((&mut w as *mut u16).offset((1) as isize))));
        assert!(((np) == ((n.as_mut_ptr().offset((1) as isize)).cast_const())));
        assert!(((w as i32) == (86)));
    }
    {
        let mut c: () = unimplemented!();
        let mut w: u16 = 0_u16;
        let mut n: [libc::c_char; 6] = [
            (-17 as libc::c_char),
            (-69 as libc::c_char),
            (-65 as libc::c_char),
            (-31 as libc::c_char),
            (-128 as libc::c_char),
            (-123 as libc::c_char),
        ];
        let mut wp: *mut u16 = std::ptr::null_mut();
        let mut m: ::libc::mbstate_t = unsafe { std::mem::zeroed() };
        let mut np: *const libc::c_char = std::ptr::null();
        let mut r: () = unimplemented!();
        assert!(((r as i32) == (unimplemented!() as i32)));
        assert!(((wp) == ((&mut w as *mut u16).offset((1) as isize))));
        assert!(((np) == ((n.as_mut_ptr().offset((6) as isize)).cast_const())));
        assert!(((w as i32) == (4101)));
        n[(0) as usize] = (-47 as libc::c_char);
        n[(1) as usize] = (-109 as libc::c_char);
        r = unimplemented!();
        assert!(((r as i32) == (unimplemented!() as i32)));
        assert!(((wp) == ((&mut w as *mut u16).offset((1) as isize))));
        assert!(((np) == ((n.as_mut_ptr().offset((2) as isize)).cast_const())));
        assert!(((w as i32) == (1107)));
        w = 86_u16;
        n[(0) as usize] = (86 as libc::c_char);
        r = unimplemented!();
        assert!(((r as i32) == (unimplemented!() as i32)));
        assert!(((wp) == ((&mut w as *mut u16).offset((1) as isize))));
        assert!(((np) == ((n.as_mut_ptr().offset((1) as isize)).cast_const())));
        assert!(((w as i32) == (86)));
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
