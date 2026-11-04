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
    m_data: std_vector_double__std_allocator_double__,
}
impl NaturalParameters {
    pub fn new() -> Self {
        Self {
            m_data: std_vector_double__std_allocator_double__::new_1({ 2_u64 }, None),
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
    let mut i: u64 = 0_u64;
    'loop_: while (i < 1000_u64) {
        let ChildrenNP: Value<std_vector_NaturalParameters__std_allocator_NaturalParameters__> =
            Rc::new(RefCell::new(
                std_vector_NaturalParameters__std_allocator_NaturalParameters__::new_1(
                    { 1000_u64 },
                    None,
                ),
            ));
        let init: Value<NaturalParameters> = Rc::new(RefCell::new(NaturalParameters::new()));
        let mut NP: NaturalParameters = ({
            let ___first: __gnu_cxx___normal_iterator_NaturalParameters_ptr__std_vector_NaturalParameters__std_allocator_NaturalParameters___   = ( { (*ChildrenNP.borrow()) . begin_28 ( ) } )   ;
            let ___last: __gnu_cxx___normal_iterator_NaturalParameters_ptr__std_vector_NaturalParameters__std_allocator_NaturalParameters___   = ( { (*ChildrenNP.borrow()) . end_30 ( ) } )   ;
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
    fn begin(
        &self,
    ) -> __gnu_cxx___normal_iterator_const_double_ptr__std_vector_double__std_allocator_double___;
    fn end(
        &self,
    ) -> __gnu_cxx___normal_iterator_const_double_ptr__std_vector_double__std_allocator_double___;
    fn operator_add_assign(&self, _a0: Ptr<NaturalParameters>) -> Ptr<NaturalParameters>;
    fn move_assign(&self, _a0: Ptr<NaturalParameters>) -> Ptr<NaturalParameters>;
}
impl NaturalParametersImpl for Ptr<NaturalParameters> {
    fn begin(
        &self,
    ) -> __gnu_cxx___normal_iterator_const_double_ptr__std_vector_double__std_allocator_double___
    {
        return ({ (*(*self).upgrade().deref()).m_data.begin_29() });
    }
    fn end(
        &self,
    ) -> __gnu_cxx___normal_iterator_const_double_ptr__std_vector_double__std_allocator_double___
    {
        return ({ (*(*self).upgrade().deref()).m_data.begin_29() });
    }
    fn operator_add_assign(&self, _a0: Ptr<NaturalParameters>) -> Ptr<NaturalParameters> {
        return (*self).clone();
    }
    fn move_assign(&self, _a0: Ptr<NaturalParameters>) -> Ptr<NaturalParameters> {
        field!((*self), m_data).write({ { (*_a0.upgrade().deref()).m_data.clone() } });
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
