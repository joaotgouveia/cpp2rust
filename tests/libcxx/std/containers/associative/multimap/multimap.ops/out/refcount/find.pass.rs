extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub fn outstanding_alloc() -> usize {
        assert!((alloc_count_1.with(|rc| *rc.borrow()) >= dealloc_count_2.with(|rc| *rc.borrow())));
        return ((alloc_count_1.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_2.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        assert!((({ malloc_allocator_base::outstanding_alloc() }) == 0_usize));
        disable_default_constructor_3.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_0.with(|rc| *rc.borrow_mut() = 0_usize);
        alloc_count_1.with(|rc| *rc.borrow_mut() = 0_usize);
        dealloc_count_2.with(|rc| *rc.borrow_mut() = 0_usize);
    }
}
thread_local!(
    pub static outstanding_bytes_0: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static alloc_count_1: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static dealloc_count_2: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static disable_default_constructor_3: Value<bool> = Rc::new(RefCell::new(false));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: Ptr<(Value<i32>, Value<f64>)>,
}
impl min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    #[offset(0)]
    #[byte_size(8)]
    ptr_: AnyPtr,
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_std_pair_const_int__double__ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct PrivateConstructor {
    #[offset(0)]
    val: i32,
}
impl PrivateConstructor {
    pub fn make(mut v: i32) -> PrivateConstructor {
        return PrivateConstructor::new({ v });
    }
    fn new(mut v: i32) -> Self {
        Self { val: v }
    }
}
impl CppLess for PrivateConstructor {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_4(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for PrivateConstructor {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        !(operator_lt_4(__lhs.clone(), __rhs.clone()))
            && !(operator_lt_4(__rhs.clone(), __lhs.clone()))
    }
}
pub fn operator_lt_4(lhs: Ptr<PrivateConstructor>, rhs: Ptr<PrivateConstructor>) -> bool {
    return ({ ({ PrivateConstructorImpl::get(&lhs) }) } < {
        ({ PrivateConstructorImpl::get(&rhs) })
    });
}
pub fn operator_lt_5(lhs: Ptr<PrivateConstructor>, mut rhs: i32) -> bool {
    return ({ ({ PrivateConstructorImpl::get(&lhs) }) } < { rhs });
}
pub fn operator_lt_6(mut lhs: i32, rhs: Ptr<PrivateConstructor>) -> bool {
    return ({ lhs } < { ({ PrivateConstructorImpl::get(&rhs) }) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct transparent_less {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct transparent_less_not_referenceable {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct transparent_less_nonempty_is_transparent {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct transparent_less_nonempty {
    #[offset(0)]
    #[byte_size(1)]
    pub pad_: transparent_less_nonempty_is_transparent,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct transparent_less_no_type {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct transparent_less_private {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct transparent_less_not_a_type {
    #[offset(0)]
    pub is_transparent: i32,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom)]
#[cpp_from(i32, new_1, value)]
#[byte_size(4)]
pub struct C2Int {
    #[offset(0)]
    i_: i32,
}
impl C2Int {
    pub fn new_2() -> Self {
        Self { i_: 0 }
    }
    pub fn new_1(mut i: i32) -> Self {
        Self { i_: i }
    }
}
impl CppLess for C2Int {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_7(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for C2Int {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        !(operator_lt_7(__lhs.clone(), __rhs.clone()))
            && !(operator_lt_7(__rhs.clone(), __lhs.clone()))
    }
}
impl Default for C2Int {
    fn default() -> Self {
        { C2Int::new_2() }
    }
}
pub fn operator_lt_8(mut rhs: i32, lhs: Ptr<C2Int>) -> bool {
    return ({ rhs } < { ({ C2IntImpl::get(&lhs) }) });
}
pub fn operator_lt_7(rhs: Ptr<C2Int>, lhs: Ptr<C2Int>) -> bool {
    return ({ ({ C2IntImpl::get(&rhs) }) } < { ({ C2IntImpl::get(&lhs) }) });
}
pub fn operator_lt_9(rhs: Ptr<C2Int>, mut lhs: i32) -> bool {
    return ({ ({ C2IntImpl::get(&rhs) }) } < { lhs });
}
pub fn iter_in_range_10(
    first: BidirectionalIterator<(Value<i32>, Value<f64>)>,
    last: BidirectionalIterator<(Value<i32>, Value<f64>)>,
    to_find: BidirectionalIterator<(Value<i32>, Value<f64>)>,
) -> bool {
    let first: Value<BidirectionalIterator<(Value<i32>, Value<f64>)>> =
        Rc::new(RefCell::new(first));
    let last: Value<BidirectionalIterator<(Value<i32>, Value<f64>)>> = Rc::new(RefCell::new(last));
    let to_find: Value<BidirectionalIterator<(Value<i32>, Value<f64>)>> =
        Rc::new(RefCell::new(to_find));
    'loop_: while (*first.borrow()) != (*last.borrow()) {
        if (*first.borrow()) == (*to_find.borrow()) {
            return true;
        }
        (*first.borrow_mut()).iter_inc();
    }
    return false;
}
pub fn iter_in_range_11(
    first: BidirectionalIterator<(Value<i32>, Value<f64>)>,
    last: BidirectionalIterator<(Value<i32>, Value<f64>)>,
    to_find: BidirectionalIterator<(Value<i32>, Value<f64>)>,
) -> bool {
    let first: Value<BidirectionalIterator<(Value<i32>, Value<f64>)>> =
        Rc::new(RefCell::new(first));
    let last: Value<BidirectionalIterator<(Value<i32>, Value<f64>)>> = Rc::new(RefCell::new(last));
    let to_find: Value<BidirectionalIterator<(Value<i32>, Value<f64>)>> =
        Rc::new(RefCell::new(to_find));
    'loop_: while (*first.borrow()) != (*last.borrow()) {
        if (*first.borrow()) == (*to_find.borrow()) {
            return true;
        }
        (*first.borrow_mut()).iter_inc();
    }
    return false;
}
pub fn iter_in_range_12(
    first: BidirectionalIterator<(Value<i32>, Value<i32>)>,
    last: BidirectionalIterator<(Value<i32>, Value<i32>)>,
    to_find: BidirectionalIterator<(Value<i32>, Value<i32>)>,
) -> bool {
    let first: Value<BidirectionalIterator<(Value<i32>, Value<i32>)>> =
        Rc::new(RefCell::new(first));
    let last: Value<BidirectionalIterator<(Value<i32>, Value<i32>)>> = Rc::new(RefCell::new(last));
    let to_find: Value<BidirectionalIterator<(Value<i32>, Value<i32>)>> =
        Rc::new(RefCell::new(to_find));
    'loop_: while (*first.borrow()) != (*last.borrow()) {
        if (*first.borrow()) == (*to_find.borrow()) {
            return true;
        }
        (*first.borrow_mut()).iter_inc();
    }
    return false;
}
pub fn iter_in_range_13(
    first: BidirectionalIterator<(Value<i32>, Value<f64>)>,
    last: BidirectionalIterator<(Value<i32>, Value<f64>)>,
    to_find: BidirectionalIterator<(Value<i32>, Value<f64>)>,
) -> bool {
    let first: Value<BidirectionalIterator<(Value<i32>, Value<f64>)>> =
        Rc::new(RefCell::new(first));
    let last: Value<BidirectionalIterator<(Value<i32>, Value<f64>)>> = Rc::new(RefCell::new(last));
    let to_find: Value<BidirectionalIterator<(Value<i32>, Value<f64>)>> =
        Rc::new(RefCell::new(to_find));
    'loop_: while (*first.borrow()) != (*last.borrow()) {
        if (*first.borrow()) == (*to_find.borrow()) {
            return true;
        }
        (*first.borrow_mut()).iter_inc();
    }
    return false;
}
pub fn iter_in_range_14(
    first: BidirectionalIterator<(Value<i32>, Value<f64>)>,
    last: BidirectionalIterator<(Value<i32>, Value<f64>)>,
    to_find: BidirectionalIterator<(Value<i32>, Value<f64>)>,
) -> bool {
    let first: Value<BidirectionalIterator<(Value<i32>, Value<f64>)>> =
        Rc::new(RefCell::new(first));
    let last: Value<BidirectionalIterator<(Value<i32>, Value<f64>)>> = Rc::new(RefCell::new(last));
    let to_find: Value<BidirectionalIterator<(Value<i32>, Value<f64>)>> =
        Rc::new(RefCell::new(to_find));
    'loop_: while (*first.borrow()) != (*last.borrow()) {
        if (*first.borrow()) == (*to_find.borrow()) {
            return true;
        }
        (*first.borrow_mut()).iter_inc();
    }
    return false;
}
pub fn iter_in_range_15(
    first: BidirectionalIterator<(Value<PrivateConstructor>, Value<f64>)>,
    last: BidirectionalIterator<(Value<PrivateConstructor>, Value<f64>)>,
    to_find: BidirectionalIterator<(Value<PrivateConstructor>, Value<f64>)>,
) -> bool {
    let first: Value<BidirectionalIterator<(Value<PrivateConstructor>, Value<f64>)>> =
        Rc::new(RefCell::new(first));
    let last: Value<BidirectionalIterator<(Value<PrivateConstructor>, Value<f64>)>> =
        Rc::new(RefCell::new(last));
    let to_find: Value<BidirectionalIterator<(Value<PrivateConstructor>, Value<f64>)>> =
        Rc::new(RefCell::new(to_find));
    'loop_: while (*first.borrow()) != (*last.borrow()) {
        if (*first.borrow()) == (*to_find.borrow()) {
            return true;
        }
        (*first.borrow_mut()).iter_inc();
    }
    return false;
}
pub fn test_16() -> bool {
    {
        {
            let ar: Value<Box<[(Value<i32>, Value<f64>)]>> = Rc::new(RefCell::new(Box::new([
                (Rc::new(RefCell::new(5)), Rc::new(RefCell::new((1 as f64)))),
                (Rc::new(RefCell::new(5)), Rc::new(RefCell::new((2 as f64)))),
                (Rc::new(RefCell::new(5)), Rc::new(RefCell::new((3 as f64)))),
                (Rc::new(RefCell::new(7)), Rc::new(RefCell::new((1 as f64)))),
                (Rc::new(RefCell::new(7)), Rc::new(RefCell::new((2 as f64)))),
                (Rc::new(RefCell::new(7)), Rc::new(RefCell::new((3 as f64)))),
                (Rc::new(RefCell::new(9)), Rc::new(RefCell::new((1 as f64)))),
                (Rc::new(RefCell::new(9)), Rc::new(RefCell::new((2 as f64)))),
                (Rc::new(RefCell::new(9)), Rc::new(RefCell::new((3 as f64)))),
            ])));
            let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            assert!(
                ({
                    let _first: BidirectionalIterator<(Value<i32>, Value<f64>)> =
                        libcc2rs::next_refcount(unimplemented!(), 0_isize);
                    let _last: BidirectionalIterator<(Value<i32>, Value<f64>)> =
                        libcc2rs::next_refcount(unimplemented!(), 3_isize);
                    iter_in_range_10(_first, _last, (*r.borrow()).clone())
                })
            );
            (*r.borrow_mut()) = unimplemented!();
            assert!((*r.borrow()) == unimplemented!());
            (*r.borrow_mut()) = unimplemented!();
            assert!(
                ({
                    let _first: BidirectionalIterator<(Value<i32>, Value<f64>)> =
                        libcc2rs::next_refcount(unimplemented!(), 3_isize);
                    let _last: BidirectionalIterator<(Value<i32>, Value<f64>)> =
                        libcc2rs::next_refcount(unimplemented!(), 6_isize);
                    iter_in_range_10(_first, _last, (*r.borrow()).clone())
                })
            );
            (*r.borrow_mut()) = unimplemented!();
            assert!((*r.borrow()) == unimplemented!());
            (*r.borrow_mut()) = unimplemented!();
            assert!(
                ({
                    let _first: BidirectionalIterator<(Value<i32>, Value<f64>)> =
                        libcc2rs::next_refcount(unimplemented!(), 6_isize);
                    let _last: BidirectionalIterator<(Value<i32>, Value<f64>)> =
                        libcc2rs::next_refcount(unimplemented!(), 9_isize);
                    iter_in_range_10(_first, _last, (*r.borrow()).clone())
                })
            );
            (*r.borrow_mut()) = unimplemented!();
            assert!((*r.borrow()) == unimplemented!());
        }
        {
            let ar: Value<Box<[(Value<i32>, Value<f64>)]>> = Rc::new(RefCell::new(Box::new([
                (Rc::new(RefCell::new(5)), Rc::new(RefCell::new((1 as f64)))),
                (Rc::new(RefCell::new(5)), Rc::new(RefCell::new((2 as f64)))),
                (Rc::new(RefCell::new(5)), Rc::new(RefCell::new((3 as f64)))),
                (Rc::new(RefCell::new(7)), Rc::new(RefCell::new((1 as f64)))),
                (Rc::new(RefCell::new(7)), Rc::new(RefCell::new((2 as f64)))),
                (Rc::new(RefCell::new(7)), Rc::new(RefCell::new((3 as f64)))),
                (Rc::new(RefCell::new(9)), Rc::new(RefCell::new((1 as f64)))),
                (Rc::new(RefCell::new(9)), Rc::new(RefCell::new((2 as f64)))),
                (Rc::new(RefCell::new(9)), Rc::new(RefCell::new((3 as f64)))),
            ])));
            let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            assert!(
                ({
                    let _first: BidirectionalIterator<(Value<i32>, Value<f64>)> =
                        libcc2rs::next_refcount(unimplemented!(), 0_isize);
                    let _last: BidirectionalIterator<(Value<i32>, Value<f64>)> =
                        libcc2rs::next_refcount(unimplemented!(), 3_isize);
                    iter_in_range_11(_first, _last, (*r.borrow()).clone())
                })
            );
            (*r.borrow_mut()) = unimplemented!();
            assert!((*r.borrow()) == unimplemented!());
            (*r.borrow_mut()) = unimplemented!();
            assert!(
                ({
                    let _first: BidirectionalIterator<(Value<i32>, Value<f64>)> =
                        libcc2rs::next_refcount(unimplemented!(), 3_isize);
                    let _last: BidirectionalIterator<(Value<i32>, Value<f64>)> =
                        libcc2rs::next_refcount(unimplemented!(), 6_isize);
                    iter_in_range_11(_first, _last, (*r.borrow()).clone())
                })
            );
            (*r.borrow_mut()) = unimplemented!();
            assert!((*r.borrow()) == unimplemented!());
            (*r.borrow_mut()) = unimplemented!();
            assert!(
                ({
                    let _first: BidirectionalIterator<(Value<i32>, Value<f64>)> =
                        libcc2rs::next_refcount(unimplemented!(), 6_isize);
                    let _last: BidirectionalIterator<(Value<i32>, Value<f64>)> =
                        libcc2rs::next_refcount(unimplemented!(), 9_isize);
                    iter_in_range_11(_first, _last, (*r.borrow()).clone())
                })
            );
            (*r.borrow_mut()) = unimplemented!();
            assert!((*r.borrow()) == unimplemented!());
        }
    }
    {
        let arr: Value<Box<[(Value<i32>, Value<i32>)]>> = Rc::new(RefCell::new(Box::new([
            (Rc::new(RefCell::new(5)), Rc::new(RefCell::new(1))),
            (Rc::new(RefCell::new(5)), Rc::new(RefCell::new(2))),
            (Rc::new(RefCell::new(5)), Rc::new(RefCell::new(3))),
            (Rc::new(RefCell::new(7)), Rc::new(RefCell::new(1))),
            (Rc::new(RefCell::new(7)), Rc::new(RefCell::new(2))),
            (Rc::new(RefCell::new(7)), Rc::new(RefCell::new(3))),
            (Rc::new(RefCell::new(9)), Rc::new(RefCell::new(1))),
            (Rc::new(RefCell::new(9)), Rc::new(RefCell::new(2))),
            (Rc::new(RefCell::new(9)), Rc::new(RefCell::new(3))),
        ])));
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            ({
                let _first: BidirectionalIterator<(Value<i32>, Value<i32>)> =
                    libcc2rs::next_refcount(unimplemented!(), 6_isize);
                let _last: BidirectionalIterator<(Value<i32>, Value<i32>)> =
                    libcc2rs::next_refcount(unimplemented!(), 9_isize);
                let _to_find: BidirectionalIterator<(Value<i32>, Value<i32>)> = unimplemented!();
                iter_in_range_12(_first, _last, _to_find)
            })
        );
        assert!(unimplemented!() == unimplemented!());
        assert!(
            ({
                let _first: BidirectionalIterator<(Value<i32>, Value<i32>)> =
                    libcc2rs::next_refcount(unimplemented!(), 3_isize);
                let _last: BidirectionalIterator<(Value<i32>, Value<i32>)> =
                    libcc2rs::next_refcount(unimplemented!(), 6_isize);
                let _to_find: BidirectionalIterator<(Value<i32>, Value<i32>)> = unimplemented!();
                iter_in_range_12(_first, _last, _to_find)
            })
        );
        assert!(unimplemented!() == unimplemented!());
        assert!(
            ({
                let _first: BidirectionalIterator<(Value<i32>, Value<i32>)> =
                    libcc2rs::next_refcount(unimplemented!(), 0_isize);
                let _last: BidirectionalIterator<(Value<i32>, Value<i32>)> =
                    libcc2rs::next_refcount(unimplemented!(), 3_isize);
                let _to_find: BidirectionalIterator<(Value<i32>, Value<i32>)> = unimplemented!();
                iter_in_range_12(_first, _last, _to_find)
            })
        );
        assert!(unimplemented!() == unimplemented!());
    }
    {
        {
            let ar: Value<Box<[(Value<i32>, Value<f64>)]>> = Rc::new(RefCell::new(Box::new([
                (Rc::new(RefCell::new(5)), Rc::new(RefCell::new((1 as f64)))),
                (Rc::new(RefCell::new(5)), Rc::new(RefCell::new((2 as f64)))),
                (Rc::new(RefCell::new(5)), Rc::new(RefCell::new((3 as f64)))),
                (Rc::new(RefCell::new(7)), Rc::new(RefCell::new((1 as f64)))),
                (Rc::new(RefCell::new(7)), Rc::new(RefCell::new((2 as f64)))),
                (Rc::new(RefCell::new(7)), Rc::new(RefCell::new((3 as f64)))),
                (Rc::new(RefCell::new(9)), Rc::new(RefCell::new((1 as f64)))),
                (Rc::new(RefCell::new(9)), Rc::new(RefCell::new((2 as f64)))),
                (Rc::new(RefCell::new(9)), Rc::new(RefCell::new((3 as f64)))),
            ])));
            let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            assert!(
                ({
                    let _first: BidirectionalIterator<(Value<i32>, Value<f64>)> =
                        libcc2rs::next_refcount(unimplemented!(), 0_isize);
                    let _last: BidirectionalIterator<(Value<i32>, Value<f64>)> =
                        libcc2rs::next_refcount(unimplemented!(), 3_isize);
                    iter_in_range_13(_first, _last, (*r.borrow()).clone())
                })
            );
            (*r.borrow_mut()) = unimplemented!().clone();
            assert!((*r.borrow()) == unimplemented!());
            (*r.borrow_mut()) = unimplemented!().clone();
            assert!(
                ({
                    let _first: BidirectionalIterator<(Value<i32>, Value<f64>)> =
                        libcc2rs::next_refcount(unimplemented!(), 3_isize);
                    let _last: BidirectionalIterator<(Value<i32>, Value<f64>)> =
                        libcc2rs::next_refcount(unimplemented!(), 6_isize);
                    iter_in_range_13(_first, _last, (*r.borrow()).clone())
                })
            );
            (*r.borrow_mut()) = unimplemented!().clone();
            assert!((*r.borrow()) == unimplemented!());
            (*r.borrow_mut()) = unimplemented!().clone();
            assert!(
                ({
                    let _first: BidirectionalIterator<(Value<i32>, Value<f64>)> =
                        libcc2rs::next_refcount(unimplemented!(), 6_isize);
                    let _last: BidirectionalIterator<(Value<i32>, Value<f64>)> =
                        libcc2rs::next_refcount(unimplemented!(), 9_isize);
                    iter_in_range_13(_first, _last, (*r.borrow()).clone())
                })
            );
            (*r.borrow_mut()) = unimplemented!().clone();
            assert!((*r.borrow()) == unimplemented!());
        }
        {
            let ar: Value<Box<[(Value<i32>, Value<f64>)]>> = Rc::new(RefCell::new(Box::new([
                (Rc::new(RefCell::new(5)), Rc::new(RefCell::new((1 as f64)))),
                (Rc::new(RefCell::new(5)), Rc::new(RefCell::new((2 as f64)))),
                (Rc::new(RefCell::new(5)), Rc::new(RefCell::new((3 as f64)))),
                (Rc::new(RefCell::new(7)), Rc::new(RefCell::new((1 as f64)))),
                (Rc::new(RefCell::new(7)), Rc::new(RefCell::new((2 as f64)))),
                (Rc::new(RefCell::new(7)), Rc::new(RefCell::new((3 as f64)))),
                (Rc::new(RefCell::new(9)), Rc::new(RefCell::new((1 as f64)))),
                (Rc::new(RefCell::new(9)), Rc::new(RefCell::new((2 as f64)))),
                (Rc::new(RefCell::new(9)), Rc::new(RefCell::new((3 as f64)))),
            ])));
            let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            assert!(
                ({
                    let _first: BidirectionalIterator<(Value<i32>, Value<f64>)> =
                        libcc2rs::next_refcount(unimplemented!(), 0_isize);
                    let _last: BidirectionalIterator<(Value<i32>, Value<f64>)> =
                        libcc2rs::next_refcount(unimplemented!(), 3_isize);
                    iter_in_range_14(_first, _last, (*r.borrow()).clone())
                })
            );
            (*r.borrow_mut()) = unimplemented!().clone();
            assert!((*r.borrow()) == unimplemented!());
            (*r.borrow_mut()) = unimplemented!().clone();
            assert!(
                ({
                    let _first: BidirectionalIterator<(Value<i32>, Value<f64>)> =
                        libcc2rs::next_refcount(unimplemented!(), 3_isize);
                    let _last: BidirectionalIterator<(Value<i32>, Value<f64>)> =
                        libcc2rs::next_refcount(unimplemented!(), 6_isize);
                    iter_in_range_14(_first, _last, (*r.borrow()).clone())
                })
            );
            (*r.borrow_mut()) = unimplemented!().clone();
            assert!((*r.borrow()) == unimplemented!());
            (*r.borrow_mut()) = unimplemented!().clone();
            assert!(
                ({
                    let _first: BidirectionalIterator<(Value<i32>, Value<f64>)> =
                        libcc2rs::next_refcount(unimplemented!(), 6_isize);
                    let _last: BidirectionalIterator<(Value<i32>, Value<f64>)> =
                        libcc2rs::next_refcount(unimplemented!(), 9_isize);
                    iter_in_range_14(_first, _last, (*r.borrow()).clone())
                })
            );
            (*r.borrow_mut()) = unimplemented!().clone();
            assert!((*r.borrow()) == unimplemented!());
        }
    }
    {
        let ar: Value<Box<[(Value<i32>, Value<f64>)]>> = Rc::new(RefCell::new(Box::new([
            (Rc::new(RefCell::new(5)), Rc::new(RefCell::new((1 as f64)))),
            (Rc::new(RefCell::new(5)), Rc::new(RefCell::new((2 as f64)))),
            (Rc::new(RefCell::new(5)), Rc::new(RefCell::new((3 as f64)))),
            (Rc::new(RefCell::new(7)), Rc::new(RefCell::new((1 as f64)))),
            (Rc::new(RefCell::new(7)), Rc::new(RefCell::new((2 as f64)))),
            (Rc::new(RefCell::new(7)), Rc::new(RefCell::new((3 as f64)))),
            (Rc::new(RefCell::new(9)), Rc::new(RefCell::new((1 as f64)))),
            (Rc::new(RefCell::new(9)), Rc::new(RefCell::new((2 as f64)))),
            (Rc::new(RefCell::new(9)), Rc::new(RefCell::new((3 as f64)))),
        ])));
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            ({
                let _first: BidirectionalIterator<(Value<i32>, Value<f64>)> =
                    libcc2rs::next_refcount(unimplemented!(), 0_isize);
                let _last: BidirectionalIterator<(Value<i32>, Value<f64>)> =
                    libcc2rs::next_refcount(unimplemented!(), 3_isize);
                iter_in_range_10(_first, _last, (*r.borrow()).clone())
            })
        );
        (*r.borrow_mut()) = unimplemented!();
        assert!((*r.borrow()) == unimplemented!());
        (*r.borrow_mut()) = unimplemented!();
        assert!(
            ({
                let _first: BidirectionalIterator<(Value<i32>, Value<f64>)> =
                    libcc2rs::next_refcount(unimplemented!(), 3_isize);
                let _last: BidirectionalIterator<(Value<i32>, Value<f64>)> =
                    libcc2rs::next_refcount(unimplemented!(), 6_isize);
                iter_in_range_10(_first, _last, (*r.borrow()).clone())
            })
        );
        (*r.borrow_mut()) = unimplemented!();
        assert!((*r.borrow()) == unimplemented!());
        (*r.borrow_mut()) = unimplemented!();
        assert!(
            ({
                let _first: BidirectionalIterator<(Value<i32>, Value<f64>)> =
                    libcc2rs::next_refcount(unimplemented!(), 6_isize);
                let _last: BidirectionalIterator<(Value<i32>, Value<f64>)> =
                    libcc2rs::next_refcount(unimplemented!(), 9_isize);
                iter_in_range_10(_first, _last, (*r.borrow()).clone())
            })
        );
        (*r.borrow_mut()) = unimplemented!();
        assert!((*r.borrow()) == unimplemented!());
        (*r.borrow_mut()) = unimplemented!();
        assert!(
            ({
                let _first: BidirectionalIterator<(Value<i32>, Value<f64>)> =
                    libcc2rs::next_refcount(unimplemented!(), 0_isize);
                let _last: BidirectionalIterator<(Value<i32>, Value<f64>)> =
                    libcc2rs::next_refcount(unimplemented!(), 3_isize);
                iter_in_range_10(_first, _last, (*r.borrow()).clone())
            })
        );
        (*r.borrow_mut()) = unimplemented!();
        assert!((*r.borrow()) == unimplemented!());
        (*r.borrow_mut()) = unimplemented!();
        assert!(
            ({
                let _first: BidirectionalIterator<(Value<i32>, Value<f64>)> =
                    libcc2rs::next_refcount(unimplemented!(), 3_isize);
                let _last: BidirectionalIterator<(Value<i32>, Value<f64>)> =
                    libcc2rs::next_refcount(unimplemented!(), 6_isize);
                iter_in_range_10(_first, _last, (*r.borrow()).clone())
            })
        );
        (*r.borrow_mut()) = unimplemented!();
        assert!((*r.borrow()) == unimplemented!());
        (*r.borrow_mut()) = unimplemented!();
        assert!(
            ({
                let _first: BidirectionalIterator<(Value<i32>, Value<f64>)> =
                    libcc2rs::next_refcount(unimplemented!(), 6_isize);
                let _last: BidirectionalIterator<(Value<i32>, Value<f64>)> =
                    libcc2rs::next_refcount(unimplemented!(), 9_isize);
                iter_in_range_10(_first, _last, (*r.borrow()).clone())
            })
        );
        (*r.borrow_mut()) = unimplemented!();
        assert!((*r.borrow()) == unimplemented!());
    }
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            ({
                let _first: BidirectionalIterator<(Value<PrivateConstructor>, Value<f64>)> =
                    libcc2rs::next_refcount(unimplemented!(), 0_isize);
                let _last: BidirectionalIterator<(Value<PrivateConstructor>, Value<f64>)> =
                    libcc2rs::next_refcount(unimplemented!(), 3_isize);
                iter_in_range_15(_first, _last, (*r.borrow()).clone())
            })
        );
        (*r.borrow_mut()) = unimplemented!();
        assert!((*r.borrow()) == unimplemented!());
        (*r.borrow_mut()) = unimplemented!();
        assert!(
            ({
                let _first: BidirectionalIterator<(Value<PrivateConstructor>, Value<f64>)> =
                    libcc2rs::next_refcount(unimplemented!(), 3_isize);
                let _last: BidirectionalIterator<(Value<PrivateConstructor>, Value<f64>)> =
                    libcc2rs::next_refcount(unimplemented!(), 6_isize);
                iter_in_range_15(_first, _last, (*r.borrow()).clone())
            })
        );
        (*r.borrow_mut()) = unimplemented!();
        assert!((*r.borrow()) == unimplemented!());
        (*r.borrow_mut()) = unimplemented!();
        assert!(
            ({
                let _first: BidirectionalIterator<(Value<PrivateConstructor>, Value<f64>)> =
                    libcc2rs::next_refcount(unimplemented!(), 6_isize);
                let _last: BidirectionalIterator<(Value<PrivateConstructor>, Value<f64>)> =
                    libcc2rs::next_refcount(unimplemented!(), 9_isize);
                iter_in_range_15(_first, _last, (*r.borrow()).clone())
            })
        );
        (*r.borrow_mut()) = unimplemented!();
        assert!((*r.borrow()) == unimplemented!());
    }
    return true;
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
    ({ test_16() });
    return 0;
}
pub trait C2IntImpl {
    fn get(&self) -> i32;
}
impl C2IntImpl for Ptr<C2Int> {
    fn get(&self) -> i32 {
        return (*self).with(|__s| __s.i_);
    }
}
pub trait PrivateConstructorImpl {
    fn get(&self) -> i32;
}
impl PrivateConstructorImpl for Ptr<PrivateConstructor> {
    fn get(&self) -> i32 {
        return (*self).with(|__s| __s.val);
    }
}
pub trait min_allocator_std_pair_const_int__double__Impl {
    fn allocate(
        &self,
        mut n: usize,
    ) -> min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn deallocate(
        &self,
        mut p: min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        unimplemented!()
    }
}
impl min_allocator_std_pair_const_int__double__Impl
    for Ptr<min_allocator_std_pair_const_int__double__>
{
}
pub trait min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__Impl {
    fn to_bool(&self) -> bool {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<(Value<i32>, Value<f64>)> {
        unimplemented!()
    }
    fn operator_arrow(&self) -> Ptr<(Value<i32>, Value<f64>)> {
        unimplemented!()
    }
    fn operator_inc_8(
        &self,
    ) -> Ptr<min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__>
    {
        unimplemented!()
    }
    fn operator_post_inc_9(
        &self,
        mut _a0: i32,
    ) -> min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_dec_10(
        &self,
    ) -> Ptr<min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__>
    {
        unimplemented!()
    }
    fn operator_post_dec_11(
        &self,
        mut _a0: i32,
    ) -> min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_add_assign(
        &self,
        mut n: isize,
    ) -> Ptr<min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__>
    {
        unimplemented!()
    }
    fn operator_sub_assign(
        &self,
        mut n: isize,
    ) -> Ptr<min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__>
    {
        unimplemented!()
    }
    fn operator_add(
        &self,
        mut n: isize,
    ) -> min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_sub(
        &self,
        mut n: isize,
    ) -> min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__ {
        unimplemented!()
    }
    fn operator_index(&self, mut n: isize) -> Ptr<(Value<i32>, Value<f64>)> {
        unimplemented!()
    }
}
impl min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__>
{
}
pub trait min_pointer_void__std_integral_constant_unsigned_long__0__Impl {
    fn to_bool(&self) -> bool {
        unimplemented!()
    }
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__Impl
    for Ptr<min_pointer_void__std_integral_constant_unsigned_long__0__>
{
}
pub fn __cpp2rust_init_globals() {
    let _ = outstanding_bytes_0.with(|_| ());
    let _ = alloc_count_1.with(|_| ());
    let _ = dealloc_count_2.with(|_| ());
    let _ = disable_default_constructor_3.with(|_| ());
}
