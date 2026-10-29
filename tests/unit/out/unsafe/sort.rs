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
    let mut v: std_vector_int__std_allocator_int__ =
        <std_vector_int__std_allocator_int__>::default();
    (unsafe {
        let mut ___x: () = 10_();
        v.push_back_58(&mut ___x)
    });
    (unsafe {
        let mut ___x: () = 1_();
        v.push_back_58(&mut ___x)
    });
    (unsafe {
        let mut ___x: () = 9_();
        v.push_back_58(&mut ___x)
    });
    (unsafe {
        let mut ___x: () = 2_();
        v.push_back_58(&mut ___x)
    });
    (unsafe {
        let mut ___x: () = 8_();
        v.push_back_58(&mut ___x)
    });
    (unsafe {
        let mut ___x: () = 3_();
        v.push_back_58(&mut ___x)
    });
    (unsafe {
        let mut ___x: () = 7_();
        v.push_back_58(&mut ___x)
    });
    (unsafe {
        let mut ___x: () = 4_();
        v.push_back_58(&mut ___x)
    });
    (unsafe {
        let mut ___x: () = 5_();
        v.push_back_58(&mut ___x)
    });
    (unsafe {
        let mut ___x: () = 6_();
        v.push_back_58(&mut ___x)
    });
    (unsafe {
        let ___first: __gnu_cxx___normal_iterator_int_ptr__std_vector_int__std_allocator_int___ =
            (unsafe { v.begin_26() });
        let ___last: __gnu_cxx___normal_iterator_int_ptr__std_vector_int__std_allocator_int___ =
            (unsafe { v.end_28() });
        sort_0(___first, ___last)
    });
    let mut i: () = 0_();
    'loop_: while ((i) < ((unsafe { v.size() }).wrapping_sub(1_()))) {
        unimplemented!();
        i.prefix_inc();
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
