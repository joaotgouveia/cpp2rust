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
    let __tmp_0: Value<()> = Rc::new(RefCell::new(5_()));
    let mut r: Ptr<()> = <Ptr<()>>::default();
    goto_block!({
        '__entry: {
            r = __tmp_0.as_pointer();
            goto!('body);
        }
        'body: {
            unimplemented!();
            return unimplemented!();
        }
    });
    panic!("ub: non-void function does not return a value")
}
pub fn __cpp2rust_init_globals() {}
