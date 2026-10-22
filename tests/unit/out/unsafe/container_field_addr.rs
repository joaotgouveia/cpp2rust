extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, Default)]
pub struct S {
    pub tag: (),
    pub v: (),
}
pub unsafe fn add_0(mut v: *mut (), mut k: ()) {
    unimplemented!();
}
pub unsafe fn run_1(mut h: *mut S) {
    (unsafe {
        let _v: *mut () = (&mut (*h).v as *mut ());
        let _k: () = (*h).tag;
        add_0(_v, _k)
    });
    let mut pv: *mut () = (&mut (*h).v as *mut ());
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
    let mut local: S = <S>::default();
    local.tag = 7_();
    (unsafe { run_1((&mut local as *mut S)) });
    let mut heap: *mut S = (Box::leak(Box::new(<S>::default())) as *mut S);
    (*heap).tag = 7_();
    (unsafe { run_1(heap) });
    {
        let __p = heap;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(__p))
        }
    };
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
