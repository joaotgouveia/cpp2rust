extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(128)]
pub struct __gnu_test_func_callback {
    #[offset(0)]
    _M_size: (),
    #[offset(8)]
    #[byte_size(120)]
    _M_tests: Box<[FnPtr<fn()>]>,
}
impl __gnu_test_func_callback {
    pub fn new() -> Self {
        Self {
            _M_size: 0_(),
            _M_tests: (0..15)
                .map(|_| FnPtr::<fn()>::null())
                .collect::<Box<[FnPtr<fn()>]>>(),
        }
    }
}
impl Default for __gnu_test_func_callback {
    fn default() -> Self {
        { __gnu_test_func_callback::new() }
    }
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Destructor)]
#[byte_size(1)]
pub struct __gnu_test_object_counter {}
impl __gnu_test_object_counter {
    pub fn new() -> Self {
        let __this: __gnu_test_object_counter = Self {};
        (*count_0.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
    pub fn copy_from(_a0: Ptr<__gnu_test_object_counter>) -> Self {
        let __this: __gnu_test_object_counter = Self {};
        (*count_0.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
}
impl Clone for __gnu_test_object_counter {
    fn clone(&self) -> Self {
        let __src: Value<__gnu_test_object_counter> =
            Rc::new(RefCell::new(__gnu_test_object_counter {}));
        __gnu_test_object_counter::copy_from(__src.as_pointer())
    }
}
impl Default for __gnu_test_object_counter {
    fn default() -> Self {
        { __gnu_test_object_counter::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct __gnu_test_copy_constructor {}
impl __gnu_test_copy_constructor {
    pub fn count() -> () {
        return count__1.with(|rc| *rc.borrow());
    }
    pub fn mark_call() {
        (*count__1.with(Value::clone).borrow_mut()).postfix_inc();
        if (count__1.with(|rc| *rc.borrow()) == throw_on__2.with(|rc| *rc.borrow())) {
            unimplemented!();
        }
    }
    pub fn reset() {
        count__1.with(|rc| *rc.borrow_mut() = 0_());
        throw_on__2.with(|rc| *rc.borrow_mut() = 0_());
    }
    pub fn throw_on(mut count: ()) {
        throw_on__2.with(|rc| *rc.borrow_mut() = count);
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct __gnu_test_assignment_operator {}
impl __gnu_test_assignment_operator {
    pub fn count() -> () {
        return count__3.with(|rc| *rc.borrow());
    }
    pub fn mark_call() {
        (*count__3.with(Value::clone).borrow_mut()).postfix_inc();
        if (count__3.with(|rc| *rc.borrow()) == throw_on__4.with(|rc| *rc.borrow())) {
            unimplemented!();
        }
    }
    pub fn reset() {
        count__3.with(|rc| *rc.borrow_mut() = 0_());
        throw_on__4.with(|rc| *rc.borrow_mut() = 0_());
    }
    pub fn throw_on(mut count: ()) {
        throw_on__4.with(|rc| *rc.borrow_mut() = count);
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct __gnu_test_destructor {}
impl __gnu_test_destructor {
    pub fn count() -> () {
        return _M_count_5.with(|rc| *rc.borrow());
    }
    pub fn mark_call() {
        (*_M_count_5.with(Value::clone).borrow_mut()).postfix_inc();
    }
    pub fn reset() {
        _M_count_5.with(|rc| *rc.borrow_mut() = 0_());
    }
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Destructor)]
#[byte_size(8)]
pub struct __gnu_test_copy_tracker {
    #[offset(0)]
    id_: (),
    #[offset(4)]
    throw_on_copy_: (),
}
impl __gnu_test_copy_tracker {
    pub fn new(id: Option<()>, throw_on_copy: Option<()>) -> Self {
        let mut id: () =
            id.unwrap_or_else(|| (*next_id__6.with(Value::clone).borrow_mut()).postfix_dec());
        let mut throw_on_copy: () = throw_on_copy.unwrap_or_else(|| false);
        Self {
            id_: id,
            throw_on_copy_: throw_on_copy,
        }
    }
    pub fn copy_from(rhs: Ptr<__gnu_test_copy_tracker>) -> Self {
        let __this: Value<__gnu_test_copy_tracker> = Rc::new(RefCell::new(Self {
            id_: ({ __gnu_test_copy_trackerImpl::id(&rhs) }),
            throw_on_copy_: rhs.with(|__s| __s.throw_on_copy_),
        }));
        let this: Ptr<__gnu_test_copy_tracker> = __this.as_pointer();
        if this.with(|__s| __s.throw_on_copy_) {
            ({
                __gnu_test_copy_constructor::throw_on(
                    ({ __gnu_test_copy_constructor::count() }).wrapping_add(1_()),
                )
            });
        }
        ({ __gnu_test_copy_constructor::mark_call() });
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn reset() {
        ({ __gnu_test_copy_constructor::reset() });
        ({ __gnu_test_assignment_operator::reset() });
        ({ __gnu_test_destructor::reset() });
    }
}
impl CppLess for __gnu_test_copy_tracker {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_7(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for __gnu_test_copy_tracker {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_8(__lhs.clone(), __rhs.clone())
    }
}
impl Clone for __gnu_test_copy_tracker {
    fn clone(&self) -> Self {
        let __src: Value<__gnu_test_copy_tracker> =
            Rc::new(RefCell::new(__gnu_test_copy_tracker {
                id_: self.id_.clone(),
                throw_on_copy_: self.throw_on_copy_.clone(),
            }));
        __gnu_test_copy_tracker::copy_from(__src.as_pointer())
    }
}
impl Default for __gnu_test_copy_tracker {
    fn default() -> Self {
        { __gnu_test_copy_tracker::new(None, None) }
    }
}
pub fn operator_eq_8(lhs: Ptr<__gnu_test_copy_tracker>, rhs: Ptr<__gnu_test_copy_tracker>) -> () {
    return ({ ({ __gnu_test_copy_trackerImpl::id(&lhs) }) } == {
        ({ __gnu_test_copy_trackerImpl::id(&rhs) })
    });
}
pub fn operator_lt_7(lhs: Ptr<__gnu_test_copy_tracker>, rhs: Ptr<__gnu_test_copy_tracker>) -> () {
    return ({ ({ __gnu_test_copy_trackerImpl::id(&lhs) }) } < {
        ({ __gnu_test_copy_trackerImpl::id(&rhs) })
    });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct __gnu_test_semaphore {
    #[offset(0)]
    sem_set_: (),
    #[offset(4)]
    pid_: (),
}
impl __gnu_test_semaphore {}
thread_local! ( pub static arr_9 : Value<Box<[ ()  ]> > = Rc::new(RefCell::new(()::array_from_literal(b"efgh\0abcd\0ijkl\0") )) ;
);
thread_local!(
    pub static s1_10: Value<Ptr<()>> =
        Rc::new(RefCell::new((arr_9.with(|v| v.as_pointer()) as Ptr<()>)));
);
thread_local!(
    pub static s2_11: Value<Ptr<()>> = Rc::new(RefCell::new(
        (arr_9.with(|v| v.as_pointer()) as Ptr<()>).offset((5_()) as isize),
    ));
);
thread_local!(
    pub static s3_12: Value<Ptr<()>> = Rc::new(RefCell::new(
        (arr_9.with(|v| v.as_pointer()) as Ptr<()>).offset((6_()) as isize),
    ));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct CStrNone {
    #[offset(0)]
    #[byte_size(8)]
    pub str: Ptr<()>,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct CStrMem_ReturnOrd_std_strong_ordering__ {
    #[offset(0)]
    #[byte_size(8)]
    pub str: Ptr<()>,
}
impl CppLess for CStrMem_ReturnOrd_std_strong_ordering__ {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        CStrMem_ReturnOrd_std_strong_ordering__Impl::operator_cmp_2(__lhs, __rhs.clone())
            == std::cmp::Ordering::Less
    }
}
impl CppEq for CStrMem_ReturnOrd_std_strong_ordering__ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        CStrMem_ReturnOrd_std_strong_ordering__Impl::operator_cmp_2(__lhs, __rhs.clone())
            == std::cmp::Ordering::Equal
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct CStrMem_ReturnOrd_std_weak_ordering__ {
    #[offset(0)]
    #[byte_size(8)]
    pub str: Ptr<()>,
}
impl CppLess for CStrMem_ReturnOrd_std_weak_ordering__ {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        CStrMem_ReturnOrd_std_weak_ordering__Impl::operator_cmp_2(__lhs, __rhs.clone())
            == std::cmp::Ordering::Less
    }
}
impl CppEq for CStrMem_ReturnOrd_std_weak_ordering__ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        CStrMem_ReturnOrd_std_weak_ordering__Impl::operator_cmp_2(__lhs, __rhs.clone())
            == std::cmp::Ordering::Equal
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct CStrMem_ReturnOrd_std_partial_ordering__ {
    #[offset(0)]
    #[byte_size(8)]
    pub str: Ptr<()>,
}
impl CppLess for CStrMem_ReturnOrd_std_partial_ordering__ {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        CStrMem_ReturnOrd_std_partial_ordering__Impl::operator_cmp_2(__lhs, __rhs.clone())
            == std::cmp::Ordering::Less
    }
}
impl CppEq for CStrMem_ReturnOrd_std_partial_ordering__ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        CStrMem_ReturnOrd_std_partial_ordering__Impl::operator_cmp_2(__lhs, __rhs.clone())
            == std::cmp::Ordering::Equal
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct CStrMem_ReturnInt_ {
    #[offset(0)]
    #[byte_size(8)]
    pub str: Ptr<()>,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct CStrMem_CustomOrd_ {
    #[offset(0)]
    #[byte_size(8)]
    pub str: Ptr<()>,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct CStrMem_ReturnVoid_ {
    #[offset(0)]
    #[byte_size(8)]
    pub str: Ptr<()>,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct CStrMem_NoOperators_ {
    #[offset(0)]
    #[byte_size(8)]
    pub str: Ptr<()>,
}
pub fn operator_cmp_13(
    mut lhs: CStrFriend_ReturnOrd_std_strong_ordering__,
    mut rhs: CStrFriend_ReturnOrd_std_strong_ordering__,
) -> () {
    return ({ ReturnOrd_std_strong_ordering_::create(unimplemented!()) });
}
pub fn operator_cmp_14(
    mut lhs: CStrFriend_ReturnOrd_std_strong_ordering__,
    mut rhs: Ptr<()>,
) -> () {
    return ({ ReturnOrd_std_strong_ordering_::create(unimplemented!()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct CStrFriend_ReturnOrd_std_strong_ordering__ {
    #[offset(0)]
    #[byte_size(8)]
    pub str: Ptr<()>,
}
impl CppLess for CStrFriend_ReturnOrd_std_strong_ordering__ {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_cmp_13(__lhs.read(), __rhs.read()) == std::cmp::Ordering::Less
    }
}
impl CppEq for CStrFriend_ReturnOrd_std_strong_ordering__ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_cmp_13(__lhs.read(), __rhs.read()) == std::cmp::Ordering::Equal
    }
}
pub fn operator_cmp_15(
    mut lhs: CStrFriend_ReturnOrd_std_weak_ordering__,
    mut rhs: CStrFriend_ReturnOrd_std_weak_ordering__,
) -> () {
    return ({ ReturnOrd_std_weak_ordering_::create(unimplemented!()) });
}
pub fn operator_cmp_16(mut lhs: CStrFriend_ReturnOrd_std_weak_ordering__, mut rhs: Ptr<()>) -> () {
    return ({ ReturnOrd_std_weak_ordering_::create(unimplemented!()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct CStrFriend_ReturnOrd_std_weak_ordering__ {
    #[offset(0)]
    #[byte_size(8)]
    pub str: Ptr<()>,
}
impl CppLess for CStrFriend_ReturnOrd_std_weak_ordering__ {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_cmp_15(__lhs.read(), __rhs.read()) == std::cmp::Ordering::Less
    }
}
impl CppEq for CStrFriend_ReturnOrd_std_weak_ordering__ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_cmp_15(__lhs.read(), __rhs.read()) == std::cmp::Ordering::Equal
    }
}
pub fn operator_cmp_17(
    mut lhs: CStrFriend_ReturnOrd_std_partial_ordering__,
    mut rhs: CStrFriend_ReturnOrd_std_partial_ordering__,
) -> () {
    return ({ ReturnOrd_std_partial_ordering_::create(unimplemented!()) });
}
pub fn operator_cmp_18(
    mut lhs: CStrFriend_ReturnOrd_std_partial_ordering__,
    mut rhs: Ptr<()>,
) -> () {
    return ({ ReturnOrd_std_partial_ordering_::create(unimplemented!()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct CStrFriend_ReturnOrd_std_partial_ordering__ {
    #[offset(0)]
    #[byte_size(8)]
    pub str: Ptr<()>,
}
impl CppLess for CStrFriend_ReturnOrd_std_partial_ordering__ {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_cmp_17(__lhs.read(), __rhs.read()) == std::cmp::Ordering::Less
    }
}
impl CppEq for CStrFriend_ReturnOrd_std_partial_ordering__ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_cmp_17(__lhs.read(), __rhs.read()) == std::cmp::Ordering::Equal
    }
}
pub fn operator_cmp_19(mut lhs: CStrFriend_ReturnInt_, mut rhs: CStrFriend_ReturnInt_) -> () {
    return ({ ReturnInt::create(unimplemented!()) });
}
pub fn operator_cmp_20(mut lhs: CStrFriend_ReturnInt_, mut rhs: Ptr<()>) -> () {
    return ({ ReturnInt::create(unimplemented!()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct CStrFriend_ReturnInt_ {
    #[offset(0)]
    #[byte_size(8)]
    pub str: Ptr<()>,
}
pub fn operator_cmp_21(
    mut lhs: CStrFriend_CustomOrd_,
    mut rhs: CStrFriend_CustomOrd_,
) -> CustomOrd {
    return ({ CustomOrd::create(unimplemented!()) });
}
pub fn operator_cmp_22(mut lhs: CStrFriend_CustomOrd_, mut rhs: Ptr<()>) -> CustomOrd {
    return ({ CustomOrd::create(unimplemented!()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct CStrFriend_CustomOrd_ {
    #[offset(0)]
    #[byte_size(8)]
    pub str: Ptr<()>,
}
pub fn operator_cmp_23(mut lhs: CStrFriend_ReturnVoid_, mut rhs: CStrFriend_ReturnVoid_) {
    ({ ReturnVoid::create(unimplemented!()) });
    return;
}
pub fn operator_cmp_24(mut lhs: CStrFriend_ReturnVoid_, mut rhs: Ptr<()>) {
    ({ ReturnVoid::create(unimplemented!()) });
    return;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct CStrFriend_ReturnVoid_ {
    #[offset(0)]
    #[byte_size(8)]
    pub str: Ptr<()>,
}
pub fn operator_cmp_25(
    mut lhs: CStrFriend_NoOperators_,
    mut rhs: CStrFriend_NoOperators_,
) -> NoOperators {
    return ({ NoOperators::create(unimplemented!()) });
}
pub fn operator_cmp_26(mut lhs: CStrFriend_NoOperators_, mut rhs: Ptr<()>) -> NoOperators {
    return ({ NoOperators::create(unimplemented!()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct CStrFriend_NoOperators_ {
    #[offset(0)]
    #[byte_size(8)]
    pub str: Ptr<()>,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct CStrFree_ReturnOrd_std_strong_ordering__ {
    #[offset(0)]
    #[byte_size(8)]
    pub str: Ptr<()>,
}
impl CppLess for CStrFree_ReturnOrd_std_strong_ordering__ {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_cmp_27(__lhs.read(), __rhs.read()) == std::cmp::Ordering::Less
    }
}
impl CppEq for CStrFree_ReturnOrd_std_strong_ordering__ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_cmp_27(__lhs.read(), __rhs.read()) == std::cmp::Ordering::Equal
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct CStrFree_ReturnOrd_std_weak_ordering__ {
    #[offset(0)]
    #[byte_size(8)]
    pub str: Ptr<()>,
}
impl CppLess for CStrFree_ReturnOrd_std_weak_ordering__ {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_cmp_28(__lhs.read(), __rhs.read()) == std::cmp::Ordering::Less
    }
}
impl CppEq for CStrFree_ReturnOrd_std_weak_ordering__ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_cmp_28(__lhs.read(), __rhs.read()) == std::cmp::Ordering::Equal
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct CStrFree_ReturnOrd_std_partial_ordering__ {
    #[offset(0)]
    #[byte_size(8)]
    pub str: Ptr<()>,
}
impl CppLess for CStrFree_ReturnOrd_std_partial_ordering__ {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_cmp_29(__lhs.read(), __rhs.read()) == std::cmp::Ordering::Less
    }
}
impl CppEq for CStrFree_ReturnOrd_std_partial_ordering__ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_cmp_29(__lhs.read(), __rhs.read()) == std::cmp::Ordering::Equal
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct CStrFree_ReturnInt_ {
    #[offset(0)]
    #[byte_size(8)]
    pub str: Ptr<()>,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct CStrFree_CustomOrd_ {
    #[offset(0)]
    #[byte_size(8)]
    pub str: Ptr<()>,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct CStrFree_ReturnVoid_ {
    #[offset(0)]
    #[byte_size(8)]
    pub str: Ptr<()>,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct CStrFree_NoOperators_ {
    #[offset(0)]
    #[byte_size(8)]
    pub str: Ptr<()>,
}
pub fn operator_cmp_27(
    mut lhs: CStrFree_ReturnOrd_std_strong_ordering__,
    mut rhs: CStrFree_ReturnOrd_std_strong_ordering__,
) -> () {
    return ({ ReturnOrd_std_strong_ordering_::create(unimplemented!()) });
}
pub fn operator_cmp_28(
    mut lhs: CStrFree_ReturnOrd_std_weak_ordering__,
    mut rhs: CStrFree_ReturnOrd_std_weak_ordering__,
) -> () {
    return ({ ReturnOrd_std_weak_ordering_::create(unimplemented!()) });
}
pub fn operator_cmp_29(
    mut lhs: CStrFree_ReturnOrd_std_partial_ordering__,
    mut rhs: CStrFree_ReturnOrd_std_partial_ordering__,
) -> () {
    return ({ ReturnOrd_std_partial_ordering_::create(unimplemented!()) });
}
pub fn operator_cmp_30(mut lhs: CStrFree_ReturnInt_, mut rhs: CStrFree_ReturnInt_) -> () {
    return ({ ReturnInt::create(unimplemented!()) });
}
pub fn operator_cmp_31(mut lhs: CStrFree_CustomOrd_, mut rhs: CStrFree_CustomOrd_) -> CustomOrd {
    return ({ CustomOrd::create(unimplemented!()) });
}
pub fn operator_cmp_32(mut lhs: CStrFree_ReturnVoid_, mut rhs: CStrFree_ReturnVoid_) {
    ({ ReturnVoid::create(unimplemented!()) });
    return;
}
pub fn operator_cmp_33(
    mut lhs: CStrFree_NoOperators_,
    mut rhs: CStrFree_NoOperators_,
) -> NoOperators {
    return ({ NoOperators::create(unimplemented!()) });
}
pub fn operator_cmp_34(mut lhs: CStrFree_ReturnOrd_std_strong_ordering__, mut rhs: Ptr<()>) -> () {
    return ({ ReturnOrd_std_strong_ordering_::create(unimplemented!()) });
}
pub fn operator_cmp_35(mut lhs: CStrFree_ReturnOrd_std_weak_ordering__, mut rhs: Ptr<()>) -> () {
    return ({ ReturnOrd_std_weak_ordering_::create(unimplemented!()) });
}
pub fn operator_cmp_36(mut lhs: CStrFree_ReturnOrd_std_partial_ordering__, mut rhs: Ptr<()>) -> () {
    return ({ ReturnOrd_std_partial_ordering_::create(unimplemented!()) });
}
pub fn operator_cmp_37(mut lhs: CStrFree_ReturnInt_, mut rhs: Ptr<()>) -> () {
    return ({ ReturnInt::create(unimplemented!()) });
}
pub fn operator_cmp_38(mut lhs: CStrFree_CustomOrd_, mut rhs: Ptr<()>) -> CustomOrd {
    return ({ CustomOrd::create(unimplemented!()) });
}
pub fn operator_cmp_39(mut lhs: CStrFree_ReturnVoid_, mut rhs: Ptr<()>) {
    ({ ReturnVoid::create(unimplemented!()) });
    return;
}
pub fn operator_cmp_40(mut lhs: CStrFree_NoOperators_, mut rhs: Ptr<()>) -> NoOperators {
    return ({ NoOperators::create(unimplemented!()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct CStrMixed_ReturnOrd_std_strong_ordering__ {
    #[offset(0)]
    #[byte_size(8)]
    pub str: Ptr<()>,
}
impl CppLess for CStrMixed_ReturnOrd_std_strong_ordering__ {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_cmp_41(__lhs.read(), __rhs.read()) == std::cmp::Ordering::Less
    }
}
impl CppEq for CStrMixed_ReturnOrd_std_strong_ordering__ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_cmp_41(__lhs.read(), __rhs.read()) == std::cmp::Ordering::Equal
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct CStrMixed_ReturnOrd_std_weak_ordering__ {
    #[offset(0)]
    #[byte_size(8)]
    pub str: Ptr<()>,
}
impl CppLess for CStrMixed_ReturnOrd_std_weak_ordering__ {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_cmp_42(__lhs.read(), __rhs.read()) == std::cmp::Ordering::Less
    }
}
impl CppEq for CStrMixed_ReturnOrd_std_weak_ordering__ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_cmp_42(__lhs.read(), __rhs.read()) == std::cmp::Ordering::Equal
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct CStrMixed_ReturnOrd_std_partial_ordering__ {
    #[offset(0)]
    #[byte_size(8)]
    pub str: Ptr<()>,
}
impl CppLess for CStrMixed_ReturnOrd_std_partial_ordering__ {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_cmp_43(__lhs.read(), __rhs.read()) == std::cmp::Ordering::Less
    }
}
impl CppEq for CStrMixed_ReturnOrd_std_partial_ordering__ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_cmp_43(__lhs.read(), __rhs.read()) == std::cmp::Ordering::Equal
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct CStrMixed_ReturnInt_ {
    #[offset(0)]
    #[byte_size(8)]
    pub str: Ptr<()>,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct CStrMixed_CustomOrd_ {
    #[offset(0)]
    #[byte_size(8)]
    pub str: Ptr<()>,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct CStrMixed_ReturnVoid_ {
    #[offset(0)]
    #[byte_size(8)]
    pub str: Ptr<()>,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct CStrMixed_NoOperators_ {
    #[offset(0)]
    #[byte_size(8)]
    pub str: Ptr<()>,
}
pub fn operator_cmp_41(
    mut lhs: CStrMixed_ReturnOrd_std_strong_ordering__,
    mut rhs: CStrMixed_ReturnOrd_std_strong_ordering__,
) -> () {
    return ({ ReturnOrd_std_strong_ordering_::create(unimplemented!()) });
}
pub fn operator_cmp_42(
    mut lhs: CStrMixed_ReturnOrd_std_weak_ordering__,
    mut rhs: CStrMixed_ReturnOrd_std_weak_ordering__,
) -> () {
    return ({ ReturnOrd_std_weak_ordering_::create(unimplemented!()) });
}
pub fn operator_cmp_43(
    mut lhs: CStrMixed_ReturnOrd_std_partial_ordering__,
    mut rhs: CStrMixed_ReturnOrd_std_partial_ordering__,
) -> () {
    return ({ ReturnOrd_std_partial_ordering_::create(unimplemented!()) });
}
pub fn operator_cmp_44(mut lhs: CStrMixed_ReturnInt_, mut rhs: CStrMixed_ReturnInt_) -> () {
    return ({ ReturnInt::create(unimplemented!()) });
}
pub fn operator_cmp_45(mut lhs: CStrMixed_CustomOrd_, mut rhs: CStrMixed_CustomOrd_) -> CustomOrd {
    return ({ CustomOrd::create(unimplemented!()) });
}
pub fn operator_cmp_46(
    mut lhs: CStrMixed_ReturnOrd_std_strong_ordering__,
    mut rhs: CStrFree_ReturnOrd_std_strong_ordering__,
) -> () {
    return ({ ReturnOrd_std_strong_ordering_::create(unimplemented!()) });
}
pub fn operator_cmp_47(
    mut lhs: CStrMixed_ReturnOrd_std_weak_ordering__,
    mut rhs: CStrFree_ReturnOrd_std_weak_ordering__,
) -> () {
    return ({ ReturnOrd_std_weak_ordering_::create(unimplemented!()) });
}
pub fn operator_cmp_48(
    mut lhs: CStrMixed_ReturnOrd_std_partial_ordering__,
    mut rhs: CStrFree_ReturnOrd_std_partial_ordering__,
) -> () {
    return ({ ReturnOrd_std_partial_ordering_::create(unimplemented!()) });
}
pub fn operator_cmp_49(mut lhs: CStrMixed_ReturnInt_, mut rhs: CStrFree_ReturnInt_) -> () {
    return ({ ReturnInt::create(unimplemented!()) });
}
pub fn operator_cmp_50(mut lhs: CStrMixed_CustomOrd_, mut rhs: CStrFree_CustomOrd_) -> CustomOrd {
    return ({ CustomOrd::create(unimplemented!()) });
}
pub fn operator_cmp_51(mut lhs: CStrMixed_ReturnVoid_, mut rhs: CStrFree_ReturnVoid_) {
    ({ ReturnVoid::create(unimplemented!()) });
    return;
}
pub fn operator_cmp_52(
    mut lhs: CStrMixed_NoOperators_,
    mut rhs: CStrFree_NoOperators_,
) -> NoOperators {
    return ({ NoOperators::create(unimplemented!()) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ReturnVoid {}
impl ReturnVoid {
    pub fn create(mut _a0: ()) {}
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct NoOperators {}
impl NoOperators {
    pub fn create(mut _a0: ()) -> NoOperators {
        return <NoOperators>::default();
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ReturnOrd_std_strong_ordering_ {}
impl ReturnOrd_std_strong_ordering_ {
    pub fn create(mut cmp: ()) -> () {
        return std::cmp::Ord::cmp(&(cmp), &(0_()));
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ReturnOrd_std_weak_ordering_ {}
impl ReturnOrd_std_weak_ordering_ {
    pub fn create(mut cmp: ()) -> () {
        return unimplemented!();
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ReturnOrd_std_partial_ordering_ {}
impl ReturnOrd_std_partial_ordering_ {
    pub fn create(mut cmp: ()) -> () {
        return unimplemented!();
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct ReturnInt {}
impl ReturnInt {
    pub fn create(cmp: ()) -> () {
        let cmp: Value<()> = Rc::new(RefCell::new(cmp));
        return (*cmp.borrow());
    }
}
pub fn operator_lt_53(mut c: CustomOrd, mut _a1: ()) -> () {
    return (c.v < 0_());
}
pub fn operator_lt_54(mut _a0: (), mut c: CustomOrd) -> () {
    return (0_() < c.v);
}
pub fn operator_gt_55(mut c: CustomOrd, mut _a1: ()) -> () {
    return (c.v > 0_());
}
pub fn operator_gt_56(mut _a0: (), mut c: CustomOrd) -> () {
    return (0_() > c.v);
}
pub fn operator_le_57(mut c: CustomOrd, mut _a1: ()) -> () {
    return (c.v <= 0_());
}
pub fn operator_le_58(mut _a0: (), mut c: CustomOrd) -> () {
    return (0_() <= c.v);
}
pub fn operator_ge_59(mut c: CustomOrd, mut _a1: ()) -> () {
    return (c.v >= 0_());
}
pub fn operator_ge_60(mut _a0: (), mut c: CustomOrd) -> () {
    return (0_() >= c.v);
}
pub fn operator_cmp_61(mut c: CustomOrd, mut _a1: ()) -> CustomOrd {
    return c;
}
pub fn operator_cmp_62(mut _a0: (), mut c: CustomOrd) -> CustomOrd {
    return CustomOrd::new_1({ -c.v });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct CustomOrd {
    #[offset(0)]
    v: (),
}
impl CustomOrd {
    pub fn create(mut cmp: ()) -> CustomOrd {
        return CustomOrd::new_1({ cmp });
    }
    pub fn new_1(mut cmp: ()) -> Self {
        Self { v: cmp }
    }
}
impl Default for CustomOrd {
    fn default() -> Self {
        { CustomOrd { v: 0_() } }
    }
}
pub fn test_relational_63(mut use_overloaded: ()) {
    let mut cs1: CStrMixed_ReturnOrd_std_strong_ordering__ =
        CStrMixed_ReturnOrd_std_strong_ordering__ {
            str: (*s1_10.with(Value::clone).borrow()).clone(),
        };
    let mut cs2: CStrFree_ReturnOrd_std_strong_ordering__ =
        CStrFree_ReturnOrd_std_strong_ordering__ {
            str: (*s2_11.with(Value::clone).borrow()).clone(),
        };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_relational_64(mut use_overloaded: ()) {
    let mut cs1: CStrFree_ReturnOrd_std_strong_ordering__ =
        CStrFree_ReturnOrd_std_strong_ordering__ {
            str: (*s1_10.with(Value::clone).borrow()).clone(),
        };
    let mut cs2: CStrMixed_ReturnOrd_std_strong_ordering__ =
        CStrMixed_ReturnOrd_std_strong_ordering__ {
            str: (*s2_11.with(Value::clone).borrow()).clone(),
        };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_relational_65(mut use_overloaded: ()) {
    let cs1: Value<CStrMem_ReturnOrd_std_strong_ordering__> =
        Rc::new(RefCell::new(CStrMem_ReturnOrd_std_strong_ordering__ {
            str: (*s1_10.with(Value::clone).borrow()).clone(),
        }));
    let cs2: Value<CStrMem_ReturnOrd_std_strong_ordering__> =
        Rc::new(RefCell::new(CStrMem_ReturnOrd_std_strong_ordering__ {
            str: (*s2_11.with(Value::clone).borrow()).clone(),
        }));
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_relational_66(mut use_overloaded: ()) {
    let cs1: Value<CStrMem_ReturnOrd_std_strong_ordering__> =
        Rc::new(RefCell::new(CStrMem_ReturnOrd_std_strong_ordering__ {
            str: (*s1_10.with(Value::clone).borrow()).clone(),
        }));
    let mut cs2: Ptr<()> = (*s2_11.with(Value::clone).borrow()).clone();
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_relational_67(mut use_overloaded: ()) {
    let mut cs1: Ptr<()> = (*s1_10.with(Value::clone).borrow()).clone();
    let cs2: Value<CStrMem_ReturnOrd_std_strong_ordering__> =
        Rc::new(RefCell::new(CStrMem_ReturnOrd_std_strong_ordering__ {
            str: (*s2_11.with(Value::clone).borrow()).clone(),
        }));
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_relational_68(mut use_overloaded: ()) {
    let mut cs1: CStrFriend_ReturnOrd_std_strong_ordering__ =
        CStrFriend_ReturnOrd_std_strong_ordering__ {
            str: (*s1_10.with(Value::clone).borrow()).clone(),
        };
    let mut cs2: CStrFriend_ReturnOrd_std_strong_ordering__ =
        CStrFriend_ReturnOrd_std_strong_ordering__ {
            str: (*s2_11.with(Value::clone).borrow()).clone(),
        };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_relational_69(mut use_overloaded: ()) {
    let mut cs1: CStrFriend_ReturnOrd_std_strong_ordering__ =
        CStrFriend_ReturnOrd_std_strong_ordering__ {
            str: (*s1_10.with(Value::clone).borrow()).clone(),
        };
    let mut cs2: Ptr<()> = (*s2_11.with(Value::clone).borrow()).clone();
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_relational_70(mut use_overloaded: ()) {
    let mut cs1: Ptr<()> = (*s1_10.with(Value::clone).borrow()).clone();
    let mut cs2: CStrFriend_ReturnOrd_std_strong_ordering__ =
        CStrFriend_ReturnOrd_std_strong_ordering__ {
            str: (*s2_11.with(Value::clone).borrow()).clone(),
        };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_relational_71(mut use_overloaded: ()) {
    let mut cs1: CStrFree_ReturnOrd_std_strong_ordering__ =
        CStrFree_ReturnOrd_std_strong_ordering__ {
            str: (*s1_10.with(Value::clone).borrow()).clone(),
        };
    let mut cs2: CStrFree_ReturnOrd_std_strong_ordering__ =
        CStrFree_ReturnOrd_std_strong_ordering__ {
            str: (*s2_11.with(Value::clone).borrow()).clone(),
        };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_relational_72(mut use_overloaded: ()) {
    let mut cs1: CStrFree_ReturnOrd_std_strong_ordering__ =
        CStrFree_ReturnOrd_std_strong_ordering__ {
            str: (*s1_10.with(Value::clone).borrow()).clone(),
        };
    let mut cs2: Ptr<()> = (*s2_11.with(Value::clone).borrow()).clone();
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_relational_73(mut use_overloaded: ()) {
    let mut cs1: Ptr<()> = (*s1_10.with(Value::clone).borrow()).clone();
    let mut cs2: CStrFree_ReturnOrd_std_strong_ordering__ =
        CStrFree_ReturnOrd_std_strong_ordering__ {
            str: (*s2_11.with(Value::clone).borrow()).clone(),
        };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_relational_74(mut use_overloaded: ()) {
    let mut cs1: CStrMixed_ReturnOrd_std_weak_ordering__ =
        CStrMixed_ReturnOrd_std_weak_ordering__ {
            str: (*s1_10.with(Value::clone).borrow()).clone(),
        };
    let mut cs2: CStrFree_ReturnOrd_std_weak_ordering__ = CStrFree_ReturnOrd_std_weak_ordering__ {
        str: (*s2_11.with(Value::clone).borrow()).clone(),
    };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_relational_75(mut use_overloaded: ()) {
    let mut cs1: CStrFree_ReturnOrd_std_weak_ordering__ = CStrFree_ReturnOrd_std_weak_ordering__ {
        str: (*s1_10.with(Value::clone).borrow()).clone(),
    };
    let mut cs2: CStrMixed_ReturnOrd_std_weak_ordering__ =
        CStrMixed_ReturnOrd_std_weak_ordering__ {
            str: (*s2_11.with(Value::clone).borrow()).clone(),
        };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_relational_76(mut use_overloaded: ()) {
    let cs1: Value<CStrMem_ReturnOrd_std_weak_ordering__> =
        Rc::new(RefCell::new(CStrMem_ReturnOrd_std_weak_ordering__ {
            str: (*s1_10.with(Value::clone).borrow()).clone(),
        }));
    let cs2: Value<CStrMem_ReturnOrd_std_weak_ordering__> =
        Rc::new(RefCell::new(CStrMem_ReturnOrd_std_weak_ordering__ {
            str: (*s2_11.with(Value::clone).borrow()).clone(),
        }));
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_relational_77(mut use_overloaded: ()) {
    let cs1: Value<CStrMem_ReturnOrd_std_weak_ordering__> =
        Rc::new(RefCell::new(CStrMem_ReturnOrd_std_weak_ordering__ {
            str: (*s1_10.with(Value::clone).borrow()).clone(),
        }));
    let mut cs2: Ptr<()> = (*s2_11.with(Value::clone).borrow()).clone();
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_relational_78(mut use_overloaded: ()) {
    let mut cs1: Ptr<()> = (*s1_10.with(Value::clone).borrow()).clone();
    let cs2: Value<CStrMem_ReturnOrd_std_weak_ordering__> =
        Rc::new(RefCell::new(CStrMem_ReturnOrd_std_weak_ordering__ {
            str: (*s2_11.with(Value::clone).borrow()).clone(),
        }));
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_relational_79(mut use_overloaded: ()) {
    let mut cs1: CStrFriend_ReturnOrd_std_weak_ordering__ =
        CStrFriend_ReturnOrd_std_weak_ordering__ {
            str: (*s1_10.with(Value::clone).borrow()).clone(),
        };
    let mut cs2: CStrFriend_ReturnOrd_std_weak_ordering__ =
        CStrFriend_ReturnOrd_std_weak_ordering__ {
            str: (*s2_11.with(Value::clone).borrow()).clone(),
        };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_relational_80(mut use_overloaded: ()) {
    let mut cs1: CStrFriend_ReturnOrd_std_weak_ordering__ =
        CStrFriend_ReturnOrd_std_weak_ordering__ {
            str: (*s1_10.with(Value::clone).borrow()).clone(),
        };
    let mut cs2: Ptr<()> = (*s2_11.with(Value::clone).borrow()).clone();
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_relational_81(mut use_overloaded: ()) {
    let mut cs1: Ptr<()> = (*s1_10.with(Value::clone).borrow()).clone();
    let mut cs2: CStrFriend_ReturnOrd_std_weak_ordering__ =
        CStrFriend_ReturnOrd_std_weak_ordering__ {
            str: (*s2_11.with(Value::clone).borrow()).clone(),
        };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_relational_82(mut use_overloaded: ()) {
    let mut cs1: CStrFree_ReturnOrd_std_weak_ordering__ = CStrFree_ReturnOrd_std_weak_ordering__ {
        str: (*s1_10.with(Value::clone).borrow()).clone(),
    };
    let mut cs2: CStrFree_ReturnOrd_std_weak_ordering__ = CStrFree_ReturnOrd_std_weak_ordering__ {
        str: (*s2_11.with(Value::clone).borrow()).clone(),
    };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_relational_83(mut use_overloaded: ()) {
    let mut cs1: CStrFree_ReturnOrd_std_weak_ordering__ = CStrFree_ReturnOrd_std_weak_ordering__ {
        str: (*s1_10.with(Value::clone).borrow()).clone(),
    };
    let mut cs2: Ptr<()> = (*s2_11.with(Value::clone).borrow()).clone();
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_relational_84(mut use_overloaded: ()) {
    let mut cs1: Ptr<()> = (*s1_10.with(Value::clone).borrow()).clone();
    let mut cs2: CStrFree_ReturnOrd_std_weak_ordering__ = CStrFree_ReturnOrd_std_weak_ordering__ {
        str: (*s2_11.with(Value::clone).borrow()).clone(),
    };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_relational_85(mut use_overloaded: ()) {
    let mut cs1: CStrMixed_ReturnOrd_std_partial_ordering__ =
        CStrMixed_ReturnOrd_std_partial_ordering__ {
            str: (*s1_10.with(Value::clone).borrow()).clone(),
        };
    let mut cs2: CStrFree_ReturnOrd_std_partial_ordering__ =
        CStrFree_ReturnOrd_std_partial_ordering__ {
            str: (*s2_11.with(Value::clone).borrow()).clone(),
        };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_relational_86(mut use_overloaded: ()) {
    let mut cs1: CStrFree_ReturnOrd_std_partial_ordering__ =
        CStrFree_ReturnOrd_std_partial_ordering__ {
            str: (*s1_10.with(Value::clone).borrow()).clone(),
        };
    let mut cs2: CStrMixed_ReturnOrd_std_partial_ordering__ =
        CStrMixed_ReturnOrd_std_partial_ordering__ {
            str: (*s2_11.with(Value::clone).borrow()).clone(),
        };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_relational_87(mut use_overloaded: ()) {
    let cs1: Value<CStrMem_ReturnOrd_std_partial_ordering__> =
        Rc::new(RefCell::new(CStrMem_ReturnOrd_std_partial_ordering__ {
            str: (*s1_10.with(Value::clone).borrow()).clone(),
        }));
    let cs2: Value<CStrMem_ReturnOrd_std_partial_ordering__> =
        Rc::new(RefCell::new(CStrMem_ReturnOrd_std_partial_ordering__ {
            str: (*s2_11.with(Value::clone).borrow()).clone(),
        }));
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_relational_88(mut use_overloaded: ()) {
    let cs1: Value<CStrMem_ReturnOrd_std_partial_ordering__> =
        Rc::new(RefCell::new(CStrMem_ReturnOrd_std_partial_ordering__ {
            str: (*s1_10.with(Value::clone).borrow()).clone(),
        }));
    let mut cs2: Ptr<()> = (*s2_11.with(Value::clone).borrow()).clone();
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_relational_89(mut use_overloaded: ()) {
    let mut cs1: Ptr<()> = (*s1_10.with(Value::clone).borrow()).clone();
    let cs2: Value<CStrMem_ReturnOrd_std_partial_ordering__> =
        Rc::new(RefCell::new(CStrMem_ReturnOrd_std_partial_ordering__ {
            str: (*s2_11.with(Value::clone).borrow()).clone(),
        }));
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_relational_90(mut use_overloaded: ()) {
    let mut cs1: CStrFriend_ReturnOrd_std_partial_ordering__ =
        CStrFriend_ReturnOrd_std_partial_ordering__ {
            str: (*s1_10.with(Value::clone).borrow()).clone(),
        };
    let mut cs2: CStrFriend_ReturnOrd_std_partial_ordering__ =
        CStrFriend_ReturnOrd_std_partial_ordering__ {
            str: (*s2_11.with(Value::clone).borrow()).clone(),
        };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_relational_91(mut use_overloaded: ()) {
    let mut cs1: CStrFriend_ReturnOrd_std_partial_ordering__ =
        CStrFriend_ReturnOrd_std_partial_ordering__ {
            str: (*s1_10.with(Value::clone).borrow()).clone(),
        };
    let mut cs2: Ptr<()> = (*s2_11.with(Value::clone).borrow()).clone();
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_relational_92(mut use_overloaded: ()) {
    let mut cs1: Ptr<()> = (*s1_10.with(Value::clone).borrow()).clone();
    let mut cs2: CStrFriend_ReturnOrd_std_partial_ordering__ =
        CStrFriend_ReturnOrd_std_partial_ordering__ {
            str: (*s2_11.with(Value::clone).borrow()).clone(),
        };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_relational_93(mut use_overloaded: ()) {
    let mut cs1: CStrFree_ReturnOrd_std_partial_ordering__ =
        CStrFree_ReturnOrd_std_partial_ordering__ {
            str: (*s1_10.with(Value::clone).borrow()).clone(),
        };
    let mut cs2: CStrFree_ReturnOrd_std_partial_ordering__ =
        CStrFree_ReturnOrd_std_partial_ordering__ {
            str: (*s2_11.with(Value::clone).borrow()).clone(),
        };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_relational_94(mut use_overloaded: ()) {
    let mut cs1: CStrFree_ReturnOrd_std_partial_ordering__ =
        CStrFree_ReturnOrd_std_partial_ordering__ {
            str: (*s1_10.with(Value::clone).borrow()).clone(),
        };
    let mut cs2: Ptr<()> = (*s2_11.with(Value::clone).borrow()).clone();
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_relational_95(mut use_overloaded: ()) {
    let mut cs1: Ptr<()> = (*s1_10.with(Value::clone).borrow()).clone();
    let mut cs2: CStrFree_ReturnOrd_std_partial_ordering__ =
        CStrFree_ReturnOrd_std_partial_ordering__ {
            str: (*s2_11.with(Value::clone).borrow()).clone(),
        };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_relational_96(mut use_overloaded: ()) {
    let cs1: Value<CStrNone> = Rc::new(RefCell::new(CStrNone {
        str: (*s1_10.with(Value::clone).borrow()).clone(),
    }));
    let cs2: Value<CStrNone> = Rc::new(RefCell::new(CStrNone {
        str: (*s2_11.with(Value::clone).borrow()).clone(),
    }));
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(({ CStrNoneImpl::to_pconst(&cs1.as_pointer()) })
                < ({ CStrNoneImpl::to_pconst(&cs2.as_pointer()) })))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({ CStrNoneImpl::to_pconst(&cs1.as_pointer()) })
                > ({ CStrNoneImpl::to_pconst(&cs2.as_pointer()) }))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(({ CStrNoneImpl::to_pconst(&cs1.as_pointer()) })
                < ({ CStrNoneImpl::to_pconst(&cs2.as_pointer()) })))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({ CStrNoneImpl::to_pconst(&cs1.as_pointer()) })
                > ({ CStrNoneImpl::to_pconst(&cs2.as_pointer()) }))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({ CStrNoneImpl::to_pconst(&cs1.as_pointer()) })
                < ({ CStrNoneImpl::to_pconst(&cs2.as_pointer()) }))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(({ CStrNoneImpl::to_pconst(&cs1.as_pointer()) })
                > ({ CStrNoneImpl::to_pconst(&cs2.as_pointer()) })))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({ CStrNoneImpl::to_pconst(&cs1.as_pointer()) })
                < ({ CStrNoneImpl::to_pconst(&cs2.as_pointer()) }))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(({ CStrNoneImpl::to_pconst(&cs1.as_pointer()) })
                > ({ CStrNoneImpl::to_pconst(&cs2.as_pointer()) })))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_relational_97(mut use_overloaded: ()) {
    let cs1: Value<CStrNone> = Rc::new(RefCell::new(CStrNone {
        str: (*s1_10.with(Value::clone).borrow()).clone(),
    }));
    let mut cs2: Ptr<()> = (*s2_11.with(Value::clone).borrow()).clone();
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!({ ({ CStrNoneImpl::to_pconst(&cs1.as_pointer()) }) } < { (cs2).clone() })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ ({ CStrNoneImpl::to_pconst(&cs1.as_pointer()) }) } > { (cs2).clone() }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!({ ({ CStrNoneImpl::to_pconst(&cs1.as_pointer()) }) } < { (cs2).clone() })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ ({ CStrNoneImpl::to_pconst(&cs1.as_pointer()) }) } > { (cs2).clone() }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ ({ CStrNoneImpl::to_pconst(&cs1.as_pointer()) }) } < { (cs2).clone() }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!({ ({ CStrNoneImpl::to_pconst(&cs1.as_pointer()) }) } > { (cs2).clone() })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ ({ CStrNoneImpl::to_pconst(&cs1.as_pointer()) }) } < { (cs2).clone() }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!({ ({ CStrNoneImpl::to_pconst(&cs1.as_pointer()) }) } > { (cs2).clone() })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_relational_98(mut use_overloaded: ()) {
    let mut cs1: Ptr<()> = (*s1_10.with(Value::clone).borrow()).clone();
    let cs2: Value<CStrNone> = Rc::new(RefCell::new(CStrNone {
        str: (*s2_11.with(Value::clone).borrow()).clone(),
    }));
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!({ (cs1).clone() } < { ({ CStrNoneImpl::to_pconst(&cs2.as_pointer()) }) })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ (cs1).clone() } > { ({ CStrNoneImpl::to_pconst(&cs2.as_pointer()) }) }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!({ (cs1).clone() } < { ({ CStrNoneImpl::to_pconst(&cs2.as_pointer()) }) })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ (cs1).clone() } > { ({ CStrNoneImpl::to_pconst(&cs2.as_pointer()) }) }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ (cs1).clone() } < { ({ CStrNoneImpl::to_pconst(&cs2.as_pointer()) }) }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!({ (cs1).clone() } > { ({ CStrNoneImpl::to_pconst(&cs2.as_pointer()) }) })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({ (cs1).clone() } < { ({ CStrNoneImpl::to_pconst(&cs2.as_pointer()) }) }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!({ (cs1).clone() } > { ({ CStrNoneImpl::to_pconst(&cs2.as_pointer()) }) })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_relational_99(mut use_overloaded: ()) {
    let mut cs1: CStrMixed_ReturnInt_ = CStrMixed_ReturnInt_ {
        str: (*s1_10.with(Value::clone).borrow()).clone(),
    };
    let mut cs2: CStrFree_ReturnInt_ = CStrFree_ReturnInt_ {
        str: (*s2_11.with(Value::clone).borrow()).clone(),
    };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(({
                let _lhs: CStrMixed_ReturnInt_ = (cs1).clone();
                operator_cmp_49(_lhs, (cs2).clone())
            }) < 0_()))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({
                let _lhs: CStrMixed_ReturnInt_ = (cs1).clone();
                operator_cmp_49(_lhs, (cs2).clone())
            }) > 0_())
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(({
                let _lhs: CStrMixed_ReturnInt_ = (cs1).clone();
                operator_cmp_49(_lhs, (cs2).clone())
            }) < 0_()))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({
                let _lhs: CStrMixed_ReturnInt_ = (cs1).clone();
                operator_cmp_49(_lhs, (cs2).clone())
            }) > 0_())
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({
                let _lhs: CStrMixed_ReturnInt_ = (cs1).clone();
                operator_cmp_49(_lhs, (cs2).clone())
            }) < 0_())
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(({
                let _lhs: CStrMixed_ReturnInt_ = (cs1).clone();
                operator_cmp_49(_lhs, (cs2).clone())
            }) > 0_()))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({
                let _lhs: CStrMixed_ReturnInt_ = (cs1).clone();
                operator_cmp_49(_lhs, (cs2).clone())
            }) < 0_())
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(({
                let _lhs: CStrMixed_ReturnInt_ = (cs1).clone();
                operator_cmp_49(_lhs, (cs2).clone())
            }) > 0_()))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_relational_100(mut use_overloaded: ()) {
    let mut cs1: CStrFree_ReturnInt_ = CStrFree_ReturnInt_ {
        str: (*s1_10.with(Value::clone).borrow()).clone(),
    };
    let mut cs2: CStrMixed_ReturnInt_ = CStrMixed_ReturnInt_ {
        str: (*s2_11.with(Value::clone).borrow()).clone(),
    };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(0_()
                < ({
                    let _lhs: CStrMixed_ReturnInt_ = (cs2).clone();
                    operator_cmp_49(_lhs, (cs1).clone())
                })))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(0_()
                > ({
                    let _lhs: CStrMixed_ReturnInt_ = (cs2).clone();
                    operator_cmp_49(_lhs, (cs1).clone())
                }))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(0_()
                < ({
                    let _lhs: CStrMixed_ReturnInt_ = (cs2).clone();
                    operator_cmp_49(_lhs, (cs1).clone())
                })))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(0_()
                > ({
                    let _lhs: CStrMixed_ReturnInt_ = (cs2).clone();
                    operator_cmp_49(_lhs, (cs1).clone())
                }))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(0_()
                < ({
                    let _lhs: CStrMixed_ReturnInt_ = (cs2).clone();
                    operator_cmp_49(_lhs, (cs1).clone())
                }))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(0_()
                > ({
                    let _lhs: CStrMixed_ReturnInt_ = (cs2).clone();
                    operator_cmp_49(_lhs, (cs1).clone())
                })))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(0_()
                < ({
                    let _lhs: CStrMixed_ReturnInt_ = (cs2).clone();
                    operator_cmp_49(_lhs, (cs1).clone())
                }))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(0_()
                > ({
                    let _lhs: CStrMixed_ReturnInt_ = (cs2).clone();
                    operator_cmp_49(_lhs, (cs1).clone())
                })))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_relational_101(mut use_overloaded: ()) {
    let cs1: Value<CStrMem_ReturnInt_> = Rc::new(RefCell::new(CStrMem_ReturnInt_ {
        str: (*s1_10.with(Value::clone).borrow()).clone(),
    }));
    let cs2: Value<CStrMem_ReturnInt_> = Rc::new(RefCell::new(CStrMem_ReturnInt_ {
        str: (*s2_11.with(Value::clone).borrow()).clone(),
    }));
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(({
                CStrMem_ReturnInt_Impl::operator_cmp_2(&cs1.as_pointer(), cs2.as_pointer())
            }) < 0_()))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({ CStrMem_ReturnInt_Impl::operator_cmp_2(&cs1.as_pointer(), cs2.as_pointer()) })
                > 0_())
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(({
                CStrMem_ReturnInt_Impl::operator_cmp_2(&cs1.as_pointer(), cs2.as_pointer())
            }) < 0_()))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({ CStrMem_ReturnInt_Impl::operator_cmp_2(&cs1.as_pointer(), cs2.as_pointer()) })
                > 0_())
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({ CStrMem_ReturnInt_Impl::operator_cmp_2(&cs1.as_pointer(), cs2.as_pointer()) })
                < 0_())
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(({
                CStrMem_ReturnInt_Impl::operator_cmp_2(&cs1.as_pointer(), cs2.as_pointer())
            }) > 0_()))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({ CStrMem_ReturnInt_Impl::operator_cmp_2(&cs1.as_pointer(), cs2.as_pointer()) })
                < 0_())
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(({
                CStrMem_ReturnInt_Impl::operator_cmp_2(&cs1.as_pointer(), cs2.as_pointer())
            }) > 0_()))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_relational_102(mut use_overloaded: ()) {
    let cs1: Value<CStrMem_ReturnInt_> = Rc::new(RefCell::new(CStrMem_ReturnInt_ {
        str: (*s1_10.with(Value::clone).borrow()).clone(),
    }));
    let mut cs2: Ptr<()> = (*s2_11.with(Value::clone).borrow()).clone();
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(({ CStrMem_ReturnInt_Impl::operator_cmp_3(&cs1.as_pointer(), (cs2).clone()) })
                < 0_()))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({ CStrMem_ReturnInt_Impl::operator_cmp_3(&cs1.as_pointer(), (cs2).clone()) })
                > 0_())
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(({ CStrMem_ReturnInt_Impl::operator_cmp_3(&cs1.as_pointer(), (cs2).clone()) })
                < 0_()))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({ CStrMem_ReturnInt_Impl::operator_cmp_3(&cs1.as_pointer(), (cs2).clone()) })
                > 0_())
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({ CStrMem_ReturnInt_Impl::operator_cmp_3(&cs1.as_pointer(), (cs2).clone()) })
                < 0_())
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(({ CStrMem_ReturnInt_Impl::operator_cmp_3(&cs1.as_pointer(), (cs2).clone()) })
                > 0_()))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({ CStrMem_ReturnInt_Impl::operator_cmp_3(&cs1.as_pointer(), (cs2).clone()) })
                < 0_())
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(({ CStrMem_ReturnInt_Impl::operator_cmp_3(&cs1.as_pointer(), (cs2).clone()) })
                > 0_()))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_relational_103(mut use_overloaded: ()) {
    let mut cs1: Ptr<()> = (*s1_10.with(Value::clone).borrow()).clone();
    let cs2: Value<CStrMem_ReturnInt_> = Rc::new(RefCell::new(CStrMem_ReturnInt_ {
        str: (*s2_11.with(Value::clone).borrow()).clone(),
    }));
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(0_()
                < ({ CStrMem_ReturnInt_Impl::operator_cmp_3(&cs2.as_pointer(), (cs1).clone()) })))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(0_()
                > ({ CStrMem_ReturnInt_Impl::operator_cmp_3(&cs2.as_pointer(), (cs1).clone()) }))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(0_()
                < ({ CStrMem_ReturnInt_Impl::operator_cmp_3(&cs2.as_pointer(), (cs1).clone()) })))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(0_()
                > ({ CStrMem_ReturnInt_Impl::operator_cmp_3(&cs2.as_pointer(), (cs1).clone()) }))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(0_()
                < ({ CStrMem_ReturnInt_Impl::operator_cmp_3(&cs2.as_pointer(), (cs1).clone()) }))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(0_()
                > ({ CStrMem_ReturnInt_Impl::operator_cmp_3(&cs2.as_pointer(), (cs1).clone()) })))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(0_()
                < ({ CStrMem_ReturnInt_Impl::operator_cmp_3(&cs2.as_pointer(), (cs1).clone()) }))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(0_()
                > ({ CStrMem_ReturnInt_Impl::operator_cmp_3(&cs2.as_pointer(), (cs1).clone()) })))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_relational_104(mut use_overloaded: ()) {
    let mut cs1: CStrFriend_ReturnInt_ = CStrFriend_ReturnInt_ {
        str: (*s1_10.with(Value::clone).borrow()).clone(),
    };
    let mut cs2: CStrFriend_ReturnInt_ = CStrFriend_ReturnInt_ {
        str: (*s2_11.with(Value::clone).borrow()).clone(),
    };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(({
                let _lhs: CStrFriend_ReturnInt_ = (cs1).clone();
                operator_cmp_19(_lhs, (cs2).clone())
            }) < 0_()))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({
                let _lhs: CStrFriend_ReturnInt_ = (cs1).clone();
                operator_cmp_19(_lhs, (cs2).clone())
            }) > 0_())
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(({
                let _lhs: CStrFriend_ReturnInt_ = (cs1).clone();
                operator_cmp_19(_lhs, (cs2).clone())
            }) < 0_()))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({
                let _lhs: CStrFriend_ReturnInt_ = (cs1).clone();
                operator_cmp_19(_lhs, (cs2).clone())
            }) > 0_())
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({
                let _lhs: CStrFriend_ReturnInt_ = (cs1).clone();
                operator_cmp_19(_lhs, (cs2).clone())
            }) < 0_())
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(({
                let _lhs: CStrFriend_ReturnInt_ = (cs1).clone();
                operator_cmp_19(_lhs, (cs2).clone())
            }) > 0_()))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({
                let _lhs: CStrFriend_ReturnInt_ = (cs1).clone();
                operator_cmp_19(_lhs, (cs2).clone())
            }) < 0_())
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(({
                let _lhs: CStrFriend_ReturnInt_ = (cs1).clone();
                operator_cmp_19(_lhs, (cs2).clone())
            }) > 0_()))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_relational_105(mut use_overloaded: ()) {
    let mut cs1: CStrFriend_ReturnInt_ = CStrFriend_ReturnInt_ {
        str: (*s1_10.with(Value::clone).borrow()).clone(),
    };
    let mut cs2: Ptr<()> = (*s2_11.with(Value::clone).borrow()).clone();
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(({
                let _lhs: CStrFriend_ReturnInt_ = (cs1).clone();
                operator_cmp_20(_lhs, (cs2).clone())
            }) < 0_()))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({
                let _lhs: CStrFriend_ReturnInt_ = (cs1).clone();
                operator_cmp_20(_lhs, (cs2).clone())
            }) > 0_())
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(({
                let _lhs: CStrFriend_ReturnInt_ = (cs1).clone();
                operator_cmp_20(_lhs, (cs2).clone())
            }) < 0_()))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({
                let _lhs: CStrFriend_ReturnInt_ = (cs1).clone();
                operator_cmp_20(_lhs, (cs2).clone())
            }) > 0_())
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({
                let _lhs: CStrFriend_ReturnInt_ = (cs1).clone();
                operator_cmp_20(_lhs, (cs2).clone())
            }) < 0_())
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(({
                let _lhs: CStrFriend_ReturnInt_ = (cs1).clone();
                operator_cmp_20(_lhs, (cs2).clone())
            }) > 0_()))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({
                let _lhs: CStrFriend_ReturnInt_ = (cs1).clone();
                operator_cmp_20(_lhs, (cs2).clone())
            }) < 0_())
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(({
                let _lhs: CStrFriend_ReturnInt_ = (cs1).clone();
                operator_cmp_20(_lhs, (cs2).clone())
            }) > 0_()))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_relational_106(mut use_overloaded: ()) {
    let mut cs1: Ptr<()> = (*s1_10.with(Value::clone).borrow()).clone();
    let mut cs2: CStrFriend_ReturnInt_ = CStrFriend_ReturnInt_ {
        str: (*s2_11.with(Value::clone).borrow()).clone(),
    };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(0_()
                < ({
                    let _lhs: CStrFriend_ReturnInt_ = (cs2).clone();
                    operator_cmp_20(_lhs, (cs1).clone())
                })))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(0_()
                > ({
                    let _lhs: CStrFriend_ReturnInt_ = (cs2).clone();
                    operator_cmp_20(_lhs, (cs1).clone())
                }))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(0_()
                < ({
                    let _lhs: CStrFriend_ReturnInt_ = (cs2).clone();
                    operator_cmp_20(_lhs, (cs1).clone())
                })))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(0_()
                > ({
                    let _lhs: CStrFriend_ReturnInt_ = (cs2).clone();
                    operator_cmp_20(_lhs, (cs1).clone())
                }))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(0_()
                < ({
                    let _lhs: CStrFriend_ReturnInt_ = (cs2).clone();
                    operator_cmp_20(_lhs, (cs1).clone())
                }))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(0_()
                > ({
                    let _lhs: CStrFriend_ReturnInt_ = (cs2).clone();
                    operator_cmp_20(_lhs, (cs1).clone())
                })))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(0_()
                < ({
                    let _lhs: CStrFriend_ReturnInt_ = (cs2).clone();
                    operator_cmp_20(_lhs, (cs1).clone())
                }))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(0_()
                > ({
                    let _lhs: CStrFriend_ReturnInt_ = (cs2).clone();
                    operator_cmp_20(_lhs, (cs1).clone())
                })))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_relational_107(mut use_overloaded: ()) {
    let mut cs1: CStrFree_ReturnInt_ = CStrFree_ReturnInt_ {
        str: (*s1_10.with(Value::clone).borrow()).clone(),
    };
    let mut cs2: CStrFree_ReturnInt_ = CStrFree_ReturnInt_ {
        str: (*s2_11.with(Value::clone).borrow()).clone(),
    };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(({
                let _lhs: CStrFree_ReturnInt_ = (cs1).clone();
                operator_cmp_30(_lhs, (cs2).clone())
            }) < 0_()))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({
                let _lhs: CStrFree_ReturnInt_ = (cs1).clone();
                operator_cmp_30(_lhs, (cs2).clone())
            }) > 0_())
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(({
                let _lhs: CStrFree_ReturnInt_ = (cs1).clone();
                operator_cmp_30(_lhs, (cs2).clone())
            }) < 0_()))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({
                let _lhs: CStrFree_ReturnInt_ = (cs1).clone();
                operator_cmp_30(_lhs, (cs2).clone())
            }) > 0_())
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({
                let _lhs: CStrFree_ReturnInt_ = (cs1).clone();
                operator_cmp_30(_lhs, (cs2).clone())
            }) < 0_())
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(({
                let _lhs: CStrFree_ReturnInt_ = (cs1).clone();
                operator_cmp_30(_lhs, (cs2).clone())
            }) > 0_()))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({
                let _lhs: CStrFree_ReturnInt_ = (cs1).clone();
                operator_cmp_30(_lhs, (cs2).clone())
            }) < 0_())
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(({
                let _lhs: CStrFree_ReturnInt_ = (cs1).clone();
                operator_cmp_30(_lhs, (cs2).clone())
            }) > 0_()))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_relational_108(mut use_overloaded: ()) {
    let mut cs1: CStrFree_ReturnInt_ = CStrFree_ReturnInt_ {
        str: (*s1_10.with(Value::clone).borrow()).clone(),
    };
    let mut cs2: Ptr<()> = (*s2_11.with(Value::clone).borrow()).clone();
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(({
                let _lhs: CStrFree_ReturnInt_ = (cs1).clone();
                operator_cmp_37(_lhs, (cs2).clone())
            }) < 0_()))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({
                let _lhs: CStrFree_ReturnInt_ = (cs1).clone();
                operator_cmp_37(_lhs, (cs2).clone())
            }) > 0_())
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(({
                let _lhs: CStrFree_ReturnInt_ = (cs1).clone();
                operator_cmp_37(_lhs, (cs2).clone())
            }) < 0_()))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({
                let _lhs: CStrFree_ReturnInt_ = (cs1).clone();
                operator_cmp_37(_lhs, (cs2).clone())
            }) > 0_())
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({
                let _lhs: CStrFree_ReturnInt_ = (cs1).clone();
                operator_cmp_37(_lhs, (cs2).clone())
            }) < 0_())
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(({
                let _lhs: CStrFree_ReturnInt_ = (cs1).clone();
                operator_cmp_37(_lhs, (cs2).clone())
            }) > 0_()))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(({
                let _lhs: CStrFree_ReturnInt_ = (cs1).clone();
                operator_cmp_37(_lhs, (cs2).clone())
            }) < 0_())
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(({
                let _lhs: CStrFree_ReturnInt_ = (cs1).clone();
                operator_cmp_37(_lhs, (cs2).clone())
            }) > 0_()))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_relational_109(mut use_overloaded: ()) {
    let mut cs1: Ptr<()> = (*s1_10.with(Value::clone).borrow()).clone();
    let mut cs2: CStrFree_ReturnInt_ = CStrFree_ReturnInt_ {
        str: (*s2_11.with(Value::clone).borrow()).clone(),
    };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(0_()
                < ({
                    let _lhs: CStrFree_ReturnInt_ = (cs2).clone();
                    operator_cmp_37(_lhs, (cs1).clone())
                })))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(0_()
                > ({
                    let _lhs: CStrFree_ReturnInt_ = (cs2).clone();
                    operator_cmp_37(_lhs, (cs1).clone())
                }))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(0_()
                < ({
                    let _lhs: CStrFree_ReturnInt_ = (cs2).clone();
                    operator_cmp_37(_lhs, (cs1).clone())
                })))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(0_()
                > ({
                    let _lhs: CStrFree_ReturnInt_ = (cs2).clone();
                    operator_cmp_37(_lhs, (cs1).clone())
                }))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(0_()
                < ({
                    let _lhs: CStrFree_ReturnInt_ = (cs2).clone();
                    operator_cmp_37(_lhs, (cs1).clone())
                }))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(0_()
                > ({
                    let _lhs: CStrFree_ReturnInt_ = (cs2).clone();
                    operator_cmp_37(_lhs, (cs1).clone())
                })))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(0_()
                < ({
                    let _lhs: CStrFree_ReturnInt_ = (cs2).clone();
                    operator_cmp_37(_lhs, (cs1).clone())
                }))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(0_()
                > ({
                    let _lhs: CStrFree_ReturnInt_ = (cs2).clone();
                    operator_cmp_37(_lhs, (cs1).clone())
                })))
            {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_relational_110(mut use_overloaded: ()) {
    let mut cs1: CStrMixed_CustomOrd_ = CStrMixed_CustomOrd_ {
        str: (*s1_10.with(Value::clone).borrow()).clone(),
    };
    let mut cs2: CStrFree_CustomOrd_ = CStrFree_CustomOrd_ {
        str: (*s2_11.with(Value::clone).borrow()).clone(),
    };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!({
                let _c: CustomOrd = ({
                    let _lhs: CStrMixed_CustomOrd_ = (cs1).clone();
                    operator_cmp_50(_lhs, (cs2).clone())
                });
                operator_lt_53(_c, unimplemented!())
            })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({
                let _c: CustomOrd = ({
                    let _lhs: CStrMixed_CustomOrd_ = (cs1).clone();
                    operator_cmp_50(_lhs, (cs2).clone())
                });
                operator_gt_55(_c, unimplemented!())
            }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!({
                let _c: CustomOrd = ({
                    let _lhs: CStrMixed_CustomOrd_ = (cs1).clone();
                    operator_cmp_50(_lhs, (cs2).clone())
                });
                operator_lt_53(_c, unimplemented!())
            })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({
                let _c: CustomOrd = ({
                    let _lhs: CStrMixed_CustomOrd_ = (cs1).clone();
                    operator_cmp_50(_lhs, (cs2).clone())
                });
                operator_gt_55(_c, unimplemented!())
            }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({
                let _c: CustomOrd = ({
                    let _lhs: CStrMixed_CustomOrd_ = (cs1).clone();
                    operator_cmp_50(_lhs, (cs2).clone())
                });
                operator_lt_53(_c, unimplemented!())
            }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!({
                let _c: CustomOrd = ({
                    let _lhs: CStrMixed_CustomOrd_ = (cs1).clone();
                    operator_cmp_50(_lhs, (cs2).clone())
                });
                operator_gt_55(_c, unimplemented!())
            })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({
                let _c: CustomOrd = ({
                    let _lhs: CStrMixed_CustomOrd_ = (cs1).clone();
                    operator_cmp_50(_lhs, (cs2).clone())
                });
                operator_lt_53(_c, unimplemented!())
            }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!({
                let _c: CustomOrd = ({
                    let _lhs: CStrMixed_CustomOrd_ = (cs1).clone();
                    operator_cmp_50(_lhs, (cs2).clone())
                });
                operator_gt_55(_c, unimplemented!())
            })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_relational_111(mut use_overloaded: ()) {
    let mut cs1: CStrFree_CustomOrd_ = CStrFree_CustomOrd_ {
        str: (*s1_10.with(Value::clone).borrow()).clone(),
    };
    let mut cs2: CStrMixed_CustomOrd_ = CStrMixed_CustomOrd_ {
        str: (*s2_11.with(Value::clone).borrow()).clone(),
    };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!({
                operator_lt_54(
                    unimplemented!(),
                    ({
                        let _lhs: CStrMixed_CustomOrd_ = (cs2).clone();
                        operator_cmp_50(_lhs, (cs1).clone())
                    }),
                )
            })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({
                operator_gt_56(
                    unimplemented!(),
                    ({
                        let _lhs: CStrMixed_CustomOrd_ = (cs2).clone();
                        operator_cmp_50(_lhs, (cs1).clone())
                    }),
                )
            }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!({
                operator_lt_54(
                    unimplemented!(),
                    ({
                        let _lhs: CStrMixed_CustomOrd_ = (cs2).clone();
                        operator_cmp_50(_lhs, (cs1).clone())
                    }),
                )
            })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({
                operator_gt_56(
                    unimplemented!(),
                    ({
                        let _lhs: CStrMixed_CustomOrd_ = (cs2).clone();
                        operator_cmp_50(_lhs, (cs1).clone())
                    }),
                )
            }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({
                operator_lt_54(
                    unimplemented!(),
                    ({
                        let _lhs: CStrMixed_CustomOrd_ = (cs2).clone();
                        operator_cmp_50(_lhs, (cs1).clone())
                    }),
                )
            }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!({
                operator_gt_56(
                    unimplemented!(),
                    ({
                        let _lhs: CStrMixed_CustomOrd_ = (cs2).clone();
                        operator_cmp_50(_lhs, (cs1).clone())
                    }),
                )
            })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({
                operator_lt_54(
                    unimplemented!(),
                    ({
                        let _lhs: CStrMixed_CustomOrd_ = (cs2).clone();
                        operator_cmp_50(_lhs, (cs1).clone())
                    }),
                )
            }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!({
                operator_gt_56(
                    unimplemented!(),
                    ({
                        let _lhs: CStrMixed_CustomOrd_ = (cs2).clone();
                        operator_cmp_50(_lhs, (cs1).clone())
                    }),
                )
            })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_relational_112(mut use_overloaded: ()) {
    let cs1: Value<CStrMem_CustomOrd_> = Rc::new(RefCell::new(CStrMem_CustomOrd_ {
        str: (*s1_10.with(Value::clone).borrow()).clone(),
    }));
    let cs2: Value<CStrMem_CustomOrd_> = Rc::new(RefCell::new(CStrMem_CustomOrd_ {
        str: (*s2_11.with(Value::clone).borrow()).clone(),
    }));
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!({
                let _c: CustomOrd = ({
                    CStrMem_CustomOrd_Impl::operator_cmp_2(&cs1.as_pointer(), cs2.as_pointer())
                });
                operator_lt_53(_c, unimplemented!())
            })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({
                let _c: CustomOrd = ({
                    CStrMem_CustomOrd_Impl::operator_cmp_2(&cs1.as_pointer(), cs2.as_pointer())
                });
                operator_gt_55(_c, unimplemented!())
            }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!({
                let _c: CustomOrd = ({
                    CStrMem_CustomOrd_Impl::operator_cmp_2(&cs1.as_pointer(), cs2.as_pointer())
                });
                operator_lt_53(_c, unimplemented!())
            })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({
                let _c: CustomOrd = ({
                    CStrMem_CustomOrd_Impl::operator_cmp_2(&cs1.as_pointer(), cs2.as_pointer())
                });
                operator_gt_55(_c, unimplemented!())
            }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({
                let _c: CustomOrd = ({
                    CStrMem_CustomOrd_Impl::operator_cmp_2(&cs1.as_pointer(), cs2.as_pointer())
                });
                operator_lt_53(_c, unimplemented!())
            }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!({
                let _c: CustomOrd = ({
                    CStrMem_CustomOrd_Impl::operator_cmp_2(&cs1.as_pointer(), cs2.as_pointer())
                });
                operator_gt_55(_c, unimplemented!())
            })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({
                let _c: CustomOrd = ({
                    CStrMem_CustomOrd_Impl::operator_cmp_2(&cs1.as_pointer(), cs2.as_pointer())
                });
                operator_lt_53(_c, unimplemented!())
            }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!({
                let _c: CustomOrd = ({
                    CStrMem_CustomOrd_Impl::operator_cmp_2(&cs1.as_pointer(), cs2.as_pointer())
                });
                operator_gt_55(_c, unimplemented!())
            })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_relational_113(mut use_overloaded: ()) {
    let cs1: Value<CStrMem_CustomOrd_> = Rc::new(RefCell::new(CStrMem_CustomOrd_ {
        str: (*s1_10.with(Value::clone).borrow()).clone(),
    }));
    let mut cs2: Ptr<()> = (*s2_11.with(Value::clone).borrow()).clone();
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!({
                let _c: CustomOrd =
                    ({ CStrMem_CustomOrd_Impl::operator_cmp_3(&cs1.as_pointer(), (cs2).clone()) });
                operator_lt_53(_c, unimplemented!())
            })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({
                let _c: CustomOrd =
                    ({ CStrMem_CustomOrd_Impl::operator_cmp_3(&cs1.as_pointer(), (cs2).clone()) });
                operator_gt_55(_c, unimplemented!())
            }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!({
                let _c: CustomOrd =
                    ({ CStrMem_CustomOrd_Impl::operator_cmp_3(&cs1.as_pointer(), (cs2).clone()) });
                operator_lt_53(_c, unimplemented!())
            })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({
                let _c: CustomOrd =
                    ({ CStrMem_CustomOrd_Impl::operator_cmp_3(&cs1.as_pointer(), (cs2).clone()) });
                operator_gt_55(_c, unimplemented!())
            }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({
                let _c: CustomOrd =
                    ({ CStrMem_CustomOrd_Impl::operator_cmp_3(&cs1.as_pointer(), (cs2).clone()) });
                operator_lt_53(_c, unimplemented!())
            }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!({
                let _c: CustomOrd =
                    ({ CStrMem_CustomOrd_Impl::operator_cmp_3(&cs1.as_pointer(), (cs2).clone()) });
                operator_gt_55(_c, unimplemented!())
            })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({
                let _c: CustomOrd =
                    ({ CStrMem_CustomOrd_Impl::operator_cmp_3(&cs1.as_pointer(), (cs2).clone()) });
                operator_lt_53(_c, unimplemented!())
            }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!({
                let _c: CustomOrd =
                    ({ CStrMem_CustomOrd_Impl::operator_cmp_3(&cs1.as_pointer(), (cs2).clone()) });
                operator_gt_55(_c, unimplemented!())
            })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_relational_114(mut use_overloaded: ()) {
    let mut cs1: Ptr<()> = (*s1_10.with(Value::clone).borrow()).clone();
    let cs2: Value<CStrMem_CustomOrd_> = Rc::new(RefCell::new(CStrMem_CustomOrd_ {
        str: (*s2_11.with(Value::clone).borrow()).clone(),
    }));
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!({
                operator_lt_54(
                    unimplemented!(),
                    ({ CStrMem_CustomOrd_Impl::operator_cmp_3(&cs2.as_pointer(), (cs1).clone()) }),
                )
            })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({
                operator_gt_56(
                    unimplemented!(),
                    ({ CStrMem_CustomOrd_Impl::operator_cmp_3(&cs2.as_pointer(), (cs1).clone()) }),
                )
            }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!({
                operator_lt_54(
                    unimplemented!(),
                    ({ CStrMem_CustomOrd_Impl::operator_cmp_3(&cs2.as_pointer(), (cs1).clone()) }),
                )
            })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({
                operator_gt_56(
                    unimplemented!(),
                    ({ CStrMem_CustomOrd_Impl::operator_cmp_3(&cs2.as_pointer(), (cs1).clone()) }),
                )
            }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({
                operator_lt_54(
                    unimplemented!(),
                    ({ CStrMem_CustomOrd_Impl::operator_cmp_3(&cs2.as_pointer(), (cs1).clone()) }),
                )
            }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!({
                operator_gt_56(
                    unimplemented!(),
                    ({ CStrMem_CustomOrd_Impl::operator_cmp_3(&cs2.as_pointer(), (cs1).clone()) }),
                )
            })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({
                operator_lt_54(
                    unimplemented!(),
                    ({ CStrMem_CustomOrd_Impl::operator_cmp_3(&cs2.as_pointer(), (cs1).clone()) }),
                )
            }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!({
                operator_gt_56(
                    unimplemented!(),
                    ({ CStrMem_CustomOrd_Impl::operator_cmp_3(&cs2.as_pointer(), (cs1).clone()) }),
                )
            })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_relational_115(mut use_overloaded: ()) {
    let mut cs1: CStrFriend_CustomOrd_ = CStrFriend_CustomOrd_ {
        str: (*s1_10.with(Value::clone).borrow()).clone(),
    };
    let mut cs2: CStrFriend_CustomOrd_ = CStrFriend_CustomOrd_ {
        str: (*s2_11.with(Value::clone).borrow()).clone(),
    };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!({
                let _c: CustomOrd = ({
                    let _lhs: CStrFriend_CustomOrd_ = (cs1).clone();
                    operator_cmp_21(_lhs, (cs2).clone())
                });
                operator_lt_53(_c, unimplemented!())
            })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({
                let _c: CustomOrd = ({
                    let _lhs: CStrFriend_CustomOrd_ = (cs1).clone();
                    operator_cmp_21(_lhs, (cs2).clone())
                });
                operator_gt_55(_c, unimplemented!())
            }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!({
                let _c: CustomOrd = ({
                    let _lhs: CStrFriend_CustomOrd_ = (cs1).clone();
                    operator_cmp_21(_lhs, (cs2).clone())
                });
                operator_lt_53(_c, unimplemented!())
            })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({
                let _c: CustomOrd = ({
                    let _lhs: CStrFriend_CustomOrd_ = (cs1).clone();
                    operator_cmp_21(_lhs, (cs2).clone())
                });
                operator_gt_55(_c, unimplemented!())
            }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({
                let _c: CustomOrd = ({
                    let _lhs: CStrFriend_CustomOrd_ = (cs1).clone();
                    operator_cmp_21(_lhs, (cs2).clone())
                });
                operator_lt_53(_c, unimplemented!())
            }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!({
                let _c: CustomOrd = ({
                    let _lhs: CStrFriend_CustomOrd_ = (cs1).clone();
                    operator_cmp_21(_lhs, (cs2).clone())
                });
                operator_gt_55(_c, unimplemented!())
            })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({
                let _c: CustomOrd = ({
                    let _lhs: CStrFriend_CustomOrd_ = (cs1).clone();
                    operator_cmp_21(_lhs, (cs2).clone())
                });
                operator_lt_53(_c, unimplemented!())
            }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!({
                let _c: CustomOrd = ({
                    let _lhs: CStrFriend_CustomOrd_ = (cs1).clone();
                    operator_cmp_21(_lhs, (cs2).clone())
                });
                operator_gt_55(_c, unimplemented!())
            })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_relational_116(mut use_overloaded: ()) {
    let mut cs1: CStrFriend_CustomOrd_ = CStrFriend_CustomOrd_ {
        str: (*s1_10.with(Value::clone).borrow()).clone(),
    };
    let mut cs2: Ptr<()> = (*s2_11.with(Value::clone).borrow()).clone();
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!({
                let _c: CustomOrd = ({
                    let _lhs: CStrFriend_CustomOrd_ = (cs1).clone();
                    operator_cmp_22(_lhs, (cs2).clone())
                });
                operator_lt_53(_c, unimplemented!())
            })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({
                let _c: CustomOrd = ({
                    let _lhs: CStrFriend_CustomOrd_ = (cs1).clone();
                    operator_cmp_22(_lhs, (cs2).clone())
                });
                operator_gt_55(_c, unimplemented!())
            }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!({
                let _c: CustomOrd = ({
                    let _lhs: CStrFriend_CustomOrd_ = (cs1).clone();
                    operator_cmp_22(_lhs, (cs2).clone())
                });
                operator_lt_53(_c, unimplemented!())
            })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({
                let _c: CustomOrd = ({
                    let _lhs: CStrFriend_CustomOrd_ = (cs1).clone();
                    operator_cmp_22(_lhs, (cs2).clone())
                });
                operator_gt_55(_c, unimplemented!())
            }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({
                let _c: CustomOrd = ({
                    let _lhs: CStrFriend_CustomOrd_ = (cs1).clone();
                    operator_cmp_22(_lhs, (cs2).clone())
                });
                operator_lt_53(_c, unimplemented!())
            }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!({
                let _c: CustomOrd = ({
                    let _lhs: CStrFriend_CustomOrd_ = (cs1).clone();
                    operator_cmp_22(_lhs, (cs2).clone())
                });
                operator_gt_55(_c, unimplemented!())
            })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({
                let _c: CustomOrd = ({
                    let _lhs: CStrFriend_CustomOrd_ = (cs1).clone();
                    operator_cmp_22(_lhs, (cs2).clone())
                });
                operator_lt_53(_c, unimplemented!())
            }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!({
                let _c: CustomOrd = ({
                    let _lhs: CStrFriend_CustomOrd_ = (cs1).clone();
                    operator_cmp_22(_lhs, (cs2).clone())
                });
                operator_gt_55(_c, unimplemented!())
            })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_relational_117(mut use_overloaded: ()) {
    let mut cs1: Ptr<()> = (*s1_10.with(Value::clone).borrow()).clone();
    let mut cs2: CStrFriend_CustomOrd_ = CStrFriend_CustomOrd_ {
        str: (*s2_11.with(Value::clone).borrow()).clone(),
    };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!({
                operator_lt_54(
                    unimplemented!(),
                    ({
                        let _lhs: CStrFriend_CustomOrd_ = (cs2).clone();
                        operator_cmp_22(_lhs, (cs1).clone())
                    }),
                )
            })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({
                operator_gt_56(
                    unimplemented!(),
                    ({
                        let _lhs: CStrFriend_CustomOrd_ = (cs2).clone();
                        operator_cmp_22(_lhs, (cs1).clone())
                    }),
                )
            }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!({
                operator_lt_54(
                    unimplemented!(),
                    ({
                        let _lhs: CStrFriend_CustomOrd_ = (cs2).clone();
                        operator_cmp_22(_lhs, (cs1).clone())
                    }),
                )
            })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({
                operator_gt_56(
                    unimplemented!(),
                    ({
                        let _lhs: CStrFriend_CustomOrd_ = (cs2).clone();
                        operator_cmp_22(_lhs, (cs1).clone())
                    }),
                )
            }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({
                operator_lt_54(
                    unimplemented!(),
                    ({
                        let _lhs: CStrFriend_CustomOrd_ = (cs2).clone();
                        operator_cmp_22(_lhs, (cs1).clone())
                    }),
                )
            }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!({
                operator_gt_56(
                    unimplemented!(),
                    ({
                        let _lhs: CStrFriend_CustomOrd_ = (cs2).clone();
                        operator_cmp_22(_lhs, (cs1).clone())
                    }),
                )
            })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({
                operator_lt_54(
                    unimplemented!(),
                    ({
                        let _lhs: CStrFriend_CustomOrd_ = (cs2).clone();
                        operator_cmp_22(_lhs, (cs1).clone())
                    }),
                )
            }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!({
                operator_gt_56(
                    unimplemented!(),
                    ({
                        let _lhs: CStrFriend_CustomOrd_ = (cs2).clone();
                        operator_cmp_22(_lhs, (cs1).clone())
                    }),
                )
            })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_relational_118(mut use_overloaded: ()) {
    let mut cs1: CStrFree_CustomOrd_ = CStrFree_CustomOrd_ {
        str: (*s1_10.with(Value::clone).borrow()).clone(),
    };
    let mut cs2: CStrFree_CustomOrd_ = CStrFree_CustomOrd_ {
        str: (*s2_11.with(Value::clone).borrow()).clone(),
    };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!({
                let _c: CustomOrd = ({
                    let _lhs: CStrFree_CustomOrd_ = (cs1).clone();
                    operator_cmp_31(_lhs, (cs2).clone())
                });
                operator_lt_53(_c, unimplemented!())
            })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({
                let _c: CustomOrd = ({
                    let _lhs: CStrFree_CustomOrd_ = (cs1).clone();
                    operator_cmp_31(_lhs, (cs2).clone())
                });
                operator_gt_55(_c, unimplemented!())
            }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!({
                let _c: CustomOrd = ({
                    let _lhs: CStrFree_CustomOrd_ = (cs1).clone();
                    operator_cmp_31(_lhs, (cs2).clone())
                });
                operator_lt_53(_c, unimplemented!())
            })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({
                let _c: CustomOrd = ({
                    let _lhs: CStrFree_CustomOrd_ = (cs1).clone();
                    operator_cmp_31(_lhs, (cs2).clone())
                });
                operator_gt_55(_c, unimplemented!())
            }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({
                let _c: CustomOrd = ({
                    let _lhs: CStrFree_CustomOrd_ = (cs1).clone();
                    operator_cmp_31(_lhs, (cs2).clone())
                });
                operator_lt_53(_c, unimplemented!())
            }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!({
                let _c: CustomOrd = ({
                    let _lhs: CStrFree_CustomOrd_ = (cs1).clone();
                    operator_cmp_31(_lhs, (cs2).clone())
                });
                operator_gt_55(_c, unimplemented!())
            })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({
                let _c: CustomOrd = ({
                    let _lhs: CStrFree_CustomOrd_ = (cs1).clone();
                    operator_cmp_31(_lhs, (cs2).clone())
                });
                operator_lt_53(_c, unimplemented!())
            }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!({
                let _c: CustomOrd = ({
                    let _lhs: CStrFree_CustomOrd_ = (cs1).clone();
                    operator_cmp_31(_lhs, (cs2).clone())
                });
                operator_gt_55(_c, unimplemented!())
            })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_relational_119(mut use_overloaded: ()) {
    let mut cs1: CStrFree_CustomOrd_ = CStrFree_CustomOrd_ {
        str: (*s1_10.with(Value::clone).borrow()).clone(),
    };
    let mut cs2: Ptr<()> = (*s2_11.with(Value::clone).borrow()).clone();
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!({
                let _c: CustomOrd = ({
                    let _lhs: CStrFree_CustomOrd_ = (cs1).clone();
                    operator_cmp_38(_lhs, (cs2).clone())
                });
                operator_lt_53(_c, unimplemented!())
            })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({
                let _c: CustomOrd = ({
                    let _lhs: CStrFree_CustomOrd_ = (cs1).clone();
                    operator_cmp_38(_lhs, (cs2).clone())
                });
                operator_gt_55(_c, unimplemented!())
            }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!({
                let _c: CustomOrd = ({
                    let _lhs: CStrFree_CustomOrd_ = (cs1).clone();
                    operator_cmp_38(_lhs, (cs2).clone())
                });
                operator_lt_53(_c, unimplemented!())
            })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({
                let _c: CustomOrd = ({
                    let _lhs: CStrFree_CustomOrd_ = (cs1).clone();
                    operator_cmp_38(_lhs, (cs2).clone())
                });
                operator_gt_55(_c, unimplemented!())
            }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({
                let _c: CustomOrd = ({
                    let _lhs: CStrFree_CustomOrd_ = (cs1).clone();
                    operator_cmp_38(_lhs, (cs2).clone())
                });
                operator_lt_53(_c, unimplemented!())
            }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!({
                let _c: CustomOrd = ({
                    let _lhs: CStrFree_CustomOrd_ = (cs1).clone();
                    operator_cmp_38(_lhs, (cs2).clone())
                });
                operator_gt_55(_c, unimplemented!())
            })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({
                let _c: CustomOrd = ({
                    let _lhs: CStrFree_CustomOrd_ = (cs1).clone();
                    operator_cmp_38(_lhs, (cs2).clone())
                });
                operator_lt_53(_c, unimplemented!())
            }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!({
                let _c: CustomOrd = ({
                    let _lhs: CStrFree_CustomOrd_ = (cs1).clone();
                    operator_cmp_38(_lhs, (cs2).clone())
                });
                operator_gt_55(_c, unimplemented!())
            })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_relational_120(mut use_overloaded: ()) {
    let mut cs1: Ptr<()> = (*s1_10.with(Value::clone).borrow()).clone();
    let mut cs2: CStrFree_CustomOrd_ = CStrFree_CustomOrd_ {
        str: (*s2_11.with(Value::clone).borrow()).clone(),
    };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!({
                operator_lt_54(
                    unimplemented!(),
                    ({
                        let _lhs: CStrFree_CustomOrd_ = (cs2).clone();
                        operator_cmp_38(_lhs, (cs1).clone())
                    }),
                )
            })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({
                operator_gt_56(
                    unimplemented!(),
                    ({
                        let _lhs: CStrFree_CustomOrd_ = (cs2).clone();
                        operator_cmp_38(_lhs, (cs1).clone())
                    }),
                )
            }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!({
                operator_lt_54(
                    unimplemented!(),
                    ({
                        let _lhs: CStrFree_CustomOrd_ = (cs2).clone();
                        operator_cmp_38(_lhs, (cs1).clone())
                    }),
                )
            })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({
                operator_gt_56(
                    unimplemented!(),
                    ({
                        let _lhs: CStrFree_CustomOrd_ = (cs2).clone();
                        operator_cmp_38(_lhs, (cs1).clone())
                    }),
                )
            }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({
                operator_lt_54(
                    unimplemented!(),
                    ({
                        let _lhs: CStrFree_CustomOrd_ = (cs2).clone();
                        operator_cmp_38(_lhs, (cs1).clone())
                    }),
                )
            }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!({
                operator_gt_56(
                    unimplemented!(),
                    ({
                        let _lhs: CStrFree_CustomOrd_ = (cs2).clone();
                        operator_cmp_38(_lhs, (cs1).clone())
                    }),
                )
            })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !({
                operator_lt_54(
                    unimplemented!(),
                    ({
                        let _lhs: CStrFree_CustomOrd_ = (cs2).clone();
                        operator_cmp_38(_lhs, (cs1).clone())
                    }),
                )
            }) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!({
                operator_gt_56(
                    unimplemented!(),
                    ({
                        let _lhs: CStrFree_CustomOrd_ = (cs2).clone();
                        operator_cmp_38(_lhs, (cs1).clone())
                    }),
                )
            })) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(!(unimplemented!())) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_relational_type_121(mut use_overloaded: ()) {
    ({ test_relational_96(use_overloaded) });
    ({ test_relational_97(use_overloaded) });
    ({ test_relational_98(use_overloaded) });
}
pub fn test_relational_type_122(mut use_overloaded: ()) {
    ({ test_relational_65(use_overloaded) });
    ({ test_relational_66(use_overloaded) });
    ({ test_relational_67(use_overloaded) });
}
pub fn test_relational_type_123(mut use_overloaded: ()) {
    ({ test_relational_68(use_overloaded) });
    ({ test_relational_69(use_overloaded) });
    ({ test_relational_70(use_overloaded) });
}
pub fn test_relational_type_124(mut use_overloaded: ()) {
    ({ test_relational_71(use_overloaded) });
    ({ test_relational_72(use_overloaded) });
    ({ test_relational_73(use_overloaded) });
}
pub fn test_relational_type_125(mut use_overloaded: ()) {
    ({ test_relational_76(use_overloaded) });
    ({ test_relational_77(use_overloaded) });
    ({ test_relational_78(use_overloaded) });
}
pub fn test_relational_type_126(mut use_overloaded: ()) {
    ({ test_relational_79(use_overloaded) });
    ({ test_relational_80(use_overloaded) });
    ({ test_relational_81(use_overloaded) });
}
pub fn test_relational_type_127(mut use_overloaded: ()) {
    ({ test_relational_82(use_overloaded) });
    ({ test_relational_83(use_overloaded) });
    ({ test_relational_84(use_overloaded) });
}
pub fn test_relational_type_128(mut use_overloaded: ()) {
    ({ test_relational_87(use_overloaded) });
    ({ test_relational_88(use_overloaded) });
    ({ test_relational_89(use_overloaded) });
}
pub fn test_relational_type_129(mut use_overloaded: ()) {
    ({ test_relational_90(use_overloaded) });
    ({ test_relational_91(use_overloaded) });
    ({ test_relational_92(use_overloaded) });
}
pub fn test_relational_type_130(mut use_overloaded: ()) {
    ({ test_relational_93(use_overloaded) });
    ({ test_relational_94(use_overloaded) });
    ({ test_relational_95(use_overloaded) });
}
pub fn test_relational_type_131(mut use_overloaded: ()) {
    ({ test_relational_101(use_overloaded) });
    ({ test_relational_102(use_overloaded) });
    ({ test_relational_103(use_overloaded) });
}
pub fn test_relational_type_132(mut use_overloaded: ()) {
    ({ test_relational_104(use_overloaded) });
    ({ test_relational_105(use_overloaded) });
    ({ test_relational_106(use_overloaded) });
}
pub fn test_relational_type_133(mut use_overloaded: ()) {
    ({ test_relational_107(use_overloaded) });
    ({ test_relational_108(use_overloaded) });
    ({ test_relational_109(use_overloaded) });
}
pub fn test_relational_type_134(mut use_overloaded: ()) {
    ({ test_relational_112(use_overloaded) });
    ({ test_relational_113(use_overloaded) });
    ({ test_relational_114(use_overloaded) });
}
pub fn test_relational_type_135(mut use_overloaded: ()) {
    ({ test_relational_115(use_overloaded) });
    ({ test_relational_116(use_overloaded) });
    ({ test_relational_117(use_overloaded) });
}
pub fn test_relational_type_136(mut use_overloaded: ()) {
    ({ test_relational_118(use_overloaded) });
    ({ test_relational_119(use_overloaded) });
    ({ test_relational_120(use_overloaded) });
}
pub fn test_relational_return_137() {
    ({ test_relational_type_131(true) });
    ({ test_relational_type_132(true) });
    ({ test_relational_type_133(true) });
    ({ test_relational_99(true) });
    ({ test_relational_100(true) });
}
pub fn test_relational_return_138() {
    ({ test_relational_type_134(true) });
    ({ test_relational_type_135(true) });
    ({ test_relational_type_136(true) });
    ({ test_relational_110(true) });
    ({ test_relational_111(true) });
}
pub fn test_relational_return_139() {
    ({ test_relational_type_122(true) });
    ({ test_relational_type_123(true) });
    ({ test_relational_type_124(true) });
    ({ test_relational_63(true) });
    ({ test_relational_64(true) });
}
pub fn test_relational_return_140() {
    ({ test_relational_type_125(true) });
    ({ test_relational_type_126(true) });
    ({ test_relational_type_127(true) });
    ({ test_relational_74(true) });
    ({ test_relational_75(true) });
}
pub fn test_relational_return_141() {
    ({ test_relational_type_128(true) });
    ({ test_relational_type_129(true) });
    ({ test_relational_type_130(true) });
    ({ test_relational_85(true) });
    ({ test_relational_86(true) });
}
pub fn test_spaceship_142(mut use_overloaded: ()) {
    let mut cs1: CStrMixed_ReturnOrd_std_strong_ordering__ =
        CStrMixed_ReturnOrd_std_strong_ordering__ {
            str: (*s1_10.with(Value::clone).borrow()).clone(),
        };
    let mut cs2: CStrFree_ReturnOrd_std_strong_ordering__ =
        CStrFree_ReturnOrd_std_strong_ordering__ {
            str: (*s2_11.with(Value::clone).borrow()).clone(),
        };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_spaceship_143(mut use_overloaded: ()) {
    let mut cs1: CStrFree_ReturnOrd_std_strong_ordering__ =
        CStrFree_ReturnOrd_std_strong_ordering__ {
            str: (*s1_10.with(Value::clone).borrow()).clone(),
        };
    let mut cs2: CStrMixed_ReturnOrd_std_strong_ordering__ =
        CStrMixed_ReturnOrd_std_strong_ordering__ {
            str: (*s2_11.with(Value::clone).borrow()).clone(),
        };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_spaceship_144(mut use_overloaded: ()) {
    let cs1: Value<CStrMem_ReturnOrd_std_strong_ordering__> =
        Rc::new(RefCell::new(CStrMem_ReturnOrd_std_strong_ordering__ {
            str: (*s1_10.with(Value::clone).borrow()).clone(),
        }));
    let cs2: Value<CStrMem_ReturnOrd_std_strong_ordering__> =
        Rc::new(RefCell::new(CStrMem_ReturnOrd_std_strong_ordering__ {
            str: (*s2_11.with(Value::clone).borrow()).clone(),
        }));
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_spaceship_145(mut use_overloaded: ()) {
    let cs1: Value<CStrMem_ReturnOrd_std_strong_ordering__> =
        Rc::new(RefCell::new(CStrMem_ReturnOrd_std_strong_ordering__ {
            str: (*s1_10.with(Value::clone).borrow()).clone(),
        }));
    let mut cs2: Ptr<()> = (*s2_11.with(Value::clone).borrow()).clone();
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_spaceship_146(mut use_overloaded: ()) {
    let mut cs1: Ptr<()> = (*s1_10.with(Value::clone).borrow()).clone();
    let cs2: Value<CStrMem_ReturnOrd_std_strong_ordering__> =
        Rc::new(RefCell::new(CStrMem_ReturnOrd_std_strong_ordering__ {
            str: (*s2_11.with(Value::clone).borrow()).clone(),
        }));
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_spaceship_147(mut use_overloaded: ()) {
    let mut cs1: CStrFriend_ReturnOrd_std_strong_ordering__ =
        CStrFriend_ReturnOrd_std_strong_ordering__ {
            str: (*s1_10.with(Value::clone).borrow()).clone(),
        };
    let mut cs2: CStrFriend_ReturnOrd_std_strong_ordering__ =
        CStrFriend_ReturnOrd_std_strong_ordering__ {
            str: (*s2_11.with(Value::clone).borrow()).clone(),
        };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_spaceship_148(mut use_overloaded: ()) {
    let mut cs1: CStrFriend_ReturnOrd_std_strong_ordering__ =
        CStrFriend_ReturnOrd_std_strong_ordering__ {
            str: (*s1_10.with(Value::clone).borrow()).clone(),
        };
    let mut cs2: Ptr<()> = (*s2_11.with(Value::clone).borrow()).clone();
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_spaceship_149(mut use_overloaded: ()) {
    let mut cs1: Ptr<()> = (*s1_10.with(Value::clone).borrow()).clone();
    let mut cs2: CStrFriend_ReturnOrd_std_strong_ordering__ =
        CStrFriend_ReturnOrd_std_strong_ordering__ {
            str: (*s2_11.with(Value::clone).borrow()).clone(),
        };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_spaceship_150(mut use_overloaded: ()) {
    let mut cs1: CStrFree_ReturnOrd_std_strong_ordering__ =
        CStrFree_ReturnOrd_std_strong_ordering__ {
            str: (*s1_10.with(Value::clone).borrow()).clone(),
        };
    let mut cs2: CStrFree_ReturnOrd_std_strong_ordering__ =
        CStrFree_ReturnOrd_std_strong_ordering__ {
            str: (*s2_11.with(Value::clone).borrow()).clone(),
        };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_spaceship_151(mut use_overloaded: ()) {
    let mut cs1: CStrFree_ReturnOrd_std_strong_ordering__ =
        CStrFree_ReturnOrd_std_strong_ordering__ {
            str: (*s1_10.with(Value::clone).borrow()).clone(),
        };
    let mut cs2: Ptr<()> = (*s2_11.with(Value::clone).borrow()).clone();
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_spaceship_152(mut use_overloaded: ()) {
    let mut cs1: Ptr<()> = (*s1_10.with(Value::clone).borrow()).clone();
    let mut cs2: CStrFree_ReturnOrd_std_strong_ordering__ =
        CStrFree_ReturnOrd_std_strong_ordering__ {
            str: (*s2_11.with(Value::clone).borrow()).clone(),
        };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_spaceship_153(mut use_overloaded: ()) {
    let mut cs1: CStrMixed_ReturnOrd_std_weak_ordering__ =
        CStrMixed_ReturnOrd_std_weak_ordering__ {
            str: (*s1_10.with(Value::clone).borrow()).clone(),
        };
    let mut cs2: CStrFree_ReturnOrd_std_weak_ordering__ = CStrFree_ReturnOrd_std_weak_ordering__ {
        str: (*s2_11.with(Value::clone).borrow()).clone(),
    };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_spaceship_154(mut use_overloaded: ()) {
    let mut cs1: CStrFree_ReturnOrd_std_weak_ordering__ = CStrFree_ReturnOrd_std_weak_ordering__ {
        str: (*s1_10.with(Value::clone).borrow()).clone(),
    };
    let mut cs2: CStrMixed_ReturnOrd_std_weak_ordering__ =
        CStrMixed_ReturnOrd_std_weak_ordering__ {
            str: (*s2_11.with(Value::clone).borrow()).clone(),
        };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_spaceship_155(mut use_overloaded: ()) {
    let cs1: Value<CStrMem_ReturnOrd_std_weak_ordering__> =
        Rc::new(RefCell::new(CStrMem_ReturnOrd_std_weak_ordering__ {
            str: (*s1_10.with(Value::clone).borrow()).clone(),
        }));
    let cs2: Value<CStrMem_ReturnOrd_std_weak_ordering__> =
        Rc::new(RefCell::new(CStrMem_ReturnOrd_std_weak_ordering__ {
            str: (*s2_11.with(Value::clone).borrow()).clone(),
        }));
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_spaceship_156(mut use_overloaded: ()) {
    let cs1: Value<CStrMem_ReturnOrd_std_weak_ordering__> =
        Rc::new(RefCell::new(CStrMem_ReturnOrd_std_weak_ordering__ {
            str: (*s1_10.with(Value::clone).borrow()).clone(),
        }));
    let mut cs2: Ptr<()> = (*s2_11.with(Value::clone).borrow()).clone();
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_spaceship_157(mut use_overloaded: ()) {
    let mut cs1: Ptr<()> = (*s1_10.with(Value::clone).borrow()).clone();
    let cs2: Value<CStrMem_ReturnOrd_std_weak_ordering__> =
        Rc::new(RefCell::new(CStrMem_ReturnOrd_std_weak_ordering__ {
            str: (*s2_11.with(Value::clone).borrow()).clone(),
        }));
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_spaceship_158(mut use_overloaded: ()) {
    let mut cs1: CStrFriend_ReturnOrd_std_weak_ordering__ =
        CStrFriend_ReturnOrd_std_weak_ordering__ {
            str: (*s1_10.with(Value::clone).borrow()).clone(),
        };
    let mut cs2: CStrFriend_ReturnOrd_std_weak_ordering__ =
        CStrFriend_ReturnOrd_std_weak_ordering__ {
            str: (*s2_11.with(Value::clone).borrow()).clone(),
        };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_spaceship_159(mut use_overloaded: ()) {
    let mut cs1: CStrFriend_ReturnOrd_std_weak_ordering__ =
        CStrFriend_ReturnOrd_std_weak_ordering__ {
            str: (*s1_10.with(Value::clone).borrow()).clone(),
        };
    let mut cs2: Ptr<()> = (*s2_11.with(Value::clone).borrow()).clone();
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_spaceship_160(mut use_overloaded: ()) {
    let mut cs1: Ptr<()> = (*s1_10.with(Value::clone).borrow()).clone();
    let mut cs2: CStrFriend_ReturnOrd_std_weak_ordering__ =
        CStrFriend_ReturnOrd_std_weak_ordering__ {
            str: (*s2_11.with(Value::clone).borrow()).clone(),
        };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_spaceship_161(mut use_overloaded: ()) {
    let mut cs1: CStrFree_ReturnOrd_std_weak_ordering__ = CStrFree_ReturnOrd_std_weak_ordering__ {
        str: (*s1_10.with(Value::clone).borrow()).clone(),
    };
    let mut cs2: CStrFree_ReturnOrd_std_weak_ordering__ = CStrFree_ReturnOrd_std_weak_ordering__ {
        str: (*s2_11.with(Value::clone).borrow()).clone(),
    };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_spaceship_162(mut use_overloaded: ()) {
    let mut cs1: CStrFree_ReturnOrd_std_weak_ordering__ = CStrFree_ReturnOrd_std_weak_ordering__ {
        str: (*s1_10.with(Value::clone).borrow()).clone(),
    };
    let mut cs2: Ptr<()> = (*s2_11.with(Value::clone).borrow()).clone();
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_spaceship_163(mut use_overloaded: ()) {
    let mut cs1: Ptr<()> = (*s1_10.with(Value::clone).borrow()).clone();
    let mut cs2: CStrFree_ReturnOrd_std_weak_ordering__ = CStrFree_ReturnOrd_std_weak_ordering__ {
        str: (*s2_11.with(Value::clone).borrow()).clone(),
    };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_spaceship_164(mut use_overloaded: ()) {
    let mut cs1: CStrMixed_ReturnOrd_std_partial_ordering__ =
        CStrMixed_ReturnOrd_std_partial_ordering__ {
            str: (*s1_10.with(Value::clone).borrow()).clone(),
        };
    let mut cs2: CStrFree_ReturnOrd_std_partial_ordering__ =
        CStrFree_ReturnOrd_std_partial_ordering__ {
            str: (*s2_11.with(Value::clone).borrow()).clone(),
        };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_spaceship_165(mut use_overloaded: ()) {
    let mut cs1: CStrFree_ReturnOrd_std_partial_ordering__ =
        CStrFree_ReturnOrd_std_partial_ordering__ {
            str: (*s1_10.with(Value::clone).borrow()).clone(),
        };
    let mut cs2: CStrMixed_ReturnOrd_std_partial_ordering__ =
        CStrMixed_ReturnOrd_std_partial_ordering__ {
            str: (*s2_11.with(Value::clone).borrow()).clone(),
        };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_spaceship_166(mut use_overloaded: ()) {
    let cs1: Value<CStrMem_ReturnOrd_std_partial_ordering__> =
        Rc::new(RefCell::new(CStrMem_ReturnOrd_std_partial_ordering__ {
            str: (*s1_10.with(Value::clone).borrow()).clone(),
        }));
    let cs2: Value<CStrMem_ReturnOrd_std_partial_ordering__> =
        Rc::new(RefCell::new(CStrMem_ReturnOrd_std_partial_ordering__ {
            str: (*s2_11.with(Value::clone).borrow()).clone(),
        }));
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_spaceship_167(mut use_overloaded: ()) {
    let cs1: Value<CStrMem_ReturnOrd_std_partial_ordering__> =
        Rc::new(RefCell::new(CStrMem_ReturnOrd_std_partial_ordering__ {
            str: (*s1_10.with(Value::clone).borrow()).clone(),
        }));
    let mut cs2: Ptr<()> = (*s2_11.with(Value::clone).borrow()).clone();
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_spaceship_168(mut use_overloaded: ()) {
    let mut cs1: Ptr<()> = (*s1_10.with(Value::clone).borrow()).clone();
    let cs2: Value<CStrMem_ReturnOrd_std_partial_ordering__> =
        Rc::new(RefCell::new(CStrMem_ReturnOrd_std_partial_ordering__ {
            str: (*s2_11.with(Value::clone).borrow()).clone(),
        }));
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_spaceship_169(mut use_overloaded: ()) {
    let mut cs1: CStrFriend_ReturnOrd_std_partial_ordering__ =
        CStrFriend_ReturnOrd_std_partial_ordering__ {
            str: (*s1_10.with(Value::clone).borrow()).clone(),
        };
    let mut cs2: CStrFriend_ReturnOrd_std_partial_ordering__ =
        CStrFriend_ReturnOrd_std_partial_ordering__ {
            str: (*s2_11.with(Value::clone).borrow()).clone(),
        };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_spaceship_170(mut use_overloaded: ()) {
    let mut cs1: CStrFriend_ReturnOrd_std_partial_ordering__ =
        CStrFriend_ReturnOrd_std_partial_ordering__ {
            str: (*s1_10.with(Value::clone).borrow()).clone(),
        };
    let mut cs2: Ptr<()> = (*s2_11.with(Value::clone).borrow()).clone();
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_spaceship_171(mut use_overloaded: ()) {
    let mut cs1: Ptr<()> = (*s1_10.with(Value::clone).borrow()).clone();
    let mut cs2: CStrFriend_ReturnOrd_std_partial_ordering__ =
        CStrFriend_ReturnOrd_std_partial_ordering__ {
            str: (*s2_11.with(Value::clone).borrow()).clone(),
        };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_spaceship_172(mut use_overloaded: ()) {
    let mut cs1: CStrFree_ReturnOrd_std_partial_ordering__ =
        CStrFree_ReturnOrd_std_partial_ordering__ {
            str: (*s1_10.with(Value::clone).borrow()).clone(),
        };
    let mut cs2: CStrFree_ReturnOrd_std_partial_ordering__ =
        CStrFree_ReturnOrd_std_partial_ordering__ {
            str: (*s2_11.with(Value::clone).borrow()).clone(),
        };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_spaceship_173(mut use_overloaded: ()) {
    let mut cs1: CStrFree_ReturnOrd_std_partial_ordering__ =
        CStrFree_ReturnOrd_std_partial_ordering__ {
            str: (*s1_10.with(Value::clone).borrow()).clone(),
        };
    let mut cs2: Ptr<()> = (*s2_11.with(Value::clone).borrow()).clone();
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_spaceship_174(mut use_overloaded: ()) {
    let mut cs1: Ptr<()> = (*s1_10.with(Value::clone).borrow()).clone();
    let mut cs2: CStrFree_ReturnOrd_std_partial_ordering__ =
        CStrFree_ReturnOrd_std_partial_ordering__ {
            str: (*s2_11.with(Value::clone).borrow()).clone(),
        };
    if use_overloaded {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    }
}
pub fn test_spaceship_type_175(mut use_overloaded: ()) {
    ({ test_spaceship_144(use_overloaded) });
    ({ test_spaceship_145(use_overloaded) });
    ({ test_spaceship_146(use_overloaded) });
}
pub fn test_spaceship_type_176(mut use_overloaded: ()) {
    ({ test_spaceship_147(use_overloaded) });
    ({ test_spaceship_148(use_overloaded) });
    ({ test_spaceship_149(use_overloaded) });
}
pub fn test_spaceship_type_177(mut use_overloaded: ()) {
    ({ test_spaceship_150(use_overloaded) });
    ({ test_spaceship_151(use_overloaded) });
    ({ test_spaceship_152(use_overloaded) });
}
pub fn test_spaceship_type_178(mut use_overloaded: ()) {
    ({ test_spaceship_155(use_overloaded) });
    ({ test_spaceship_156(use_overloaded) });
    ({ test_spaceship_157(use_overloaded) });
}
pub fn test_spaceship_type_179(mut use_overloaded: ()) {
    ({ test_spaceship_158(use_overloaded) });
    ({ test_spaceship_159(use_overloaded) });
    ({ test_spaceship_160(use_overloaded) });
}
pub fn test_spaceship_type_180(mut use_overloaded: ()) {
    ({ test_spaceship_161(use_overloaded) });
    ({ test_spaceship_162(use_overloaded) });
    ({ test_spaceship_163(use_overloaded) });
}
pub fn test_spaceship_type_181(mut use_overloaded: ()) {
    ({ test_spaceship_166(use_overloaded) });
    ({ test_spaceship_167(use_overloaded) });
    ({ test_spaceship_168(use_overloaded) });
}
pub fn test_spaceship_type_182(mut use_overloaded: ()) {
    ({ test_spaceship_169(use_overloaded) });
    ({ test_spaceship_170(use_overloaded) });
    ({ test_spaceship_171(use_overloaded) });
}
pub fn test_spaceship_type_183(mut use_overloaded: ()) {
    ({ test_spaceship_172(use_overloaded) });
    ({ test_spaceship_173(use_overloaded) });
    ({ test_spaceship_174(use_overloaded) });
}
pub fn test_std_ordering_184() {
    ({ test_relational_return_139() });
    ({ test_spaceship_type_175(true) });
    ({ test_spaceship_type_176(true) });
    ({ test_spaceship_type_177(true) });
    ({ test_spaceship_142(true) });
    ({ test_spaceship_143(true) });
}
pub fn test_std_ordering_185() {
    ({ test_relational_return_140() });
    ({ test_spaceship_type_178(true) });
    ({ test_spaceship_type_179(true) });
    ({ test_spaceship_type_180(true) });
    ({ test_spaceship_153(true) });
    ({ test_spaceship_154(true) });
}
pub fn test_std_ordering_186() {
    ({ test_relational_return_141() });
    ({ test_spaceship_type_181(true) });
    ({ test_spaceship_type_182(true) });
    ({ test_spaceship_type_183(true) });
    ({ test_spaceship_164(true) });
    ({ test_spaceship_165(true) });
}
pub fn test_no_relational_187() {
    let mut c1: CStrMixed_ReturnVoid_ = <CStrMixed_ReturnVoid_>::default();
    let mut c2: CStrFree_ReturnVoid_ = <CStrFree_ReturnVoid_>::default();
    const _: () = assert!(true, "!requires {{ c1 < c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 > c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 <= c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 >= c2; }}");;
}
pub fn test_no_relational_188() {
    let mut c1: CStrFree_ReturnVoid_ = <CStrFree_ReturnVoid_>::default();
    let mut c2: CStrMixed_ReturnVoid_ = <CStrMixed_ReturnVoid_>::default();
    const _: () = assert!(true, "!requires {{ c1 < c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 > c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 <= c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 >= c2; }}");;
}
pub fn test_no_relational_189() {
    let mut c1: CStrMem_ReturnVoid_ = <CStrMem_ReturnVoid_>::default();
    let mut c2: CStrMem_ReturnVoid_ = <CStrMem_ReturnVoid_>::default();
    const _: () = assert!(true, "!requires {{ c1 < c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 > c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 <= c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 >= c2; }}");;
}
pub fn test_no_relational_190() {
    let mut c1: CStrMem_ReturnVoid_ = <CStrMem_ReturnVoid_>::default();
    let mut c2: Ptr<()> = Ptr::<()>::null();
    const _: () = assert!(true, "!requires {{ c1 < c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 > c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 <= c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 >= c2; }}");;
}
pub fn test_no_relational_191() {
    let mut c1: Ptr<()> = Ptr::<()>::null();
    let mut c2: CStrMem_ReturnVoid_ = <CStrMem_ReturnVoid_>::default();
    const _: () = assert!(true, "!requires {{ c1 < c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 > c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 <= c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 >= c2; }}");;
}
pub fn test_no_relational_192() {
    let mut c1: CStrFriend_ReturnVoid_ = <CStrFriend_ReturnVoid_>::default();
    let mut c2: CStrFriend_ReturnVoid_ = <CStrFriend_ReturnVoid_>::default();
    const _: () = assert!(true, "!requires {{ c1 < c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 > c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 <= c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 >= c2; }}");;
}
pub fn test_no_relational_193() {
    let mut c1: CStrFriend_ReturnVoid_ = <CStrFriend_ReturnVoid_>::default();
    let mut c2: Ptr<()> = Ptr::<()>::null();
    const _: () = assert!(true, "!requires {{ c1 < c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 > c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 <= c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 >= c2; }}");;
}
pub fn test_no_relational_194() {
    let mut c1: Ptr<()> = Ptr::<()>::null();
    let mut c2: CStrFriend_ReturnVoid_ = <CStrFriend_ReturnVoid_>::default();
    const _: () = assert!(true, "!requires {{ c1 < c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 > c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 <= c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 >= c2; }}");;
}
pub fn test_no_relational_195() {
    let mut c1: CStrFree_ReturnVoid_ = <CStrFree_ReturnVoid_>::default();
    let mut c2: CStrFree_ReturnVoid_ = <CStrFree_ReturnVoid_>::default();
    const _: () = assert!(true, "!requires {{ c1 < c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 > c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 <= c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 >= c2; }}");;
}
pub fn test_no_relational_196() {
    let mut c1: CStrFree_ReturnVoid_ = <CStrFree_ReturnVoid_>::default();
    let mut c2: Ptr<()> = Ptr::<()>::null();
    const _: () = assert!(true, "!requires {{ c1 < c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 > c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 <= c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 >= c2; }}");;
}
pub fn test_no_relational_197() {
    let mut c1: Ptr<()> = Ptr::<()>::null();
    let mut c2: CStrFree_ReturnVoid_ = <CStrFree_ReturnVoid_>::default();
    const _: () = assert!(true, "!requires {{ c1 < c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 > c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 <= c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 >= c2; }}");;
}
pub fn test_no_relational_198() {
    let mut c1: CStrMixed_NoOperators_ = <CStrMixed_NoOperators_>::default();
    let mut c2: CStrFree_NoOperators_ = <CStrFree_NoOperators_>::default();
    const _: () = assert!(true, "!requires {{ c1 < c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 > c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 <= c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 >= c2; }}");;
}
pub fn test_no_relational_199() {
    let mut c1: CStrFree_NoOperators_ = <CStrFree_NoOperators_>::default();
    let mut c2: CStrMixed_NoOperators_ = <CStrMixed_NoOperators_>::default();
    const _: () = assert!(true, "!requires {{ c1 < c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 > c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 <= c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 >= c2; }}");;
}
pub fn test_no_relational_200() {
    let mut c1: CStrMem_NoOperators_ = <CStrMem_NoOperators_>::default();
    let mut c2: CStrMem_NoOperators_ = <CStrMem_NoOperators_>::default();
    const _: () = assert!(true, "!requires {{ c1 < c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 > c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 <= c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 >= c2; }}");;
}
pub fn test_no_relational_201() {
    let mut c1: CStrMem_NoOperators_ = <CStrMem_NoOperators_>::default();
    let mut c2: Ptr<()> = Ptr::<()>::null();
    const _: () = assert!(true, "!requires {{ c1 < c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 > c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 <= c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 >= c2; }}");;
}
pub fn test_no_relational_202() {
    let mut c1: Ptr<()> = Ptr::<()>::null();
    let mut c2: CStrMem_NoOperators_ = <CStrMem_NoOperators_>::default();
    const _: () = assert!(true, "!requires {{ c1 < c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 > c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 <= c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 >= c2; }}");;
}
pub fn test_no_relational_203() {
    let mut c1: CStrFriend_NoOperators_ = <CStrFriend_NoOperators_>::default();
    let mut c2: CStrFriend_NoOperators_ = <CStrFriend_NoOperators_>::default();
    const _: () = assert!(true, "!requires {{ c1 < c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 > c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 <= c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 >= c2; }}");;
}
pub fn test_no_relational_204() {
    let mut c1: CStrFriend_NoOperators_ = <CStrFriend_NoOperators_>::default();
    let mut c2: Ptr<()> = Ptr::<()>::null();
    const _: () = assert!(true, "!requires {{ c1 < c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 > c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 <= c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 >= c2; }}");;
}
pub fn test_no_relational_205() {
    let mut c1: Ptr<()> = Ptr::<()>::null();
    let mut c2: CStrFriend_NoOperators_ = <CStrFriend_NoOperators_>::default();
    const _: () = assert!(true, "!requires {{ c1 < c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 > c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 <= c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 >= c2; }}");;
}
pub fn test_no_relational_206() {
    let mut c1: CStrFree_NoOperators_ = <CStrFree_NoOperators_>::default();
    let mut c2: CStrFree_NoOperators_ = <CStrFree_NoOperators_>::default();
    const _: () = assert!(true, "!requires {{ c1 < c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 > c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 <= c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 >= c2; }}");;
}
pub fn test_no_relational_207() {
    let mut c1: CStrFree_NoOperators_ = <CStrFree_NoOperators_>::default();
    let mut c2: Ptr<()> = Ptr::<()>::null();
    const _: () = assert!(true, "!requires {{ c1 < c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 > c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 <= c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 >= c2; }}");;
}
pub fn test_no_relational_208() {
    let mut c1: Ptr<()> = Ptr::<()>::null();
    let mut c2: CStrFree_NoOperators_ = <CStrFree_NoOperators_>::default();
    const _: () = assert!(true, "!requires {{ c1 < c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 > c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 <= c2; }}");;
    const _: () = assert!(true, "!requires {{ c1 >= c2; }}");;
}
pub fn test_no_relational_type_209() {
    ({ test_no_relational_189() });
    ({ test_no_relational_190() });
    ({ test_no_relational_191() });
}
pub fn test_no_relational_type_210() {
    ({ test_no_relational_192() });
    ({ test_no_relational_193() });
    ({ test_no_relational_194() });
}
pub fn test_no_relational_type_211() {
    ({ test_no_relational_195() });
    ({ test_no_relational_196() });
    ({ test_no_relational_197() });
}
pub fn test_no_relational_type_212() {
    ({ test_no_relational_200() });
    ({ test_no_relational_201() });
    ({ test_no_relational_202() });
}
pub fn test_no_relational_type_213() {
    ({ test_no_relational_203() });
    ({ test_no_relational_204() });
    ({ test_no_relational_205() });
}
pub fn test_no_relational_type_214() {
    ({ test_no_relational_206() });
    ({ test_no_relational_207() });
    ({ test_no_relational_208() });
}
pub fn test_no_relational_return_215() {
    ({ test_no_relational_type_209() });
    ({ test_no_relational_type_210() });
    ({ test_no_relational_type_211() });
    ({ test_no_relational_187() });
    ({ test_no_relational_188() });
}
pub fn test_no_relational_return_216() {
    ({ test_no_relational_type_212() });
    ({ test_no_relational_type_213() });
    ({ test_no_relational_type_214() });
    ({ test_no_relational_198() });
    ({ test_no_relational_199() });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    ({ test_std_ordering_184() });
    ({ test_std_ordering_185() });
    ({ test_std_ordering_186() });
    ({ test_relational_type_121(false) });
    ({ test_relational_return_137() });
    ({ test_relational_return_138() });
    ({ test_no_relational_return_215() });
    ({ test_no_relational_return_216() });
    0
}
impl __gnu_test_semaphore {
    pub fn new() -> Self {
        let __this: Value<__gnu_test_semaphore> = Rc::new(RefCell::new(Self {
            sem_set_: unimplemented!(),
            pid_: unimplemented!(),
        }));
        let this: Ptr<__gnu_test_semaphore> = __this.as_pointer();
        field!(this, pid_).write(unimplemented!());
        field!(this, sem_set_).write(unimplemented!());
        if (this.with(|__s| __s.sem_set_) == -1_()) {
            unimplemented!();
        }
        let val: Value<__gnu_test_semun> = Rc::new(RefCell::new(<__gnu_test_semun>::default()));
        __gnu_test_semun::val(val.as_pointer()).write(0_());
        if (unimplemented!() == -1_()) {
            unimplemented!();
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
pub fn set_memory_limits_217(size: Option<()>) {
    let mut size: () = size.unwrap_or_else(|| 16_f64);
    let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut limit: () = (size * 1048576_());
    unimplemented!();
    (*r.borrow_mut()).rlim_cur = limit;
    unimplemented!();
    unimplemented!();
    (*r.borrow_mut()).rlim_cur = limit;
    unimplemented!();
    if (limit < 20971520_()) {
        limit = 20971520_();
    }
    unimplemented!();
    (*r.borrow_mut()).rlim_cur = limit;
    unimplemented!();
}
pub fn set_file_limit_218(mut size: ()) {
    let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut limit: () = ((size) as ());
    unimplemented!();
    (*r.borrow_mut()).rlim_cur = limit;
    unimplemented!();
}
pub fn verify_demangle_219(mut mangled: Ptr<()>, mut wanted: Ptr<()>) {
    let status: Value<()> = Rc::new(RefCell::new(0_()));
    let mut s: Ptr<()> = Ptr::<()>::null();
    let mut demangled: Ptr<()> = unimplemented!();
    if !(demangled).is_null() {
        s = (demangled).clone();
    } else {
        switch!(match (*status.borrow()) {
            __v if __v == 0_() => {
                s = Ptr::<()>::from_string_literal(b"error code = 0: success");
                break;
            }
            __v if __v == -1_() => {
                s = Ptr::<()>::from_string_literal(b"error code = -1: memory allocation failure");
                break;
            }
            __v if __v == -2_() => {
                s = Ptr::<()>::from_string_literal(b"error code = -2: invalid mangled name");
                break;
            }
            __v if __v == -3_() => {
                s = Ptr::<()>::from_string_literal(b"error code = -3: invalid arguments");
                break;
            }
            _ => {
                s = Ptr::<()>::from_string_literal(b"error code unknown - who knows what happened");
            }
        });
    }
    let w: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    if !(unimplemented!()) {
        unimplemented!();
    }
    unimplemented!();
}
pub fn run_tests_wrapped_locale_220(mut name: Ptr<()>, l: Ptr<__gnu_test_func_callback>) {
    let loc_name: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut res: Ptr<()> = unimplemented!();
    if !(res).is_null() {
        let mut preLC_ALL: () = unimplemented!();
        let mut tests: Ptr<FnPtr<fn()>> = ({ __gnu_test_func_callbackImpl::tests(&l) });
        let mut i: () = 0_();
        'loop_: while ({ i } < { ({ __gnu_test_func_callbackImpl::size(&l) }) }) {
            ({ (elem!(tests, i).read()).call() });
            i.prefix_inc();
        }
        let mut postLC_ALL: () = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (false) {
            __do_while = false;
            if !(unimplemented!()) {
                unimplemented!();
                unimplemented!();
            }
        }
    } else {
        let mut s: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn run_tests_wrapped_env_221(
    mut name: Ptr<()>,
    mut env: Ptr<()>,
    l: Ptr<__gnu_test_func_callback>,
) {
    let loc_name: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let orig: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut oldENV: Ptr<()> = unimplemented!();
    if !(unimplemented!() != 0) {
        let mut tests: Ptr<FnPtr<fn()>> = ({ __gnu_test_func_callbackImpl::tests(&l) });
        let mut i: () = 0_();
        'loop_: while ({ i } < { ({ __gnu_test_func_callbackImpl::size(&l) }) }) {
            ({ (elem!(tests, i).read()).call() });
            i.prefix_inc();
        }
        unimplemented!();
    } else {
        let mut s: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
thread_local!(
    pub static count_0: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    static count__1: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    static throw_on__2: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    static count__3: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    static throw_on__4: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    static _M_count_5: Value<()> = Rc::new(RefCell::new(0_()));
);
thread_local!(
    static next_id__6: Value<()> = Rc::new(RefCell::new(0_()));
);
#[derive(VaArg, FnPtrArg, ByteRepr, Clone)]
#[byte_size(8)]
pub struct __gnu_test_semun {
    #[offset(0)]
    #[byte_size(8)]
    __bytes: Box<[u8]>,
}
impl __gnu_test_semun {
    pub fn val(this: Ptr<Self>) -> Ptr<()> {
        this.reinterpret_cast()
    }
    pub fn buf(this: Ptr<Self>) -> Ptr<Ptr<()>> {
        this.reinterpret_cast()
    }
    pub fn array(this: Ptr<Self>) -> Ptr<Ptr<()>> {
        this.reinterpret_cast()
    }
}
impl Default for __gnu_test_semun {
    fn default() -> Self {
        __gnu_test_semun {
            __bytes: Box::from([0u8; 8]),
        }
    }
}
impl __gnu_test_semaphore {}
pub fn test_tm_222(
    mut sec: (),
    mut min: (),
    mut hour: (),
    mut mday: (),
    mut mon: (),
    mut year: (),
    mut wday: (),
    mut yday: (),
    mut isdst: (),
) -> () {
    (*tmp_223.with(Value::clone).borrow_mut()).tm_sec = sec;
    (*tmp_223.with(Value::clone).borrow_mut()).tm_min = min;
    (*tmp_223.with(Value::clone).borrow_mut()).tm_hour = hour;
    (*tmp_223.with(Value::clone).borrow_mut()).tm_mday = mday;
    (*tmp_223.with(Value::clone).borrow_mut()).tm_mon = mon;
    (*tmp_223.with(Value::clone).borrow_mut()).tm_year = year;
    (*tmp_223.with(Value::clone).borrow_mut()).tm_wday = wday;
    (*tmp_223.with(Value::clone).borrow_mut()).tm_yday = yday;
    (*tmp_223.with(Value::clone).borrow_mut()).tm_isdst = isdst;
    return (*tmp_223.with(Value::clone).borrow()).clone();
}
thread_local!(
    static tmp_223: Value<()> = Rc::new(RefCell::new(unimplemented!()));
);
pub trait CStrFree_CustomOrd_Impl {
    fn to_pconst(&self) -> Ptr<()> {
        unimplemented!()
    }
}
impl CStrFree_CustomOrd_Impl for Ptr<CStrFree_CustomOrd_> {}
pub trait CStrFree_NoOperators_Impl {
    fn to_pconst(&self) -> Ptr<()> {
        unimplemented!()
    }
}
impl CStrFree_NoOperators_Impl for Ptr<CStrFree_NoOperators_> {}
pub trait CStrFree_ReturnInt_Impl {
    fn to_pconst(&self) -> Ptr<()> {
        unimplemented!()
    }
}
impl CStrFree_ReturnInt_Impl for Ptr<CStrFree_ReturnInt_> {}
pub trait CStrFree_ReturnOrd_std_partial_ordering__Impl {
    fn to_pconst(&self) -> Ptr<()> {
        unimplemented!()
    }
}
impl CStrFree_ReturnOrd_std_partial_ordering__Impl
    for Ptr<CStrFree_ReturnOrd_std_partial_ordering__>
{
}
pub trait CStrFree_ReturnOrd_std_strong_ordering__Impl {
    fn to_pconst(&self) -> Ptr<()> {
        unimplemented!()
    }
}
impl CStrFree_ReturnOrd_std_strong_ordering__Impl
    for Ptr<CStrFree_ReturnOrd_std_strong_ordering__>
{
}
pub trait CStrFree_ReturnOrd_std_weak_ordering__Impl {
    fn to_pconst(&self) -> Ptr<()> {
        unimplemented!()
    }
}
impl CStrFree_ReturnOrd_std_weak_ordering__Impl for Ptr<CStrFree_ReturnOrd_std_weak_ordering__> {}
pub trait CStrFree_ReturnVoid_Impl {
    fn to_pconst(&self) -> Ptr<()> {
        unimplemented!()
    }
}
impl CStrFree_ReturnVoid_Impl for Ptr<CStrFree_ReturnVoid_> {}
pub trait CStrFriend_CustomOrd_Impl {
    fn to_pconst(&self) -> Ptr<()> {
        unimplemented!()
    }
}
impl CStrFriend_CustomOrd_Impl for Ptr<CStrFriend_CustomOrd_> {}
pub trait CStrFriend_NoOperators_Impl {
    fn to_pconst(&self) -> Ptr<()> {
        unimplemented!()
    }
}
impl CStrFriend_NoOperators_Impl for Ptr<CStrFriend_NoOperators_> {}
pub trait CStrFriend_ReturnInt_Impl {
    fn to_pconst(&self) -> Ptr<()> {
        unimplemented!()
    }
}
impl CStrFriend_ReturnInt_Impl for Ptr<CStrFriend_ReturnInt_> {}
pub trait CStrFriend_ReturnOrd_std_partial_ordering__Impl {
    fn to_pconst(&self) -> Ptr<()> {
        unimplemented!()
    }
}
impl CStrFriend_ReturnOrd_std_partial_ordering__Impl
    for Ptr<CStrFriend_ReturnOrd_std_partial_ordering__>
{
}
pub trait CStrFriend_ReturnOrd_std_strong_ordering__Impl {
    fn to_pconst(&self) -> Ptr<()> {
        unimplemented!()
    }
}
impl CStrFriend_ReturnOrd_std_strong_ordering__Impl
    for Ptr<CStrFriend_ReturnOrd_std_strong_ordering__>
{
}
pub trait CStrFriend_ReturnOrd_std_weak_ordering__Impl {
    fn to_pconst(&self) -> Ptr<()> {
        unimplemented!()
    }
}
impl CStrFriend_ReturnOrd_std_weak_ordering__Impl
    for Ptr<CStrFriend_ReturnOrd_std_weak_ordering__>
{
}
pub trait CStrFriend_ReturnVoid_Impl {
    fn to_pconst(&self) -> Ptr<()> {
        unimplemented!()
    }
}
impl CStrFriend_ReturnVoid_Impl for Ptr<CStrFriend_ReturnVoid_> {}
pub trait CStrMem_CustomOrd_Impl {
    fn to_pconst(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_cmp_2(&self, rhs: Ptr<CStrMem_CustomOrd_>) -> CustomOrd;
    fn operator_cmp_3(&self, rhs: Ptr<()>) -> CustomOrd;
}
impl CStrMem_CustomOrd_Impl for Ptr<CStrMem_CustomOrd_> {
    fn operator_cmp_2(&self, rhs: Ptr<CStrMem_CustomOrd_>) -> CustomOrd {
        return ({ CustomOrd::create(unimplemented!()) });
    }
    fn operator_cmp_3(&self, mut rhs: Ptr<()>) -> CustomOrd {
        return ({ CustomOrd::create(unimplemented!()) });
    }
}
pub trait CStrMem_NoOperators_Impl {
    fn to_pconst(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_cmp_2(&self, rhs: Ptr<CStrMem_NoOperators_>) -> NoOperators;
    fn operator_cmp_3(&self, rhs: Ptr<()>) -> NoOperators;
}
impl CStrMem_NoOperators_Impl for Ptr<CStrMem_NoOperators_> {
    fn operator_cmp_2(&self, rhs: Ptr<CStrMem_NoOperators_>) -> NoOperators {
        return ({ NoOperators::create(unimplemented!()) });
    }
    fn operator_cmp_3(&self, mut rhs: Ptr<()>) -> NoOperators {
        return ({ NoOperators::create(unimplemented!()) });
    }
}
pub trait CStrMem_ReturnInt_Impl {
    fn to_pconst(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_cmp_2(&self, rhs: Ptr<CStrMem_ReturnInt_>) -> ();
    fn operator_cmp_3(&self, rhs: Ptr<()>) -> ();
}
impl CStrMem_ReturnInt_Impl for Ptr<CStrMem_ReturnInt_> {
    fn operator_cmp_2(&self, rhs: Ptr<CStrMem_ReturnInt_>) -> () {
        return ({ ReturnInt::create(unimplemented!()) });
    }
    fn operator_cmp_3(&self, mut rhs: Ptr<()>) -> () {
        return ({ ReturnInt::create(unimplemented!()) });
    }
}
pub trait CStrMem_ReturnOrd_std_partial_ordering__Impl {
    fn to_pconst(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_cmp_2(&self, rhs: Ptr<CStrMem_ReturnOrd_std_partial_ordering__>) -> ();
    fn operator_cmp_3(&self, rhs: Ptr<()>) -> ();
}
impl CStrMem_ReturnOrd_std_partial_ordering__Impl
    for Ptr<CStrMem_ReturnOrd_std_partial_ordering__>
{
    fn operator_cmp_2(&self, rhs: Ptr<CStrMem_ReturnOrd_std_partial_ordering__>) -> () {
        return ({ ReturnOrd_std_partial_ordering_::create(unimplemented!()) });
    }
    fn operator_cmp_3(&self, mut rhs: Ptr<()>) -> () {
        return ({ ReturnOrd_std_partial_ordering_::create(unimplemented!()) });
    }
}
pub trait CStrMem_ReturnOrd_std_strong_ordering__Impl {
    fn to_pconst(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_cmp_2(&self, rhs: Ptr<CStrMem_ReturnOrd_std_strong_ordering__>) -> ();
    fn operator_cmp_3(&self, rhs: Ptr<()>) -> ();
}
impl CStrMem_ReturnOrd_std_strong_ordering__Impl for Ptr<CStrMem_ReturnOrd_std_strong_ordering__> {
    fn operator_cmp_2(&self, rhs: Ptr<CStrMem_ReturnOrd_std_strong_ordering__>) -> () {
        return ({ ReturnOrd_std_strong_ordering_::create(unimplemented!()) });
    }
    fn operator_cmp_3(&self, mut rhs: Ptr<()>) -> () {
        return ({ ReturnOrd_std_strong_ordering_::create(unimplemented!()) });
    }
}
pub trait CStrMem_ReturnOrd_std_weak_ordering__Impl {
    fn to_pconst(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_cmp_2(&self, rhs: Ptr<CStrMem_ReturnOrd_std_weak_ordering__>) -> ();
    fn operator_cmp_3(&self, rhs: Ptr<()>) -> ();
}
impl CStrMem_ReturnOrd_std_weak_ordering__Impl for Ptr<CStrMem_ReturnOrd_std_weak_ordering__> {
    fn operator_cmp_2(&self, rhs: Ptr<CStrMem_ReturnOrd_std_weak_ordering__>) -> () {
        return ({ ReturnOrd_std_weak_ordering_::create(unimplemented!()) });
    }
    fn operator_cmp_3(&self, mut rhs: Ptr<()>) -> () {
        return ({ ReturnOrd_std_weak_ordering_::create(unimplemented!()) });
    }
}
pub trait CStrMem_ReturnVoid_Impl {
    fn to_pconst(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn operator_cmp_2(&self, rhs: Ptr<CStrMem_ReturnVoid_>);
    fn operator_cmp_3(&self, rhs: Ptr<()>);
}
impl CStrMem_ReturnVoid_Impl for Ptr<CStrMem_ReturnVoid_> {
    fn operator_cmp_2(&self, rhs: Ptr<CStrMem_ReturnVoid_>) {
        ({ ReturnVoid::create(unimplemented!()) });
        return;
    }
    fn operator_cmp_3(&self, mut rhs: Ptr<()>) {
        ({ ReturnVoid::create(unimplemented!()) });
        return;
    }
}
pub trait CStrMixed_CustomOrd_Impl {
    fn to_pconst(&self) -> Ptr<()> {
        unimplemented!()
    }
}
impl CStrMixed_CustomOrd_Impl for Ptr<CStrMixed_CustomOrd_> {}
pub trait CStrMixed_NoOperators_Impl {
    fn to_pconst(&self) -> Ptr<()> {
        unimplemented!()
    }
}
impl CStrMixed_NoOperators_Impl for Ptr<CStrMixed_NoOperators_> {}
pub trait CStrMixed_ReturnInt_Impl {
    fn to_pconst(&self) -> Ptr<()> {
        unimplemented!()
    }
}
impl CStrMixed_ReturnInt_Impl for Ptr<CStrMixed_ReturnInt_> {}
pub trait CStrMixed_ReturnOrd_std_partial_ordering__Impl {
    fn to_pconst(&self) -> Ptr<()> {
        unimplemented!()
    }
}
impl CStrMixed_ReturnOrd_std_partial_ordering__Impl
    for Ptr<CStrMixed_ReturnOrd_std_partial_ordering__>
{
}
pub trait CStrMixed_ReturnOrd_std_strong_ordering__Impl {
    fn to_pconst(&self) -> Ptr<()> {
        unimplemented!()
    }
}
impl CStrMixed_ReturnOrd_std_strong_ordering__Impl
    for Ptr<CStrMixed_ReturnOrd_std_strong_ordering__>
{
}
pub trait CStrMixed_ReturnOrd_std_weak_ordering__Impl {
    fn to_pconst(&self) -> Ptr<()> {
        unimplemented!()
    }
}
impl CStrMixed_ReturnOrd_std_weak_ordering__Impl for Ptr<CStrMixed_ReturnOrd_std_weak_ordering__> {}
pub trait CStrMixed_ReturnVoid_Impl {
    fn to_pconst(&self) -> Ptr<()> {
        unimplemented!()
    }
}
impl CStrMixed_ReturnVoid_Impl for Ptr<CStrMixed_ReturnVoid_> {}
pub trait CStrNoneImpl {
    fn to_pconst(&self) -> Ptr<()>;
}
impl CStrNoneImpl for Ptr<CStrNone> {
    fn to_pconst(&self) -> Ptr<()> {
        return (*self).with(|__s| __s.str.clone());
    }
}
pub trait __gnu_test_copy_trackerImpl {
    fn copy_assign(&self, rhs: Ptr<__gnu_test_copy_tracker>) -> Ptr<__gnu_test_copy_tracker>;
    fn destructor(&self);
    fn id(&self) -> ();
}
impl __gnu_test_copy_trackerImpl for Ptr<__gnu_test_copy_tracker> {
    fn copy_assign(&self, rhs: Ptr<__gnu_test_copy_tracker>) -> Ptr<__gnu_test_copy_tracker> {
        field!((*self), id_).write(({ __gnu_test_copy_trackerImpl::id(&rhs) }));
        if rhs.with(|__s| __s.throw_on_copy_) {
            ({
                __gnu_test_assignment_operator::throw_on(
                    ({ __gnu_test_assignment_operator::count() }).wrapping_add(1_()),
                )
            });
        }
        ({ __gnu_test_assignment_operator::mark_call() });
        return (*self).clone();
    }
    fn destructor(&self) {
        ({ __gnu_test_destructor::mark_call() });
    }
    fn id(&self) -> () {
        return (*self).with(|__s| __s.id_);
    }
}
pub trait __gnu_test_func_callbackImpl {
    fn copy_assign(&self, _a0: Ptr<__gnu_test_func_callback>) -> Ptr<__gnu_test_func_callback> {
        unimplemented!()
    }
    fn size(&self) -> ();
    fn tests(&self) -> Ptr<FnPtr<fn()>>;
    fn push_back(&self, test: FnPtr<fn()>);
}
impl __gnu_test_func_callbackImpl for Ptr<__gnu_test_func_callback> {
    fn size(&self) -> () {
        return (*self).with(|__s| __s._M_size);
    }
    fn tests(&self) -> Ptr<FnPtr<fn()>> {
        return (array_field_ptr!((*self), _M_tests) as Ptr<FnPtr<fn()>>);
    }
    fn push_back(&self, mut test: FnPtr<fn()>) {
        array_elem!((*self), _M_tests, (*self).with(|__s| __s._M_size)).write((test).clone());
        field!((*self), _M_size).with_mut(|__v| __v.prefix_inc());
    }
}
pub trait __gnu_test_object_counterImpl {
    fn destructor(&self);
    fn copy_assign(&self, _a0: Ptr<__gnu_test_object_counter>) -> Ptr<__gnu_test_object_counter>;
}
impl __gnu_test_object_counterImpl for Ptr<__gnu_test_object_counter> {
    fn destructor(&self) {
        (*count_0.with(Value::clone).borrow_mut()).prefix_dec();
    }
    fn copy_assign(&self, _a0: Ptr<__gnu_test_object_counter>) -> Ptr<__gnu_test_object_counter> {
        return (*self).clone();
    }
}
pub trait __gnu_test_semaphoreImpl {
    fn destructor(&self) {
        unimplemented!()
    }
    fn signal(&self) {
        unimplemented!()
    }
    fn wait(&self) {
        unimplemented!()
    }
}
impl __gnu_test_semaphoreImpl for Ptr<__gnu_test_semaphore> {
    fn destructor(&self) {
        let val: Value<__gnu_test_semun> = Rc::new(RefCell::new(<__gnu_test_semun>::default()));
        __gnu_test_semun::val(val.as_pointer()).write(0_());
        if ((*self).with(|__s| __s.pid_) == unimplemented!()) {
            unimplemented!();
        }
    }
    fn signal(&self) {
        let op: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([unimplemented!()])));
        if (unimplemented!() == -1_()) {
            unimplemented!();
        }
    }
    fn wait(&self) {
        let op: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([unimplemented!()])));
        if (unimplemented!() == -1_()) {
            unimplemented!();
        }
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = arr_9.with(|_| ());
    let _ = s1_10.with(|_| ());
    let _ = s2_11.with(|_| ());
    let _ = s3_12.with(|_| ());
    let _ = count_0.with(|_| ());
    let _ = count__1.with(|_| ());
    let _ = throw_on__2.with(|_| ());
    let _ = count__3.with(|_| ());
    let _ = throw_on__4.with(|_| ());
    let _ = _M_count_5.with(|_| ());
    let _ = next_id__6.with(|_| ());
}
