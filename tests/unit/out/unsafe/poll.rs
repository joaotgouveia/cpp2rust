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
    let mut pfd: [(); 2] = std::array::from_fn::<_, 2, _>(|_| unimplemented!());
    pfd[(unimplemented!()) as usize].fd = fds[(unimplemented!()) as usize];
    pfd[(unimplemented!()) as usize].events = 1_();
    pfd[(unimplemented!()) as usize].revents = unimplemented!();
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
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
