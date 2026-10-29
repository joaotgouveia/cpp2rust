extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn base_name_0(mut path: Ptr<()>) -> Ptr<()> {
    let mut slash: Ptr<()> = unimplemented!();
    return if !(slash).is_null() {
        slash.offset((1_()) as isize)
    } else {
        path
    };
}
pub fn log_1(mut file: Ptr<()>, mut line: (), mut func: Ptr<()>) {
    println!("{} {} {}", file, line, func);
}
pub fn line_2() -> () {
    return line!() as ();
}
pub fn function_3() -> Ptr<()> {
    return Ptr::<()>::from_string_literal(b"function");
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    println!(
        "{} {} {}",
        Ptr::<()>::from_string_literal(b"macros.cpp"),
        19_(),
        Ptr::<()>::from_string_literal(b"main")
    );
    ({
        log_1(
            Ptr::<()>::from_string_literal(b"macros.cpp"),
            20_(),
            Ptr::<()>::from_string_literal(b"main"),
        )
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    println!("{}", ({ function_3() }));
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
