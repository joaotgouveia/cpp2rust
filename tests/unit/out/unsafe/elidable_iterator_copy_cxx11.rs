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
        let mut ___x: () = 4_();
        v.push_back_58(&mut ___x)
    });
    (unsafe {
        let mut ___x: () = 7_();
        v.push_back_58(&mut ___x)
    });
    let mut it: __gnu_cxx___normal_iterator_int_ptr__std_vector_int__std_allocator_int___ = (unsafe {
        let ___first: __gnu_cxx___normal_iterator_int_ptr__std_vector_int__std_allocator_int___ =
            (unsafe { v.begin_26() });
        let ___last: __gnu_cxx___normal_iterator_int_ptr__std_vector_int__std_allocator_int___ =
            (unsafe { v.end_28() });
        let mut ___val: () = 7_();
        find_0(___first, ___last, &mut ___val)
    });
    unimplemented!();
    unimplemented!();
    (unsafe {
        let ___first: __gnu_cxx___normal_iterator_int_ptr__std_vector_int__std_allocator_int___ =
            (unsafe { v.begin_26() });
        let ___last: __gnu_cxx___normal_iterator_int_ptr__std_vector_int__std_allocator_int___ =
            (unsafe { v.end_28() });
        random_shuffle_1(___first, ___last)
    });
    (unsafe {
        let ___first: __gnu_cxx___normal_iterator_int_ptr__std_vector_int__std_allocator_int___ =
            (unsafe { v.begin_26() });
        let ___last: __gnu_cxx___normal_iterator_int_ptr__std_vector_int__std_allocator_int___ =
            (unsafe { v.end_28() });
        sort_2(___first, ___last)
    });
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
