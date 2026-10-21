extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn inc_0(mut x: ()) -> () {
    return (x + 1_());
}
pub fn dec_1(mut x: ()) -> () {
    return (x - 1_());
}
pub fn pick_2(mut choose_inc: ()) -> FnPtr<fn(()) -> ()> {
    if (choose_inc != 0) {
        return FnPtr::<fn(()) -> ()>::new(inc_0);
    }
    return FnPtr::<fn(()) -> ()>::new(dec_1);
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut f: FnPtr<fn(()) -> ()> = ({ pick_2(1_()) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut g: FnPtr<fn(()) -> ()> = ({ pick_2(0_()) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
