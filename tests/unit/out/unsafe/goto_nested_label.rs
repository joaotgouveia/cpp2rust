extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn scan_0(mut n: ()) -> () {
    let mut total: () = unimplemented!();
    let mut i: () = unimplemented!();
    'loop_: while ((((i) < (n)) as ()) != 0) {
        let mut j: () = unimplemented!();
        goto_block!({
            '__entry: {
                j = unimplemented!();
                'loop_: while ((((j) < (10_())) as ()) != 0) {
                    if ((((j) == (5_())) as ()) != 0) {
                        goto!('next);
                    }
                    total += 1_();
                    j.postfix_inc();
                }
                total += 100_();
            }
            'next: {
                total += 1000_();
            }
        });
        i.postfix_inc();
    }
    return total;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
