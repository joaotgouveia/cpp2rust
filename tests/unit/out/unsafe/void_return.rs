extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn f1_0(mut first: *mut (), mut last: *mut ()) {
    (unsafe { sort_1(first, last) });
    return;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut v: std_vector_int__std_allocator_int__ =
        std_vector_int__std_allocator_int__::new_1({ unimplemented!() }, None);
    (unsafe {
        let _first: __gnu_cxx___normal_iterator_int_ptr__std_vector_int__std_allocator_int___ =
            (unsafe { v.begin_26() });
        let _last: __gnu_cxx___normal_iterator_int_ptr__std_vector_int__std_allocator_int___ =
            (unsafe { v.end_28() });
        f1_2(_first, _last)
    });
    let mut a: [(); 3] = [1_(), 2_(), 3_()];
    (unsafe {
        let _first: *mut () = a.as_mut_ptr();
        let _last: *mut () = a.as_mut_ptr().offset((2_()) as isize);
        f1_0(_first, _last)
    });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
