extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn first_0(u: *const ()) -> *mut u8 {
    return (unimplemented!() as *mut ()).cast::<u8>();
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut v : () = std_unique_ptr_std_vector_unsigned_char__std_allocator_unsigned_char____std_default_delete_std_vector_unsigned_char__std_allocator_unsigned_char____ :: new ( { (Box::leak(Box::new(unimplemented!() )) as *mut () ) } , )  ;
    let mut p: *mut u8 = (unimplemented!() as *mut ()).cast::<u8>();
    (*p.offset((2_()) as isize)) = 1_u8;
    (*(unsafe { first_0(&v) }).offset((0_()) as isize)) = 5_u8;
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
