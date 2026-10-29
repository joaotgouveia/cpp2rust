extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn first_0(
    u : *const std_unique_ptr_std_vector_unsigned_char__std_allocator_unsigned_char____std_default_delete_std_vector_unsigned_char__std_allocator_unsigned_char____,
) -> *mut () {
    return (unsafe { (*(*u)).data_55() });
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut v : std_unique_ptr_std_vector_unsigned_char__std_allocator_unsigned_char____std_default_delete_std_vector_unsigned_char__std_allocator_unsigned_char____ = std_unique_ptr_std_vector_unsigned_char__std_allocator_unsigned_char____std_default_delete_std_vector_unsigned_char__std_allocator_unsigned_char____ :: new_1 ( { (Box::leak(Box::new(std_vector_unsigned_char__std_allocator_unsigned_char__ :: new_1 ( { 3_() } , None , )  )) as *mut std_vector_unsigned_char__std_allocator_unsigned_char__ ) } , )  ;
    let mut p: *mut () = (unsafe { (*v).data_55() });
    (*p.offset((2_()) as isize)) = 1_();
    (*(unsafe { first_0(&v) }).offset((0_()) as isize)) = 5_();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
