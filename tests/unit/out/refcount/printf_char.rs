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
    let mut c: () = ('a' as ());
    let mut n: () = 3_();
    println!("{}", c as u8 as char);
    println!("{} {}", n, c as u8 as char);
    println!("100% {}", c as u8 as char);
    println!("{}{}{}", c as u8 as char, (c + 1_()) as u8 as char, n);
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
