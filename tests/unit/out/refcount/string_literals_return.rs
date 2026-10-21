extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn get_greeting_0() -> Ptr<()> {
    return Ptr::<()>::from_string_literal(b"hello");
}
pub fn get_empty_1() -> Ptr<()> {
    return Ptr::<()>::from_string_literal(b"");
}
pub fn get_branch_2(mut x: ()) -> Ptr<()> {
    if (x > unimplemented!()) {
        return Ptr::<()>::from_string_literal(b"positive");
    }
    return Ptr::<()>::from_string_literal(b"non-positive");
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut a: Ptr<()> = ({ get_greeting_0() });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut b: Ptr<()> = ({ get_empty_1() });
    unimplemented!();
    let mut c: Ptr<()> = ({ get_branch_2(1_()) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut d: Ptr<()> = ({ get_branch_2(-1_()) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
