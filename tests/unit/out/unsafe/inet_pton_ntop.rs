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
    let mut addr: in6_addr = unsafe { std::mem::zeroed::<in6_addr>() };
    let mut ap: *mut in6_addr = (&mut addr as *mut in6_addr);
    (unsafe {
        memcpy_1(
            ((*(&mut addr as *mut ::libc::in6_addr)).s6_addr.as_mut_ptr() as ()),
            (loopback.as_ptr() as ()),
            16_(),
        )
    });
    unimplemented!();
    (*(&mut addr as *mut ::libc::in6_addr)).s6_addr[(0_()) as usize] = 32_u8;
    (*(ap as *mut ::libc::in6_addr)).s6_addr[(1_()) as usize] = 1_u8;
    unimplemented!();
    (*(&mut addr as *mut ::libc::in6_addr)).s6_addr[(15_()) as usize] =
        (((*(&mut addr as *mut ::libc::in6_addr)).s6_addr[(15_()) as usize] as ()) + 4_()) as u8;
    (*(ap as *mut ::libc::in6_addr)).s6_addr[(15_()) as usize].postfix_inc();
    unimplemented!();
    let mut bytes: *mut () =
        ((*(&mut addr as *mut ::libc::in6_addr)).s6_addr.as_mut_ptr() as *mut u8).cast::<()>();
    (*bytes.offset((2_()) as isize)) = 7_();
    unimplemented!();
    let mut fourth: *mut () = ((&mut (*(ap as *mut ::libc::in6_addr)).s6_addr[(3_()) as usize]
        as *mut u8) as *mut u8)
        .cast::<()>();
    (*fourth) = 8_();
    unimplemented!();
    (unsafe {
        memcpy_1(
            ((*(ap as *mut ::libc::in6_addr)).s6_addr.as_mut_ptr() as ()),
            (loopback.as_ptr() as ()),
            16_(),
        )
    });
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
