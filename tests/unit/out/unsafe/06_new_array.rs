extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn take_0(p: *mut *mut ()) -> () {
    (*(*p).offset((0_()) as isize)) = 9_();
    let mut c: () = (*(*p).offset((0_()) as isize));
    {
        let __p = (*p);
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                __p,
                libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                    / ::std::mem::size_of::<()>(),
            )))
        }
    };
    return c;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut e: *mut () =
        Box::leak((0..2_()).map(|_| unimplemented!()).collect::<Box<[()]>>()).as_mut_ptr();
    (*e.offset((0_()) as isize)) = 6_();
    (*e.offset((1_()) as isize)) = 7_();
    {
        let __p = e;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                __p,
                libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                    / ::std::mem::size_of::<()>(),
            )))
        }
    };
    let mut c: () = (unsafe {
        let mut _p: *mut () =
            Box::leak((0..4_()).map(|_| unimplemented!()).collect::<Box<[()]>>()).as_mut_ptr();
        take_0(&mut _p)
    });
    &(c);
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
