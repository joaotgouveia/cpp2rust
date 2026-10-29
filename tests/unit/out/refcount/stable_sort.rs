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
    let arr1: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([5_(), 2_(), 8_(), 1_(), 3_()])));
    ({
        let ___first: Ptr<()> = (arr1.as_pointer() as Ptr<()>);
        let ___last: Ptr<()> = (arr1.as_pointer() as Ptr<()>).offset((5_()) as isize);
        stable_sort_0(
            ___first,
            ___last,
            FnPtr::<fn((), ()) -> ()>::new(|mut x: (), mut y: ()| -> () {
                {
                    return (x < y);
                }
            }),
        )
    });
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
