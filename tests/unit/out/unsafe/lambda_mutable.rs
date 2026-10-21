extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut start: () = 5_();
    let mut next: FnPtr<fn() -> ()> = lambda_unsafe!(
        {
            let start: () = start;
        },
        || -> () {
            return start.postfix_inc();
        }
    );
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut total: () = unimplemented!();
    let mut accumulate: FnPtr<fn(()) -> ()> = lambda_unsafe!(
        {
            let total: () = total;
        },
        |mut x: ()| -> () {
            total += x;
            return total;
        }
    );
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
