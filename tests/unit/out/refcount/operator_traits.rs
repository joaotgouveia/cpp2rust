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
pub struct Lt {
    #[offset(0)]
    pub v: (),
}
impl CppLess for Lt {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        LtImpl::operator_lt(__lhs, __rhs.clone())
    }
}
impl CppEq for Lt {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        !(LtImpl::operator_lt(__lhs, __rhs.clone())) && !(LtImpl::operator_lt(__rhs, __lhs.clone()))
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Eq {
    #[offset(0)]
    pub v: (),
}
impl CppEq for Eq {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        EqImpl::operator_eq(__lhs, __rhs.clone())
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Cmp {
    #[offset(0)]
    pub v: (),
}
impl CppLess for Cmp {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        CmpImpl::operator_cmp(__lhs, __rhs.clone()) == std::cmp::Ordering::Less
    }
}
impl CppEq for Cmp {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        CmpImpl::operator_eq(__lhs, __rhs.clone())
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Free {
    #[offset(0)]
    pub v: (),
}
impl CppLess for Free {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_0(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for Free {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_1(__lhs.clone(), __rhs.clone())
    }
}
pub fn operator_lt_0(a: Ptr<Free>, b: Ptr<Free>) -> () {
    return ({ a.with(|__s| __s.v) } < { b.with(|__s| __s.v) });
}
pub fn operator_eq_1(a: Ptr<Free>, b: Ptr<Free>) -> () {
    return ({ a.with(|__s| __s.v) } == { b.with(|__s| __s.v) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Wrapped_int_ {
    #[offset(0)]
    pub v: (),
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut w: Wrapped_int_ = Wrapped_int_ { v: 2_() };
    unimplemented!();
    let lts: Value<std_vector_Lt__std_allocator_Lt__> = Rc::new(RefCell::new(
        std_vector_Lt__std_allocator_Lt__::new_1({ unimplemented!() }, None),
    ));
    ({
        let ___first: __gnu_cxx___normal_iterator_Lt_ptr__std_vector_Lt__std_allocator_Lt___ =
            ({ (*lts.borrow()).begin_26() });
        let ___last: __gnu_cxx___normal_iterator_Lt_ptr__std_vector_Lt__std_allocator_Lt___ =
            ({ (*lts.borrow()).end_28() });
        sort_2(___first, ___last)
    });
    unimplemented!();
    let eqs: Value<std_vector_Eq__std_allocator_Eq__> = Rc::new(RefCell::new(
        std_vector_Eq__std_allocator_Eq__::new_1({ unimplemented!() }, None),
    ));
    let two: Value<Eq> = Rc::new(RefCell::new(Eq { v: 2_() }));
    let nine: Value<Eq> = Rc::new(RefCell::new(Eq { v: 9_() }));
    unimplemented!();
    unimplemented!();
    let cmps: Value<std_vector_Cmp__std_allocator_Cmp__> = Rc::new(RefCell::new(
        std_vector_Cmp__std_allocator_Cmp__::new_1({ unimplemented!() }, None),
    ));
    ({
        let ___first: __gnu_cxx___normal_iterator_Cmp_ptr__std_vector_Cmp__std_allocator_Cmp___ =
            ({ (*cmps.borrow()).begin_26() });
        let ___last: __gnu_cxx___normal_iterator_Cmp_ptr__std_vector_Cmp__std_allocator_Cmp___ =
            ({ (*cmps.borrow()).end_28() });
        sort_3(___first, ___last)
    });
    unimplemented!();
    let three: Value<Cmp> = Rc::new(RefCell::new(Cmp { v: 3_() }));
    unimplemented!();
    let frees: Value<std_vector_Free__std_allocator_Free__> = Rc::new(RefCell::new(
        std_vector_Free__std_allocator_Free__::new_1({ unimplemented!() }, None),
    ));
    ({
        let ___first: __gnu_cxx___normal_iterator_Free_ptr__std_vector_Free__std_allocator_Free___ =
            ({ (*frees.borrow()).begin_26() });
        let ___last: __gnu_cxx___normal_iterator_Free_ptr__std_vector_Free__std_allocator_Free___ =
            ({ (*frees.borrow()).end_28() });
        sort_4(___first, ___last)
    });
    unimplemented!();
    let ftwo: Value<Free> = Rc::new(RefCell::new(Free { v: 2_() }));
    unimplemented!();
    let m: Value<std_map_Lt__int__std_less_Lt___std_allocator_std_pair_const_Lt__int___> =
        Rc::new(RefCell::new(
            <std_map_Lt__int__std_less_Lt___std_allocator_std_pair_const_Lt__int___>::default(),
        ));
    (*m.borrow_mut())[Lt { v: 2_() }] = 20_();
    (*m.borrow_mut())[Lt { v: 1_() }] = 10_();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub trait CmpImpl {
    fn operator_cmp(&self, o: Ptr<Cmp>) -> std_strong_ordering;
    fn operator_eq(&self, o: Ptr<Cmp>) -> ();
}
impl CmpImpl for Ptr<Cmp> {
    fn operator_cmp(&self, o: Ptr<Cmp>) -> std_strong_ordering {
        return std::cmp::Ord::cmp(&((*self).with(|__s| __s.v)), &(o.with(|__s| __s.v)));
    }
    fn operator_eq(&self, o: Ptr<Cmp>) -> () {
        return ({ (*self).with(|__s| __s.v) } == { o.with(|__s| __s.v) });
    }
}
pub trait EqImpl {
    fn operator_eq(&self, o: Ptr<Eq>) -> ();
}
impl EqImpl for Ptr<Eq> {
    fn operator_eq(&self, o: Ptr<Eq>) -> () {
        return ({ (*self).with(|__s| __s.v) } == { o.with(|__s| __s.v) });
    }
}
pub trait LtImpl {
    fn operator_lt(&self, o: Ptr<Lt>) -> ();
}
impl LtImpl for Ptr<Lt> {
    fn operator_lt(&self, o: Ptr<Lt>) -> () {
        return ({ (*self).with(|__s| __s.v) } < { o.with(|__s| __s.v) });
    }
}
pub fn __cpp2rust_init_globals() {}
