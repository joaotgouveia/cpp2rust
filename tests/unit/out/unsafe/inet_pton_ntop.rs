extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_s6_addr_access_0() {
    let loopback: [(); 16] = [
        0_(),
        0_(),
        0_(),
        0_(),
        0_(),
        0_(),
        0_(),
        0_(),
        0_(),
        0_(),
        0_(),
        0_(),
        0_(),
        0_(),
        0_(),
        1_(),
    ];
    let mut addr: () = unimplemented!();
    let mut ap: *mut () = (&mut addr as *mut ());
    unimplemented!();
    unimplemented!();
    unimplemented!()[(0_()) as usize] = 32_();
    unimplemented!()[(1_()) as usize] = 1_();
    unimplemented!();
    unimplemented!()[(15_()) as usize] += 4_();
    unimplemented!()[(15_()) as usize].postfix_inc();
    unimplemented!();
    let mut bytes: *mut () = unimplemented!().as_mut_ptr();
    (*bytes.offset((2_()) as isize)) = 7_();
    unimplemented!();
    let mut fourth: *mut () = (&mut unimplemented!()[(3_()) as usize] as *mut ());
    (*fourth) = 8_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    (unsafe { test_s6_addr_access_0() });
    let mut buf: [(); 16] = [unimplemented!(); 16];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut text: [(); 64] = [unimplemented!(); 64];
    let mut four: [(); 4] = [10_(), 0_(), 0_(), 1_()];
    unimplemented!();
    let mut sixteen: [(); 16] = [
        0_(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ];
    sixteen[(15_()) as usize] = 1_();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
