extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn bump_0(mut arg: ()) -> () {
    let mut value: Ptr<()> = arg.reinterpret_cast::<()>();
    {
        value.with_mut(|__v| *__v = *__v + 1_())
    };
    return (value.read());
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let value: Value<()> = Rc::new(RefCell::new(41_()));
    let mut opaque: () = ((value.as_pointer()) as Ptr<()>).to_any();
    let mut typed: Ptr<()> = opaque.reinterpret_cast::<()>();
    unimplemented!();
    unimplemented!();
    typed.write(7_());
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
