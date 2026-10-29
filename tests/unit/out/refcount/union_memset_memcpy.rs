extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(16)]
pub struct shape_a {
    #[offset(0)]
    pub code: (),
    #[offset(2)]
    #[byte_size(14)]
    pub pad: Box<[()]>,
}
impl Default for shape_a {
    fn default() -> Self {
        {
            shape_a {
                code: unimplemented!(),
                pad: (0..14).map(|_| unimplemented!()).collect::<Box<[()]>>(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(16)]
pub struct shape_b {
    #[offset(0)]
    pub code: (),
    #[offset(2)]
    pub lo: (),
    #[offset(4)]
    pub hi: (),
    #[offset(8)]
    #[byte_size(8)]
    pub fill: Box<[()]>,
}
impl Default for shape_b {
    fn default() -> Self {
        {
            shape_b {
                code: unimplemented!(),
                lo: unimplemented!(),
                hi: unimplemented!(),
                fill: (0..8).map(|_| unimplemented!()).collect::<Box<[()]>>(),
            }
        }
    }
}
#[derive(VaArg, FnPtrArg, ByteRepr, Clone)]
#[byte_size(256)]
pub struct anon_0 {
    #[offset(0)]
    #[byte_size(256)]
    __bytes: Box<[u8]>,
}
impl anon_0 {
    pub fn a(this: Ptr<Self>) -> Ptr<shape_a> {
        this.reinterpret_cast()
    }
    pub fn b(this: Ptr<Self>) -> Ptr<shape_b> {
        this.reinterpret_cast()
    }
    pub fn raw_(this: Ptr<Self>) -> Ptr<()> {
        this.reinterpret_cast()
    }
}
impl Default for anon_0 {
    fn default() -> Self {
        anon_0 {
            __bytes: Box::from([0u8; 256]),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(256)]
pub struct Container {
    #[offset(0)]
    #[byte_size(256)]
    pub view: anon_0,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let c: Value<Container> = <Value<Container>>::default();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let src: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        0_(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ])));
    (*src.borrow_mut())[(0_()) as usize] = 2_();
    (*src.borrow_mut())[(2_()) as usize] = 80_();
    (*src.borrow_mut())[(3_()) as usize] = 0_();
    (*src.borrow_mut())[(4_()) as usize] = 127_();
    (*src.borrow_mut())[(5_()) as usize] = 0_();
    (*src.borrow_mut())[(6_()) as usize] = 0_();
    (*src.borrow_mut())[(7_()) as usize] = 1_();
    let mut len: () = 16_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
