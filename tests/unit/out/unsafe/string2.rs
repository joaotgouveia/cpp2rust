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
    let mut arr: std_basic_string_char__std_char_traits_char___std_allocator_char__ =
        std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"foo".as_ptr() },
            None,
        );
    arr[(1_()) as usize] = ('b' as ());
    let mut p: *const () = (unsafe { arr.c_str() }).offset((1_()) as isize);
    unimplemented!();
    unimplemented!();
    let mut names: [*mut (); 1] = [(unsafe { arr.data_159() })];
    let copy: std_basic_string_char__std_char_traits_char___std_allocator_char__ =
        std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { (names[(0_()) as usize]).cast_const() },
            None,
        );
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
