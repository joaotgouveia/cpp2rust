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
    let mut found_loopback: () = unimplemented!();
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
        let mut lo_be: [(); 4] = [127_(), unimplemented!(), unimplemented!(), 1_()];
        if ((((unsafe {
            memcmp_0(
                ((&mut (*sin).sin_addr as *mut ()) as ()),
                (lo_be.as_mut_ptr() as ()),
                4_(),
            )
        }) == (unimplemented!())) as ())
            != 0)
        {
            found_loopback = 1_();
            unimplemented!();
            unimplemented!();
            let mut mask: *mut () = ((*ifa).ifa_netmask as *mut ());
            let mut mask_be: [(); 4] =
                [255_(), unimplemented!(), unimplemented!(), unimplemented!()];
            unimplemented!();
            unimplemented!();
        }
        ifa = (*ifa).ifa_next;
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
