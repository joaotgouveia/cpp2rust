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
    pub before: (),
    pub mask: [(); 4],
    pub after: (),
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut s: *mut S = (unimplemented!() as *mut S);
    unimplemented!();
    (*s).before = 1_();
    (unsafe {
        let ___s: () = ((*s).mask.as_mut_ptr() as ());
        let ___n: () = ::std::mem::size_of::<[(); 4]>();
        memset_0(___s, 5_(), ___n)
    });
    (*s).after = 2_();
    (*((&mut (*s).mask as *mut [(); 4]) as *mut ())) = 7_();
    let mut out: [(); 4] = [unimplemented!(); 4];
    (unsafe {
        let ___src: () = ((&mut (*s).mask as *mut [(); 4]) as ());
        let ___n: () = ::std::mem::size_of::<[(); 4]>();
        memcpy_1((out.as_mut_ptr() as ()), ___src, ___n)
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
