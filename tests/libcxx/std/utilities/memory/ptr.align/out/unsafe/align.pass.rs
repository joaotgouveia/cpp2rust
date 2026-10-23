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
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    let N: () = 20_();
    let mut buf: [(); 20] = [unimplemented!(); 20];
    let mut r: () = unimplemented!();
    let mut p: () = ((&mut buf[(0_()) as usize] as *mut ()) as ());
    let mut s: () = N;
    r = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    p = ((&mut buf[(1_()) as usize] as *mut ()) as ());
    s = N;
    r = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    p = ((&mut buf[(2_()) as usize] as *mut ()) as ());
    s = N;
    r = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    p = ((&mut buf[(3_()) as usize] as *mut ()) as ());
    s = N;
    r = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    p = ((&mut buf[(4_()) as usize] as *mut ()) as ());
    s = N;
    r = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    p = ((&mut buf[(0_()) as usize] as *mut ()) as ());
    s = N;
    r = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    p = ((&mut buf[(1_()) as usize] as *mut ()) as ());
    s = (N).wrapping_sub(1_());
    r = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    p = ((&mut buf[(1_()) as usize] as *mut ()) as ());
    s = (N).wrapping_sub(1_());
    r = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    p = ((&mut buf[(0_()) as usize] as *mut ()) as ());
    s = N;
    r = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
