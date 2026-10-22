extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, FnPtrArg, VaArg)]
pub union basic {
    pub i: (),
    pub f: (),
}
impl Default for basic {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, FnPtrArg, VaArg)]
pub union empty {
    __empty: u8,
}
impl Default for empty {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, FnPtrArg, VaArg)]
pub union underscores {
    pub __: (),
    pub ___: (),
}
impl Default for underscores {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut u: basic = <basic>::default();
    let mut e: empty = <empty>::default();
    &(e);
    u.i = 42_();
    unimplemented!();
    u.f = 3.14_f32;
    unimplemented!();
    let mut buf: [(); 4] = [unimplemented!(); 4];
    (unsafe {
        let ___s: () = (buf.as_mut_ptr() as ());
        let ___n: () = ::std::mem::size_of::<[(); 4]>();
        memset_0(___s, 0_(), ___n)
    });
    let mut ru: *mut basic = (buf.as_mut_ptr() as *mut basic);
    let mut pi: *mut () = (&mut (*ru).i as *mut ());
    let mut pf: *mut () = (&mut (*ru).f as *mut ());
    (*ru).i = 7_();
    unimplemented!();
    (*pi) = 1065353216_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut braced: basic = basic { i: 5_() };
    unimplemented!();
    let mut copied: basic = basic { i: 7_() };
    unimplemented!();
    {
        let mut anon_1: anon_1 = <anon_1>::default();
        anon_1.ai = 11_();
        unimplemented!();
        anon_1.ai = 1065353216_();
        unimplemented!();
    }
    let mut us: underscores = <underscores>::default();
    us.__ = 5_();
    unimplemented!();
    us.___ = 2.5_f64;
    unimplemented!();
    let mut __: () = 1_();
    let mut ___: () = 2_();
    unimplemented!();
    return 0_();
}
#[repr(C)]
#[derive(Copy, Clone, FnPtrArg, VaArg)]
pub union anon_1 {
    pub ai: (),
    pub af: (),
}
impl Default for anon_1 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
pub unsafe fn __cpp2rust_init_globals() {}
