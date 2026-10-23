extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test1_0() {
    let mut a: [(); 3] = [3_(), 5_(), 7_()];
    let mut sseq: () = unimplemented!();
    let mut e1: () = unimplemented!();
    let mut e2 : () = std_shuffle_order_engine_std_linear_congruential_engine_unsigned_long__16807__0__2147483647___256_ :: new_1 ( { & mut sseq } , )  ;
    unimplemented!();
    (unsafe { e1.seed_10_class_std_seed_seq_0(&mut sseq) });
    unimplemented!();
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
    (unsafe { test1_0() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
