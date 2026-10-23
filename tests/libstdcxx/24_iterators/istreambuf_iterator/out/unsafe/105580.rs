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
    let mut in_: () = unimplemented!();
    let mut it: () = unimplemented!();
    let mut end: () = unimplemented!();
    let mut ss: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
        { unimplemented!() },
        { unimplemented!() },
        None,
    );
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
