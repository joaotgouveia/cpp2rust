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
pub struct record {
    #[offset(0)]
    pub code: u16,
    #[offset(2)]
    pub lo: u16,
    #[offset(4)]
    pub hi: u32,
    #[offset(8)]
    #[byte_size(8)]
    pub pad: Box<[()]>,
}
impl Default for record {
    fn default() -> Self {
        {
            record {
                code: 0_u16,
                lo: 0_u16,
                hi: 0_u32,
                pad: (0..8).map(|_| unimplemented!()).collect::<Box<[()]>>(),
            }
        }
    }
}
#[derive(VaArg, FnPtrArg, ByteRepr, Clone)]
#[byte_size(128)]
pub struct anon_0 {
    #[offset(0)]
    #[byte_size(128)]
    __bytes: Box<[u8]>,
}
impl anon_0 {
    pub fn h(this: Ptr<Self>) -> Ptr<record> {
        this.reinterpret_cast()
    }
    pub fn raw_(this: Ptr<Self>) -> Ptr<()> {
        this.reinterpret_cast()
    }
}
impl Default for anon_0 {
    fn default() -> Self {
        anon_0 {
            __bytes: Box::from([0u8; 128]),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(128)]
pub struct Container {
    #[offset(0)]
    #[byte_size(128)]
    pub view: anon_0,
}
pub fn fill_1(mut out: (), mut cap: ()) {
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
    (*src.borrow_mut())[(1_()) as usize] = 0_();
    (*src.borrow_mut())[(2_()) as usize] = 0_();
    (*src.borrow_mut())[(3_()) as usize] = 80_();
    (*src.borrow_mut())[(4_()) as usize] = 127_();
    (*src.borrow_mut())[(5_()) as usize] = 0_();
    (*src.borrow_mut())[(6_()) as usize] = 0_();
    (*src.borrow_mut())[(7_()) as usize] = 1_();
    let mut n: () = if (((::std::mem::size_of::<[(); 16]>() < cap) as ()) != 0) {
        ::std::mem::size_of::<[(); 16]>()
    } else {
        cap
    };
    ({
        memcpy_2(
            (out).clone(),
            ((src.as_pointer() as Ptr<()>) as Ptr<()>).to_any(),
            n,
        )
    });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let c: Value<Container> = <Value<Container>>::default();
    ({
        let ___s: () = ((c.as_pointer()) as Ptr<Container>).to_any();
        let ___n: () = 128usize;
        memset_3(___s, 0_(), ___n)
    });
    ({
        let _out: () = (field_ptr!(c.as_pointer(), view)).to_any();
        let _cap: () = 128usize;
        fill_1(_out, _cap)
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
