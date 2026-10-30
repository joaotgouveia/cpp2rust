extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn fn_0(mut v: ()) -> () {
    return unimplemented!();
}
pub fn fn2_1(v: Ptr<()>) -> Ptr<()> {
    return (v).clone();
}
pub fn log_to_2(mut out: (), mut fmt: Ptr<()>, mut args_2: Ptr<()>, mut args_3: ()) {
    unimplemented!();
}
pub fn log_to_3(mut out: (), mut fmt: Ptr<()>, mut args: ()) {
    unimplemented!();
}
pub fn log_fmt_4(mut fmt: Ptr<()>, mut v: ()) {
    unimplemented!();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    println!("{}", Ptr::<()>::from_string_literal(b"fprintf stdout"));
    println!("{} {} {}", 1_(), 2_(), 3_());
    print!("hello world");
    let mut in_: () = unimplemented!();
    unimplemented!();
    println!("{}", Ptr::<()>::from_string_literal(b"printf"));
    print!("hello world");
    let s: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    println!("{}", unimplemented!());
    println!("{}", unimplemented!());
    println!("{}", unimplemented!());
    ({
        log_to_2(
            unimplemented!(),
            Ptr::<()>::from_string_literal(b"%s %d\n"),
            Ptr::<()>::from_string_literal(b"runtime"),
            4_(),
        )
    });
    ({
        log_to_3(
            unimplemented!(),
            Ptr::<()>::from_string_literal(b"%d\n"),
            5_(),
        )
    });
    ({ log_fmt_4(Ptr::<()>::from_string_literal(b"%d\n"), 6_()) });
    println!("{{}} {} {}", 5_(), -6_());
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
