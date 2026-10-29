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
    let mut n: () = 17_();
    let d: Value<std_chrono_day> = Rc::new(RefCell::new(std_chrono_day::new_1({ n })));
    unimplemented!();
    let name: Value<Ptr<()>> = Rc::new(RefCell::new(Ptr::<()>::from_string_literal(
        b"dir/file.txt",
    )));
    let p: Value<std_filesystem_path> = Rc::new(RefCell::new(std_filesystem_path::new_1(
        { name.as_pointer() },
        None,
    )));
    let s: Value<std_basic_string_char__std_char_traits_char___std_allocator_char__> =
        Rc::new(RefCell::new(({ (*p.borrow())() })));
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
