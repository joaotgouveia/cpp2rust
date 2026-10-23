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
    let mut tmp : *mut __gnu_cxx_hash_map_int__int____gnu_cxx_hash_int___std_equal_to_int___std_allocator_int__ = std::ptr::null_mut() ;
    let mut j: () = 1_();
    'loop_: while ((j) <= (10_())) {
        tmp  = (Box::leak(Box::new(<__gnu_cxx_hash_map_int__int____gnu_cxx_hash_int___std_equal_to_int___std_allocator_int__ >::default() )) as *mut __gnu_cxx_hash_map_int__int____gnu_cxx_hash_int___std_equal_to_int___std_allocator_int__ )  ;
        let mut i: () = 0_();
        'loop_: while ((i) < ((10000_()) * (j))) {
            (&mut (*tmp))[(i) as usize] = i;
            i.postfix_inc();
        }
        {
            let __p = tmp;
            if !__p.is_null() {
                ::std::mem::drop(Box::from_raw(__p))
            }
        };
        j.postfix_inc();
    }
    0
}
pub unsafe fn __cpp2rust_init_globals() {}
