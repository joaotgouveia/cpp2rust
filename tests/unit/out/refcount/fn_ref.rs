extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn square_0(mut x: ()) -> () {
    return (x * x);
}
pub fn twice_1(mut x: ()) -> () {
    return (2_() * x);
}
pub fn call_ref_2(f: FnPtr<fn(()) -> ()>, mut x: ()) -> () {
    return ({ f.call(x) });
}
pub fn call_deduced_3(f: FnPtr<fn(()) -> ()>, mut x: ()) -> () {
    return ({ f.call(x) });
}
pub fn call_forwarded_4(f: FnPtr<fn(()) -> ()>, mut x: ()) -> () {
    return ({ f.call(x) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Holder {
    #[offset(0)]
    #[byte_size(8)]
    pub f: FnPtr<fn(()) -> ()>,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    unimplemented!();
    unimplemented!();
    let r: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(square_0);
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let h: Value<Holder> = Rc::new(RefCell::new(Holder {
        f: FnPtr::<fn(()) -> ()>::new(twice_1),
    }));
    unimplemented!();
    return unimplemented!();
}
pub trait HolderImpl {
    fn run(&self, x: ()) -> ();
}
impl HolderImpl for Ptr<Holder> {
    fn run(&self, mut x: ()) -> () {
        return ({ (*self).with(|__s| __s.f.clone()).call(x) });
    }
}
pub fn __cpp2rust_init_globals() {}
