extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
        { 512_() },
        { ('1' as ()) },
        None,
    );
    unimplemented!().write(('.' as ()));
    let mut d: () = unimplemented!();
    unimplemented!();
    0
}
pub fn __cpp2rust_init_globals() {}
