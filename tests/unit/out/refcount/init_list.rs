extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn f_0(list: std_initializer_list_int_) {
    let list: Value<std_initializer_list_int_> = Rc::new(RefCell::new(list));
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Pair {
    #[offset(0)]
    pub a: (),
    #[offset(4)]
    pub b: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct Holder {
    #[offset(0)]
    #[byte_size(8)]
    pub p: Pair,
}
impl Default for Holder {
    fn default() -> Self {
        {
            Holder {
                p: Pair { a: 0_(), b: 0_() },
            }
        }
    }
}
pub fn sum_1(p: Ptr<Pair>) -> () {
    return ({ p.with(|__s| __s.a) } + { p.with(|__s| __s.b) });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut i1: () = 3_();
    let mut i2: () = unimplemented!();
    let mut carr1: [(); 2] = [1_(), 2_()];
    let mut carr2: [(); 3] = [1_(), unimplemented!(), unimplemented!()];
    let arr: Value<std_array_int__3_> = Rc::new(RefCell::new(unimplemented!()));
    let mut vec_: std_vector_int__std_allocator_int__ =
        std_vector_int__std_allocator_int__::new_1({ unimplemented!() }, None);
    ({ f_0(unimplemented!()) });
    let p: Value<Pair> = Rc::new(RefCell::new(Pair { a: 1_(), b: 2_() }));
    let h: Value<Holder> = Rc::new(RefCell::new(<Holder>::default()));
    ({ HolderImpl::operator_assign_1(&h.as_pointer(), p.as_pointer()) });
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub trait HolderImpl {
    fn operator_assign_1(&self, o: Ptr<Pair>) -> Ptr<Holder>;
}
impl HolderImpl for Ptr<Holder> {
    fn operator_assign_1(&self, o: Ptr<Pair>) -> Ptr<Holder> {
        field!((*self), p).write({ (*o.upgrade().deref()).clone() });
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
