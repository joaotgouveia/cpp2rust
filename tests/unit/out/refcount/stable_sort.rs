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
fn main_0() -> i32 {
    let arr1: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([5, 2, 8, 1, 3])));
    ({
        let ___first: Ptr<i32> = (arr1.as_pointer() as Ptr<i32>);
        let ___last: Ptr<i32> = (arr1.as_pointer() as Ptr<i32>).offset((5) as isize);
        stable_sort_0(
            ___first,
            ___last,
            FnPtr::<fn(i32, i32) -> bool>::new(|mut x: i32, mut y: i32| -> bool {
                {
                    return (x < y);
                }
            }),
        )
    });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
