extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn operator_eq_0(a: Ptr<test_less_int_>, b: Ptr<test_less_int_>) -> () {
    return ({ a.with(|__s| __s.data_) } == { b.with(|__s| __s.data_) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct test_less_int_ {
    #[offset(0)]
    pub data_: (),
}
impl test_less_int_ {
    pub fn new_1() -> Self {
        Self { data_: 0_() }
    }
    pub fn new_2(mut data: ()) -> Self {
        Self { data_: data }
    }
}
impl std::cmp::PartialEq for test_less_int_ {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_0(
                Rc::new(RefCell::new(test_less_int_ {
                    data_: self.data_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(test_less_int_ {
                    data_: other.data_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for test_less_int_ {}
impl Default for test_less_int_ {
    fn default() -> Self {
        { test_less_int_::new_1() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(40)]
pub struct test_allocator_statistics {
    #[offset(0)]
    pub time_to_throw: (),
    #[offset(4)]
    pub throw_after: (),
    #[offset(8)]
    pub count: (),
    #[offset(12)]
    pub alloc_count: (),
    #[offset(16)]
    pub allocated_size: (),
    #[offset(20)]
    pub construct_count: (),
    #[offset(24)]
    pub destroy_count: (),
    #[offset(28)]
    pub copied: (),
    #[offset(32)]
    pub moved: (),
    #[offset(36)]
    pub converted: (),
}
impl Default for test_allocator_statistics {
    fn default() -> Self {
        {
            test_allocator_statistics {
                time_to_throw: 0_(),
                throw_after: 2147483647_(),
                count: 0_(),
                alloc_count: 0_(),
                allocated_size: 0_(),
                construct_count: 0_(),
                destroy_count: 0_(),
                copied: 0_(),
                moved: 0_(),
                converted: 0_(),
            }
        }
    }
}
thread_local!(
    pub static destructed_value_1: Value<()> = Rc::new(RefCell::new(-1_()));
);
thread_local!(
    pub static moved_value_2: Value<()> = Rc::new(RefCell::new(2147483647_()));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_alloc_base {}
pub fn operator_eq_3(x: Ptr<test_allocator_void_>, y: Ptr<test_allocator_void_>) -> () {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_ne_4(x: Ptr<test_allocator_void_>, y: Ptr<test_allocator_void_>) -> () {
    return !({
        let _x: Ptr<test_allocator_void_> = (x).clone();
        let _y: Ptr<test_allocator_void_> = (y).clone();
        operator_eq_3(_x, _y)
    });
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Destructor)]
#[byte_size(16)]
pub struct test_allocator_void_ {
    #[offset(0)]
    data_: (),
    #[offset(4)]
    id_: (),
    #[offset(8)]
    #[byte_size(8)]
    stats_: Ptr<test_allocator_statistics>,
}
impl test_allocator_void_ {
    pub fn new_1(mut stats: Ptr<test_allocator_statistics>) -> Self {
        Self {
            data_: 0_(),
            id_: 0_(),
            stats_: (stats).clone(),
        }
    }
    pub fn new_2(mut data: ()) -> Self {
        Self {
            data_: data,
            id_: 0_(),
            stats_: Ptr::<test_allocator_statistics>::null(),
        }
    }
    pub fn new_3(mut data: (), mut stats: Ptr<test_allocator_statistics>) -> Self {
        Self {
            data_: data,
            id_: 0_(),
            stats_: (stats).clone(),
        }
    }
    pub fn new_4(mut data: (), mut id: ()) -> Self {
        Self {
            data_: data,
            id_: id,
            stats_: Ptr::<test_allocator_statistics>::null(),
        }
    }
    pub fn new_5(mut data: (), mut id: (), mut stats: Ptr<test_allocator_statistics>) -> Self {
        Self {
            data_: data,
            id_: id,
            stats_: (stats).clone(),
        }
    }
    pub fn copy_from(a: Ptr<test_allocator_void_>) -> Self {
        Self {
            data_: a.with(|__s| __s.data_),
            id_: a.with(|__s| __s.id_),
            stats_: a.with(|__s| __s.stats_.clone()),
        }
    }
}
impl std::cmp::PartialEq for test_allocator_void_ {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_3(
                Rc::new(RefCell::new(test_allocator_void_ {
                    data_: self.data_.clone(),
                    id_: self.id_.clone(),
                    stats_: self.stats_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(test_allocator_void_ {
                    data_: other.data_.clone(),
                    id_: other.id_.clone(),
                    stats_: other.stats_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for test_allocator_void_ {}
impl Clone for test_allocator_void_ {
    fn clone(&self) -> Self {
        let __src: Value<test_allocator_void_> = Rc::new(RefCell::new(test_allocator_void_ {
            data_: self.data_.clone(),
            id_: self.id_.clone(),
            stats_: self.stats_.clone(),
        }));
        test_allocator_void_::copy_from(__src.as_pointer())
    }
}
impl Default for test_allocator_void_ {
    fn default() -> Self {
        {
            test_allocator_void_ {
                data_: 0_(),
                id_: 0_(),
                stats_: Ptr::<test_allocator_statistics>::null(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Ctor_Tag {}
#[derive(Record, ByteRepr, FnPtrArg, CopyAssign, MoveAssign, Default)]
#[byte_size(1)]
pub struct Tag_X {}
impl Tag_X {
    pub fn new_1(mut _a0: Ctor_Tag) -> Self {
        Self {}
    }
    pub fn new_2(mut _a0: Ctor_Tag, _a1: Ptr<Tag_X>) -> Self {
        Self {}
    }
    pub fn new_3(mut _a0: Ctor_Tag, _a1: Ptr<Tag_X>) -> Self {
        Self {}
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub fn outstanding_alloc() -> () {
        unimplemented!();
        return ((alloc_count_6.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_7.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        unimplemented!();
        disable_default_constructor_8.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_5.with(|rc| *rc.borrow_mut() = 0_());
        alloc_count_6.with(|rc| *rc.borrow_mut() = 0_());
        dealloc_count_7.with(|rc| *rc.borrow_mut() = 0_());
    }
}
thread_local!(
    pub static outstanding_bytes_5: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static alloc_count_6: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static dealloc_count_7: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    pub static disable_default_constructor_8: Value<()> = Rc::new(RefCell::new(false));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<()>,
}
impl min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: (),
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_std_pair_const_int__int__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct tracking_allocator_std_pair_const_int__int__ {
    #[offset(0)]
    #[byte_size(8)]
    allocs_: Ptr<()>,
}
impl tracking_allocator_std_pair_const_int__int__ {
    pub fn new(allocs: Ptr<()>) -> Self {
        Self {
            allocs_: (allocs).clone(),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct NoOp {}
pub fn test_alloc_9(
    lhs_alloc: Option<Ptr<()>>,
    rhs_alloc: Option<Ptr<()>>,
    check_alloc_invariant: Option<NoOp>,
) {
    let mut __lhs_alloc_default: Option<Value<()>> = None;
    let lhs_alloc: Ptr<()> = lhs_alloc.unwrap_or_else(|| {
        __lhs_alloc_default
            .insert(Rc::new(RefCell::new(unimplemented!())))
            .as_pointer()
    });
    let mut __rhs_alloc_default: Option<Value<()>> = None;
    let rhs_alloc: Ptr<()> = rhs_alloc.unwrap_or_else(|| {
        __rhs_alloc_default
            .insert(Rc::new(RefCell::new(unimplemented!())))
            .as_pointer()
    });
    let check_alloc_invariant: Value<NoOp> = Rc::new(RefCell::new(
        check_alloc_invariant.unwrap_or_else(|| <NoOp>::default()),
    ));
    {
        {
            let mut arr: [(); 3] = [unimplemented!(), unimplemented!(), unimplemented!()];
            let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        ({ NoOpImpl::operator_call(&check_alloc_invariant.as_pointer()) });
        {
            let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        ({ NoOpImpl::operator_call(&check_alloc_invariant.as_pointer()) });
        {
            let mut arr: [(); 3] = [unimplemented!(), unimplemented!(), unimplemented!()];
            let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        {
            let mut arr: [(); 3] = [unimplemented!(), unimplemented!(), unimplemented!()];
            let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        ({ NoOpImpl::operator_call(&check_alloc_invariant.as_pointer()) });
        {
            let mut lhs_arr: [(); 3] = [unimplemented!(), unimplemented!(), unimplemented!()];
            let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let mut rhs_arr: [(); 2] = [unimplemented!(), unimplemented!()];
            let copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        ({ NoOpImpl::operator_call(&check_alloc_invariant.as_pointer()) });
        {
            let mut lhs_arr: [(); 3] = [unimplemented!(), unimplemented!(), unimplemented!()];
            let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let mut rhs_arr: [(); 3] = [unimplemented!(), unimplemented!(), unimplemented!()];
            let copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        ({ NoOpImpl::operator_call(&check_alloc_invariant.as_pointer()) });
        {
            let mut lhs_arr: [(); 3] = [unimplemented!(), unimplemented!(), unimplemented!()];
            let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let mut rhs_arr: [(); 4] = [
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
            ];
            let copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        ({ NoOpImpl::operator_call(&check_alloc_invariant.as_pointer()) });
    }
    {
        let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut i: () = 0_();
        'loop_: while (i != 50_()) {
            unimplemented!();
            unimplemented!();
            i.prefix_inc();
        }
        let copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        let mut i: () = 0_();
        let iter: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        'loop_: while unimplemented!() {
            unimplemented!();
            unimplemented!();
            i.prefix_inc();
        }
    }
    ({ NoOpImpl::operator_call(&check_alloc_invariant.as_pointer()) });
}
pub fn test_alloc_10(
    lhs_alloc: Option<Ptr<min_allocator_std_pair_const_int__int__>>,
    rhs_alloc: Option<Ptr<min_allocator_std_pair_const_int__int__>>,
    check_alloc_invariant: Option<NoOp>,
) {
    let mut __lhs_alloc_default: Option<Value<min_allocator_std_pair_const_int__int__>> = None;
    let lhs_alloc: Ptr<min_allocator_std_pair_const_int__int__> = lhs_alloc.unwrap_or_else(|| {
        __lhs_alloc_default
            .insert(Rc::new(RefCell::new(
                <min_allocator_std_pair_const_int__int__>::default(),
            )))
            .as_pointer()
    });
    let mut __rhs_alloc_default: Option<Value<min_allocator_std_pair_const_int__int__>> = None;
    let rhs_alloc: Ptr<min_allocator_std_pair_const_int__int__> = rhs_alloc.unwrap_or_else(|| {
        __rhs_alloc_default
            .insert(Rc::new(RefCell::new(
                <min_allocator_std_pair_const_int__int__>::default(),
            )))
            .as_pointer()
    });
    let check_alloc_invariant: Value<NoOp> = Rc::new(RefCell::new(
        check_alloc_invariant.unwrap_or_else(|| <NoOp>::default()),
    ));
    {
        {
            let mut arr: [(); 3] = [unimplemented!(), unimplemented!(), unimplemented!()];
            let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        ({ NoOpImpl::operator_call(&check_alloc_invariant.as_pointer()) });
        {
            let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        ({ NoOpImpl::operator_call(&check_alloc_invariant.as_pointer()) });
        {
            let mut arr: [(); 3] = [unimplemented!(), unimplemented!(), unimplemented!()];
            let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        {
            let mut arr: [(); 3] = [unimplemented!(), unimplemented!(), unimplemented!()];
            let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        ({ NoOpImpl::operator_call(&check_alloc_invariant.as_pointer()) });
        {
            let mut lhs_arr: [(); 3] = [unimplemented!(), unimplemented!(), unimplemented!()];
            let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let mut rhs_arr: [(); 2] = [unimplemented!(), unimplemented!()];
            let copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        ({ NoOpImpl::operator_call(&check_alloc_invariant.as_pointer()) });
        {
            let mut lhs_arr: [(); 3] = [unimplemented!(), unimplemented!(), unimplemented!()];
            let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let mut rhs_arr: [(); 3] = [unimplemented!(), unimplemented!(), unimplemented!()];
            let copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        ({ NoOpImpl::operator_call(&check_alloc_invariant.as_pointer()) });
        {
            let mut lhs_arr: [(); 3] = [unimplemented!(), unimplemented!(), unimplemented!()];
            let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let mut rhs_arr: [(); 4] = [
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
            ];
            let copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        ({ NoOpImpl::operator_call(&check_alloc_invariant.as_pointer()) });
    }
    {
        let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut i: () = 0_();
        'loop_: while (i != 50_()) {
            unimplemented!();
            unimplemented!();
            i.prefix_inc();
        }
        let copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        let mut i: () = 0_();
        let iter: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        'loop_: while unimplemented!() {
            unimplemented!();
            unimplemented!();
            i.prefix_inc();
        }
    }
    ({ NoOpImpl::operator_call(&check_alloc_invariant.as_pointer()) });
}
pub fn test_alloc_11(
    lhs_alloc: Ptr<tracking_allocator_std_pair_const_int__int__>,
    rhs_alloc: Ptr<tracking_allocator_std_pair_const_int__int__>,
    check_alloc_invariant: AssertEmpty_12,
) {
    let check_alloc_invariant: Value<AssertEmpty_12> = Rc::new(RefCell::new(check_alloc_invariant));
    {
        {
            let mut arr: [(); 3] = [unimplemented!(), unimplemented!(), unimplemented!()];
            let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        ({ AssertEmpty_12Impl::operator_call(&check_alloc_invariant.as_pointer()) });
        {
            let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        ({ AssertEmpty_12Impl::operator_call(&check_alloc_invariant.as_pointer()) });
        {
            let mut arr: [(); 3] = [unimplemented!(), unimplemented!(), unimplemented!()];
            let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        {
            let mut arr: [(); 3] = [unimplemented!(), unimplemented!(), unimplemented!()];
            let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        ({ AssertEmpty_12Impl::operator_call(&check_alloc_invariant.as_pointer()) });
        {
            let mut lhs_arr: [(); 3] = [unimplemented!(), unimplemented!(), unimplemented!()];
            let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let mut rhs_arr: [(); 2] = [unimplemented!(), unimplemented!()];
            let copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        ({ AssertEmpty_12Impl::operator_call(&check_alloc_invariant.as_pointer()) });
        {
            let mut lhs_arr: [(); 3] = [unimplemented!(), unimplemented!(), unimplemented!()];
            let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let mut rhs_arr: [(); 3] = [unimplemented!(), unimplemented!(), unimplemented!()];
            let copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        ({ AssertEmpty_12Impl::operator_call(&check_alloc_invariant.as_pointer()) });
        {
            let mut lhs_arr: [(); 3] = [unimplemented!(), unimplemented!(), unimplemented!()];
            let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let mut rhs_arr: [(); 4] = [
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
            ];
            let copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        ({ AssertEmpty_12Impl::operator_call(&check_alloc_invariant.as_pointer()) });
    }
    {
        let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut i: () = 0_();
        'loop_: while (i != 50_()) {
            unimplemented!();
            unimplemented!();
            i.prefix_inc();
        }
        let copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        let mut i: () = 0_();
        let iter: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        'loop_: while unimplemented!() {
            unimplemented!();
            unimplemented!();
            i.prefix_inc();
        }
    }
    ({ AssertEmpty_12Impl::operator_call(&check_alloc_invariant.as_pointer()) });
}
pub fn test_13() -> () {
    ({ test_alloc_9(None, None, None) });
    ({ test_alloc_10(None, None, None) });
    {
        let lhs_allocs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let rhs_allocs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({
            let _lhs_alloc: Value<tracking_allocator_std_pair_const_int__int__> =
                Rc::new(RefCell::new(
                    tracking_allocator_std_pair_const_int__int__::new({ lhs_allocs.as_pointer() }),
                ));
            let _rhs_alloc: Value<tracking_allocator_std_pair_const_int__int__> =
                Rc::new(RefCell::new(
                    tracking_allocator_std_pair_const_int__int__::new({ rhs_allocs.as_pointer() }),
                ));
            let _check_alloc_invariant: AssertEmpty_12 =
                AssertEmpty_12::new({ lhs_allocs.as_pointer() }, { rhs_allocs.as_pointer() });
            test_alloc_11(
                _lhs_alloc.as_pointer(),
                _rhs_alloc.as_pointer(),
                _check_alloc_invariant,
            )
        });
    }
    {
        let mut arr: [(); 3] = [unimplemented!(), unimplemented!(), unimplemented!()];
        let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let copy: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct AssertEmpty_12 {
    #[offset(0)]
    #[byte_size(8)]
    lhs_allocs_: Ptr<()>,
    #[offset(8)]
    #[byte_size(8)]
    rhs_allocs_: Ptr<()>,
}
impl AssertEmpty_12 {
    pub fn new(lhs_allocs: Ptr<()>, rhs_allocs: Ptr<()>) -> Self {
        Self {
            lhs_allocs_: (lhs_allocs).clone(),
            rhs_allocs_: (rhs_allocs).clone(),
        }
    }
}
pub fn main() {
    let size: usize = ::std::env::args().map(|arg| arg.len() + 1).sum();
    let args: Value<Box<[i8]>> = Rc::new(RefCell::new(vec![0; size].into_boxed_slice()));
    let args_ptr: Ptr<i8> = args.as_pointer();
    let mut off = 0;
    let argv: Value<Box<[Ptr<i8>]>> = Rc::new(RefCell::new(
        ::std::env::args()
            .map(|arg| {
                let ptr = args_ptr.offset(off);
                ptr.write_c_bytes(arg.as_bytes());
                off += arg.len() + 1;
                ptr
            })
            .chain(::std::iter::once(Ptr::null()))
            .collect(),
    ));
    __cpp2rust_init_globals();
    ::std::process::exit(main_0(::std::env::args().len() as i32, argv.as_pointer()));
}
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    ({ test_13() });
    return 0_();
}
pub trait AssertEmpty_12Impl {
    fn operator_call(&self);
}
impl AssertEmpty_12Impl for Ptr<AssertEmpty_12> {
    fn operator_call(&self) {
        unimplemented!();
        unimplemented!();
    }
}
pub trait NoOpImpl {
    fn operator_call(&self);
}
impl NoOpImpl for Ptr<NoOp> {
    fn operator_call(&self) {}
}
pub trait Tag_XImpl {
    fn copy_assign(&self, _a0: Ptr<Tag_X>) -> Ptr<Tag_X>;
    fn move_assign(&self, _a0: Ptr<Tag_X>) -> Ptr<Tag_X>;
}
impl Tag_XImpl for Ptr<Tag_X> {
    fn copy_assign(&self, _a0: Ptr<Tag_X>) -> Ptr<Tag_X> {
        return (*self).clone();
    }
    fn move_assign(&self, _a0: Ptr<Tag_X>) -> Ptr<Tag_X> {
        return (*self).clone();
    }
}
pub trait min_allocator_std_pair_const_int__int__Impl {
    fn allocate(
        &self,
        mut n: (),
    ) -> min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn deallocate(
        &self,
        mut p: min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!()
    }
}
impl min_allocator_std_pair_const_int__int__Impl for Ptr<min_allocator_std_pair_const_int__int__> {}
pub trait min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_inc_8(
        &self,
    ) -> Ptr<min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_inc_9(
        &self,
        mut _a0: (),
    ) -> min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_10(
        &self,
    ) -> Ptr<min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_post_dec_11(
        &self,
        mut _a0: (),
    ) -> min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_sub_assign(
        &self,
        mut n: (),
    ) -> Ptr<min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__> {
        unimplemented!()
    }
    fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_sub(
        &self,
        mut n: (),
    ) -> min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
}
impl min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_std_pair_const_int__int___std_integral_constant_unsigned_long__0__>
{
}
pub trait min_pointer_void__std_integral_constant_unsigned_long__0__Impl {
    fn to_(&self) -> () {
        unimplemented!()
    }
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_void__std_integral_constant_unsigned_long__0__>
{
}
pub trait test_allocator_statisticsImpl {
    fn clear(&self);
    fn clear_ctor_counters(&self);
}
impl test_allocator_statisticsImpl for Ptr<test_allocator_statistics> {
    fn clear(&self) {
        unimplemented!();
        field!((*self), count).write(0_());
        field!((*self), time_to_throw).write(0_());
        field!((*self), alloc_count).write(0_());
        field!((*self), allocated_size).write(0_());
        field!((*self), construct_count).write(0_());
        field!((*self), destroy_count).write(0_());
        field!((*self), throw_after).write(2147483647_());
        ({ test_allocator_statisticsImpl::clear_ctor_counters(self) });
    }
    fn clear_ctor_counters(&self) {
        field!((*self), copied).write(0_());
        field!((*self), moved).write(0_());
        field!((*self), converted).write(0_());
    }
}
pub trait test_allocator_void_Impl {
    fn destructor(&self);
    fn get_id(&self) -> ();
    fn get_data(&self) -> ();
    fn copy_assign(&self, _a0: Ptr<test_allocator_void_>) -> Ptr<test_allocator_void_>;
}
impl test_allocator_void_Impl for Ptr<test_allocator_void_> {
    fn destructor(&self) {
        field!((*self), data_).write(destructed_value_1.with(|rc| *rc.borrow()));
        field!((*self), id_).write(destructed_value_1.with(|rc| *rc.borrow()));
    }
    fn get_id(&self) -> () {
        return (*self).with(|__s| __s.id_);
    }
    fn get_data(&self) -> () {
        return (*self).with(|__s| __s.data_);
    }
    fn copy_assign(&self, _a0: Ptr<test_allocator_void_>) -> Ptr<test_allocator_void_> {
        field!((*self), data_).write({ _a0.with(|__s| __s.data_) });
        field!((*self), id_).write({ _a0.with(|__s| __s.id_) });
        field!((*self), stats_).write({ _a0.with(|__s| __s.stats_.clone()) });
        return (*self).clone();
    }
}
pub trait test_less_int_Impl {
    fn operator_call(&self, a: Ptr<()>, b: Ptr<()>) -> ();
}
impl test_less_int_Impl for Ptr<test_less_int_> {
    fn operator_call(&self, a: Ptr<()>, b: Ptr<()>) -> () {
        return ({ (a.read()) } < { (b.read()) });
    }
}
pub trait tracking_allocator_std_pair_const_int__int__Impl {
    fn allocate(&self, mut n: ()) -> Ptr<()> {
        unimplemented!()
    }
    fn deallocate(&self, mut ptr: Ptr<()>, mut n: ()) {
        unimplemented!()
    }
}
impl tracking_allocator_std_pair_const_int__int__Impl
    for Ptr<tracking_allocator_std_pair_const_int__int__>
{
}
pub fn __cpp2rust_init_globals() {
    let _ = destructed_value_1.with(|_| ());
    let _ = moved_value_2.with(|_| ());
    let _ = outstanding_bytes_5.with(|_| ());
    let _ = alloc_count_6.with(|_| ());
    let _ = dealloc_count_7.with(|_| ());
    let _ = disable_default_constructor_8.with(|_| ());
}
