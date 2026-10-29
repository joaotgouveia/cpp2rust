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
    let mut list: *mut () = std::ptr::null_mut();
    unimplemented!();
    unimplemented!();
    let mut found_loopback: () = 0_();
    let mut ifa: *mut () = std::ptr::null_mut();
    ifa = list;
    'loop_: while (((!((ifa).is_null())) as ()) != 0) {
        unimplemented!();
        if (((((*ifa).ifa_addr).is_null()) as ()) != 0) {
            ifa = (*ifa).ifa_next;
            continue 'loop_;
        }
        if (((((*(*ifa).ifa_addr).sa_family) != (unimplemented!())) as ()) != 0) {
            ifa = (*ifa).ifa_next;
            continue 'loop_;
        }
        let mut sin: *mut () = ((*ifa).ifa_addr as *mut ());
        let mut lo_be: [(); 4] = [127_(), 0_(), 0_(), 1_()];
        if ((((unimplemented!()) == (0_())) as ()) != 0) {
            found_loopback = 1_();
            unimplemented!();
            unimplemented!();
            let mut mask: *mut () = ((*ifa).ifa_netmask as *mut ());
            let mut mask_be: [(); 4] = [255_(), 0_(), 0_(), 0_()];
            unimplemented!();
            unimplemented!();
        }
        ifa = (*ifa).ifa_next;
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
