extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn strlen_0(mut ptr: Ptr<()>) -> () {
    let mut count: () = unimplemented!();
    'loop_: while ((ptr.postfix_inc().read()) != ('\0' as ())) {
        count.prefix_inc();
    }
    return count;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let string: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        ('h' as ()),
        ('e' as ()),
        ('l' as ()),
        ('l' as ()),
        ('o' as ()),
        ('\0' as ()),
    ])));
    unimplemented!();
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
