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
    let fds: Value<Box<[()]>> = Rc::new(RefCell::new(
        (0..2).map(|_| unimplemented!()).collect::<Box<[()]>>(),
    ));
    unimplemented!();
    let rset: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    let tv: Value<::libc::timeval> = Rc::new(RefCell::new(unsafe { std::mem::zeroed() }));
    ({
        let ___s: () = ((tv.as_pointer()) as Ptr<::libc::timeval>).to_any();
        let ___n: () = 16usize;
        memset_0(___s, 0_(), ___n)
    });
    (*tv.borrow_mut()).tv_sec = 0_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (*tv.borrow_mut()).tv_sec = 1_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
