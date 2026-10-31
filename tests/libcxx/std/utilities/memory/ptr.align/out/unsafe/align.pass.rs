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
    let N: u32 = 20_u32;
    let mut buf: [libc::c_char; 20] = [(0 as libc::c_char); 20];
    let mut r: *mut ::libc::c_void = std::ptr::null_mut();
    let mut p: *mut ::libc::c_void =
        ((&mut buf[(0) as usize] as *mut libc::c_char) as *mut ::libc::c_void);
    let mut s: usize = (N as usize);
    r = unimplemented!();
    assert!(((p) == ((&mut buf[(0) as usize] as *mut libc::c_char) as *mut ::libc::c_void)));
    assert!(((r) == (p)));
    assert!(((s) == (N as usize)));
    p = ((&mut buf[(1) as usize] as *mut libc::c_char) as *mut ::libc::c_void);
    s = (N as usize);
    r = unimplemented!();
    assert!(((p) == ((&mut buf[(4) as usize] as *mut libc::c_char) as *mut ::libc::c_void)));
    assert!(((r) == (p)));
    assert!(((s) == (((N).wrapping_sub(3_u32)) as usize)));
    p = ((&mut buf[(2) as usize] as *mut libc::c_char) as *mut ::libc::c_void);
    s = (N as usize);
    r = unimplemented!();
    assert!(((p) == ((&mut buf[(4) as usize] as *mut libc::c_char) as *mut ::libc::c_void)));
    assert!(((r) == (p)));
    assert!(((s) == (((N).wrapping_sub(2_u32)) as usize)));
    p = ((&mut buf[(3) as usize] as *mut libc::c_char) as *mut ::libc::c_void);
    s = (N as usize);
    r = unimplemented!();
    assert!(((p) == ((&mut buf[(4) as usize] as *mut libc::c_char) as *mut ::libc::c_void)));
    assert!(((r) == (p)));
    assert!(((s) == (((N).wrapping_sub(1_u32)) as usize)));
    p = ((&mut buf[(4) as usize] as *mut libc::c_char) as *mut ::libc::c_void);
    s = (N as usize);
    r = unimplemented!();
    assert!(((p) == ((&mut buf[(4) as usize] as *mut libc::c_char) as *mut ::libc::c_void)));
    assert!(((r) == (p)));
    assert!(((s) == (N as usize)));
    p = ((&mut buf[(0) as usize] as *mut libc::c_char) as *mut ::libc::c_void);
    s = (N as usize);
    r = unimplemented!();
    assert!(((p) == ((&mut buf[(0) as usize] as *mut libc::c_char) as *mut ::libc::c_void)));
    assert!(((r) == (p)));
    assert!(((s) == (N as usize)));
    p = ((&mut buf[(1) as usize] as *mut libc::c_char) as *mut ::libc::c_void);
    s = (((N).wrapping_sub(1_u32)) as usize);
    r = unimplemented!();
    assert!(((p) == ((&mut buf[(4) as usize] as *mut libc::c_char) as *mut ::libc::c_void)));
    assert!(((r) == (p)));
    assert!(((s) == (((N).wrapping_sub(4_u32)) as usize)));
    p = ((&mut buf[(1) as usize] as *mut libc::c_char) as *mut ::libc::c_void);
    s = (((N).wrapping_sub(1_u32)) as usize);
    r = unimplemented!();
    assert!(((p) == ((&mut buf[(1) as usize] as *mut libc::c_char) as *mut ::libc::c_void)));
    assert!((r).is_null());
    assert!(((s) == (((N).wrapping_sub(1_u32)) as usize)));
    p = ((&mut buf[(0) as usize] as *mut libc::c_char) as *mut ::libc::c_void);
    s = (N as usize);
    r = unimplemented!();
    assert!(((p) == ((&mut buf[(0) as usize] as *mut libc::c_char) as *mut ::libc::c_void)));
    assert!((r).is_null());
    assert!(((s) == (N as usize)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
