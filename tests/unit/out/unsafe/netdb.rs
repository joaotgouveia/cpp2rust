extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_ipv4_literal_0() {
    let mut hints: () = unimplemented!();
    hints.ai_flags = unimplemented!();
    hints.ai_protocol = unimplemented!();
    hints.ai_addrlen = unimplemented!();
    hints.ai_addr = std::ptr::null_mut();
    hints.ai_canonname = std::ptr::null_mut();
    hints.ai_next = std::ptr::null_mut();
    hints.ai_family = unimplemented!();
    hints.ai_socktype = unimplemented!();
    let mut res: *mut () = std::ptr::null_mut();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut sin: *mut () = ((*res).ai_addr as *mut ());
    unimplemented!();
    let mut port_be: [(); 2] = [((8080_()) / (256_())), ((8080_()) % (256_()))];
    unimplemented!();
    let mut addr_be: [(); 4] = [127_(), unimplemented!(), unimplemented!(), 1_()];
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_ipv6_literal_1() {
    let mut hints: () = unimplemented!();
    hints.ai_flags = unimplemented!();
    hints.ai_protocol = unimplemented!();
    hints.ai_addrlen = unimplemented!();
    hints.ai_addr = std::ptr::null_mut();
    hints.ai_canonname = std::ptr::null_mut();
    hints.ai_next = std::ptr::null_mut();
    hints.ai_family = unimplemented!();
    hints.ai_socktype = unimplemented!();
    let mut res: *mut () = std::ptr::null_mut();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut sin6: *mut () = ((*res).ai_addr as *mut ());
    unimplemented!();
    let mut port_be: [(); 2] = [((443_()) / (256_())), ((443_()) % (256_()))];
    unimplemented!();
    let mut addr_be: [(); 16] = [
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
        1_(),
    ];
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_null_hints_2() {
    let mut res: *mut () = std::ptr::null_mut();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut sin: *mut () = ((*res).ai_addr as *mut ());
    let mut addr_be: [(); 4] = [127_(), unimplemented!(), unimplemented!(), 1_()];
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
    (unsafe { test_ipv4_literal_0() });
    (unsafe { test_ipv6_literal_1() });
    (unsafe { test_null_hints_2() });
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
