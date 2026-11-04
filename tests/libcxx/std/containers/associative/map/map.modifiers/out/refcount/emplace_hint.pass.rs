extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign)]
#[byte_size(16)]
pub struct Emplaceable {
    #[offset(0)]
    int_: i32,
    #[offset(8)]
    double_: f64,
}
impl Emplaceable {
    pub fn new_1() -> Self {
        Self {
            int_: 0,
            double_: 0_f64,
        }
    }
    pub fn new_2(mut i: i32, mut d: f64) -> Self {
        Self {
            int_: i,
            double_: d,
        }
    }
    pub fn move_from(x: Ptr<Emplaceable>) -> Self {
        let __this: Emplaceable = Self {
            int_: x.with(|__s| __s.int_),
            double_: x.with(|__s| __s.double_),
        };
        field!(x, int_).write(0);
        field!(x, double_).write(0_f64);
        __this
    }
}
impl CppLess for Emplaceable {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        EmplaceableImpl::operator_lt(__lhs, __rhs.clone())
    }
}
impl CppEq for Emplaceable {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        EmplaceableImpl::operator_eq(__lhs, __rhs.clone())
    }
}
impl CppMove for Emplaceable {
    fn cpp_move(src: Ptr<Self>) -> Self {
        Self::move_from(src)
    }
}
impl Default for Emplaceable {
    fn default() -> Self {
        { Emplaceable::new_1() }
    }
}
pub fn operator_eq_1(x: Ptr<DefaultOnly>, y: Ptr<DefaultOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } == { y.with(|__s| __s.data_) });
}
pub fn operator_lt_2(x: Ptr<DefaultOnly>, y: Ptr<DefaultOnly>) -> bool {
    return ({ x.with(|__s| __s.data_) } < { y.with(|__s| __s.data_) });
}
#[derive(Record, ByteRepr, FnPtrArg, Destructor)]
#[byte_size(4)]
pub struct DefaultOnly {
    #[offset(0)]
    data_: i32,
}
impl DefaultOnly {
    pub fn new() -> Self {
        let __this: DefaultOnly = Self { data_: -1_i32 };
        (*count_0.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
}
impl CppLess for DefaultOnly {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_2(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for DefaultOnly {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_1(__lhs.clone(), __rhs.clone())
    }
}
impl Default for DefaultOnly {
    fn default() -> Self {
        { DefaultOnly::new() }
    }
}
thread_local!(
    pub static count_0: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub fn outstanding_alloc() -> usize {
        assert!((alloc_count_4.with(|rc| *rc.borrow()) >= dealloc_count_5.with(|rc| *rc.borrow())));
        return ((alloc_count_4.with(|rc| *rc.borrow()))
            .wrapping_sub(dealloc_count_5.with(|rc| *rc.borrow())));
    }
    pub fn reset() {
        assert!((({ malloc_allocator_base::outstanding_alloc() }) == 0_usize));
        disable_default_constructor_6.with(|rc| *rc.borrow_mut() = false);
        outstanding_bytes_3.with(|rc| *rc.borrow_mut() = 0_usize);
        alloc_count_4.with(|rc| *rc.borrow_mut() = 0_usize);
        dealloc_count_5.with(|rc| *rc.borrow_mut() = 0_usize);
    }
}
impl CppMove for malloc_allocator_base {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
thread_local!(
    pub static outstanding_bytes_3: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static alloc_count_4: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static dealloc_count_5: Value<usize> = Rc::new(RefCell::new(0_usize));
);
thread_local!(
    pub static disable_default_constructor_6: Value<bool> = Rc::new(RefCell::new(false));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_std_pair_const_int__DefaultOnly__ {}
impl CppMove for min_allocator_std_pair_const_int__DefaultOnly__ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_std_pair_const_int__Emplaceable__ {}
impl CppMove for min_allocator_std_pair_const_int__Emplaceable__ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct min_allocator_std_pair_const_int__double__ {}
impl CppMove for min_allocator_std_pair_const_int__double__ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn test_7() -> bool {
    {
        {
            let m: Value<Map<i32, (Value<i32>, Value<DefaultOnly>)>> =
                Rc::new(RefCell::new(Map::new()));
            assert!((count_0.with(|rc| *rc.borrow()) == 0));
            let r: Value<BidirectionalIterator<(Value<i32>, Value<DefaultOnly>)>> =
                Rc::new(RefCell::new({
                    let _ = <BidirectionalIterator<(Value<i32>, Value<DefaultOnly>)>>::from(
                        m.as_pointer()
                            .with(|__m: &Map<i32, (Value<i32>, Value<DefaultOnly>)>| __m.end()),
                    );
                    let __init = (
                        Rc::new(RefCell::new(0_i32)),
                        Rc::new(RefCell::new(<DefaultOnly>::default())),
                    );
                    let __key = __init.0.as_pointer();
                    if m.as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<DefaultOnly>)>| {
                            __m.get(&__key).is_none()
                        })
                    {
                        Map::subscript(m.as_pointer().clone(), __key.clone())
                            .write(__init.1.borrow().clone());
                    }
                    m.as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<DefaultOnly>)>| __m.find(&__key))
                }));
            assert!(
                (*r.borrow())
                    == m.as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<DefaultOnly>)>| __m.begin())
            );
            assert!(({ ((*m.borrow()).len() == 1_usize) }));
            assert!(
                ((*(*m
                    .as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<DefaultOnly>)>| __m.begin())
                    .iter_deref()
                    .upgrade()
                    .deref())
                .0
                .borrow())
                    == 0)
            );
            assert!(
                ({
                    let _x: Ptr<DefaultOnly> = (*m
                        .as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<DefaultOnly>)>| __m.begin())
                        .iter_deref()
                        .upgrade()
                        .deref())
                    .1
                    .as_pointer();
                    let _y: Value<DefaultOnly> = Rc::new(RefCell::new(DefaultOnly::new()));
                    operator_eq_1(_x, _y.as_pointer())
                })
            );
            assert!((count_0.with(|rc| *rc.borrow()) == 1));
            (*r.borrow_mut()) = {
                let _ = <BidirectionalIterator<(Value<i32>, Value<DefaultOnly>)>>::from(
                    m.as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<DefaultOnly>)>| __m.end()),
                );
                let __init = std_pair_const_int__DefaultOnly_::new_1(
                    { (*piecewise_construct_8.with(Value::clone).borrow()).clone() },
                    {
                        ({
                            let ___args: Value<i32> = Rc::new(RefCell::new(1));
                            forward_as_tuple_9(___args.as_pointer())
                        })
                    },
                    { ({ forward_as_tuple_10() }) },
                );
                let __key = __init.0.as_pointer();
                if m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<DefaultOnly>)>| {
                        __m.get(&__key).is_none()
                    })
                {
                    Map::subscript(m.as_pointer().clone(), __key.clone())
                        .write(__init.1.borrow().clone());
                }
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<DefaultOnly>)>| __m.find(&__key))
            };
            assert!(
                (*r.borrow())
                    == libcc2rs::next_refcount(
                        m.as_pointer()
                            .with(|__m: &Map<i32, (Value<i32>, Value<DefaultOnly>)>| __m.begin()),
                        1_isize
                    )
            );
            assert!(({ ((*m.borrow()).len() == 2_usize) }));
            assert!(
                ((*(*libcc2rs::next_refcount(
                    m.as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<DefaultOnly>)>| __m.begin()),
                    1_isize
                )
                .iter_deref()
                .upgrade()
                .deref())
                .0
                .borrow())
                    == 1)
            );
            assert!(
                ({
                    let _x: Ptr<DefaultOnly> = (*libcc2rs::next_refcount(
                        m.as_pointer()
                            .with(|__m: &Map<i32, (Value<i32>, Value<DefaultOnly>)>| __m.begin()),
                        1_isize,
                    )
                    .iter_deref()
                    .upgrade()
                    .deref())
                    .1
                    .as_pointer();
                    let _y: Value<DefaultOnly> = Rc::new(RefCell::new(DefaultOnly::new()));
                    operator_eq_1(_x, _y.as_pointer())
                })
            );
            assert!((count_0.with(|rc| *rc.borrow()) == 2));
            (*r.borrow_mut()) = {
                let _ = <BidirectionalIterator<(Value<i32>, Value<DefaultOnly>)>>::from(
                    m.as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<DefaultOnly>)>| __m.end()),
                );
                let __init = std_pair_const_int__DefaultOnly_::new_1(
                    { (*piecewise_construct_8.with(Value::clone).borrow()).clone() },
                    {
                        ({
                            let ___args: Value<i32> = Rc::new(RefCell::new(1));
                            forward_as_tuple_9(___args.as_pointer())
                        })
                    },
                    { ({ forward_as_tuple_10() }) },
                );
                let __key = __init.0.as_pointer();
                if m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<DefaultOnly>)>| {
                        __m.get(&__key).is_none()
                    })
                {
                    Map::subscript(m.as_pointer().clone(), __key.clone())
                        .write(__init.1.borrow().clone());
                }
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<DefaultOnly>)>| __m.find(&__key))
            };
            assert!(
                (*r.borrow())
                    == libcc2rs::next_refcount(
                        m.as_pointer()
                            .with(|__m: &Map<i32, (Value<i32>, Value<DefaultOnly>)>| __m.begin()),
                        1_isize
                    )
            );
            assert!(({ ((*m.borrow()).len() == 2_usize) }));
            assert!(
                ((*(*libcc2rs::next_refcount(
                    m.as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<DefaultOnly>)>| __m.begin()),
                    1_isize
                )
                .iter_deref()
                .upgrade()
                .deref())
                .0
                .borrow())
                    == 1)
            );
            assert!(
                ({
                    let _x: Ptr<DefaultOnly> = (*libcc2rs::next_refcount(
                        m.as_pointer()
                            .with(|__m: &Map<i32, (Value<i32>, Value<DefaultOnly>)>| __m.begin()),
                        1_isize,
                    )
                    .iter_deref()
                    .upgrade()
                    .deref())
                    .1
                    .as_pointer();
                    let _y: Value<DefaultOnly> = Rc::new(RefCell::new(DefaultOnly::new()));
                    operator_eq_1(_x, _y.as_pointer())
                })
            );
            assert!((count_0.with(|rc| *rc.borrow()) == 2));
        }
        assert!((count_0.with(|rc| *rc.borrow()) == 0));
    }
    {
        let m: Value<Map<i32, (Value<i32>, Value<Emplaceable>)>> =
            Rc::new(RefCell::new(Map::new()));
        let r: Value<BidirectionalIterator<(Value<i32>, Value<Emplaceable>)>> =
            Rc::new(RefCell::new({
                let _ = <BidirectionalIterator<(Value<i32>, Value<Emplaceable>)>>::from(
                    m.as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<Emplaceable>)>| __m.end()),
                );
                let __init = std_pair_const_int__Emplaceable_::new_1(
                    { (*piecewise_construct_8.with(Value::clone).borrow()).clone() },
                    {
                        ({
                            let ___args: Value<i32> = Rc::new(RefCell::new(2));
                            forward_as_tuple_9(___args.as_pointer())
                        })
                    },
                    { ({ forward_as_tuple_10() }) },
                );
                let __key = __init.0.as_pointer();
                if m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<Emplaceable>)>| {
                        __m.get(&__key).is_none()
                    })
                {
                    Map::subscript(m.as_pointer().clone(), __key.clone())
                        .write(__init.1.borrow().clone());
                }
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<Emplaceable>)>| __m.find(&__key))
            }));
        assert!(
            (*r.borrow())
                == m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<Emplaceable>)>| __m.begin())
        );
        assert!(({ ((*m.borrow()).len() == 1_usize) }));
        assert!(
            ((*(*m
                .as_pointer()
                .with(|__m: &Map<i32, (Value<i32>, Value<Emplaceable>)>| __m.begin())
                .iter_deref()
                .upgrade()
                .deref())
            .0
            .borrow())
                == 2)
        );
        assert!(
            ({
                let _x: Value<Emplaceable> = Rc::new(RefCell::new(Emplaceable::new_1()));
                EmplaceableImpl::operator_eq(
                    &(*m.as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<Emplaceable>)>| __m.begin())
                        .iter_deref()
                        .upgrade()
                        .deref())
                    .1
                    .as_pointer(),
                    _x.as_pointer(),
                )
            })
        );
        (*r.borrow_mut()) = {
            let _ = <BidirectionalIterator<(Value<i32>, Value<Emplaceable>)>>::from(
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<Emplaceable>)>| __m.end()),
            );
            let __init = std_pair_const_int__Emplaceable_::new_2(
                { (*piecewise_construct_8.with(Value::clone).borrow()).clone() },
                {
                    ({
                        let ___args: Value<i32> = Rc::new(RefCell::new(1));
                        forward_as_tuple_9(___args.as_pointer())
                    })
                },
                {
                    ({
                        let ___args_0: Value<i32> = Rc::new(RefCell::new(2));
                        let ___args_1: Value<f64> = Rc::new(RefCell::new(3.5_f64));
                        forward_as_tuple_11(___args_0.as_pointer(), ___args_1.as_pointer())
                    })
                },
            );
            let __key = __init.0.as_pointer();
            if m.as_pointer()
                .with(|__m: &Map<i32, (Value<i32>, Value<Emplaceable>)>| __m.get(&__key).is_none())
            {
                Map::subscript(m.as_pointer().clone(), __key.clone())
                    .write(__init.1.borrow().clone());
            }
            m.as_pointer()
                .with(|__m: &Map<i32, (Value<i32>, Value<Emplaceable>)>| __m.find(&__key))
        };
        assert!(
            (*r.borrow())
                == m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<Emplaceable>)>| __m.begin())
        );
        assert!(({ ((*m.borrow()).len() == 2_usize) }));
        assert!(
            ((*(*m
                .as_pointer()
                .with(|__m: &Map<i32, (Value<i32>, Value<Emplaceable>)>| __m.begin())
                .iter_deref()
                .upgrade()
                .deref())
            .0
            .borrow())
                == 1)
        );
        assert!(
            ({
                let _x: Value<Emplaceable> =
                    Rc::new(RefCell::new(Emplaceable::new_2({ 2 }, { 3.5_f64 })));
                EmplaceableImpl::operator_eq(
                    &(*m.as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<Emplaceable>)>| __m.begin())
                        .iter_deref()
                        .upgrade()
                        .deref())
                    .1
                    .as_pointer(),
                    _x.as_pointer(),
                )
            })
        );
        (*r.borrow_mut()) = {
            let _ = <BidirectionalIterator<(Value<i32>, Value<Emplaceable>)>>::from(
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<Emplaceable>)>| __m.end()),
            );
            let __init = std_pair_const_int__Emplaceable_::new_2(
                { (*piecewise_construct_8.with(Value::clone).borrow()).clone() },
                {
                    ({
                        let ___args: Value<i32> = Rc::new(RefCell::new(1));
                        forward_as_tuple_9(___args.as_pointer())
                    })
                },
                {
                    ({
                        let ___args_0: Value<i32> = Rc::new(RefCell::new(2));
                        let ___args_1: Value<f64> = Rc::new(RefCell::new(3.5_f64));
                        forward_as_tuple_11(___args_0.as_pointer(), ___args_1.as_pointer())
                    })
                },
            );
            let __key = __init.0.as_pointer();
            if m.as_pointer()
                .with(|__m: &Map<i32, (Value<i32>, Value<Emplaceable>)>| __m.get(&__key).is_none())
            {
                Map::subscript(m.as_pointer().clone(), __key.clone())
                    .write(__init.1.borrow().clone());
            }
            m.as_pointer()
                .with(|__m: &Map<i32, (Value<i32>, Value<Emplaceable>)>| __m.find(&__key))
        };
        assert!(
            (*r.borrow())
                == m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<Emplaceable>)>| __m.begin())
        );
        assert!(({ ((*m.borrow()).len() == 2_usize) }));
        assert!(
            ((*(*m
                .as_pointer()
                .with(|__m: &Map<i32, (Value<i32>, Value<Emplaceable>)>| __m.begin())
                .iter_deref()
                .upgrade()
                .deref())
            .0
            .borrow())
                == 1)
        );
        assert!(
            ({
                let _x: Value<Emplaceable> =
                    Rc::new(RefCell::new(Emplaceable::new_2({ 2 }, { 3.5_f64 })));
                EmplaceableImpl::operator_eq(
                    &(*m.as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<Emplaceable>)>| __m.begin())
                        .iter_deref()
                        .upgrade()
                        .deref())
                    .1
                    .as_pointer(),
                    _x.as_pointer(),
                )
            })
        );
    }
    {
        let m: Value<Map<i32, (Value<i32>, Value<f64>)>> = Rc::new(RefCell::new(Map::new()));
        let r: Value<BidirectionalIterator<(Value<i32>, Value<f64>)>> = Rc::new(RefCell::new({
            let _ = <BidirectionalIterator<(Value<i32>, Value<f64>)>>::from(
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.end()),
            );
            let __init = (Rc::new(RefCell::new(2)), Rc::new(RefCell::new(3.5_f64)));
            let __key = __init.0.as_pointer();
            if m.as_pointer()
                .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.get(&__key).is_none())
            {
                Map::subscript(m.as_pointer().clone(), __key.clone())
                    .write(__init.1.borrow().clone());
            }
            m.as_pointer()
                .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.find(&__key))
        }));
        assert!(
            (*r.borrow())
                == m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin())
        );
        assert!(({ ((*m.borrow()).len() == 1_usize) }));
        assert!(
            ((*(*m
                .as_pointer()
                .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin())
                .iter_deref()
                .upgrade()
                .deref())
            .0
            .borrow())
                == 2)
        );
        assert!(
            ((*(*m
                .as_pointer()
                .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin())
                .iter_deref()
                .upgrade()
                .deref())
            .1
            .borrow())
                == 3.5_f64)
        );
    }
    {
        {
            let m: Value<Map<i32, (Value<i32>, Value<DefaultOnly>)>> =
                Rc::new(RefCell::new(Map::new()));
            assert!((count_0.with(|rc| *rc.borrow()) == 0));
            let r: Value<BidirectionalIterator<(Value<i32>, Value<DefaultOnly>)>> =
                Rc::new(RefCell::new({
                    let _ = <BidirectionalIterator<(Value<i32>, Value<DefaultOnly>)>>::from(
                        m.as_pointer()
                            .with(|__m: &Map<i32, (Value<i32>, Value<DefaultOnly>)>| __m.end()),
                    );
                    let __init = (
                        Rc::new(RefCell::new(0_i32)),
                        Rc::new(RefCell::new(<DefaultOnly>::default())),
                    );
                    let __key = __init.0.as_pointer();
                    if m.as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<DefaultOnly>)>| {
                            __m.get(&__key).is_none()
                        })
                    {
                        Map::subscript(m.as_pointer().clone(), __key.clone())
                            .write(__init.1.borrow().clone());
                    }
                    m.as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<DefaultOnly>)>| __m.find(&__key))
                }));
            assert!(
                (*r.borrow())
                    == m.as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<DefaultOnly>)>| __m.begin())
            );
            assert!(({ ((*m.borrow()).len() == 1_usize) }));
            assert!(
                ((*(*m
                    .as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<DefaultOnly>)>| __m.begin())
                    .iter_deref()
                    .upgrade()
                    .deref())
                .0
                .borrow())
                    == 0)
            );
            assert!(
                ({
                    let _x: Ptr<DefaultOnly> = (*m
                        .as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<DefaultOnly>)>| __m.begin())
                        .iter_deref()
                        .upgrade()
                        .deref())
                    .1
                    .as_pointer();
                    let _y: Value<DefaultOnly> = Rc::new(RefCell::new(DefaultOnly::new()));
                    operator_eq_1(_x, _y.as_pointer())
                })
            );
            assert!((count_0.with(|rc| *rc.borrow()) == 1));
            (*r.borrow_mut()) = {
                let _ = <BidirectionalIterator<(Value<i32>, Value<DefaultOnly>)>>::from(
                    m.as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<DefaultOnly>)>| __m.end()),
                );
                let __init = std_pair_const_int__DefaultOnly_::new_1(
                    { (*piecewise_construct_8.with(Value::clone).borrow()).clone() },
                    {
                        ({
                            let ___args: Value<i32> = Rc::new(RefCell::new(1));
                            forward_as_tuple_9(___args.as_pointer())
                        })
                    },
                    { ({ forward_as_tuple_10() }) },
                );
                let __key = __init.0.as_pointer();
                if m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<DefaultOnly>)>| {
                        __m.get(&__key).is_none()
                    })
                {
                    Map::subscript(m.as_pointer().clone(), __key.clone())
                        .write(__init.1.borrow().clone());
                }
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<DefaultOnly>)>| __m.find(&__key))
            }
            .clone();
            assert!(
                (*r.borrow())
                    == libcc2rs::next_refcount(
                        m.as_pointer()
                            .with(|__m: &Map<i32, (Value<i32>, Value<DefaultOnly>)>| __m.begin()),
                        1_isize
                    )
            );
            assert!(({ ((*m.borrow()).len() == 2_usize) }));
            assert!(
                ((*(*libcc2rs::next_refcount(
                    m.as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<DefaultOnly>)>| __m.begin()),
                    1_isize
                )
                .iter_deref()
                .upgrade()
                .deref())
                .0
                .borrow())
                    == 1)
            );
            assert!(
                ({
                    let _x: Ptr<DefaultOnly> = (*libcc2rs::next_refcount(
                        m.as_pointer()
                            .with(|__m: &Map<i32, (Value<i32>, Value<DefaultOnly>)>| __m.begin()),
                        1_isize,
                    )
                    .iter_deref()
                    .upgrade()
                    .deref())
                    .1
                    .as_pointer();
                    let _y: Value<DefaultOnly> = Rc::new(RefCell::new(DefaultOnly::new()));
                    operator_eq_1(_x, _y.as_pointer())
                })
            );
            assert!((count_0.with(|rc| *rc.borrow()) == 2));
            (*r.borrow_mut()) = {
                let _ = <BidirectionalIterator<(Value<i32>, Value<DefaultOnly>)>>::from(
                    m.as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<DefaultOnly>)>| __m.end()),
                );
                let __init = std_pair_const_int__DefaultOnly_::new_1(
                    { (*piecewise_construct_8.with(Value::clone).borrow()).clone() },
                    {
                        ({
                            let ___args: Value<i32> = Rc::new(RefCell::new(1));
                            forward_as_tuple_9(___args.as_pointer())
                        })
                    },
                    { ({ forward_as_tuple_10() }) },
                );
                let __key = __init.0.as_pointer();
                if m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<DefaultOnly>)>| {
                        __m.get(&__key).is_none()
                    })
                {
                    Map::subscript(m.as_pointer().clone(), __key.clone())
                        .write(__init.1.borrow().clone());
                }
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<DefaultOnly>)>| __m.find(&__key))
            }
            .clone();
            assert!(
                (*r.borrow())
                    == libcc2rs::next_refcount(
                        m.as_pointer()
                            .with(|__m: &Map<i32, (Value<i32>, Value<DefaultOnly>)>| __m.begin()),
                        1_isize
                    )
            );
            assert!(({ ((*m.borrow()).len() == 2_usize) }));
            assert!(
                ((*(*libcc2rs::next_refcount(
                    m.as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<DefaultOnly>)>| __m.begin()),
                    1_isize
                )
                .iter_deref()
                .upgrade()
                .deref())
                .0
                .borrow())
                    == 1)
            );
            assert!(
                ({
                    let _x: Ptr<DefaultOnly> = (*libcc2rs::next_refcount(
                        m.as_pointer()
                            .with(|__m: &Map<i32, (Value<i32>, Value<DefaultOnly>)>| __m.begin()),
                        1_isize,
                    )
                    .iter_deref()
                    .upgrade()
                    .deref())
                    .1
                    .as_pointer();
                    let _y: Value<DefaultOnly> = Rc::new(RefCell::new(DefaultOnly::new()));
                    operator_eq_1(_x, _y.as_pointer())
                })
            );
            assert!((count_0.with(|rc| *rc.borrow()) == 2));
        }
        assert!((count_0.with(|rc| *rc.borrow()) == 0));
    }
    {
        let m: Value<Map<i32, (Value<i32>, Value<Emplaceable>)>> =
            Rc::new(RefCell::new(Map::new()));
        let r: Value<BidirectionalIterator<(Value<i32>, Value<Emplaceable>)>> =
            Rc::new(RefCell::new({
                let _ = <BidirectionalIterator<(Value<i32>, Value<Emplaceable>)>>::from(
                    m.as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<Emplaceable>)>| __m.end()),
                );
                let __init = std_pair_const_int__Emplaceable_::new_1(
                    { (*piecewise_construct_8.with(Value::clone).borrow()).clone() },
                    {
                        ({
                            let ___args: Value<i32> = Rc::new(RefCell::new(2));
                            forward_as_tuple_9(___args.as_pointer())
                        })
                    },
                    { ({ forward_as_tuple_10() }) },
                );
                let __key = __init.0.as_pointer();
                if m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<Emplaceable>)>| {
                        __m.get(&__key).is_none()
                    })
                {
                    Map::subscript(m.as_pointer().clone(), __key.clone())
                        .write(__init.1.borrow().clone());
                }
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<Emplaceable>)>| __m.find(&__key))
            }));
        assert!(
            (*r.borrow())
                == m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<Emplaceable>)>| __m.begin())
        );
        assert!(({ ((*m.borrow()).len() == 1_usize) }));
        assert!(
            ((*(*m
                .as_pointer()
                .with(|__m: &Map<i32, (Value<i32>, Value<Emplaceable>)>| __m.begin())
                .iter_deref()
                .upgrade()
                .deref())
            .0
            .borrow())
                == 2)
        );
        assert!(
            ({
                let _x: Value<Emplaceable> = Rc::new(RefCell::new(Emplaceable::new_1()));
                EmplaceableImpl::operator_eq(
                    &(*m.as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<Emplaceable>)>| __m.begin())
                        .iter_deref()
                        .upgrade()
                        .deref())
                    .1
                    .as_pointer(),
                    _x.as_pointer(),
                )
            })
        );
        (*r.borrow_mut()) = {
            let _ = <BidirectionalIterator<(Value<i32>, Value<Emplaceable>)>>::from(
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<Emplaceable>)>| __m.end()),
            );
            let __init = std_pair_const_int__Emplaceable_::new_2(
                { (*piecewise_construct_8.with(Value::clone).borrow()).clone() },
                {
                    ({
                        let ___args: Value<i32> = Rc::new(RefCell::new(1));
                        forward_as_tuple_9(___args.as_pointer())
                    })
                },
                {
                    ({
                        let ___args_0: Value<i32> = Rc::new(RefCell::new(2));
                        let ___args_1: Value<f64> = Rc::new(RefCell::new(3.5_f64));
                        forward_as_tuple_11(___args_0.as_pointer(), ___args_1.as_pointer())
                    })
                },
            );
            let __key = __init.0.as_pointer();
            if m.as_pointer()
                .with(|__m: &Map<i32, (Value<i32>, Value<Emplaceable>)>| __m.get(&__key).is_none())
            {
                Map::subscript(m.as_pointer().clone(), __key.clone())
                    .write(__init.1.borrow().clone());
            }
            m.as_pointer()
                .with(|__m: &Map<i32, (Value<i32>, Value<Emplaceable>)>| __m.find(&__key))
        }
        .clone();
        assert!(
            (*r.borrow())
                == m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<Emplaceable>)>| __m.begin())
        );
        assert!(({ ((*m.borrow()).len() == 2_usize) }));
        assert!(
            ((*(*m
                .as_pointer()
                .with(|__m: &Map<i32, (Value<i32>, Value<Emplaceable>)>| __m.begin())
                .iter_deref()
                .upgrade()
                .deref())
            .0
            .borrow())
                == 1)
        );
        assert!(
            ({
                let _x: Value<Emplaceable> =
                    Rc::new(RefCell::new(Emplaceable::new_2({ 2 }, { 3.5_f64 })));
                EmplaceableImpl::operator_eq(
                    &(*m.as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<Emplaceable>)>| __m.begin())
                        .iter_deref()
                        .upgrade()
                        .deref())
                    .1
                    .as_pointer(),
                    _x.as_pointer(),
                )
            })
        );
        (*r.borrow_mut()) = {
            let _ = <BidirectionalIterator<(Value<i32>, Value<Emplaceable>)>>::from(
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<Emplaceable>)>| __m.end()),
            );
            let __init = std_pair_const_int__Emplaceable_::new_2(
                { (*piecewise_construct_8.with(Value::clone).borrow()).clone() },
                {
                    ({
                        let ___args: Value<i32> = Rc::new(RefCell::new(1));
                        forward_as_tuple_9(___args.as_pointer())
                    })
                },
                {
                    ({
                        let ___args_0: Value<i32> = Rc::new(RefCell::new(2));
                        let ___args_1: Value<f64> = Rc::new(RefCell::new(3.5_f64));
                        forward_as_tuple_11(___args_0.as_pointer(), ___args_1.as_pointer())
                    })
                },
            );
            let __key = __init.0.as_pointer();
            if m.as_pointer()
                .with(|__m: &Map<i32, (Value<i32>, Value<Emplaceable>)>| __m.get(&__key).is_none())
            {
                Map::subscript(m.as_pointer().clone(), __key.clone())
                    .write(__init.1.borrow().clone());
            }
            m.as_pointer()
                .with(|__m: &Map<i32, (Value<i32>, Value<Emplaceable>)>| __m.find(&__key))
        }
        .clone();
        assert!(
            (*r.borrow())
                == m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<Emplaceable>)>| __m.begin())
        );
        assert!(({ ((*m.borrow()).len() == 2_usize) }));
        assert!(
            ((*(*m
                .as_pointer()
                .with(|__m: &Map<i32, (Value<i32>, Value<Emplaceable>)>| __m.begin())
                .iter_deref()
                .upgrade()
                .deref())
            .0
            .borrow())
                == 1)
        );
        assert!(
            ({
                let _x: Value<Emplaceable> =
                    Rc::new(RefCell::new(Emplaceable::new_2({ 2 }, { 3.5_f64 })));
                EmplaceableImpl::operator_eq(
                    &(*m.as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<Emplaceable>)>| __m.begin())
                        .iter_deref()
                        .upgrade()
                        .deref())
                    .1
                    .as_pointer(),
                    _x.as_pointer(),
                )
            })
        );
    }
    {
        let m: Value<Map<i32, (Value<i32>, Value<f64>)>> = Rc::new(RefCell::new(Map::new()));
        let r: Value<BidirectionalIterator<(Value<i32>, Value<f64>)>> = Rc::new(RefCell::new({
            let _ = <BidirectionalIterator<(Value<i32>, Value<f64>)>>::from(
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.end()),
            );
            let __init = (Rc::new(RefCell::new(2)), Rc::new(RefCell::new(3.5_f64)));
            let __key = __init.0.as_pointer();
            if m.as_pointer()
                .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.get(&__key).is_none())
            {
                Map::subscript(m.as_pointer().clone(), __key.clone())
                    .write(__init.1.borrow().clone());
            }
            m.as_pointer()
                .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.find(&__key))
        }));
        assert!(
            (*r.borrow())
                == m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin())
        );
        assert!(({ ((*m.borrow()).len() == 1_usize) }));
        assert!(
            ((*(*m
                .as_pointer()
                .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin())
                .iter_deref()
                .upgrade()
                .deref())
            .0
            .borrow())
                == 2)
        );
        assert!(
            ((*(*m
                .as_pointer()
                .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin())
                .iter_deref()
                .upgrade()
                .deref())
            .1
            .borrow())
                == 3.5_f64)
        );
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
    ({ test_7() });
    return 0;
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct min_pointer_std_pair_const_int__Emplaceable___std_integral_constant_unsigned_long__0__;
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__;
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct min_pointer_std_pair_const_int__DefaultOnly___std_integral_constant_unsigned_long__0__;
pub trait DefaultOnlyImpl {
    fn copy_assign(&self, _a0: Ptr<DefaultOnly>) -> Ptr<DefaultOnly> {
        unimplemented!()
    }
    fn destructor(&self);
}
impl DefaultOnlyImpl for Ptr<DefaultOnly> {
    fn destructor(&self) {
        field!((*self), data_).write(0);
        (*count_0.with(Value::clone).borrow_mut()).prefix_dec();
    }
}
pub trait EmplaceableImpl {
    fn copy_assign(&self, _a0: Ptr<Emplaceable>) -> Ptr<Emplaceable> {
        unimplemented!()
    }
    fn move_assign(&self, x: Ptr<Emplaceable>) -> Ptr<Emplaceable>;
    fn operator_eq(&self, x: Ptr<Emplaceable>) -> bool;
    fn operator_lt(&self, x: Ptr<Emplaceable>) -> bool;
    fn get(&self) -> i32;
}
impl EmplaceableImpl for Ptr<Emplaceable> {
    fn move_assign(&self, x: Ptr<Emplaceable>) -> Ptr<Emplaceable> {
        field!((*self), int_).write({ x.with(|__s| __s.int_) });
        field!(x, int_).write(0);
        field!((*self), double_).write({ x.with(|__s| __s.double_) });
        field!(x, double_).write(0_f64);
        return (*self).clone();
    }
    fn operator_eq(&self, x: Ptr<Emplaceable>) -> bool {
        return ({ (*self).with(|__s| __s.int_) } == { x.with(|__s| __s.int_) })
            && ({ (*self).with(|__s| __s.double_) } == { x.with(|__s| __s.double_) });
    }
    fn operator_lt(&self, x: Ptr<Emplaceable>) -> bool {
        return ({ (*self).with(|__s| __s.int_) } < { x.with(|__s| __s.int_) })
            || (({ (*self).with(|__s| __s.int_) } == { x.with(|__s| __s.int_) })
                && ({ (*self).with(|__s| __s.double_) } < { x.with(|__s| __s.double_) }));
    }
    fn get(&self) -> i32 {
        return (*self).with(|__s| __s.int_);
    }
}
pub trait min_allocator_std_pair_const_int__DefaultOnly__Impl {
    fn allocate(
        &self,
        mut n: usize,
    ) -> min_pointer_std_pair_const_int__DefaultOnly___std_integral_constant_unsigned_long__0__
    {
        unimplemented!()
    }
    fn deallocate(
        &self,
        mut p : min_pointer_std_pair_const_int__DefaultOnly___std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        unimplemented!()
    }
}
impl min_allocator_std_pair_const_int__DefaultOnly__Impl
    for Ptr<min_allocator_std_pair_const_int__DefaultOnly__>
{
}
pub trait min_allocator_std_pair_const_int__Emplaceable__Impl {
    fn allocate(
        &self,
        mut n: usize,
    ) -> min_pointer_std_pair_const_int__Emplaceable___std_integral_constant_unsigned_long__0__
    {
        unimplemented!()
    }
    fn deallocate(
        &self,
        mut p : min_pointer_std_pair_const_int__Emplaceable___std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        unimplemented!()
    }
}
impl min_allocator_std_pair_const_int__Emplaceable__Impl
    for Ptr<min_allocator_std_pair_const_int__Emplaceable__>
{
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
pub fn __cpp2rust_init_globals() {
    let _ = count_0.with(|_| ());
    let _ = outstanding_bytes_3.with(|_| ());
    let _ = alloc_count_4.with(|_| ());
    let _ = dealloc_count_5.with(|_| ());
    let _ = disable_default_constructor_6.with(|_| ());
}
