extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut g_0: std::cell::LazyCell<std_vector_unsigned_char__std_allocator_unsigned_char__> =
    std::cell::LazyCell::new(|| unsafe {
        std_vector_unsigned_char__std_allocator_unsigned_char__::new_1({ 3_() }, None)
    });
pub unsafe fn get_1() -> *mut std_vector_unsigned_char__std_allocator_unsigned_char__ {
    return &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut g_0));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    (&mut (*(unsafe { get_1() })))[(1_()) as usize] = 4_();
    let mut v : std_unique_ptr_std_vector_unsigned_char__std_allocator_unsigned_char____std_default_delete_std_vector_unsigned_char__std_allocator_unsigned_char____ = std_unique_ptr_std_vector_unsigned_char__std_allocator_unsigned_char____std_default_delete_std_vector_unsigned_char__std_allocator_unsigned_char____ :: new_1 ( { (Box::leak(Box::new(std_vector_unsigned_char__std_allocator_unsigned_char__ :: new_1 ( { 3_() } , None , )  )) as *mut std_vector_unsigned_char__std_allocator_unsigned_char__ ) } , )  ;
    (*v)[(2_()) as usize] = 5_();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const g_0);
}
