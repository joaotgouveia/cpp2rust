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
        let mut a1: [i32; 5] = [1, 2, 3, 4, 5];
        let mut a2: [i32; 5] = [6, 12, 18, 24, 30];
        let N: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
            .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
        let mut v1: () = unimplemented!();
        let mut v2: () = unimplemented!();
        unimplemented!();
        assert!(((unimplemented!()) == (unimplemented!())));
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (unimplemented!())) {
            assert!(((*unimplemented!()) == (*unimplemented!())));
            i.prefix_inc();
        }
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
