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
    let mut path: Ptr<()> = Ptr::<()>::from_string_literal(b"cpp2rust_lseek_ftruncate_test.tmp");
    let mut fd: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let buf: Value<Box<[()]>> = Rc::new(RefCell::new(
        (0..16).map(|_| unimplemented!()).collect::<Box<[()]>>(),
    ));
    ({
        let ___s: () = ((buf.as_pointer() as Ptr<()>) as Ptr<()>).to_any();
        let ___n: () = ::std::mem::size_of::<[(); 16]>();
        memset_0(___s, 0_(), ___n)
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
