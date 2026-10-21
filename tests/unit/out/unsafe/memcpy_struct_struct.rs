extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Entry {
    pub bits: (),
    pub value: (),
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut table: [Entry; 8] = [
        Entry {
            bits: 1_(),
            value: 4369_(),
        },
        Entry {
            bits: 2_(),
            value: 8738_(),
        },
        Entry {
            bits: 3_(),
            value: 13107_(),
        },
        Entry {
            bits: 4_(),
            value: 17476_(),
        },
        Entry {
            bits: unimplemented!(),
            value: unimplemented!(),
        },
        Entry {
            bits: unimplemented!(),
            value: unimplemented!(),
        },
        Entry {
            bits: unimplemented!(),
            value: unimplemented!(),
        },
        Entry {
            bits: unimplemented!(),
            value: unimplemented!(),
        },
    ];
    let mut table_size: () = 4_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
