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
pub struct table {
    pub rows: [[(); 10]; 3],
    pub count: (),
}
pub static mut T1_0: std::cell::LazyCell<table> = std::cell::LazyCell::new(|| unsafe {
    table {
        rows: [std::mem::transmute(*b"alpha\0\0\0\0\0"), [0; 10], [0; 10]],
        count: 1_(),
    }
});
pub static mut T2_1: std::cell::LazyCell<table> = std::cell::LazyCell::new(|| unsafe {
    table {
        rows: [
            std::mem::transmute(*b"alpha\0\0\0\0\0"),
            std::mem::transmute(*b"beta\0\0\0\0\0\0"),
            [0; 10],
        ],
        count: 2_(),
    }
});
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
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut local: table = table {
        rows: [
            std::mem::transmute(*b"one\0\0\0\0\0\0\0"),
            std::mem::transmute(*b"two\0\0\0\0\0\0\0"),
            std::mem::transmute(*b"three\0\0\0\0\0"),
        ],
        count: 3_(),
    };
    unimplemented!();
    local.rows[(1_()) as usize][(0_()) as usize] = ('T' as ());
    unimplemented!();
    unimplemented!();
    let mut p: *const () = (local.rows[(2_()) as usize].as_mut_ptr()).cast_const();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const T1_0);
    std::cell::LazyCell::force(&*&raw const T2_1);
}
