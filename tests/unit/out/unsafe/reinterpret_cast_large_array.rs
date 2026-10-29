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
    let N: () = 10000_();
    let mut arr: *mut () =
        Box::leak((0..N).map(|_| unimplemented!()).collect::<Box<[()]>>()).as_mut_ptr();
    let mut i: () = 0_();
    'loop_: while ((i) < (N)) {
        (*arr.offset((i) as isize)) = 0_();
        i.postfix_inc();
    }
    (*arr.offset(((N) - (1_())) as isize)) = 3148519816_();
    let mut words: *mut () = (arr as *mut ());
    unimplemented!();
    unimplemented!();
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
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
