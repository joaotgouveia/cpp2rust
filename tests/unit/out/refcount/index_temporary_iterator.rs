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
    let v: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([1, 2, 3])));
    assert!(
        ((({
            let mut __it = v.decay().end();
            __it.iter_dec();
            __it
        })
        .iter_at(0_isize)
        .read())
            == 3)
    );
    assert!(
        ((({
            let mut __it = v.decay().end();
            __it.iter_dec();
            __it
        })
        .iter_deref()
        .read())
            == 3)
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
