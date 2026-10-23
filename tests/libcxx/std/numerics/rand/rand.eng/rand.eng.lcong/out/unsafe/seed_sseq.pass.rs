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
        let mut a: [(); 3] = [3_(), 5_(), 7_()];
        let mut sseq: () = unimplemented!();
        let mut e1: () = unimplemented!();
        let mut e2: () = unimplemented!();
        unimplemented!();
        (unsafe { e1.seed(&mut sseq) });
        unimplemented!();
    }
    {
        let mut a: [(); 5] = [3_(), 5_(), 7_(), 9_(), 11_()];
        let mut sseq: () = unimplemented!();
        let mut e1: () = unimplemented!();
        let mut e2: () =
            std_linear_congruential_engine_unsigned_long_long__1__1__8589934593_::new_1({
                &mut sseq
            });
        unimplemented!();
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
