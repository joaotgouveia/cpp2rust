extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct S {
    #[offset(0)]
    pub a: (),
    #[offset(4)]
    pub b: (),
}
impl S {
    pub fn new() -> Self {
        Self { a: 11_(), b: true }
    }
}
impl Default for S {
    fn default() -> Self {
        { S::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Declared {
    #[offset(0)]
    pub v: (),
}
impl Declared {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(16)]
pub struct Holder {
    #[offset(0)]
    #[byte_size(16)]
    pub items: Box<[S]>,
}
impl Default for Holder {
    fn default() -> Self {
        {
            Holder {
                items: (0..2).map(|_| <S>::default()).collect::<Box<[S]>>(),
            }
        }
    }
}
thread_local!(
    pub static kInit_0: Value<()> = Rc::new(RefCell::new(3_()));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct FromStatic {
    #[offset(0)]
    pub v: (),
}
impl Default for FromStatic {
    fn default() -> Self {
        {
            FromStatic {
                v: kInit_0.with(|rc| *rc.borrow()),
            }
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut d: Ptr<Declared> = Ptr::<Declared>::null();
    unimplemented!();
    let mut s: S = S::new();
    unimplemented!();
    unimplemented!();
    let mut h: Ptr<Holder> = Ptr::alloc_array(
        (0..1_())
            .map(|_| <Holder>::default())
            .collect::<Box<[Holder]>>(),
    );
    unimplemented!();
    h.delete();
    let mut fs: Ptr<FromStatic> = Ptr::alloc_array(
        (0..2_())
            .map(|_| <FromStatic>::default())
            .collect::<Box<[FromStatic]>>(),
    );
    unimplemented!();
    fs.delete();
    return 0_();
}
pub fn __cpp2rust_init_globals() {
    let _ = kInit_0.with(|_| ());
}
