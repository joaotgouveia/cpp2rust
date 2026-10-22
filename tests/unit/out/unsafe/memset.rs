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
    let N: () = 3_();
    let mut arr: *mut () =
        Box::leak((0..N).map(|_| unimplemented!()).collect::<Box<[()]>>()).as_mut_ptr();
    (unsafe {
        let ___s: () = (arr as ());
        memset_0(
            ___s,
            1_(),
            (::std::mem::size_of::<()>() as ()).wrapping_mul(N),
        )
    });
    let mut sum: () = 0_();
    let mut i: () = 0_();
    'loop_: while ((i) < (N)) {
        sum += (*arr.offset((i) as isize));
        i.prefix_inc();
    }
    {
        let __p = arr;
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
