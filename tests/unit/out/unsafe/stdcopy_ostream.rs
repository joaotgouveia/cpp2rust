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
    let mut str: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
        { c"Hello, world!\n".as_ptr() },
        None,
    );
    let file: [(); 25] = std::mem::transmute(*b"test_stdcopy_ostream.txt\0");
    {
        let mut ofs: () = unimplemented!();
        unimplemented!();
    }
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
