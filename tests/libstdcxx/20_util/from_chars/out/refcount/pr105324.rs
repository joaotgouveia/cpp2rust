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
    let s: Value<Vec<i8>> = Rc::new(RefCell::new(
        vec![('1' as i8); (512_usize) as usize]
            .iter()
            .cloned()
            .chain(std::iter::once(0_i8))
            .collect(),
    ));
    s.decay().offset(1_usize as isize).write(('.' as i8));
    let mut d: f64 = 0_f64;
    unimplemented!();
    0
}
pub fn __cpp2rust_init_globals() {}
