extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct point {
    #[offset(0)]
    pub x: i32,
    #[offset(4)]
    pub y: i32,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let src: Value<point> = Rc::new(RefCell::new(point {
        x: (3_() as i32),
        y: (7_() as i32),
    }));
    let buf: Value<Box<[()]>> = Rc::new(RefCell::new(
        (0..8).map(|_| unimplemented!()).collect::<Box<[()]>>(),
    ));
    ({
        let ___dest: () = ((buf.as_pointer() as Ptr<()>) as Ptr<()>).to_any();
        let ___n: () = ::std::mem::size_of::<[(); 8]>();
        memcpy_0(___dest, ((src.as_pointer()) as Ptr<point>).to_any(), ___n)
    });
    let dst: Value<point> = <Value<point>>::default();
    ({
        let ___dest: () = ((dst.as_pointer()) as Ptr<point>).to_any();
        let ___n: () = 8usize;
        memcpy_0(
            ___dest,
            ((buf.as_pointer() as Ptr<()>) as Ptr<()>).to_any(),
            ___n,
        )
    });
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
