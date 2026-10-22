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
    let mut arr: *mut u32 =
        Box::leak((0..2_()).map(|_| 0_u32).collect::<Box<[u32]>>()).as_mut_ptr();
    (*arr.offset((0_()) as isize)) = 67305985_u32;
    (*arr.offset((1_()) as isize)) = 134678021_u32;
    let mut bytes: *mut u8 = (arr as *mut u8);
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (*bytes.offset((0_()) as isize)) = 170_u8;
    unimplemented!();
    (*bytes.offset((5_()) as isize)) = 187_u8;
    unimplemented!();
    {
        let __p = arr;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                __p,
                libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                    / ::std::mem::size_of::<u32>(),
            )))
        }
    };
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
