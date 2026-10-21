extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn sum_0(mut args_0: (), mut args_1: (), mut args_2: ()) -> () {
    return (args_0 + (args_1 + (args_2 + unimplemented!())));
}
pub fn sum_1(mut args_0: (), mut args_1: ()) -> () {
    return (args_0 + (args_1 + unimplemented!()));
}
pub fn sum_2(mut args: ()) -> () {
    return (args + unimplemented!());
}
pub fn first_3(mut x: (), mut args_1: (), mut args_2: ()) -> () {
    return (x + ({ sum_1(args_1, args_2) }));
}
pub fn first_4(mut x: (), mut args: ()) -> () {
    return (x + ({ sum_2(args) }));
}
pub fn sizeof_pack_5() -> () {
    return 0;
}
pub fn sizeof_pack_6(mut args: ()) -> () {
    return 1;
}
pub fn sizeof_pack_7(mut args_0: (), mut args_1: ()) -> () {
    return 2;
}
pub fn sizeof_pack_8(mut args_0: (), mut args_1: (), mut args_2: ()) -> () {
    return 3;
}
pub fn sizeof_pack_9(mut args_0: (), mut args_1: (), mut args_2: (), mut args_3: ()) -> () {
    return 4;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
