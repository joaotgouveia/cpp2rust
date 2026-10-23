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
    let in_: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let it: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let end: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut ss: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
        { unimplemented!() },
        { unimplemented!() },
        None,
    );
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
