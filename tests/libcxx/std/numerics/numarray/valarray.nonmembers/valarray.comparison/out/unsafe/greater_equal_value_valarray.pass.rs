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
        let mut a2: [i32; 5] = [1, 2, 3, 4, 0];
        let mut a3: [bool; 5] = [true, true, false, false, true];
        let N: u32 = 5_u32;
        let mut v2: () = unimplemented!();
        let mut v3: () = unimplemented!();
        let mut v3a: () = unimplemented!();
        assert!(((unimplemented!()) == (N as usize)));
        assert!(((unimplemented!()) == (N as usize)));
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (N as usize)) {
            assert!((((*unimplemented!()) as i32) == (a3[(i)] as i32)));
            assert!((((*unimplemented!()) as i32) == (a3[(i)] as i32)));
            i.prefix_inc();
        }
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
