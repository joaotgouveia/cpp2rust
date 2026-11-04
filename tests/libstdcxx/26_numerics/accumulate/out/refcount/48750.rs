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
    m_data: Vec<f64>,
}
impl NaturalParameters {
    pub fn new() -> Self {
        Self {
            m_data: (0..(2_usize) as usize).map(|_| 0_f64).collect::<Vec<_>>(),
        }
    }
    pub fn move_from(_a0: Ptr<NaturalParameters>) -> Self {
        Self {
            m_data: field!(_a0, m_data).with_mut(std::mem::take),
        }
    }
}
impl CppMove for NaturalParameters {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
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
    let mut i: usize = 0_usize;
    'loop_: while (i < 1000_usize) {
        let ChildrenNP: Value<Vec<NaturalParameters>> = Rc::new(RefCell::new(
            (0..(1000_usize) as usize)
                .map(|_| <NaturalParameters>::default())
                .collect::<Vec<_>>(),
        ));
        let init: Value<NaturalParameters> = Rc::new(RefCell::new(NaturalParameters::new()));
        let mut NP: NaturalParameters = ({
            let ___first: RandomAccessIterator<NaturalParameters> =
                CppRange::begin(&ChildrenNP.decay());
            let ___last: RandomAccessIterator<NaturalParameters> =
                CppRange::end(&ChildrenNP.decay());
            accumulate_2(___first, ___last, (*init.borrow()).clone())
        });
        i.prefix_inc();
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ test01_1() });
    return 0;
}
pub trait NaturalParametersImpl {
    fn begin(&self) -> RandomAccessIterator<f64>;
    fn end(&self) -> RandomAccessIterator<f64>;
    fn operator_add_assign(&self, _a0: Ptr<NaturalParameters>) -> Ptr<NaturalParameters>;
    fn move_assign(&self, _a0: Ptr<NaturalParameters>) -> Ptr<NaturalParameters>;
}
impl NaturalParametersImpl for Ptr<NaturalParameters> {
    fn begin(&self) -> RandomAccessIterator<f64> {
        return CppRange::begin(&field_ref!((*self), m_data).decay());
    }
    fn end(&self) -> RandomAccessIterator<f64> {
        return CppRange::begin(&field_ref!((*self), m_data).decay());
    }
    fn operator_add_assign(&self, _a0: Ptr<NaturalParameters>) -> Ptr<NaturalParameters> {
        return (*self).clone();
    }
    fn move_assign(&self, _a0: Ptr<NaturalParameters>) -> Ptr<NaturalParameters> {
        field_ref!((*self), m_data)
            .as_pointer()
            .write(field!(_a0, m_data).with_mut(std::mem::take));
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
