extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test01_0() {
    let v: Value<Vec<bool>> = Rc::new(RefCell::new(Vec::new()));
    match ({ v.decay().begin() }, { v.decay().begin() }) {
        (mut __tmp_0, mut __tmp_1) => {
            let mut __it = (__tmp_0).clone();
            while __it != __tmp_1 {
                __it.iter_deref().write(Clone::clone(&false));
                __it.iter_inc();
            }
        }
    };
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ test01_0() });
    0
}
pub fn __cpp2rust_init_globals() {}
