extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test01_0() {
    let werte: [(); 5] = [1_(), 25_(), 9_(), 16_(), -36_()];
    let anzahl: () =
        (::std::mem::size_of::<[(); 5]>() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    let mut intTable : __gnu_cxx_hash_set_int____gnu_cxx_hash_int___std_equal_to_int___std_allocator_int__ = __gnu_cxx_hash_set_int____gnu_cxx_hash_int___std_equal_to_int___std_allocator_int__ :: new_1 ( { werte .as_ptr() } , { werte .as_ptr() . offset ( ( anzahl ) as isize ) } , )  ;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    (unsafe { test01_0() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
