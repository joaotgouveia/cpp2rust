extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct N_X {}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut v: () = unimplemented!();
    let w: () = unimplemented!();
    &(unimplemented!());
    &(unimplemented!());
    &(unimplemented!());
    &(unimplemented!());
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (unsafe {
        let ___position: () = unimplemented!();
        let ___first: () = unimplemented!();
        let ___last: () = unimplemented!();
        v . insert_66_class___gnu_cxx___normal_iterator_class___gnu_cxx__Pointer_adapter_class___gnu_cxx__Relative_pointer_impl_const_struct_N_X_____class_std_vector_struct_N_X__class___gnu_cxx__ExtPtr_allocator_struct_N_X______void  ( ___position , ___first , ___last , )
    });
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
