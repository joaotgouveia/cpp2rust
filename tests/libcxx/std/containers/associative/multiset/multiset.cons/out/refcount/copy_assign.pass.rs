extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn operator_eq_0(a: Ptr<test_less_int_>, b: Ptr<test_less_int_>) -> bool {
    return ({ a.with(|__s| __s.data_) } == { b.with(|__s| __s.data_) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct test_less_int_ {
    #[offset(0)]
    pub data_: i32,
}
impl test_less_int_ {
    pub fn new_1() -> Self {
        Self { data_: 0 }
    }
    pub fn new_2(mut data: i32) -> Self {
        Self { data_: data }
    }
}
impl CppEq for test_less_int_ {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_0(__lhs.clone(), __rhs.clone())
    }
}
impl Callable2<Ptr<i32>, Ptr<i32>, bool> for test_less_int_ {
    fn call(&self, a1: Ptr<i32>, a2: Ptr<i32>) -> bool {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            test_less_int_Impl::operator_call(__it, a1, a2)
        })
    }
}
impl CppMove for test_less_int_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
impl Default for test_less_int_ {
    fn default() -> Self {
        { test_less_int_::new_1() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub fn outstanding_alloc() -> usize {
        assert!((alloc_count_2.with(|rc| *rc.borrow()) >= dealloc_count_3.with(|rc| *rc.borrow())));
        return ((alloc_count_2.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_3.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        assert!((({ malloc_allocator_base::outstanding_alloc() }) == 0_usize));
        disable_default_constructor_4.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_1.with(|rc| *rc.borrow_mut() = 0_usize);
        alloc_count_2.with(|rc| *rc.borrow_mut() = 0_usize);
        dealloc_count_3.with(|rc| *rc.borrow_mut() = 0_usize);
    }
}
impl CppMove for malloc_allocator_base {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static outstanding_bytes_1: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static alloc_count_2: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static dealloc_count_3: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static disable_default_constructor_4: Value<bool> = Rc::new(RefCell::new(false));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_int_ {}
impl CppMove for min_allocator_int_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(Vec<AnyPtr  >  , new, mut_ptr)]
#[byte_size(8)]
pub struct tracking_allocator_int_ {
    #[offset(0)]
    #[byte_size(8)]
    allocs_: Ptr<Vec<AnyPtr>>,
}
impl tracking_allocator_int_ {
    pub fn new(allocs: Ptr<Vec<AnyPtr>>) -> Self {
        Self {
            allocs_: (allocs).clone(),
        }
    }
}
impl CppMove for tracking_allocator_int_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct NoOp {}
impl Callable0<()> for NoOp {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| NoOpImpl::operator_call(__it))
    }
}
impl CppMove for NoOp {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn test_alloc_5(
    lhs_alloc: Option<Ptr<min_allocator_int_>>,
    rhs_alloc: Option<Ptr<min_allocator_int_>>,
    check_alloc_invariant: Option<NoOp>,
) {
    let mut __lhs_alloc_default: Option<Value<min_allocator_int_>> = None;
    let lhs_alloc: Ptr<min_allocator_int_> = lhs_alloc.unwrap_or_else(|| {
        __lhs_alloc_default
            .insert(Rc::new(RefCell::new(<min_allocator_int_>::default())))
            .as_pointer()
    });
    let mut __rhs_alloc_default: Option<Value<min_allocator_int_>> = None;
    let rhs_alloc: Ptr<min_allocator_int_> = rhs_alloc.unwrap_or_else(|| {
        __rhs_alloc_default
            .insert(Rc::new(RefCell::new(<min_allocator_int_>::default())))
            .as_pointer()
    });
    let check_alloc_invariant: Value<NoOp> = Rc::new(RefCell::new(
        check_alloc_invariant.unwrap_or_else(|| <NoOp>::default()),
    ));
    {
        {
            let arr: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 2])));
            let orig: Value<std_multiset_int__std_less_int___min_allocator_int__> =
                Rc::new(RefCell::new({
                    let __tmp_0: Value<std_less_int_> =
                        Rc::new(RefCell::new(<std_less_int_>::default()));
                    std_multiset_int__std_less_int___min_allocator_int__::new_1(
                        { ({ begin_6(arr.as_pointer()) }) },
                        { (arr.as_pointer() as Ptr<i32>).to_end() },
                        { __tmp_0.as_pointer() },
                        { (rhs_alloc).clone() },
                    )
                }));
            let copy: Value<std_multiset_int__std_less_int___min_allocator_int__> = Rc::new(
                RefCell::new(std_multiset_int__std_less_int___min_allocator_int__::new_2(
                    { (lhs_alloc).clone() },
                )),
            );
            (*copy.borrow_mut()) = (*orig.borrow()).clone();
            assert!(({ (({ (*copy.borrow()).size() }) == 3_u64) }));
            assert!(
                ((libcc2rs::next_refcount(({ ({ (*copy.borrow()).begin() }) }), 0_isize as isize)
                    .iter_deref()
                    .read())
                    == 1)
            );
            assert!(
                ((libcc2rs::next_refcount(({ ({ (*copy.borrow()).begin() }) }), 1_isize as isize)
                    .iter_deref()
                    .read())
                    == 2)
            );
            assert!(
                ((libcc2rs::next_refcount(({ ({ (*copy.borrow()).begin() }) }), 2_isize as isize)
                    .iter_deref()
                    .read())
                    == 2)
            );
            assert!(
                libcc2rs::next_refcount(({ ({ (*copy.borrow()).begin() }) }), 3_isize as isize)
                    == ({ (*copy.borrow()).end() })
            );
            assert!(({ (({ (*orig.borrow()).size() }) == 3_u64) }));
            assert!(
                ((libcc2rs::next_refcount(({ ({ (*orig.borrow()).begin() }) }), 0_isize as isize)
                    .iter_deref()
                    .read())
                    == 1)
            );
            assert!(
                ((libcc2rs::next_refcount(({ ({ (*orig.borrow()).begin() }) }), 1_isize as isize)
                    .iter_deref()
                    .read())
                    == 2)
            );
            assert!(
                ((libcc2rs::next_refcount(({ ({ (*orig.borrow()).begin() }) }), 2_isize as isize)
                    .iter_deref()
                    .read())
                    == 2)
            );
            assert!(
                libcc2rs::next_refcount(({ ({ (*orig.borrow()).begin() }) }), 3_isize as isize)
                    == ({ (*orig.borrow()).end() })
            );
        }
        ({ NoOpImpl::operator_call(&check_alloc_invariant.as_pointer()) });
        {
            let orig: Value<std_multiset_int__std_less_int___min_allocator_int__> = Rc::new(
                RefCell::new(std_multiset_int__std_less_int___min_allocator_int__::new_2(
                    { (rhs_alloc).clone() },
                )),
            );
            let copy: Value<std_multiset_int__std_less_int___min_allocator_int__> = Rc::new(
                RefCell::new(std_multiset_int__std_less_int___min_allocator_int__::new_2(
                    { (lhs_alloc).clone() },
                )),
            );
            (*copy.borrow_mut()) = (*orig.borrow()).clone();
            assert!(({ (({ (*copy.borrow()).size() }) == 0_u64) }));
            assert!(({ (*copy.borrow()).begin() }) == ({ (*copy.borrow()).end() }));
            assert!(({ (({ (*orig.borrow()).size() }) == 0_u64) }));
            assert!(({ (*orig.borrow()).begin() }) == ({ (*orig.borrow()).end() }));
        }
        ({ NoOpImpl::operator_call(&check_alloc_invariant.as_pointer()) });
        {
            let arr: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 2])));
            let orig: Value<std_multiset_int__std_less_int___min_allocator_int__> = Rc::new(
                RefCell::new(std_multiset_int__std_less_int___min_allocator_int__::new_2(
                    { (rhs_alloc).clone() },
                )),
            );
            let copy: Value<std_multiset_int__std_less_int___min_allocator_int__> =
                Rc::new(RefCell::new({
                    let __tmp_1: Value<std_less_int_> =
                        Rc::new(RefCell::new(<std_less_int_>::default()));
                    std_multiset_int__std_less_int___min_allocator_int__::new_1(
                        { ({ begin_6(arr.as_pointer()) }) },
                        { (arr.as_pointer() as Ptr<i32>).to_end() },
                        { __tmp_1.as_pointer() },
                        { (rhs_alloc).clone() },
                    )
                }));
            (*copy.borrow_mut()) = (*orig.borrow()).clone();
            assert!(({ (({ (*copy.borrow()).size() }) == 0_u64) }));
            assert!(({ (*copy.borrow()).begin() }) == ({ (*copy.borrow()).end() }));
            assert!(({ (({ (*orig.borrow()).size() }) == 0_u64) }));
            assert!(({ (*orig.borrow()).begin() }) == ({ (*orig.borrow()).end() }));
        }
    }
    {
        {
            let arr: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 2])));
            let orig: Value<std_multiset_int__std_less_int___min_allocator_int__> =
                Rc::new(RefCell::new({
                    let __tmp_2: Value<std_less_int_> =
                        Rc::new(RefCell::new(<std_less_int_>::default()));
                    std_multiset_int__std_less_int___min_allocator_int__::new_1(
                        { ({ begin_6(arr.as_pointer()) }) },
                        { (arr.as_pointer() as Ptr<i32>).to_end() },
                        { __tmp_2.as_pointer() },
                        { (rhs_alloc).clone() },
                    )
                }));
            (*orig.borrow_mut()) = { (*orig.borrow()).clone() };
            assert!(({ (({ (*orig.borrow()).size() }) == 3_u64) }));
            assert!(
                ((libcc2rs::next_refcount(({ ({ (*orig.borrow()).begin() }) }), 0_isize as isize)
                    .iter_deref()
                    .read())
                    == 1)
            );
            assert!(
                ((libcc2rs::next_refcount(({ ({ (*orig.borrow()).begin() }) }), 1_isize as isize)
                    .iter_deref()
                    .read())
                    == 2)
            );
            assert!(
                ((libcc2rs::next_refcount(({ ({ (*orig.borrow()).begin() }) }), 2_isize as isize)
                    .iter_deref()
                    .read())
                    == 2)
            );
            assert!(
                libcc2rs::next_refcount(({ ({ (*orig.borrow()).begin() }) }), 3_isize as isize)
                    == ({ (*orig.borrow()).end() })
            );
        }
        {
            let orig: Value<std_multiset_int__std_less_int___min_allocator_int__> = Rc::new(
                RefCell::new(std_multiset_int__std_less_int___min_allocator_int__::new_2(
                    { (rhs_alloc).clone() },
                )),
            );
            (*orig.borrow_mut()) = { (*orig.borrow()).clone() };
            assert!(({ (({ (*orig.borrow()).size() }) == 0_u64) }));
            assert!(({ (*orig.borrow()).begin() }) == ({ (*orig.borrow()).end() }));
        }
    }
    {
        ({ NoOpImpl::operator_call(&check_alloc_invariant.as_pointer()) });
        {
            let lhs_arr: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 2])));
            let orig: Value<std_multiset_int__std_less_int___min_allocator_int__> =
                Rc::new(RefCell::new({
                    let __tmp_3: Value<std_less_int_> =
                        Rc::new(RefCell::new(<std_less_int_>::default()));
                    std_multiset_int__std_less_int___min_allocator_int__::new_1(
                        { ({ begin_6(lhs_arr.as_pointer()) }) },
                        { (lhs_arr.as_pointer() as Ptr<i32>).to_end() },
                        { __tmp_3.as_pointer() },
                        { (rhs_alloc).clone() },
                    )
                }));
            let rhs_arr: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([4, 5])));
            let copy: Value<std_multiset_int__std_less_int___min_allocator_int__> =
                Rc::new(RefCell::new({
                    let __tmp_4: Value<std_less_int_> =
                        Rc::new(RefCell::new(<std_less_int_>::default()));
                    std_multiset_int__std_less_int___min_allocator_int__::new_1(
                        { ({ begin_7(rhs_arr.as_pointer()) }) },
                        { (rhs_arr.as_pointer() as Ptr<i32>).to_end() },
                        { __tmp_4.as_pointer() },
                        { (lhs_alloc).clone() },
                    )
                }));
            (*copy.borrow_mut()) = (*orig.borrow()).clone();
            assert!(({ (({ (*copy.borrow()).size() }) == 3_u64) }));
            assert!(
                ((libcc2rs::next_refcount(({ ({ (*copy.borrow()).begin() }) }), 0_isize as isize)
                    .iter_deref()
                    .read())
                    == 1)
            );
            assert!(
                ((libcc2rs::next_refcount(({ ({ (*copy.borrow()).begin() }) }), 1_isize as isize)
                    .iter_deref()
                    .read())
                    == 2)
            );
            assert!(
                ((libcc2rs::next_refcount(({ ({ (*copy.borrow()).begin() }) }), 2_isize as isize)
                    .iter_deref()
                    .read())
                    == 2)
            );
            assert!(
                libcc2rs::next_refcount(({ ({ (*copy.borrow()).begin() }) }), 3_isize as isize)
                    == ({ (*copy.borrow()).end() })
            );
            assert!(({ (({ (*orig.borrow()).size() }) == 3_u64) }));
            assert!(
                ((libcc2rs::next_refcount(({ ({ (*orig.borrow()).begin() }) }), 0_isize as isize)
                    .iter_deref()
                    .read())
                    == 1)
            );
            assert!(
                ((libcc2rs::next_refcount(({ ({ (*orig.borrow()).begin() }) }), 1_isize as isize)
                    .iter_deref()
                    .read())
                    == 2)
            );
            assert!(
                ((libcc2rs::next_refcount(({ ({ (*orig.borrow()).begin() }) }), 2_isize as isize)
                    .iter_deref()
                    .read())
                    == 2)
            );
            assert!(
                libcc2rs::next_refcount(({ ({ (*orig.borrow()).begin() }) }), 3_isize as isize)
                    == ({ (*orig.borrow()).end() })
            );
        }
        ({ NoOpImpl::operator_call(&check_alloc_invariant.as_pointer()) });
        {
            let lhs_arr: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 2])));
            let orig: Value<std_multiset_int__std_less_int___min_allocator_int__> =
                Rc::new(RefCell::new({
                    let __tmp_5: Value<std_less_int_> =
                        Rc::new(RefCell::new(<std_less_int_>::default()));
                    std_multiset_int__std_less_int___min_allocator_int__::new_1(
                        { ({ begin_6(lhs_arr.as_pointer()) }) },
                        { (lhs_arr.as_pointer() as Ptr<i32>).to_end() },
                        { __tmp_5.as_pointer() },
                        { (rhs_alloc).clone() },
                    )
                }));
            let rhs_arr: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([4, 5, 6])));
            let copy: Value<std_multiset_int__std_less_int___min_allocator_int__> =
                Rc::new(RefCell::new({
                    let __tmp_6: Value<std_less_int_> =
                        Rc::new(RefCell::new(<std_less_int_>::default()));
                    std_multiset_int__std_less_int___min_allocator_int__::new_1(
                        { ({ begin_6(rhs_arr.as_pointer()) }) },
                        { (rhs_arr.as_pointer() as Ptr<i32>).to_end() },
                        { __tmp_6.as_pointer() },
                        { (lhs_alloc).clone() },
                    )
                }));
            (*copy.borrow_mut()) = (*orig.borrow()).clone();
            assert!(({ (({ (*copy.borrow()).size() }) == 3_u64) }));
            assert!(
                ((libcc2rs::next_refcount(({ ({ (*copy.borrow()).begin() }) }), 0_isize as isize)
                    .iter_deref()
                    .read())
                    == 1)
            );
            assert!(
                ((libcc2rs::next_refcount(({ ({ (*copy.borrow()).begin() }) }), 1_isize as isize)
                    .iter_deref()
                    .read())
                    == 2)
            );
            assert!(
                ((libcc2rs::next_refcount(({ ({ (*copy.borrow()).begin() }) }), 2_isize as isize)
                    .iter_deref()
                    .read())
                    == 2)
            );
            assert!(
                libcc2rs::next_refcount(({ ({ (*copy.borrow()).begin() }) }), 3_isize as isize)
                    == ({ (*copy.borrow()).end() })
            );
            assert!(({ (({ (*orig.borrow()).size() }) == 3_u64) }));
            assert!(
                ((libcc2rs::next_refcount(({ ({ (*orig.borrow()).begin() }) }), 0_isize as isize)
                    .iter_deref()
                    .read())
                    == 1)
            );
            assert!(
                ((libcc2rs::next_refcount(({ ({ (*orig.borrow()).begin() }) }), 1_isize as isize)
                    .iter_deref()
                    .read())
                    == 2)
            );
            assert!(
                ((libcc2rs::next_refcount(({ ({ (*orig.borrow()).begin() }) }), 2_isize as isize)
                    .iter_deref()
                    .read())
                    == 2)
            );
            assert!(
                libcc2rs::next_refcount(({ ({ (*orig.borrow()).begin() }) }), 3_isize as isize)
                    == ({ (*orig.borrow()).end() })
            );
        }
        ({ NoOpImpl::operator_call(&check_alloc_invariant.as_pointer()) });
        {
            let lhs_arr: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 2])));
            let orig: Value<std_multiset_int__std_less_int___min_allocator_int__> =
                Rc::new(RefCell::new({
                    let __tmp_7: Value<std_less_int_> =
                        Rc::new(RefCell::new(<std_less_int_>::default()));
                    std_multiset_int__std_less_int___min_allocator_int__::new_1(
                        { ({ begin_6(lhs_arr.as_pointer()) }) },
                        { (lhs_arr.as_pointer() as Ptr<i32>).to_end() },
                        { __tmp_7.as_pointer() },
                        { (rhs_alloc).clone() },
                    )
                }));
            let rhs_arr: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([4, 5, 6])));
            let copy: Value<std_multiset_int__std_less_int___min_allocator_int__> =
                Rc::new(RefCell::new({
                    let __tmp_8: Value<std_less_int_> =
                        Rc::new(RefCell::new(<std_less_int_>::default()));
                    std_multiset_int__std_less_int___min_allocator_int__::new_1(
                        { ({ begin_6(rhs_arr.as_pointer()) }) },
                        { (rhs_arr.as_pointer() as Ptr<i32>).to_end() },
                        { __tmp_8.as_pointer() },
                        { (lhs_alloc).clone() },
                    )
                }));
            (*copy.borrow_mut()) = (*orig.borrow()).clone();
            assert!(({ (({ (*copy.borrow()).size() }) == 3_u64) }));
            assert!(
                ((libcc2rs::next_refcount(({ ({ (*copy.borrow()).begin() }) }), 0_isize as isize)
                    .iter_deref()
                    .read())
                    == 1)
            );
            assert!(
                ((libcc2rs::next_refcount(({ ({ (*copy.borrow()).begin() }) }), 1_isize as isize)
                    .iter_deref()
                    .read())
                    == 2)
            );
            assert!(
                ((libcc2rs::next_refcount(({ ({ (*copy.borrow()).begin() }) }), 2_isize as isize)
                    .iter_deref()
                    .read())
                    == 2)
            );
            assert!(
                libcc2rs::next_refcount(({ ({ (*copy.borrow()).begin() }) }), 3_isize as isize)
                    == ({ (*copy.borrow()).end() })
            );
            assert!(({ (({ (*orig.borrow()).size() }) == 3_u64) }));
            assert!(
                ((libcc2rs::next_refcount(({ ({ (*orig.borrow()).begin() }) }), 0_isize as isize)
                    .iter_deref()
                    .read())
                    == 1)
            );
            assert!(
                ((libcc2rs::next_refcount(({ ({ (*orig.borrow()).begin() }) }), 1_isize as isize)
                    .iter_deref()
                    .read())
                    == 2)
            );
            assert!(
                ((libcc2rs::next_refcount(({ ({ (*orig.borrow()).begin() }) }), 2_isize as isize)
                    .iter_deref()
                    .read())
                    == 2)
            );
            assert!(
                libcc2rs::next_refcount(({ ({ (*orig.borrow()).begin() }) }), 3_isize as isize)
                    == ({ (*orig.borrow()).end() })
            );
        }
        ({ NoOpImpl::operator_call(&check_alloc_invariant.as_pointer()) });
        {
            let orig: Value<std_multiset_int__std_less_int___min_allocator_int__> = Rc::new(
                RefCell::new(std_multiset_int__std_less_int___min_allocator_int__::new_2(
                    { (rhs_alloc).clone() },
                )),
            );
            let i: Value<i32> = Rc::new(RefCell::new(0));
            'loop_: while ((*i.borrow()) != 50) {
                ({ (*orig.borrow()).insert_36(i.as_pointer()) });
                (*i.borrow_mut()).prefix_inc();
            }
            let copy: Value<std_multiset_int__std_less_int___min_allocator_int__> = Rc::new(
                RefCell::new(std_multiset_int__std_less_int___min_allocator_int__::new_2(
                    { (lhs_alloc).clone() },
                )),
            );
            (*copy.borrow_mut()) = (*orig.borrow()).clone();
            let mut i: i32 = 0;
            'loop_: for v in
                IterRange::new(({ (*copy.borrow()).begin() }), ({ (*copy.borrow()).end() }))
            {
                let mut v: i32 = (v.iter_deref().read());
                assert!((v == i.postfix_inc()));
            }
        }
        ({ NoOpImpl::operator_call(&check_alloc_invariant.as_pointer()) });
    }
}
pub fn test_alloc_8(
    lhs_alloc: Ptr<tracking_allocator_int_>,
    rhs_alloc: Ptr<tracking_allocator_int_>,
    check_alloc_invariant: AssertEmpty_9,
) {
    let check_alloc_invariant: Value<AssertEmpty_9> = Rc::new(RefCell::new(check_alloc_invariant));
    {
        {
            let arr: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 2])));
            let orig: Value<std_multiset_int__std_less_int___tracking_allocator_int__> =
                Rc::new(RefCell::new({
                    let __tmp_9: Value<std_less_int_> =
                        Rc::new(RefCell::new(<std_less_int_>::default()));
                    std_multiset_int__std_less_int___tracking_allocator_int__::new_1(
                        { ({ begin_6(arr.as_pointer()) }) },
                        { (arr.as_pointer() as Ptr<i32>).to_end() },
                        { __tmp_9.as_pointer() },
                        { (rhs_alloc).clone() },
                    )
                }));
            let copy: Value<std_multiset_int__std_less_int___tracking_allocator_int__> =
                Rc::new(RefCell::new(
                    std_multiset_int__std_less_int___tracking_allocator_int__::new_2({
                        (lhs_alloc).clone()
                    }),
                ));
            (*copy.borrow_mut()) = (*orig.borrow()).clone();
            assert!(({ (({ (*copy.borrow()).size() }) == 3_u64) }));
            assert!(
                ((libcc2rs::next_refcount(({ ({ (*copy.borrow()).begin() }) }), 0_isize as isize)
                    .iter_deref()
                    .read())
                    == 1)
            );
            assert!(
                ((libcc2rs::next_refcount(({ ({ (*copy.borrow()).begin() }) }), 1_isize as isize)
                    .iter_deref()
                    .read())
                    == 2)
            );
            assert!(
                ((libcc2rs::next_refcount(({ ({ (*copy.borrow()).begin() }) }), 2_isize as isize)
                    .iter_deref()
                    .read())
                    == 2)
            );
            assert!(
                libcc2rs::next_refcount(({ ({ (*copy.borrow()).begin() }) }), 3_isize as isize)
                    == ({ (*copy.borrow()).end() })
            );
            assert!(({ (({ (*orig.borrow()).size() }) == 3_u64) }));
            assert!(
                ((libcc2rs::next_refcount(({ ({ (*orig.borrow()).begin() }) }), 0_isize as isize)
                    .iter_deref()
                    .read())
                    == 1)
            );
            assert!(
                ((libcc2rs::next_refcount(({ ({ (*orig.borrow()).begin() }) }), 1_isize as isize)
                    .iter_deref()
                    .read())
                    == 2)
            );
            assert!(
                ((libcc2rs::next_refcount(({ ({ (*orig.borrow()).begin() }) }), 2_isize as isize)
                    .iter_deref()
                    .read())
                    == 2)
            );
            assert!(
                libcc2rs::next_refcount(({ ({ (*orig.borrow()).begin() }) }), 3_isize as isize)
                    == ({ (*orig.borrow()).end() })
            );
        }
        ({ AssertEmpty_9Impl::operator_call(&check_alloc_invariant.as_pointer()) });
        {
            let orig: Value<std_multiset_int__std_less_int___tracking_allocator_int__> =
                Rc::new(RefCell::new(
                    std_multiset_int__std_less_int___tracking_allocator_int__::new_2({
                        (rhs_alloc).clone()
                    }),
                ));
            let copy: Value<std_multiset_int__std_less_int___tracking_allocator_int__> =
                Rc::new(RefCell::new(
                    std_multiset_int__std_less_int___tracking_allocator_int__::new_2({
                        (lhs_alloc).clone()
                    }),
                ));
            (*copy.borrow_mut()) = (*orig.borrow()).clone();
            assert!(({ (({ (*copy.borrow()).size() }) == 0_u64) }));
            assert!(({ (*copy.borrow()).begin() }) == ({ (*copy.borrow()).end() }));
            assert!(({ (({ (*orig.borrow()).size() }) == 0_u64) }));
            assert!(({ (*orig.borrow()).begin() }) == ({ (*orig.borrow()).end() }));
        }
        ({ AssertEmpty_9Impl::operator_call(&check_alloc_invariant.as_pointer()) });
        {
            let arr: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 2])));
            let orig: Value<std_multiset_int__std_less_int___tracking_allocator_int__> =
                Rc::new(RefCell::new(
                    std_multiset_int__std_less_int___tracking_allocator_int__::new_2({
                        (rhs_alloc).clone()
                    }),
                ));
            let copy: Value<std_multiset_int__std_less_int___tracking_allocator_int__> =
                Rc::new(RefCell::new({
                    let __tmp_10: Value<std_less_int_> =
                        Rc::new(RefCell::new(<std_less_int_>::default()));
                    std_multiset_int__std_less_int___tracking_allocator_int__::new_1(
                        { ({ begin_6(arr.as_pointer()) }) },
                        { (arr.as_pointer() as Ptr<i32>).to_end() },
                        { __tmp_10.as_pointer() },
                        { (rhs_alloc).clone() },
                    )
                }));
            (*copy.borrow_mut()) = (*orig.borrow()).clone();
            assert!(({ (({ (*copy.borrow()).size() }) == 0_u64) }));
            assert!(({ (*copy.borrow()).begin() }) == ({ (*copy.borrow()).end() }));
            assert!(({ (({ (*orig.borrow()).size() }) == 0_u64) }));
            assert!(({ (*orig.borrow()).begin() }) == ({ (*orig.borrow()).end() }));
        }
    }
    {
        {
            let arr: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 2])));
            let orig: Value<std_multiset_int__std_less_int___tracking_allocator_int__> =
                Rc::new(RefCell::new({
                    let __tmp_11: Value<std_less_int_> =
                        Rc::new(RefCell::new(<std_less_int_>::default()));
                    std_multiset_int__std_less_int___tracking_allocator_int__::new_1(
                        { ({ begin_6(arr.as_pointer()) }) },
                        { (arr.as_pointer() as Ptr<i32>).to_end() },
                        { __tmp_11.as_pointer() },
                        { (rhs_alloc).clone() },
                    )
                }));
            (*orig.borrow_mut()) = { (*orig.borrow()).clone() };
            assert!(({ (({ (*orig.borrow()).size() }) == 3_u64) }));
            assert!(
                ((libcc2rs::next_refcount(({ ({ (*orig.borrow()).begin() }) }), 0_isize as isize)
                    .iter_deref()
                    .read())
                    == 1)
            );
            assert!(
                ((libcc2rs::next_refcount(({ ({ (*orig.borrow()).begin() }) }), 1_isize as isize)
                    .iter_deref()
                    .read())
                    == 2)
            );
            assert!(
                ((libcc2rs::next_refcount(({ ({ (*orig.borrow()).begin() }) }), 2_isize as isize)
                    .iter_deref()
                    .read())
                    == 2)
            );
            assert!(
                libcc2rs::next_refcount(({ ({ (*orig.borrow()).begin() }) }), 3_isize as isize)
                    == ({ (*orig.borrow()).end() })
            );
        }
        {
            let orig: Value<std_multiset_int__std_less_int___tracking_allocator_int__> =
                Rc::new(RefCell::new(
                    std_multiset_int__std_less_int___tracking_allocator_int__::new_2({
                        (rhs_alloc).clone()
                    }),
                ));
            (*orig.borrow_mut()) = { (*orig.borrow()).clone() };
            assert!(({ (({ (*orig.borrow()).size() }) == 0_u64) }));
            assert!(({ (*orig.borrow()).begin() }) == ({ (*orig.borrow()).end() }));
        }
    }
    {
        ({ AssertEmpty_9Impl::operator_call(&check_alloc_invariant.as_pointer()) });
        {
            let lhs_arr: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 2])));
            let orig: Value<std_multiset_int__std_less_int___tracking_allocator_int__> =
                Rc::new(RefCell::new({
                    let __tmp_12: Value<std_less_int_> =
                        Rc::new(RefCell::new(<std_less_int_>::default()));
                    std_multiset_int__std_less_int___tracking_allocator_int__::new_1(
                        { ({ begin_6(lhs_arr.as_pointer()) }) },
                        { (lhs_arr.as_pointer() as Ptr<i32>).to_end() },
                        { __tmp_12.as_pointer() },
                        { (rhs_alloc).clone() },
                    )
                }));
            let rhs_arr: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([4, 5])));
            let copy: Value<std_multiset_int__std_less_int___tracking_allocator_int__> =
                Rc::new(RefCell::new({
                    let __tmp_13: Value<std_less_int_> =
                        Rc::new(RefCell::new(<std_less_int_>::default()));
                    std_multiset_int__std_less_int___tracking_allocator_int__::new_1(
                        { ({ begin_7(rhs_arr.as_pointer()) }) },
                        { (rhs_arr.as_pointer() as Ptr<i32>).to_end() },
                        { __tmp_13.as_pointer() },
                        { (lhs_alloc).clone() },
                    )
                }));
            (*copy.borrow_mut()) = (*orig.borrow()).clone();
            assert!(({ (({ (*copy.borrow()).size() }) == 3_u64) }));
            assert!(
                ((libcc2rs::next_refcount(({ ({ (*copy.borrow()).begin() }) }), 0_isize as isize)
                    .iter_deref()
                    .read())
                    == 1)
            );
            assert!(
                ((libcc2rs::next_refcount(({ ({ (*copy.borrow()).begin() }) }), 1_isize as isize)
                    .iter_deref()
                    .read())
                    == 2)
            );
            assert!(
                ((libcc2rs::next_refcount(({ ({ (*copy.borrow()).begin() }) }), 2_isize as isize)
                    .iter_deref()
                    .read())
                    == 2)
            );
            assert!(
                libcc2rs::next_refcount(({ ({ (*copy.borrow()).begin() }) }), 3_isize as isize)
                    == ({ (*copy.borrow()).end() })
            );
            assert!(({ (({ (*orig.borrow()).size() }) == 3_u64) }));
            assert!(
                ((libcc2rs::next_refcount(({ ({ (*orig.borrow()).begin() }) }), 0_isize as isize)
                    .iter_deref()
                    .read())
                    == 1)
            );
            assert!(
                ((libcc2rs::next_refcount(({ ({ (*orig.borrow()).begin() }) }), 1_isize as isize)
                    .iter_deref()
                    .read())
                    == 2)
            );
            assert!(
                ((libcc2rs::next_refcount(({ ({ (*orig.borrow()).begin() }) }), 2_isize as isize)
                    .iter_deref()
                    .read())
                    == 2)
            );
            assert!(
                libcc2rs::next_refcount(({ ({ (*orig.borrow()).begin() }) }), 3_isize as isize)
                    == ({ (*orig.borrow()).end() })
            );
        }
        ({ AssertEmpty_9Impl::operator_call(&check_alloc_invariant.as_pointer()) });
        {
            let lhs_arr: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 2])));
            let orig: Value<std_multiset_int__std_less_int___tracking_allocator_int__> =
                Rc::new(RefCell::new({
                    let __tmp_14: Value<std_less_int_> =
                        Rc::new(RefCell::new(<std_less_int_>::default()));
                    std_multiset_int__std_less_int___tracking_allocator_int__::new_1(
                        { ({ begin_6(lhs_arr.as_pointer()) }) },
                        { (lhs_arr.as_pointer() as Ptr<i32>).to_end() },
                        { __tmp_14.as_pointer() },
                        { (rhs_alloc).clone() },
                    )
                }));
            let rhs_arr: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([4, 5, 6])));
            let copy: Value<std_multiset_int__std_less_int___tracking_allocator_int__> =
                Rc::new(RefCell::new({
                    let __tmp_15: Value<std_less_int_> =
                        Rc::new(RefCell::new(<std_less_int_>::default()));
                    std_multiset_int__std_less_int___tracking_allocator_int__::new_1(
                        { ({ begin_6(rhs_arr.as_pointer()) }) },
                        { (rhs_arr.as_pointer() as Ptr<i32>).to_end() },
                        { __tmp_15.as_pointer() },
                        { (lhs_alloc).clone() },
                    )
                }));
            (*copy.borrow_mut()) = (*orig.borrow()).clone();
            assert!(({ (({ (*copy.borrow()).size() }) == 3_u64) }));
            assert!(
                ((libcc2rs::next_refcount(({ ({ (*copy.borrow()).begin() }) }), 0_isize as isize)
                    .iter_deref()
                    .read())
                    == 1)
            );
            assert!(
                ((libcc2rs::next_refcount(({ ({ (*copy.borrow()).begin() }) }), 1_isize as isize)
                    .iter_deref()
                    .read())
                    == 2)
            );
            assert!(
                ((libcc2rs::next_refcount(({ ({ (*copy.borrow()).begin() }) }), 2_isize as isize)
                    .iter_deref()
                    .read())
                    == 2)
            );
            assert!(
                libcc2rs::next_refcount(({ ({ (*copy.borrow()).begin() }) }), 3_isize as isize)
                    == ({ (*copy.borrow()).end() })
            );
            assert!(({ (({ (*orig.borrow()).size() }) == 3_u64) }));
            assert!(
                ((libcc2rs::next_refcount(({ ({ (*orig.borrow()).begin() }) }), 0_isize as isize)
                    .iter_deref()
                    .read())
                    == 1)
            );
            assert!(
                ((libcc2rs::next_refcount(({ ({ (*orig.borrow()).begin() }) }), 1_isize as isize)
                    .iter_deref()
                    .read())
                    == 2)
            );
            assert!(
                ((libcc2rs::next_refcount(({ ({ (*orig.borrow()).begin() }) }), 2_isize as isize)
                    .iter_deref()
                    .read())
                    == 2)
            );
            assert!(
                libcc2rs::next_refcount(({ ({ (*orig.borrow()).begin() }) }), 3_isize as isize)
                    == ({ (*orig.borrow()).end() })
            );
        }
        ({ AssertEmpty_9Impl::operator_call(&check_alloc_invariant.as_pointer()) });
        {
            let lhs_arr: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 2])));
            let orig: Value<std_multiset_int__std_less_int___tracking_allocator_int__> =
                Rc::new(RefCell::new({
                    let __tmp_16: Value<std_less_int_> =
                        Rc::new(RefCell::new(<std_less_int_>::default()));
                    std_multiset_int__std_less_int___tracking_allocator_int__::new_1(
                        { ({ begin_6(lhs_arr.as_pointer()) }) },
                        { (lhs_arr.as_pointer() as Ptr<i32>).to_end() },
                        { __tmp_16.as_pointer() },
                        { (rhs_alloc).clone() },
                    )
                }));
            let rhs_arr: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([4, 5, 6])));
            let copy: Value<std_multiset_int__std_less_int___tracking_allocator_int__> =
                Rc::new(RefCell::new({
                    let __tmp_17: Value<std_less_int_> =
                        Rc::new(RefCell::new(<std_less_int_>::default()));
                    std_multiset_int__std_less_int___tracking_allocator_int__::new_1(
                        { ({ begin_6(rhs_arr.as_pointer()) }) },
                        { (rhs_arr.as_pointer() as Ptr<i32>).to_end() },
                        { __tmp_17.as_pointer() },
                        { (lhs_alloc).clone() },
                    )
                }));
            (*copy.borrow_mut()) = (*orig.borrow()).clone();
            assert!(({ (({ (*copy.borrow()).size() }) == 3_u64) }));
            assert!(
                ((libcc2rs::next_refcount(({ ({ (*copy.borrow()).begin() }) }), 0_isize as isize)
                    .iter_deref()
                    .read())
                    == 1)
            );
            assert!(
                ((libcc2rs::next_refcount(({ ({ (*copy.borrow()).begin() }) }), 1_isize as isize)
                    .iter_deref()
                    .read())
                    == 2)
            );
            assert!(
                ((libcc2rs::next_refcount(({ ({ (*copy.borrow()).begin() }) }), 2_isize as isize)
                    .iter_deref()
                    .read())
                    == 2)
            );
            assert!(
                libcc2rs::next_refcount(({ ({ (*copy.borrow()).begin() }) }), 3_isize as isize)
                    == ({ (*copy.borrow()).end() })
            );
            assert!(({ (({ (*orig.borrow()).size() }) == 3_u64) }));
            assert!(
                ((libcc2rs::next_refcount(({ ({ (*orig.borrow()).begin() }) }), 0_isize as isize)
                    .iter_deref()
                    .read())
                    == 1)
            );
            assert!(
                ((libcc2rs::next_refcount(({ ({ (*orig.borrow()).begin() }) }), 1_isize as isize)
                    .iter_deref()
                    .read())
                    == 2)
            );
            assert!(
                ((libcc2rs::next_refcount(({ ({ (*orig.borrow()).begin() }) }), 2_isize as isize)
                    .iter_deref()
                    .read())
                    == 2)
            );
            assert!(
                libcc2rs::next_refcount(({ ({ (*orig.borrow()).begin() }) }), 3_isize as isize)
                    == ({ (*orig.borrow()).end() })
            );
        }
        ({ AssertEmpty_9Impl::operator_call(&check_alloc_invariant.as_pointer()) });
        {
            let orig: Value<std_multiset_int__std_less_int___tracking_allocator_int__> =
                Rc::new(RefCell::new(
                    std_multiset_int__std_less_int___tracking_allocator_int__::new_2({
                        (rhs_alloc).clone()
                    }),
                ));
            let i: Value<i32> = Rc::new(RefCell::new(0));
            'loop_: while ((*i.borrow()) != 50) {
                ({ (*orig.borrow()).insert_36(i.as_pointer()) });
                (*i.borrow_mut()).prefix_inc();
            }
            let copy: Value<std_multiset_int__std_less_int___tracking_allocator_int__> =
                Rc::new(RefCell::new(
                    std_multiset_int__std_less_int___tracking_allocator_int__::new_2({
                        (lhs_alloc).clone()
                    }),
                ));
            (*copy.borrow_mut()) = (*orig.borrow()).clone();
            let mut i: i32 = 0;
            'loop_: for v in
                IterRange::new(({ (*copy.borrow()).begin() }), ({ (*copy.borrow()).end() }))
            {
                let mut v: i32 = (v.iter_deref().read());
                assert!((v == i.postfix_inc()));
            }
        }
        ({ AssertEmpty_9Impl::operator_call(&check_alloc_invariant.as_pointer()) });
    }
}
pub fn test_10() -> bool {
    ({ test_alloc_11(None, None, None) });
    ({ test_alloc_5(None, None, None) });
    {
        let lhs_allocs: Value<Vec<AnyPtr>> = Rc::new(RefCell::new(Vec::new()));
        let rhs_allocs: Value<Vec<AnyPtr>> = Rc::new(RefCell::new(Vec::new()));
        ({
            let _lhs_alloc: Value<tracking_allocator_int_> =
                Rc::new(RefCell::new(tracking_allocator_int_::new({
                    lhs_allocs.as_pointer()
                })));
            let _rhs_alloc: Value<tracking_allocator_int_> =
                Rc::new(RefCell::new(tracking_allocator_int_::new({
                    rhs_allocs.as_pointer()
                })));
            let _check_alloc_invariant: AssertEmpty_9 =
                AssertEmpty_9::new({ lhs_allocs.as_pointer() }, { rhs_allocs.as_pointer() });
            test_alloc_8(
                _lhs_alloc.as_pointer(),
                _rhs_alloc.as_pointer(),
                _check_alloc_invariant,
            )
        });
    }
    {
        let arr: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([1, 2, 2])));
        let orig: Value<std_multiset_int__test_less_int___std_allocator_int__> =
            Rc::new(RefCell::new({
                let __tmp_18: Value<test_less_int_> =
                    Rc::new(RefCell::new(test_less_int_::new_2({ 3 })));
                std_multiset_int__test_less_int___std_allocator_int__::new_1(
                    { ({ begin_6(arr.as_pointer()) }) },
                    { (arr.as_pointer() as Ptr<i32>).to_end() },
                    { __tmp_18.as_pointer() },
                    None,
                )
            }));
        let copy: Value<std_multiset_int__test_less_int___std_allocator_int__> = Rc::new(
            RefCell::new(<std_multiset_int__test_less_int___std_allocator_int__>::default()),
        );
        (*copy.borrow_mut()) = (*orig.borrow()).clone();
        assert!(({ (({ (*copy.borrow()).size() }) == 3_u64) }));
        assert!(
            ({
                let _a: Value<test_less_int_> =
                    Rc::new(RefCell::new(({ (*copy.borrow()).key_comp() })));
                let _b: Value<test_less_int_> = Rc::new(RefCell::new(test_less_int_::new_2({ 3 })));
                operator_eq_0(_a.as_pointer(), _b.as_pointer())
            })
        );
        assert!(({ (({ (*orig.borrow()).size() }) == 3_u64) }));
        assert!(
            ({
                let _a: Value<test_less_int_> =
                    Rc::new(RefCell::new(({ (*orig.borrow()).key_comp() })));
                let _b: Value<test_less_int_> = Rc::new(RefCell::new(test_less_int_::new_2({ 3 })));
                operator_eq_0(_a.as_pointer(), _b.as_pointer())
            })
        );
    }
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct AssertEmpty_9 {
    #[offset(0)]
    #[byte_size(8)]
    lhs_allocs_: Ptr<Vec<AnyPtr>>,
    #[offset(8)]
    #[byte_size(8)]
    rhs_allocs_: Ptr<Vec<AnyPtr>>,
}
impl AssertEmpty_9 {
    pub fn new(lhs_allocs: Ptr<Vec<AnyPtr>>, rhs_allocs: Ptr<Vec<AnyPtr>>) -> Self {
        Self {
            lhs_allocs_: (lhs_allocs).clone(),
            rhs_allocs_: (rhs_allocs).clone(),
        }
    }
}
impl Callable0<()> for AssertEmpty_9 {
    fn call(&self) -> () {
        libcc2rs::with_record_ptr(&mut self.clone(), |__it| {
            AssertEmpty_9Impl::operator_call(__it)
        })
    }
}
impl CppMove for AssertEmpty_9 {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    ({ test_10() });
    return 0;
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct min_pointer_int__std_integral_constant_unsigned_long__0__;
pub trait AssertEmpty_9Impl {
    fn operator_call(&self);
}
impl AssertEmpty_9Impl for Ptr<AssertEmpty_9> {
    fn operator_call(&self) {
        assert!(
            ({
                (*(*self)
                    .with(|__s| __s.lhs_allocs_.clone())
                    .upgrade()
                    .deref())
                .is_empty()
            })
        );
        assert!(
            ({
                (*(*self)
                    .with(|__s| __s.rhs_allocs_.clone())
                    .upgrade()
                    .deref())
                .is_empty()
            })
        );
    }
}
pub trait NoOpImpl {
    fn operator_call(&self);
}
impl NoOpImpl for Ptr<NoOp> {
    fn operator_call(&self) {}
}
pub trait min_allocator_int_Impl {
    fn allocate(&self, mut n: usize) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn deallocate(
        &self,
        mut p: min_pointer_int__std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        unimplemented!()
    }
}
impl min_allocator_int_Impl for Ptr<min_allocator_int_> {}
pub trait test_less_int_Impl {
    fn operator_call(&self, a: Ptr<i32>, b: Ptr<i32>) -> bool;
}
impl test_less_int_Impl for Ptr<test_less_int_> {
    fn operator_call(&self, a: Ptr<i32>, b: Ptr<i32>) -> bool {
        return ({ (a.read()) } < { (b.read()) });
    }
}
pub trait tracking_allocator_int_Impl {
    fn allocate(&self, mut n: usize) -> Ptr<i32> {
        unimplemented!()
    }
    fn deallocate(&self, mut ptr: Ptr<i32>, mut n: usize) {
        unimplemented!()
    }
}
impl tracking_allocator_int_Impl for Ptr<tracking_allocator_int_> {}
pub fn __cpp2rust_init_globals() {
    let _ = outstanding_bytes_1.with(|_| ());
    let _ = alloc_count_2.with(|_| ());
    let _ = dealloc_count_3.with(|_| ());
    let _ = disable_default_constructor_4.with(|_| ());
}
