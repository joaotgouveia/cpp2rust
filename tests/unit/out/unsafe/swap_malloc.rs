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
    let mut buf: *mut () = (unimplemented!() as *mut ());
    (*buf.offset((0_()) as isize)) = 1_();
    (*buf.offset((1_()) as isize)) = 2_();
    (*buf.offset((2_()) as isize)) = 3_();
    (unsafe {
        let ___a: *mut () = &mut (*buf.offset((0_()) as isize));
        let ___b: *mut () = &mut (*buf.offset((2_()) as isize));
        swap_0(___a, ___b)
    });
    unimplemented!();
    (unsafe {
        let ___a: *mut () = &mut (*buf.offset((1_()) as isize));
        let ___b: *mut () = &mut (*buf.offset((1_()) as isize));
        swap_0(___a, ___b)
    });
    unimplemented!();
    let mut local: () = 4_();
    (unsafe {
        let ___a: *mut () = &mut (*buf.offset((1_()) as isize));
        let ___b: *mut () = &mut local;
        swap_0(___a, ___b)
    });
    unimplemented!();
    let mut other: *mut () = (unimplemented!() as *mut ());
    (*other) = 5_();
    (unsafe {
        let ___a: *mut () = &mut (*buf);
        let ___b: *mut () = &mut (*other);
        swap_0(___a, ___b)
    });
    unimplemented!();
    let mut bytes: *mut () = (buf as *mut ());
    (*bytes.offset((0_()) as isize)) = 170_();
    (*bytes.offset((4_()) as isize)) = 187_();
    (unsafe {
        let ___a: *mut () = &mut (*bytes.offset((0_()) as isize));
        let ___b: *mut () = &mut (*bytes.offset((4_()) as isize));
        swap_1(___a, ___b)
    });
    unimplemented!();
    let mut words: [(); 2] = [16909060_(), 84281096_()];
    let mut halves: *mut () = (words.as_mut_ptr() as *mut ());
    (unsafe {
        let ___a: *mut () = &mut (*halves.offset((0_()) as isize));
        let ___b: *mut () = &mut (*halves.offset((3_()) as isize));
        swap_2(___a, ___b)
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
