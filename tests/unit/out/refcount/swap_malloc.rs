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
    let mut buf: Ptr<()> = unimplemented!().reinterpret_cast::<()>();
    elem!(buf, 0_()).write(1_());
    elem!(buf, 1_()).write(2_());
    elem!(buf, 2_()).write(3_());
    ({
        let ___a: Ptr<()> = buf.offset((0_()) as isize);
        let ___b: Ptr<()> = buf.offset((2_()) as isize);
        swap_0(___a, ___b)
    });
    unimplemented!();
    ({
        let ___a: Ptr<()> = buf.offset((1_()) as isize);
        let ___b: Ptr<()> = buf.offset((1_()) as isize);
        swap_0(___a, ___b)
    });
    unimplemented!();
    let local: Value<()> = Rc::new(RefCell::new(4_()));
    ({
        let ___a: Ptr<()> = buf.offset((1_()) as isize);
        let ___b: Ptr<()> = local.as_pointer();
        swap_0(___a, ___b)
    });
    unimplemented!();
    let mut other: Ptr<()> = unimplemented!().reinterpret_cast::<()>();
    other.write(5_());
    ({
        let ___a: Ptr<()> = (buf).clone();
        let ___b: Ptr<()> = (other).clone();
        swap_0(___a, ___b)
    });
    unimplemented!();
    let mut bytes: Ptr<()> = buf.reinterpret_cast::<()>();
    elem!(bytes, 0_()).write(170_());
    elem!(bytes, 4_()).write(187_());
    ({
        let ___a: Ptr<()> = bytes.offset((0_()) as isize);
        let ___b: Ptr<()> = bytes.offset((4_()) as isize);
        swap_1(___a, ___b)
    });
    unimplemented!();
    let words: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([16909060_(), 84281096_()])));
    let mut halves: Ptr<()> = (words.as_pointer() as Ptr<()>).reinterpret_cast::<()>();
    ({
        let ___a: Ptr<()> = halves.offset((0_()) as isize);
        let ___b: Ptr<()> = halves.offset((3_()) as isize);
        swap_2(___a, ___b)
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
