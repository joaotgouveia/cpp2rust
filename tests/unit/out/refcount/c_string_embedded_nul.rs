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
    let buf: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        ('a' as ()),
        ('b' as ()),
        ('\0' as ()),
        ('c' as ()),
        ('d' as ()),
        ('\0' as ()),
    ])));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let s: Value<std_basic_string_char__std_char_traits_char___std_allocator_char__> =
        Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { (buf.as_pointer() as Ptr<()>) },
                None,
            ),
        ));
    println!("{}", ({ (*s.borrow()).size() }));
    println!(
        "{} {}",
        unimplemented!(),
        ({ (*s.borrow()).find_last_of_179((buf.as_pointer() as Ptr::<()>), None,) })
    );
    let mut lit: Ptr<()> = Ptr::<()>::from_string_literal(b"xy\0zw");
    let t: Value<std_basic_string_char__std_char_traits_char___std_allocator_char__> =
        Rc::new(RefCell::new(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { (lit).clone() },
                None,
            ),
        ));
    println!("{}", ({ (*t.borrow()).size() }));
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
