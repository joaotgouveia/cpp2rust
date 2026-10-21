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
pub struct Inner {
    #[offset(0)]
    pub value: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Outer {
    #[offset(0)]
    #[byte_size(8)]
    pub p: Ptr<Inner>,
}
thread_local!(
    pub static alpha_0: Value<Inner> = Rc::new(RefCell::new(Inner { value: 1_() }));
);
thread_local!(
    pub static beta_1: Value<Inner> = Rc::new(RefCell::new(Inner { value: 2_() }));
);
thread_local!(
    pub static shared_2: Value<Inner> = Rc::new(RefCell::new(Inner { value: 42_() }));
);
thread_local!(
    pub static items_3: Value<Box<[Ptr<Inner>]>> = Rc::new(RefCell::new(Box::new([
        (alpha_0.with(|v| v.as_pointer())),
        (beta_1.with(|v| v.as_pointer())),
    ])));
);
thread_local!(
    pub static obj_4: Value<Outer> = Rc::new(RefCell::new(Outer {
        p: (shared_2.with(|v| v.as_pointer())),
    }));
);
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
thread_local!(
    static cache_5: Value<Box<[Ptr<Inner>]>> = Rc::new(RefCell::new(Box::new([
        (alpha_0.with(|v| v.as_pointer())),
        (beta_1.with(|v| v.as_pointer())),
    ])));
);
pub fn __cpp2rust_init_globals() {
    let _ = alpha_0.with(|_| ());
    let _ = beta_1.with(|_| ());
    let _ = shared_2.with(|_| ());
    let _ = items_3.with(|_| ());
    let _ = obj_4.with(|_| ());
}
