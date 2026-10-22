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
    let buf: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_(), 4_(), 5_(), 6_()])));
    ({
        let ___dest: () =
            ((buf.as_pointer() as Ptr<()>).offset((2_()) as isize) as Ptr<()>).to_any();
        let ___src: () = ((buf.as_pointer() as Ptr<()>) as Ptr<()>).to_any();
        memmove_0(___dest, ___src, 4_())
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
