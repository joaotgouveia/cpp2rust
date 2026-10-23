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
    let mut fds: [(); 2] = [unimplemented!(); 2];
    unimplemented!();
    unimplemented!();
    let mut pfd: [pollfd; 2] = [unsafe { std::mem::zeroed::<pollfd>() }; 2];
    pfd[(0_()) as usize].fd = fds[(0_()) as usize];
    pfd[(0_()) as usize].events = 1_();
    pfd[(0_()) as usize].revents = 0_();
    pfd[(1_()) as usize].fd = -1_();
    pfd[(1_()) as usize].events = 1_();
    pfd[(1_()) as usize].revents = 42_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut ch: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
