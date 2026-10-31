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
unsafe fn main_0() -> i32 {
    let mut menge: () = unimplemented!();
    {
        let __first = unimplemented!();
        let __last = unimplemented!();
        if __first == __last {
            __last
        } else {
            let mut __write = __first.clone();
            let mut __it = __first;
            __it.iter_inc();
            while __it != __last {
                if *__write.iter_deref() != *__it.iter_deref() {
                    __write.iter_inc();
                    *__write.iter_deref() = (*__it.iter_deref()).clone();
                }
                __it.iter_inc();
            }
            __write.iter_inc();
            __write
        }
    };
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
