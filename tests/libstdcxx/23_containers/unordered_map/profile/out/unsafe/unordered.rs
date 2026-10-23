extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_unordered_set_0() {
    let mut tmp2: *mut () = std::ptr::null_mut();
    tmp2 = (Box::leak(Box::new(unimplemented!())) as *mut ());
    unimplemented!();
    {
        let __p = tmp2;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(__p))
        }
    };
}
pub unsafe fn test_unordered_map_1() {
    let mut tmp: *mut () = std::ptr::null_mut();
    let mut i: () = 0_();
    'loop_: while ((i) < (20_())) {
        tmp = (Box::leak(Box::new(unimplemented!())) as *mut ());
        let mut j: () = 0_();
        'loop_: while ((j) < (10000_())) {
            (*unimplemented!()) = j;
            j.postfix_inc();
        }
        {
            let __p = tmp;
            if !__p.is_null() {
                ::std::mem::drop(Box::from_raw(__p))
            }
        };
        i.postfix_inc();
    }
    tmp = (Box::leak(Box::new(unimplemented!())) as *mut ());
    let mut i: () = 0_();
    'loop_: while ((i) < (150000_())) {
        unimplemented!();
        i.postfix_inc();
    }
    unimplemented!();
    {
        let __p = tmp;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(__p))
        }
    };
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    (unsafe { test_unordered_set_0() });
    (unsafe { test_unordered_map_1() });
    0
}
pub unsafe fn __cpp2rust_init_globals() {}
