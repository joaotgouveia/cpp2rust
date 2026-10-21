extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn bump_0(mut arg: ()) -> i32 {
    let mut value: Ptr<i32> = arg.reinterpret_cast::<i32>();
    value.write({ (((value.read()) as ()) + 1_()) as i32 });
    return (value.read());
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let value: Value<i32> = Rc::new(RefCell::new((41_() as i32)));
    let mut opaque: () = ((value.as_pointer()) as Ptr<i32>).to_any();
    let mut typed: Ptr<i32> = opaque.reinterpret_cast::<i32>();
    unimplemented!();
    unimplemented!();
    typed.write((7_() as i32));
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
