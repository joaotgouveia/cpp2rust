extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn fatorial_0(mut n: ()) -> () {
    if (n == unimplemented!()) {
        return 1_();
    }
    return (n * ({ fatorial_0((n - 1_())) }));
}
pub fn fatorial_by_ref_1(n: Ptr<()>) {
    if ((n.read()) == 1_()) {
        {
            n.with_mut(|__v| *__v = *__v * 1_())
        };
        return;
    }
    let n_1: Value<()> = Rc::new(RefCell::new(((n.read()) - 1_())));
    ({ fatorial_by_ref_1(n_1.as_pointer()) });
    {
        let __rhs = { (*n_1.borrow()) };
        n.with_mut(|__v| *__v = *__v * __rhs)
    };
}
pub fn fatorial_by_ptr_2(mut n: Ptr<()>) {
    if ((n.read()) == 1_()) {
        {
            n.with_mut(|__v| *__v = *__v * 1_())
        };
        return;
    }
    let n_1: Value<()> = Rc::new(RefCell::new(((n.read()) - 1_())));
    ({ fatorial_by_ptr_2((n_1.as_pointer())) });
    {
        let __rhs = { (*n_1.borrow()) };
        n.with_mut(|__v| *__v = *__v * __rhs)
    };
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let n1: Value<()> = Rc::new(RefCell::new(2_()));
    ({ fatorial_by_ptr_2((n1.as_pointer())) });
    let n: Value<()> = Rc::new(RefCell::new(((*n1.borrow()) + 1_())));
    ({ fatorial_by_ref_1(n.as_pointer()) });
    unimplemented!();
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
