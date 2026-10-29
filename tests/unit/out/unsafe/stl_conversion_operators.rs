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
    let mut n: () = 17_();
    let mut d: std_chrono_day = std_chrono_day::new_1({ n });
    unimplemented!();
    let mut name: *const () = c"dir/file.txt".as_ptr();
    let mut p: std_filesystem_path = std_filesystem_path::new_1({ &name }, None);
    let mut s: std_basic_string_char__std_char_traits_char___std_allocator_char__ =
        (unsafe { p() });
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
