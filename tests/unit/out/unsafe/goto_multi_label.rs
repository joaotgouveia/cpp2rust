extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn classify_0(mut n: ()) -> () {
    let mut ret: () = unimplemented!();
    goto_block!({
        '__entry: {
            ret = 0_();
            if ((((n) < (0_())) as ()) != 0) {
                goto!('error);
            }
            if ((((n) == (0_())) as ()) != 0) {
                goto!('out);
            }
            ret = n;
            goto!('out);
        }
        'error: {
            ret = -1_();
        }
        'out: {
            return ret;
        }
    });
    panic!("ub: non-void function does not return a value")
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
