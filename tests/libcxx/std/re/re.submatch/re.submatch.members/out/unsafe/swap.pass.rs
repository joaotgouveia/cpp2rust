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
    {
        let s1: [(); 4] = [('1' as ()), ('2' as ()), ('3' as ()), 0_()];
        let mut sm1: () = unimplemented!();
        sm1.first = s1.as_ptr();
        sm1.second = s1.as_ptr().offset((3_()) as isize);
        sm1.matched = true;
        let mut sm2: () = unimplemented!();
        let s2: [(); 4] = [('c' as ()), ('a' as ()), ('t' as ()), 0_()];
        sm2.first = s2.as_ptr();
        sm2.second = s2.as_ptr().offset((3_()) as isize);
        sm2.matched = false;
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let s1: [(); 4] = [(49 as ()), (50 as ()), (51 as ()), 0_()];
        let mut sm1: () = unimplemented!();
        sm1.first = s1.as_ptr();
        sm1.second = s1.as_ptr().offset((3_()) as isize);
        sm1.matched = true;
        let mut sm2: () = unimplemented!();
        let s2: [(); 4] = [(99 as ()), (97 as ()), (116 as ()), 0_()];
        sm2.first = s2.as_ptr();
        sm2.second = s2.as_ptr().offset((3_()) as isize);
        sm2.matched = false;
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "");;
    }
    0
}
pub unsafe fn __cpp2rust_init_globals() {}
