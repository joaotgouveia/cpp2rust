extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn double_it_0(mut x: Ptr<()>) {
    {
        x.with_mut(|__v| *__v = *__v * 2_())
    };
}
pub fn maybe_call_1(mut cb: FnPtr<fn(Ptr<()>)>, mut x: Ptr<()>) {
    if !(cb).is_null() {
        ({ cb.call((x).clone()) });
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let a: Value<()> = Rc::new(RefCell::new(5_()));
    ({ maybe_call_1(FnPtr::<fn(Ptr<()>)>::new(double_it_0), (a.as_pointer())) });
    unimplemented!();
    let b: Value<()> = Rc::new(RefCell::new(5_()));
    ({ maybe_call_1(FnPtr::<fn(Ptr<()>)>::null(), (b.as_pointer())) });
    unimplemented!();
    let mut fn_: FnPtr<fn(Ptr<()>)> = FnPtr::<fn(Ptr<()>)>::null();
    if !(!(fn_).is_null()) {
        fn_ = FnPtr::<fn(Ptr<()>)>::new(double_it_0);
    }
    let c: Value<()> = Rc::new(RefCell::new(3_()));
    if !(fn_).is_null() {
        ({ fn_.call((c.as_pointer())) });
    }
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
