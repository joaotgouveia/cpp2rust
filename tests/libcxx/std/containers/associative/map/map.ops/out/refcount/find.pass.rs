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
impl CppMove for malloc_allocator_base {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
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
#[byte_size(1)]
pub struct min_allocator_std_pair_const_int__double__ {}
impl CppMove for min_allocator_std_pair_const_int__double__ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
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
impl CppMove for PrivateConstructor {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
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
impl CppMove for transparent_less {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct transparent_less_not_referenceable {}
impl CppMove for transparent_less_not_referenceable {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct transparent_less_nonempty_is_transparent {}
impl CppMove for transparent_less_nonempty_is_transparent {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct transparent_less_nonempty {
    #[offset(0)]
    #[byte_size(1)]
    pub pad_: transparent_less_nonempty_is_transparent,
}
impl CppMove for transparent_less_nonempty {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct transparent_less_no_type {}
impl CppMove for transparent_less_no_type {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct transparent_less_private {}
impl CppMove for transparent_less_private {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct transparent_less_not_a_type {
    #[offset(0)]
    pub is_transparent: i32,
}
impl CppMove for transparent_less_not_a_type {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
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
impl CppMove for C2Int {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
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
pub fn test_10() -> bool {
    {
        {
            let ar: Value<Box<[(Value<i32>, Value<f64>)]>> = Rc::new(RefCell::new(Box::new([
                (Rc::new(RefCell::new(5)), Rc::new(RefCell::new((5 as f64)))),
                (Rc::new(RefCell::new(6)), Rc::new(RefCell::new((6 as f64)))),
                (Rc::new(RefCell::new(7)), Rc::new(RefCell::new((7 as f64)))),
                (Rc::new(RefCell::new(8)), Rc::new(RefCell::new((8 as f64)))),
                (Rc::new(RefCell::new(9)), Rc::new(RefCell::new((9 as f64)))),
                (
                    Rc::new(RefCell::new(10)),
                    Rc::new(RefCell::new((10 as f64))),
                ),
                (
                    Rc::new(RefCell::new(11)),
                    Rc::new(RefCell::new((11 as f64))),
                ),
                (
                    Rc::new(RefCell::new(12)),
                    Rc::new(RefCell::new((12 as f64))),
                ),
            ])));
            let m : Value<Map<i32  , (Value<i32  >, Value<f64  >)> > = Rc::new(RefCell::new(std_map_int__double__std_less_int___std_allocator_std_pair_const_int__double___ :: new_1 ( { ( ar .as_pointer()  as Ptr< (Value<i32  >, Value<f64  >)  >   )  } , { ( ar .as_pointer()  as Ptr< (Value<i32  >, Value<f64  >)  >   ) . offset ( ( ( ( 128usize as usize ) ) . wrapping_div ( ( 16usize as usize ) ) ) as isize )  } , )  )) ;
            let r: Value<BidirectionalIterator<(Value<i32>, Value<f64>)>> = Rc::new(RefCell::new(
                ({
                    let __tmp_0: Value<i32> = Rc::new(RefCell::new(5));
                    m.as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| {
                            __m.find(&__tmp_0.as_pointer())
                        })
                }),
            ));
            assert!(
                (*r.borrow())
                    == m.as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin())
            );
            (*r.borrow_mut()) = ({
                let __tmp_1: Value<i32> = Rc::new(RefCell::new(6));
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| {
                        __m.find(&__tmp_1.as_pointer())
                    })
            });
            assert!(
                (*r.borrow())
                    == libcc2rs::next_refcount(
                        m.as_pointer()
                            .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                        1_isize
                    )
            );
            (*r.borrow_mut()) = ({
                let __tmp_2: Value<i32> = Rc::new(RefCell::new(7));
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| {
                        __m.find(&__tmp_2.as_pointer())
                    })
            });
            assert!(
                (*r.borrow())
                    == libcc2rs::next_refcount(
                        m.as_pointer()
                            .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                        2_isize
                    )
            );
            (*r.borrow_mut()) = ({
                let __tmp_3: Value<i32> = Rc::new(RefCell::new(8));
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| {
                        __m.find(&__tmp_3.as_pointer())
                    })
            });
            assert!(
                (*r.borrow())
                    == libcc2rs::next_refcount(
                        m.as_pointer()
                            .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                        3_isize
                    )
            );
            (*r.borrow_mut()) = ({
                let __tmp_4: Value<i32> = Rc::new(RefCell::new(9));
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| {
                        __m.find(&__tmp_4.as_pointer())
                    })
            });
            assert!(
                (*r.borrow())
                    == libcc2rs::next_refcount(
                        m.as_pointer()
                            .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                        4_isize
                    )
            );
            (*r.borrow_mut()) = ({
                let __tmp_5: Value<i32> = Rc::new(RefCell::new(10));
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| {
                        __m.find(&__tmp_5.as_pointer())
                    })
            });
            assert!(
                (*r.borrow())
                    == libcc2rs::next_refcount(
                        m.as_pointer()
                            .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                        5_isize
                    )
            );
            (*r.borrow_mut()) = ({
                let __tmp_6: Value<i32> = Rc::new(RefCell::new(11));
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| {
                        __m.find(&__tmp_6.as_pointer())
                    })
            });
            assert!(
                (*r.borrow())
                    == libcc2rs::next_refcount(
                        m.as_pointer()
                            .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                        6_isize
                    )
            );
            (*r.borrow_mut()) = ({
                let __tmp_7: Value<i32> = Rc::new(RefCell::new(12));
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| {
                        __m.find(&__tmp_7.as_pointer())
                    })
            });
            assert!(
                (*r.borrow())
                    == libcc2rs::next_refcount(
                        m.as_pointer()
                            .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                        7_isize
                    )
            );
            (*r.borrow_mut()) = ({
                let __tmp_8: Value<i32> = Rc::new(RefCell::new(4));
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| {
                        __m.find(&__tmp_8.as_pointer())
                    })
            });
            assert!(
                (*r.borrow())
                    == libcc2rs::next_refcount(
                        m.as_pointer()
                            .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                        8_isize
                    )
            );
        }
        {
            let ar: Value<Box<[(Value<i32>, Value<f64>)]>> = Rc::new(RefCell::new(Box::new([
                (Rc::new(RefCell::new(5)), Rc::new(RefCell::new((5 as f64)))),
                (Rc::new(RefCell::new(6)), Rc::new(RefCell::new((6 as f64)))),
                (Rc::new(RefCell::new(7)), Rc::new(RefCell::new((7 as f64)))),
                (Rc::new(RefCell::new(8)), Rc::new(RefCell::new((8 as f64)))),
                (Rc::new(RefCell::new(9)), Rc::new(RefCell::new((9 as f64)))),
                (
                    Rc::new(RefCell::new(10)),
                    Rc::new(RefCell::new((10 as f64))),
                ),
                (
                    Rc::new(RefCell::new(11)),
                    Rc::new(RefCell::new((11 as f64))),
                ),
                (
                    Rc::new(RefCell::new(12)),
                    Rc::new(RefCell::new((12 as f64))),
                ),
            ])));
            let m : Value<Map<i32  , (Value<i32  >, Value<f64  >)> > = Rc::new(RefCell::new(std_map_int__double__std_less_int___std_allocator_std_pair_const_int__double___ :: new_1 ( { ( ar .as_pointer()  as Ptr< (Value<i32  >, Value<f64  >)  >   )  } , { ( ar .as_pointer()  as Ptr< (Value<i32  >, Value<f64  >)  >   ) . offset ( ( ( ( 128usize as usize ) ) . wrapping_div ( ( 16usize as usize ) ) ) as isize )  } , )  )) ;
            let r: Value<BidirectionalIterator<(Value<i32>, Value<f64>)>> = Rc::new(RefCell::new(
                ({
                    let __tmp_9: Value<i32> = Rc::new(RefCell::new(5));
                    m.as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| {
                            __m.find(&__tmp_9.as_pointer())
                        })
                }),
            ));
            assert!(
                (*r.borrow())
                    == m.as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin())
            );
            (*r.borrow_mut()) = ({
                let __tmp_10: Value<i32> = Rc::new(RefCell::new(6));
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| {
                        __m.find(&__tmp_10.as_pointer())
                    })
            });
            assert!(
                (*r.borrow())
                    == libcc2rs::next_refcount(
                        m.as_pointer()
                            .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                        1_isize
                    )
            );
            (*r.borrow_mut()) = ({
                let __tmp_11: Value<i32> = Rc::new(RefCell::new(7));
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| {
                        __m.find(&__tmp_11.as_pointer())
                    })
            });
            assert!(
                (*r.borrow())
                    == libcc2rs::next_refcount(
                        m.as_pointer()
                            .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                        2_isize
                    )
            );
            (*r.borrow_mut()) = ({
                let __tmp_12: Value<i32> = Rc::new(RefCell::new(8));
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| {
                        __m.find(&__tmp_12.as_pointer())
                    })
            });
            assert!(
                (*r.borrow())
                    == libcc2rs::next_refcount(
                        m.as_pointer()
                            .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                        3_isize
                    )
            );
            (*r.borrow_mut()) = ({
                let __tmp_13: Value<i32> = Rc::new(RefCell::new(9));
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| {
                        __m.find(&__tmp_13.as_pointer())
                    })
            });
            assert!(
                (*r.borrow())
                    == libcc2rs::next_refcount(
                        m.as_pointer()
                            .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                        4_isize
                    )
            );
            (*r.borrow_mut()) = ({
                let __tmp_14: Value<i32> = Rc::new(RefCell::new(10));
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| {
                        __m.find(&__tmp_14.as_pointer())
                    })
            });
            assert!(
                (*r.borrow())
                    == libcc2rs::next_refcount(
                        m.as_pointer()
                            .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                        5_isize
                    )
            );
            (*r.borrow_mut()) = ({
                let __tmp_15: Value<i32> = Rc::new(RefCell::new(11));
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| {
                        __m.find(&__tmp_15.as_pointer())
                    })
            });
            assert!(
                (*r.borrow())
                    == libcc2rs::next_refcount(
                        m.as_pointer()
                            .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                        6_isize
                    )
            );
            (*r.borrow_mut()) = ({
                let __tmp_16: Value<i32> = Rc::new(RefCell::new(12));
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| {
                        __m.find(&__tmp_16.as_pointer())
                    })
            });
            assert!(
                (*r.borrow())
                    == libcc2rs::next_refcount(
                        m.as_pointer()
                            .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                        7_isize
                    )
            );
            (*r.borrow_mut()) = ({
                let __tmp_17: Value<i32> = Rc::new(RefCell::new(4));
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| {
                        __m.find(&__tmp_17.as_pointer())
                    })
            });
            assert!(
                (*r.borrow())
                    == libcc2rs::next_refcount(
                        m.as_pointer()
                            .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                        8_isize
                    )
            );
        }
    }
    {
        let ar: Value<Box<[(Value<i32>, Value<i32>)]>> = Rc::new(RefCell::new(Box::new([
            (Rc::new(RefCell::new(5)), Rc::new(RefCell::new(5))),
            (Rc::new(RefCell::new(6)), Rc::new(RefCell::new(6))),
            (Rc::new(RefCell::new(7)), Rc::new(RefCell::new(7))),
            (Rc::new(RefCell::new(8)), Rc::new(RefCell::new(8))),
            (Rc::new(RefCell::new(9)), Rc::new(RefCell::new(9))),
            (Rc::new(RefCell::new(10)), Rc::new(RefCell::new(10))),
            (Rc::new(RefCell::new(11)), Rc::new(RefCell::new(11))),
            (Rc::new(RefCell::new(12)), Rc::new(RefCell::new(12))),
        ])));
        let m: Value<Map<i32, (Value<i32>, Value<i32>)>> = Rc::new(RefCell::new(
            std_map_int__int__std_greater_int___std_allocator_std_pair_const_int__int___::new_1(
                { (ar.as_pointer() as Ptr<(Value<i32>, Value<i32>)>) },
                {
                    (ar.as_pointer() as Ptr<(Value<i32>, Value<i32>)>)
                        .offset(((64usize as usize).wrapping_div((8usize as usize))) as isize)
                },
            ),
        ));
        assert!(
            ({
                let __tmp_18: Value<i32> = Rc::new(RefCell::new(12));
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| {
                        __m.find(&__tmp_18.as_pointer())
                    })
            }) == libcc2rs::next_refcount(
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.begin()),
                0_isize
            )
        );
        assert!(
            ({
                let __tmp_19: Value<i32> = Rc::new(RefCell::new(11));
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| {
                        __m.find(&__tmp_19.as_pointer())
                    })
            }) == libcc2rs::next_refcount(
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.begin()),
                1_isize
            )
        );
        assert!(
            ({
                let __tmp_20: Value<i32> = Rc::new(RefCell::new(10));
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| {
                        __m.find(&__tmp_20.as_pointer())
                    })
            }) == libcc2rs::next_refcount(
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.begin()),
                2_isize
            )
        );
        assert!(
            ({
                let __tmp_21: Value<i32> = Rc::new(RefCell::new(9));
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| {
                        __m.find(&__tmp_21.as_pointer())
                    })
            }) == libcc2rs::next_refcount(
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.begin()),
                3_isize
            )
        );
        assert!(
            ({
                let __tmp_22: Value<i32> = Rc::new(RefCell::new(8));
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| {
                        __m.find(&__tmp_22.as_pointer())
                    })
            }) == libcc2rs::next_refcount(
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.begin()),
                4_isize
            )
        );
        assert!(
            ({
                let __tmp_23: Value<i32> = Rc::new(RefCell::new(7));
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| {
                        __m.find(&__tmp_23.as_pointer())
                    })
            }) == libcc2rs::next_refcount(
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.begin()),
                5_isize
            )
        );
        assert!(
            ({
                let __tmp_24: Value<i32> = Rc::new(RefCell::new(6));
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| {
                        __m.find(&__tmp_24.as_pointer())
                    })
            }) == libcc2rs::next_refcount(
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.begin()),
                6_isize
            )
        );
        assert!(
            ({
                let __tmp_25: Value<i32> = Rc::new(RefCell::new(5));
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| {
                        __m.find(&__tmp_25.as_pointer())
                    })
            }) == libcc2rs::next_refcount(
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.begin()),
                7_isize
            )
        );
        assert!(
            ({
                let __tmp_26: Value<i32> = Rc::new(RefCell::new(4));
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| {
                        __m.find(&__tmp_26.as_pointer())
                    })
            }) == libcc2rs::next_refcount(
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.begin()),
                8_isize
            )
        );
        assert!(
            libcc2rs::next_refcount(
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.begin()),
                8_isize
            ) == m
                .as_pointer()
                .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.end())
        );
    }
    {
        {
            let ar: Value<Box<[(Value<i32>, Value<f64>)]>> = Rc::new(RefCell::new(Box::new([
                (Rc::new(RefCell::new(5)), Rc::new(RefCell::new((5 as f64)))),
                (Rc::new(RefCell::new(6)), Rc::new(RefCell::new((6 as f64)))),
                (Rc::new(RefCell::new(7)), Rc::new(RefCell::new((7 as f64)))),
                (Rc::new(RefCell::new(8)), Rc::new(RefCell::new((8 as f64)))),
                (Rc::new(RefCell::new(9)), Rc::new(RefCell::new((9 as f64)))),
                (
                    Rc::new(RefCell::new(10)),
                    Rc::new(RefCell::new((10 as f64))),
                ),
                (
                    Rc::new(RefCell::new(11)),
                    Rc::new(RefCell::new((11 as f64))),
                ),
                (
                    Rc::new(RefCell::new(12)),
                    Rc::new(RefCell::new((12 as f64))),
                ),
            ])));
            &(*(*ar.borrow())[(0) as usize].1.borrow_mut());
            let m : Value<Map<i32  , (Value<i32  >, Value<f64  >)> > = Rc::new(RefCell::new(std_map_int__double__std_less_int___min_allocator_std_pair_const_int__double___ :: new_1 ( { ( ar .as_pointer()  as Ptr< (Value<i32  >, Value<f64  >)  >   )  } , { ( ar .as_pointer()  as Ptr< (Value<i32  >, Value<f64  >)  >   ) . offset ( ( ( ( 128usize as usize ) ) . wrapping_div ( ( 16usize as usize ) ) ) as isize )  } , )  )) ;
            let r: Value<BidirectionalIterator<(Value<i32>, Value<f64>)>> = Rc::new(RefCell::new(
                ({
                    let __tmp_27: Value<i32> = Rc::new(RefCell::new(5));
                    m.as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| {
                            __m.find(&__tmp_27.as_pointer())
                        })
                }),
            ));
            assert!(
                (*r.borrow())
                    == m.as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin())
            );
            (*r.borrow_mut()) = ({
                let __tmp_28: Value<i32> = Rc::new(RefCell::new(6));
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| {
                        __m.find(&__tmp_28.as_pointer())
                    })
            })
            .clone();
            assert!(
                (*r.borrow())
                    == libcc2rs::next_refcount(
                        m.as_pointer()
                            .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                        1_isize
                    )
            );
            (*r.borrow_mut()) = ({
                let __tmp_29: Value<i32> = Rc::new(RefCell::new(7));
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| {
                        __m.find(&__tmp_29.as_pointer())
                    })
            })
            .clone();
            assert!(
                (*r.borrow())
                    == libcc2rs::next_refcount(
                        m.as_pointer()
                            .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                        2_isize
                    )
            );
            (*r.borrow_mut()) = ({
                let __tmp_30: Value<i32> = Rc::new(RefCell::new(8));
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| {
                        __m.find(&__tmp_30.as_pointer())
                    })
            })
            .clone();
            assert!(
                (*r.borrow())
                    == libcc2rs::next_refcount(
                        m.as_pointer()
                            .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                        3_isize
                    )
            );
            (*r.borrow_mut()) = ({
                let __tmp_31: Value<i32> = Rc::new(RefCell::new(9));
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| {
                        __m.find(&__tmp_31.as_pointer())
                    })
            })
            .clone();
            assert!(
                (*r.borrow())
                    == libcc2rs::next_refcount(
                        m.as_pointer()
                            .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                        4_isize
                    )
            );
            (*r.borrow_mut()) = ({
                let __tmp_32: Value<i32> = Rc::new(RefCell::new(10));
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| {
                        __m.find(&__tmp_32.as_pointer())
                    })
            })
            .clone();
            assert!(
                (*r.borrow())
                    == libcc2rs::next_refcount(
                        m.as_pointer()
                            .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                        5_isize
                    )
            );
            (*r.borrow_mut()) = ({
                let __tmp_33: Value<i32> = Rc::new(RefCell::new(11));
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| {
                        __m.find(&__tmp_33.as_pointer())
                    })
            })
            .clone();
            assert!(
                (*r.borrow())
                    == libcc2rs::next_refcount(
                        m.as_pointer()
                            .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                        6_isize
                    )
            );
            (*r.borrow_mut()) = ({
                let __tmp_34: Value<i32> = Rc::new(RefCell::new(12));
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| {
                        __m.find(&__tmp_34.as_pointer())
                    })
            })
            .clone();
            assert!(
                (*r.borrow())
                    == libcc2rs::next_refcount(
                        m.as_pointer()
                            .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                        7_isize
                    )
            );
            (*r.borrow_mut()) = ({
                let __tmp_35: Value<i32> = Rc::new(RefCell::new(4));
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| {
                        __m.find(&__tmp_35.as_pointer())
                    })
            })
            .clone();
            assert!(
                (*r.borrow())
                    == libcc2rs::next_refcount(
                        m.as_pointer()
                            .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                        8_isize
                    )
            );
        }
        {
            let ar: Value<Box<[(Value<i32>, Value<f64>)]>> = Rc::new(RefCell::new(Box::new([
                (Rc::new(RefCell::new(5)), Rc::new(RefCell::new((5 as f64)))),
                (Rc::new(RefCell::new(6)), Rc::new(RefCell::new((6 as f64)))),
                (Rc::new(RefCell::new(7)), Rc::new(RefCell::new((7 as f64)))),
                (Rc::new(RefCell::new(8)), Rc::new(RefCell::new((8 as f64)))),
                (Rc::new(RefCell::new(9)), Rc::new(RefCell::new((9 as f64)))),
                (
                    Rc::new(RefCell::new(10)),
                    Rc::new(RefCell::new((10 as f64))),
                ),
                (
                    Rc::new(RefCell::new(11)),
                    Rc::new(RefCell::new((11 as f64))),
                ),
                (
                    Rc::new(RefCell::new(12)),
                    Rc::new(RefCell::new((12 as f64))),
                ),
            ])));
            let m : Value<Map<i32  , (Value<i32  >, Value<f64  >)> > = Rc::new(RefCell::new(std_map_int__double__std_less_int___min_allocator_std_pair_const_int__double___ :: new_1 ( { ( ar .as_pointer()  as Ptr< (Value<i32  >, Value<f64  >)  >   )  } , { ( ar .as_pointer()  as Ptr< (Value<i32  >, Value<f64  >)  >   ) . offset ( ( ( ( 128usize as usize ) ) . wrapping_div ( ( 16usize as usize ) ) ) as isize )  } , )  )) ;
            let r: Value<BidirectionalIterator<(Value<i32>, Value<f64>)>> = Rc::new(RefCell::new(
                ({
                    let __tmp_36: Value<i32> = Rc::new(RefCell::new(5));
                    m.as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| {
                            __m.find(&__tmp_36.as_pointer())
                        })
                }),
            ));
            assert!(
                (*r.borrow())
                    == m.as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin())
            );
            (*r.borrow_mut()) = ({
                let __tmp_37: Value<i32> = Rc::new(RefCell::new(6));
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| {
                        __m.find(&__tmp_37.as_pointer())
                    })
            })
            .clone();
            assert!(
                (*r.borrow())
                    == libcc2rs::next_refcount(
                        m.as_pointer()
                            .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                        1_isize
                    )
            );
            (*r.borrow_mut()) = ({
                let __tmp_38: Value<i32> = Rc::new(RefCell::new(7));
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| {
                        __m.find(&__tmp_38.as_pointer())
                    })
            })
            .clone();
            assert!(
                (*r.borrow())
                    == libcc2rs::next_refcount(
                        m.as_pointer()
                            .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                        2_isize
                    )
            );
            (*r.borrow_mut()) = ({
                let __tmp_39: Value<i32> = Rc::new(RefCell::new(8));
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| {
                        __m.find(&__tmp_39.as_pointer())
                    })
            })
            .clone();
            assert!(
                (*r.borrow())
                    == libcc2rs::next_refcount(
                        m.as_pointer()
                            .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                        3_isize
                    )
            );
            (*r.borrow_mut()) = ({
                let __tmp_40: Value<i32> = Rc::new(RefCell::new(9));
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| {
                        __m.find(&__tmp_40.as_pointer())
                    })
            })
            .clone();
            assert!(
                (*r.borrow())
                    == libcc2rs::next_refcount(
                        m.as_pointer()
                            .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                        4_isize
                    )
            );
            (*r.borrow_mut()) = ({
                let __tmp_41: Value<i32> = Rc::new(RefCell::new(10));
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| {
                        __m.find(&__tmp_41.as_pointer())
                    })
            })
            .clone();
            assert!(
                (*r.borrow())
                    == libcc2rs::next_refcount(
                        m.as_pointer()
                            .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                        5_isize
                    )
            );
            (*r.borrow_mut()) = ({
                let __tmp_42: Value<i32> = Rc::new(RefCell::new(11));
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| {
                        __m.find(&__tmp_42.as_pointer())
                    })
            })
            .clone();
            assert!(
                (*r.borrow())
                    == libcc2rs::next_refcount(
                        m.as_pointer()
                            .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                        6_isize
                    )
            );
            (*r.borrow_mut()) = ({
                let __tmp_43: Value<i32> = Rc::new(RefCell::new(12));
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| {
                        __m.find(&__tmp_43.as_pointer())
                    })
            })
            .clone();
            assert!(
                (*r.borrow())
                    == libcc2rs::next_refcount(
                        m.as_pointer()
                            .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                        7_isize
                    )
            );
            (*r.borrow_mut()) = ({
                let __tmp_44: Value<i32> = Rc::new(RefCell::new(4));
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| {
                        __m.find(&__tmp_44.as_pointer())
                    })
            })
            .clone();
            assert!(
                (*r.borrow())
                    == libcc2rs::next_refcount(
                        m.as_pointer()
                            .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                        8_isize
                    )
            );
        }
    }
    {
        let ar: Value<Box<[(Value<i32>, Value<f64>)]>> = Rc::new(RefCell::new(Box::new([
            (Rc::new(RefCell::new(5)), Rc::new(RefCell::new((5 as f64)))),
            (Rc::new(RefCell::new(6)), Rc::new(RefCell::new((6 as f64)))),
            (Rc::new(RefCell::new(7)), Rc::new(RefCell::new((7 as f64)))),
            (Rc::new(RefCell::new(8)), Rc::new(RefCell::new((8 as f64)))),
            (Rc::new(RefCell::new(9)), Rc::new(RefCell::new((9 as f64)))),
            (
                Rc::new(RefCell::new(10)),
                Rc::new(RefCell::new((10 as f64))),
            ),
            (
                Rc::new(RefCell::new(11)),
                Rc::new(RefCell::new((11 as f64))),
            ),
            (
                Rc::new(RefCell::new(12)),
                Rc::new(RefCell::new((12 as f64))),
            ),
        ])));
        let m: Value<Map<i32, (Value<i32>, Value<f64>)>> = Rc::new(RefCell::new(
            std_map_int__double__std_less_void___std_allocator_std_pair_const_int__double___::new_1(
                { (ar.as_pointer() as Ptr<(Value<i32>, Value<f64>)>) },
                {
                    (ar.as_pointer() as Ptr<(Value<i32>, Value<f64>)>)
                        .offset(((128usize as usize).wrapping_div((16usize as usize))) as isize)
                },
            ),
        ));
        let r: Value<BidirectionalIterator<(Value<i32>, Value<f64>)>> = Rc::new(RefCell::new(
            ({
                let __tmp_45: Value<i32> = Rc::new(RefCell::new(5));
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| {
                        __m.find(&__tmp_45.as_pointer())
                    })
            }),
        ));
        assert!(
            (*r.borrow())
                == m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin())
        );
        (*r.borrow_mut()) = ({
            let __tmp_46: Value<i32> = Rc::new(RefCell::new(6));
            m.as_pointer()
                .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.find(&__tmp_46.as_pointer()))
        });
        assert!(
            (*r.borrow())
                == libcc2rs::next_refcount(
                    m.as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                    1_isize
                )
        );
        (*r.borrow_mut()) = ({
            let __tmp_47: Value<i32> = Rc::new(RefCell::new(7));
            m.as_pointer()
                .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.find(&__tmp_47.as_pointer()))
        });
        assert!(
            (*r.borrow())
                == libcc2rs::next_refcount(
                    m.as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                    2_isize
                )
        );
        (*r.borrow_mut()) = ({
            let __tmp_48: Value<i32> = Rc::new(RefCell::new(8));
            m.as_pointer()
                .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.find(&__tmp_48.as_pointer()))
        });
        assert!(
            (*r.borrow())
                == libcc2rs::next_refcount(
                    m.as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                    3_isize
                )
        );
        (*r.borrow_mut()) = ({
            let __tmp_49: Value<i32> = Rc::new(RefCell::new(9));
            m.as_pointer()
                .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.find(&__tmp_49.as_pointer()))
        });
        assert!(
            (*r.borrow())
                == libcc2rs::next_refcount(
                    m.as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                    4_isize
                )
        );
        (*r.borrow_mut()) = ({
            let __tmp_50: Value<i32> = Rc::new(RefCell::new(10));
            m.as_pointer()
                .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.find(&__tmp_50.as_pointer()))
        });
        assert!(
            (*r.borrow())
                == libcc2rs::next_refcount(
                    m.as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                    5_isize
                )
        );
        (*r.borrow_mut()) = ({
            let __tmp_51: Value<i32> = Rc::new(RefCell::new(11));
            m.as_pointer()
                .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.find(&__tmp_51.as_pointer()))
        });
        assert!(
            (*r.borrow())
                == libcc2rs::next_refcount(
                    m.as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                    6_isize
                )
        );
        (*r.borrow_mut()) = ({
            let __tmp_52: Value<i32> = Rc::new(RefCell::new(12));
            m.as_pointer()
                .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.find(&__tmp_52.as_pointer()))
        });
        assert!(
            (*r.borrow())
                == libcc2rs::next_refcount(
                    m.as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                    7_isize
                )
        );
        (*r.borrow_mut()) = ({
            let __tmp_53: Value<i32> = Rc::new(RefCell::new(4));
            m.as_pointer()
                .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.find(&__tmp_53.as_pointer()))
        });
        assert!(
            (*r.borrow())
                == libcc2rs::next_refcount(
                    m.as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                    8_isize
                )
        );
        (*r.borrow_mut()) = ({
            let ___x: Value<C2Int> = Rc::new(RefCell::new(C2Int::new_1({ 5 })));
            (*m.borrow()).find_75_struct_C2Int(___x.as_pointer())
        });
        assert!(
            (*r.borrow())
                == m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin())
        );
        (*r.borrow_mut()) = ({
            let ___x: Value<C2Int> = Rc::new(RefCell::new(C2Int::new_1({ 6 })));
            (*m.borrow()).find_75_struct_C2Int(___x.as_pointer())
        });
        assert!(
            (*r.borrow())
                == libcc2rs::next_refcount(
                    m.as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                    1_isize
                )
        );
        (*r.borrow_mut()) = ({
            let ___x: Value<C2Int> = Rc::new(RefCell::new(C2Int::new_1({ 7 })));
            (*m.borrow()).find_75_struct_C2Int(___x.as_pointer())
        });
        assert!(
            (*r.borrow())
                == libcc2rs::next_refcount(
                    m.as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                    2_isize
                )
        );
        (*r.borrow_mut()) = ({
            let ___x: Value<C2Int> = Rc::new(RefCell::new(C2Int::new_1({ 8 })));
            (*m.borrow()).find_75_struct_C2Int(___x.as_pointer())
        });
        assert!(
            (*r.borrow())
                == libcc2rs::next_refcount(
                    m.as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                    3_isize
                )
        );
        (*r.borrow_mut()) = ({
            let ___x: Value<C2Int> = Rc::new(RefCell::new(C2Int::new_1({ 9 })));
            (*m.borrow()).find_75_struct_C2Int(___x.as_pointer())
        });
        assert!(
            (*r.borrow())
                == libcc2rs::next_refcount(
                    m.as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                    4_isize
                )
        );
        (*r.borrow_mut()) = ({
            let ___x: Value<C2Int> = Rc::new(RefCell::new(C2Int::new_1({ 10 })));
            (*m.borrow()).find_75_struct_C2Int(___x.as_pointer())
        });
        assert!(
            (*r.borrow())
                == libcc2rs::next_refcount(
                    m.as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                    5_isize
                )
        );
        (*r.borrow_mut()) = ({
            let ___x: Value<C2Int> = Rc::new(RefCell::new(C2Int::new_1({ 11 })));
            (*m.borrow()).find_75_struct_C2Int(___x.as_pointer())
        });
        assert!(
            (*r.borrow())
                == libcc2rs::next_refcount(
                    m.as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                    6_isize
                )
        );
        (*r.borrow_mut()) = ({
            let ___x: Value<C2Int> = Rc::new(RefCell::new(C2Int::new_1({ 12 })));
            (*m.borrow()).find_75_struct_C2Int(___x.as_pointer())
        });
        assert!(
            (*r.borrow())
                == libcc2rs::next_refcount(
                    m.as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                    7_isize
                )
        );
        (*r.borrow_mut()) = ({
            let ___x: Value<C2Int> = Rc::new(RefCell::new(C2Int::new_1({ 4 })));
            (*m.borrow()).find_75_struct_C2Int(___x.as_pointer())
        });
        assert!(
            (*r.borrow())
                == libcc2rs::next_refcount(
                    m.as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                    8_isize
                )
        );
    }
    {
        let m: Value<Map<PrivateConstructor, (Value<PrivateConstructor>, Value<f64>)>> =
            Rc::new(RefCell::new(Map::new()));
        Map::subscript_move(
            m.as_pointer(),
            Rc::new(RefCell::new(({ PrivateConstructor::make(5) }))).as_pointer(),
            |__k| cpp_move!(__k),
        )
        .write(5_f64);
        Map::subscript_move(
            m.as_pointer(),
            Rc::new(RefCell::new(({ PrivateConstructor::make(6) }))).as_pointer(),
            |__k| cpp_move!(__k),
        )
        .write(6_f64);
        Map::subscript_move(
            m.as_pointer(),
            Rc::new(RefCell::new(({ PrivateConstructor::make(7) }))).as_pointer(),
            |__k| cpp_move!(__k),
        )
        .write(7_f64);
        Map::subscript_move(
            m.as_pointer(),
            Rc::new(RefCell::new(({ PrivateConstructor::make(8) }))).as_pointer(),
            |__k| cpp_move!(__k),
        )
        .write(8_f64);
        Map::subscript_move(
            m.as_pointer(),
            Rc::new(RefCell::new(({ PrivateConstructor::make(9) }))).as_pointer(),
            |__k| cpp_move!(__k),
        )
        .write(9_f64);
        Map::subscript_move(
            m.as_pointer(),
            Rc::new(RefCell::new(({ PrivateConstructor::make(10) }))).as_pointer(),
            |__k| cpp_move!(__k),
        )
        .write(10_f64);
        Map::subscript_move(
            m.as_pointer(),
            Rc::new(RefCell::new(({ PrivateConstructor::make(11) }))).as_pointer(),
            |__k| cpp_move!(__k),
        )
        .write(11_f64);
        Map::subscript_move(
            m.as_pointer(),
            Rc::new(RefCell::new(({ PrivateConstructor::make(12) }))).as_pointer(),
            |__k| cpp_move!(__k),
        )
        .write(12_f64);
        let r: Value<BidirectionalIterator<(Value<PrivateConstructor>, Value<f64>)>> =
            Rc::new(RefCell::new(
                ({
                    let ___x: Value<i32> = Rc::new(RefCell::new(5));
                    (*m.borrow()).find_75_int(___x.as_pointer())
                }),
            ));
        assert!(
            (*r.borrow())
                == m.as_pointer().with(
                    |__m: &Map<PrivateConstructor, (Value<PrivateConstructor>, Value<f64>)>| __m
                        .begin()
                )
        );
        (*r.borrow_mut()) = ({
            let ___x: Value<i32> = Rc::new(RefCell::new(6));
            (*m.borrow()).find_75_int(___x.as_pointer())
        });
        assert!(
            (*r.borrow())
                == libcc2rs::next_refcount(
                    m.as_pointer().with(
                        |__m: &Map<PrivateConstructor, (Value<PrivateConstructor>, Value<f64>)>| {
                            __m.begin()
                        }
                    ),
                    1_isize
                )
        );
        (*r.borrow_mut()) = ({
            let ___x: Value<i32> = Rc::new(RefCell::new(7));
            (*m.borrow()).find_75_int(___x.as_pointer())
        });
        assert!(
            (*r.borrow())
                == libcc2rs::next_refcount(
                    m.as_pointer().with(
                        |__m: &Map<PrivateConstructor, (Value<PrivateConstructor>, Value<f64>)>| {
                            __m.begin()
                        }
                    ),
                    2_isize
                )
        );
        (*r.borrow_mut()) = ({
            let ___x: Value<i32> = Rc::new(RefCell::new(8));
            (*m.borrow()).find_75_int(___x.as_pointer())
        });
        assert!(
            (*r.borrow())
                == libcc2rs::next_refcount(
                    m.as_pointer().with(
                        |__m: &Map<PrivateConstructor, (Value<PrivateConstructor>, Value<f64>)>| {
                            __m.begin()
                        }
                    ),
                    3_isize
                )
        );
        (*r.borrow_mut()) = ({
            let ___x: Value<i32> = Rc::new(RefCell::new(9));
            (*m.borrow()).find_75_int(___x.as_pointer())
        });
        assert!(
            (*r.borrow())
                == libcc2rs::next_refcount(
                    m.as_pointer().with(
                        |__m: &Map<PrivateConstructor, (Value<PrivateConstructor>, Value<f64>)>| {
                            __m.begin()
                        }
                    ),
                    4_isize
                )
        );
        (*r.borrow_mut()) = ({
            let ___x: Value<i32> = Rc::new(RefCell::new(10));
            (*m.borrow()).find_75_int(___x.as_pointer())
        });
        assert!(
            (*r.borrow())
                == libcc2rs::next_refcount(
                    m.as_pointer().with(
                        |__m: &Map<PrivateConstructor, (Value<PrivateConstructor>, Value<f64>)>| {
                            __m.begin()
                        }
                    ),
                    5_isize
                )
        );
        (*r.borrow_mut()) = ({
            let ___x: Value<i32> = Rc::new(RefCell::new(11));
            (*m.borrow()).find_75_int(___x.as_pointer())
        });
        assert!(
            (*r.borrow())
                == libcc2rs::next_refcount(
                    m.as_pointer().with(
                        |__m: &Map<PrivateConstructor, (Value<PrivateConstructor>, Value<f64>)>| {
                            __m.begin()
                        }
                    ),
                    6_isize
                )
        );
        (*r.borrow_mut()) = ({
            let ___x: Value<i32> = Rc::new(RefCell::new(12));
            (*m.borrow()).find_75_int(___x.as_pointer())
        });
        assert!(
            (*r.borrow())
                == libcc2rs::next_refcount(
                    m.as_pointer().with(
                        |__m: &Map<PrivateConstructor, (Value<PrivateConstructor>, Value<f64>)>| {
                            __m.begin()
                        }
                    ),
                    7_isize
                )
        );
        (*r.borrow_mut()) = ({
            let ___x: Value<i32> = Rc::new(RefCell::new(4));
            (*m.borrow()).find_75_int(___x.as_pointer())
        });
        assert!(
            (*r.borrow())
                == libcc2rs::next_refcount(
                    m.as_pointer().with(
                        |__m: &Map<PrivateConstructor, (Value<PrivateConstructor>, Value<f64>)>| {
                            __m.begin()
                        }
                    ),
                    8_isize
                )
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
    ({ test_10() });
    return 0;
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__;
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
pub fn __cpp2rust_init_globals() {
    let _ = outstanding_bytes_0.with(|_| ());
    let _ = alloc_count_1.with(|_| ());
    let _ = dealloc_count_2.with(|_| ());
    let _ = disable_default_constructor_3.with(|_| ());
}
