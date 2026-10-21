extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type E = ();
pub const E_A: E = 0;
pub const E_B: E = 1;
thread_local!(
    pub static global_0: Value<()> = Rc::new(RefCell::new(((E_A as ()) != (E_B as ()))));
);
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {
    let _ = global_0.with(|_| ());
}
