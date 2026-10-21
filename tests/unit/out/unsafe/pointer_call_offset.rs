extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn foo_0(mut p: *mut ()) -> *mut () {
    return (&mut (*p.offset((5_()) as isize)) as *mut ());
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut p1: *mut () =
        Box::leak((0..10_()).map(|_| unimplemented!()).collect::<Box<[()]>>()).as_mut_ptr();
    let mut i: () = unimplemented!();
    'loop_: while ((i) < (10_())) {
        (*p1.offset((i) as isize)) = i;
        i.prefix_inc();
    }
    let mut out: () = (*(unsafe { foo_0((&mut (*p1.offset((1_()) as isize)) as *mut ())) })
        .offset((3_()) as isize));
    {
        let __p = p1;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                __p,
                libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                    / ::std::mem::size_of::<()>(),
            )))
        }
    };
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
