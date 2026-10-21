extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct S {
    pub keep: (),
    pub a: (),
    pub b: (),
    pub c: [(); 5],
    pub last: (),
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut p: *mut S = (unimplemented!() as *mut S);
    unimplemented!();
    (unsafe {
        let ___s: () = (p as ());
        let ___n: () = ::std::mem::size_of::<S>();
        memset_0(___s, 255_(), ___n)
    });
    (*p).keep = 7_();
    (unsafe {
        memset_0(
            ((&mut (*p).a as *mut ()) as ()),
            unimplemented!(),
            (::std::mem::size_of::<S>() as ()).wrapping_sub((::std::mem::offset_of!(S, a) as ())),
        )
    });
    unimplemented!();
    unimplemented!();
    (*p).a = 1_();
    (*p).b = 2_();
    (*p).c[(unimplemented!()) as usize] = ('x' as ());
    (*p).last = 3_();
    (unsafe {
        memset_0(
            ((&mut (*p).b as *mut ()) as ()),
            unimplemented!(),
            (::std::mem::offset_of!(S, last) as ())
                .wrapping_sub((::std::mem::offset_of!(S, b) as ())),
        )
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
