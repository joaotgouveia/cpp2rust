extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn sum_0(mut p: *mut (), mut n: ()) -> () {
    let mut total: () = 0_();
    let mut i: () = 0_();
    'loop_: while ((i) < (n)) {
        total += (*p.offset((i) as isize));
        i.prefix_inc();
    }
    return total;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut array: *mut () =
        Box::leak((0..100_()).map(|_| unimplemented!()).collect::<Box<[()]>>()).as_mut_ptr();
    {
        let __p = array;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                __p,
                libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                    / ::std::mem::size_of::<()>(),
            )))
        }
    };
    let mut filled: *mut () =
        Box::leak((0..4_()).map(|_| unimplemented!()).collect::<Box<[()]>>()).as_mut_ptr();
    let mut i: () = 0_();
    'loop_: while ((i) < (4_())) {
        (*filled.offset((i) as isize)) = ((i) + (1_()));
        i.prefix_inc();
    }
    if ((unsafe {
        let _p: *mut () = filled;
        sum_0(_p, 4_())
    }) != (10_()))
    {
        return 1_();
    }
    {
        let __p = filled;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                __p,
                libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                    / ::std::mem::size_of::<()>(),
            )))
        }
    };
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
