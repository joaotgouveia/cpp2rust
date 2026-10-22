extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg)]
pub struct UserDefined {
    pub a: (),
    pub v: (),
}
impl Default for UserDefined {
    fn default() -> Self {
        unsafe {
            UserDefined {
                a: unimplemented!(),
                v: unimplemented!(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct FieldIsLibcType {
    pub addr: libc::sockaddr,
}
impl Default for FieldIsLibcType {
    fn default() -> Self {
        unsafe {
            FieldIsLibcType {
                addr: unsafe { std::mem::zeroed() },
            }
        }
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut p: () = unimplemented!();
    p.fd = -1_();
    p.events = 0_();
    p.revents = 2_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut ia: ::libc::in_addr = unsafe { std::mem::zeroed() };
    ia.s_addr = (1 as ::libc::in_addr_t);
    unimplemented!();
    let mut t: ::libc::tm = unsafe { std::mem::zeroed() };
    t.tm_year = 124_();
    t.tm_mon = 5_();
    t.tm_mday = 15_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut st: () = unimplemented!();
    st.st_size = 1024_();
    unimplemented!();
    let mut ud: UserDefined = <UserDefined>::default();
    unimplemented!();
    unimplemented!();
    let mut filt: FieldIsLibcType = <FieldIsLibcType>::default();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
