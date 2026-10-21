extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(12)]
pub struct Layout {
    #[offset(0)]
    pub a: (),
    #[offset(4)]
    pub b: (),
    #[offset(8)]
    pub c: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(66)]
pub struct Frame {
    #[offset(0)]
    pub tag: (),
    #[offset(2)]
    #[byte_size(64)]
    pub body: Box<[()]>,
}
impl Default for Frame {
    fn default() -> Self {
        {
            Frame {
                tag: unimplemented!(),
                body: (0..64).map(|_| unimplemented!()).collect::<Box<[()]>>(),
            }
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let v: Value<Layout> = Rc::new(RefCell::new(Layout {
        a: unimplemented!(),
        b: unimplemented!(),
        c: unimplemented!(),
    }));
    (*v.borrow_mut()).b = 3735928559_();
    let mut base: Ptr<()> = (v.as_pointer()).reinterpret_cast::<()>();
    let mut bp: Ptr<()> = (base.offset((4_usize) as isize)).reinterpret_cast::<()>();
    unimplemented!();
    (base.offset((4_usize) as isize))
        .reinterpret_cast::<()>()
        .write(305419896_());
    unimplemented!();
    let mut text: Ptr<()> = Ptr::<()>::from_string_literal(b"example-body");
    let mut len: () = (unimplemented!()).wrapping_add(1_());
    let mut total: () = (2_usize as ()).wrapping_add(len);
    unimplemented!();
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
