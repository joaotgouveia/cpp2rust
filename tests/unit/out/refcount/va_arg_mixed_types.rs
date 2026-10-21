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
pub struct pair {
    #[offset(0)]
    pub a: (),
    #[offset(4)]
    pub b: (),
}
pub fn sum_mixed_0(count: (), __args: Box<[VaArg]>) -> () {
    let __args: Value<Box<[VaArg]>> = Rc::new(RefCell::new(__args.into()));
    let count: Value<()> = Rc::new(RefCell::new(count));
    let ap: Value<Ptr<VaArg>> = Rc::new(RefCell::new(Ptr::<VaArg>::default()));
    (*ap.borrow_mut()) = __args.as_pointer();
    let mut total: () = unimplemented!();
    let mut i: () = unimplemented!();
    'loop_: while (((i < (*count.borrow())) as ()) != 0) {
        let mut tag: () = (*ap.borrow_mut()).arg::<()>();
        if (((tag == unimplemented!()) as ()) != 0) {
            total += (*ap.borrow_mut()).arg::<()>();
        } else if (((tag == 1_()) as ()) != 0) {
            total += ((*ap.borrow_mut()).arg::<()>() as ());
        } else if (((tag == 3_()) as ()) != 0) {
            let mut p: pair = (*ap.borrow_mut()).arg::<pair>();
            total += (p.a * p.b);
        } else {
            let mut val: () = (*ap.borrow_mut()).arg::<()>();
            total += (val as ());
        }
        i.postfix_inc();
    }
    return total;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut p: pair = pair { a: 7_(), b: 8_() };
    unimplemented!();
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
