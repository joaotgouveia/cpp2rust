extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign)]
#[byte_size(24)]
pub struct NaturalParameters {
    #[offset(0)]
    #[byte_size(24)]
    m_data: (),
}
impl NaturalParameters {
    pub fn new() -> Self {
        Self {
            m_data: unimplemented!(),
        }
    }
    pub fn move_from(_a0: Ptr<NaturalParameters>) -> Self {
        Self {
            m_data: unimplemented!(),
        }
    }
}
impl Default for NaturalParameters {
    fn default() -> Self {
        { NaturalParameters::new() }
    }
}
pub fn operator_add_0(a: Ptr<NaturalParameters>, b: Ptr<NaturalParameters>) -> NaturalParameters {
    let tmp: Value<NaturalParameters> = Rc::new(RefCell::new((*a.upgrade().deref()).clone()));
    return (*({
        let _arg0: Ptr<NaturalParameters> = (b).clone();
        NaturalParametersImpl::operator_add_assign(&tmp.as_pointer(), _arg0)
    })
    .upgrade()
    .deref())
    .clone();
}
pub fn test01_1() {
    let mut i: () = 0_();
    'loop_: while (i < 1000_()) {
        let mut ChildrenNP: () = unimplemented!();
        let init: Value<NaturalParameters> = Rc::new(RefCell::new(NaturalParameters::new()));
        let mut NP: NaturalParameters = unimplemented!();
        i.prefix_inc();
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    ({ test01_1() });
    return 0_();
}
pub trait NaturalParametersImpl {
    fn begin(&self) -> ();
    fn end(&self) -> ();
    fn operator_add_assign(&self, _a0: Ptr<NaturalParameters>) -> Ptr<NaturalParameters>;
    fn move_assign(&self, _a0: Ptr<NaturalParameters>) -> Ptr<NaturalParameters>;
}
impl NaturalParametersImpl for Ptr<NaturalParameters> {
    fn begin(&self) -> () {
        return unimplemented!();
    }
    fn end(&self) -> () {
        return unimplemented!();
    }
    fn operator_add_assign(&self, _a0: Ptr<NaturalParameters>) -> Ptr<NaturalParameters> {
        return (*self).clone();
    }
    fn move_assign(&self, _a0: Ptr<NaturalParameters>) -> Ptr<NaturalParameters> {
        unimplemented!();
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
