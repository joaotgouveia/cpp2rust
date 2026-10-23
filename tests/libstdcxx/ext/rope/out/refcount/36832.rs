extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test01_0() {
    let myRope: Value<__gnu_cxx_rope_char__std_allocator_char__> = Rc::new(RefCell::new(
        <__gnu_cxx_rope_char__std_allocator_char__>::default(),
    ));
    (*myRope.borrow_mut()) = __gnu_cxx_rope_char__std_allocator_char__::new_1(
        { Ptr::<()>::from_string_literal(b"1234567890") },
        None,
    );
    let buffer: Value<Box<[()]>> = Rc::new(RefCell::new(
        (0..100).map(|_| unimplemented!()).collect::<Box<[()]>>(),
    ));
    ({ (*myRope.borrow()).copy_58(1_(), 1_(), (buffer.as_pointer() as Ptr<()>)) });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    ({ test01_0() });
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
