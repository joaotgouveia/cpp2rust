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
    let d: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
    let x: Ptr<i32> = {
        let __init = 1;
        d.as_pointer()
            .with_mut(|__v: &mut Vec<i32>| __v.push(__init));
        d.as_pointer().decay().to_last()
    };
    assert!(((x.read()) == 1));
    {
        let __init = 2;
        d.as_pointer()
            .with_mut(|__v: &mut Vec<i32>| __v.push(__init));
        d.as_pointer().decay().to_last()
    };
    let y: Ptr<i32> = {
        let __init = 3;
        d.as_pointer()
            .with_mut(|__v: &mut Vec<i32>| __v.push(__init));
        d.as_pointer().decay().to_last()
    };
    y.write(30);
    assert!(((d.decay().to_last().read()) == 30));
    assert!(((d.decay().read()) == 1));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
