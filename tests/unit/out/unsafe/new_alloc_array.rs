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
    let mut array: *mut () =
        Box::leak((0..100_()).map(|_| unimplemented!()).collect::<Box<[()]>>()).as_mut_ptr();
    (unsafe {
        let ___s: () = (array as ());
        memset_0(
            ___s,
            0_(),
            (::std::mem::size_of::<()>() as ()).wrapping_mul(100_()),
        )
    });
    (*array.offset((99_()) as isize)) = -1_();
    let mut p1: *mut () = array;
    'loop_: while ((*p1) >= (0_())) {
        (*p1) = 1_();
        p1.prefix_inc();
    }
    let mut out: () = 0_();
    let mut p1: *mut () = array;
    'loop_: while ((*p1) >= (0_())) {
        out += (*p1);
        p1.prefix_inc();
    }
    let mut p2: *mut () = array;
    {
        let __p = p2;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                __p,
                libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                    / ::std::mem::size_of::<()>(),
            )))
        }
    };
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
