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
        { Ptr::<()>::from_string_literal(b"ABCD") },
        None,
    );
    let mut bytes: Ptr<u8> = unimplemented!().reinterpret_cast::<u8>();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
