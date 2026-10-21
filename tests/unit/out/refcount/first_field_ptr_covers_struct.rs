extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct In {
    #[offset(0)]
    pub a: (),
    #[offset(2)]
    pub b: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(12)]
pub struct S {
    #[offset(0)]
    pub x: (),
    #[offset(4)]
    #[byte_size(4)]
    pub in_: In,
    #[offset(8)]
    pub z: (),
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let src: Value<S> = Rc::new(RefCell::new(S {
        x: 1_(),
        in_: In { a: 2_(), b: 3_() },
        z: 4_(),
    }));
    let mut p: Ptr<S> = unimplemented!().reinterpret_cast::<S>();
    unimplemented!();
    ({
        let ___dest: () = ((field_ptr!(p, x)) as Ptr<()>).to_any();
        let ___n: () = 12usize;
        memcpy_0(___dest, ((src.as_pointer()) as Ptr<S>).to_any(), ___n)
    });
    unimplemented!();
    let n: Value<In> = Rc::new(RefCell::new(In { a: 5_(), b: 6_() }));
    ({
        let ___src: () = ((n.as_pointer()) as Ptr<In>).to_any();
        let ___n: () = 4usize;
        memcpy_0(((field_ptr!(p, in_)) as Ptr<In>).to_any(), ___src, ___n)
    });
    unimplemented!();
    let mut bz: Ptr<()> = (field_ptr!(p, z)).reinterpret_cast::<()>();
    let mut i: () = unimplemented!();
    'loop_: while (((i < 4_()) as ()) != 0) {
        elem!(bz, i).write(1_());
        i.postfix_inc();
    }
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
