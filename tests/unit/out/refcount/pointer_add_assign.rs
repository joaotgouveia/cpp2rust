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
    let arr: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_()])));
    {
        let mut p: Ptr<()> = ((arr.as_pointer() as Ptr<()>).offset(0_()));
        p += (1_u8 as ());
        unimplemented!();
    }
    {
        let mut p: Ptr<()> = ((arr.as_pointer() as Ptr<()>).offset(0_()));
        p += (1_i8 as ());
        unimplemented!();
    }
    {
        let mut p: Ptr<()> = ((arr.as_pointer() as Ptr<()>).offset(0_()));
        p += (1_u16 as ());
        unimplemented!();
    }
    {
        let mut p: Ptr<()> = ((arr.as_pointer() as Ptr<()>).offset(0_()));
        p += (1_i16 as ());
        unimplemented!();
    }
    {
        let mut p: Ptr<()> = ((arr.as_pointer() as Ptr<()>).offset(0_()));
        p += 1_u32;
        unimplemented!();
    }
    {
        let mut p: Ptr<()> = ((arr.as_pointer() as Ptr<()>).offset(0_()));
        p += (1_() as i32);
        unimplemented!();
    }
    {
        let mut p: Ptr<()> = ((arr.as_pointer() as Ptr<()>).offset(0_()));
        p += 1_u64;
        unimplemented!();
    }
    {
        let mut p: Ptr<()> = ((arr.as_pointer() as Ptr<()>).offset(0_()));
        p += 1_i64;
        unimplemented!();
    }
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
