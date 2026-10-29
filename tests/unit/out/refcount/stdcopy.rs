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
    let input: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_()])));
    let output: Value<Box<[()]>> = Rc::new(RefCell::new(
        (0..3).map(|_| unimplemented!()).collect::<Box<[()]>>(),
    ));
    ({
        let ___first: Ptr<()> = (input.as_pointer() as Ptr<()>);
        let ___last: Ptr<()> = (input.as_pointer() as Ptr<()>).offset((3_()) as isize);
        copy_0(___first, ___last, (output.as_pointer() as Ptr<()>))
    });
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
