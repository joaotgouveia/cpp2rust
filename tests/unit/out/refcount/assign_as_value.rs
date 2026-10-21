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
    let buf: Value<Box<[()]>> = Rc::new(RefCell::new(
        (0..2).map(|_| unimplemented!()).collect::<Box<[()]>>(),
    ));
    let mut p: Ptr<()> = (buf.as_pointer() as Ptr<()>);
    let mut q: Ptr<()> = Ptr::<()>::null();
    q = {
        p += 1_();
        (p).clone()
    };
    unimplemented!();
    let mut out: () = unimplemented!();
    'switch: {
        match {
            {
                out = ('x' as ());
                out
            }
        } {
            __v if __v == ('x' as ()) => {
                unimplemented!();
                break 'switch;
            }
            _ => {
                unimplemented!();
                break 'switch;
            }
        }
    };
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
