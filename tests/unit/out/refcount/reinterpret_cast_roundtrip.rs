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
    let original: Value<i32> = Rc::new(RefCell::new((67305985_() as i32)));
    let mut halves: Ptr<i16> = (original.as_pointer()).reinterpret_cast::<i16>();
    unimplemented!();
    unimplemented!();
    elem!(halves, 0_()).write(4112_i16);
    unimplemented!();
    let arr: Value<Box<[i16]>> = Rc::new(RefCell::new(Box::new([513_i16, 1027_i16])));
    let mut as_int: Ptr<i32> = (arr.as_pointer() as Ptr<i16>).reinterpret_cast::<i32>();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
