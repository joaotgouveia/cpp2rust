extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn identity_0(mut x: ()) -> () {
    return x;
}
pub fn apply_1(mut x: (), fn_: Option<FnPtr<fn(()) -> ()>>) -> () {
    let mut fn_: FnPtr<fn(()) -> ()> = fn_.unwrap_or_else(|| FnPtr::<fn(()) -> ()>::null());
    if !(fn_).is_null() {
        return ({ fn_.call(x) });
    }
    return x;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut negate: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut x: ()| -> () {
        {
            return -x;
        }
    });
    unimplemented!();
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
