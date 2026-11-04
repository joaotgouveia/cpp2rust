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
            let m : Value<std_multimap_int__double__std_less_int___std_allocator_std_pair_const_int__double___ > = Rc::new(RefCell::new(std_multimap_int__double__std_less_int___std_allocator_std_pair_const_int__double___ :: new_1 ( { ( ar .as_pointer()  as Ptr< (Value<i32  >, Value<f64  >)  >   )  } , { ( ar .as_pointer()  as Ptr< (Value<i32  >, Value<f64  >)  >   ) . offset ( ( ( ( 144usize as usize ) ) . wrapping_div ( ( 16usize as usize ) ) ) as isize )  } , )  )) ;
            let r: Value<BidirectionalIterator<(Value<i32>, Value<f64>)>> = Rc::new(RefCell::new(
                ({
                    let ___x: Value<i32> = Rc::new(RefCell::new(4));
                    (*m.borrow()).upper_bound_74(___x.as_pointer())
                }),
            ));
            assert!((*r.borrow()) == ({ (*m.borrow()).begin_20() }));
            (*r.borrow_mut()) = ({
                let ___x: Value<i32> = Rc::new(RefCell::new(5));
                (*m.borrow()).upper_bound_74(___x.as_pointer())
            });
            assert!(
                (*r.borrow())
                    == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin_20() }) }), 3_isize)
            );
            (*r.borrow_mut()) = ({
                let ___x: Value<i32> = Rc::new(RefCell::new(6));
                (*m.borrow()).upper_bound_74(___x.as_pointer())
            });
            assert!(
                (*r.borrow())
                    == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin_20() }) }), 3_isize)
            );
            (*r.borrow_mut()) = ({
                let ___x: Value<i32> = Rc::new(RefCell::new(7));
                (*m.borrow()).upper_bound_74(___x.as_pointer())
            });
            assert!(
                (*r.borrow())
                    == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin_20() }) }), 6_isize)
            );
            (*r.borrow_mut()) = ({
                let ___x: Value<i32> = Rc::new(RefCell::new(8));
                (*m.borrow()).upper_bound_74(___x.as_pointer())
            });
            assert!(
                (*r.borrow())
                    == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin_20() }) }), 6_isize)
            );
            (*r.borrow_mut()) = ({
                let ___x: Value<i32> = Rc::new(RefCell::new(9));
                (*m.borrow()).upper_bound_74(___x.as_pointer())
            });
            assert!(
                (*r.borrow())
                    == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin_20() }) }), 9_isize)
            );
            (*r.borrow_mut()) = ({
                let ___x: Value<i32> = Rc::new(RefCell::new(10));
                (*m.borrow()).upper_bound_74(___x.as_pointer())
            });
            assert!((*r.borrow()) == ({ (*m.borrow()).end_22() }));
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
            let m : Value<std_multimap_int__double__std_less_int___std_allocator_std_pair_const_int__double___ > = Rc::new(RefCell::new(std_multimap_int__double__std_less_int___std_allocator_std_pair_const_int__double___ :: new_1 ( { ( ar .as_pointer()  as Ptr< (Value<i32  >, Value<f64  >)  >   )  } , { ( ar .as_pointer()  as Ptr< (Value<i32  >, Value<f64  >)  >   ) . offset ( ( ( ( 144usize as usize ) ) . wrapping_div ( ( 16usize as usize ) ) ) as isize )  } , )  )) ;
            let r: Value<BidirectionalIterator<(Value<i32>, Value<f64>)>> = Rc::new(RefCell::new(
                ({
                    let ___x: Value<i32> = Rc::new(RefCell::new(4));
                    (*m.borrow()).upper_bound_76(___x.as_pointer())
                }),
            ));
            assert!((*r.borrow()) == ({ (*m.borrow()).begin_21() }));
            (*r.borrow_mut()) = ({
                let ___x: Value<i32> = Rc::new(RefCell::new(5));
                (*m.borrow()).upper_bound_76(___x.as_pointer())
            });
            assert!(
                (*r.borrow())
                    == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin_21() }) }), 3_isize)
            );
            (*r.borrow_mut()) = ({
                let ___x: Value<i32> = Rc::new(RefCell::new(6));
                (*m.borrow()).upper_bound_76(___x.as_pointer())
            });
            assert!(
                (*r.borrow())
                    == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin_21() }) }), 3_isize)
            );
            (*r.borrow_mut()) = ({
                let ___x: Value<i32> = Rc::new(RefCell::new(7));
                (*m.borrow()).upper_bound_76(___x.as_pointer())
            });
            assert!(
                (*r.borrow())
                    == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin_21() }) }), 6_isize)
            );
            (*r.borrow_mut()) = ({
                let ___x: Value<i32> = Rc::new(RefCell::new(8));
                (*m.borrow()).upper_bound_76(___x.as_pointer())
            });
            assert!(
                (*r.borrow())
                    == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin_21() }) }), 6_isize)
            );
            (*r.borrow_mut()) = ({
                let ___x: Value<i32> = Rc::new(RefCell::new(9));
                (*m.borrow()).upper_bound_76(___x.as_pointer())
            });
            assert!(
                (*r.borrow())
                    == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin_21() }) }), 9_isize)
            );
            (*r.borrow_mut()) = ({
                let ___x: Value<i32> = Rc::new(RefCell::new(10));
                (*m.borrow()).upper_bound_76(___x.as_pointer())
            });
            assert!((*r.borrow()) == ({ (*m.borrow()).end_23() }));
        }
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
            let m : Value<std_multimap_int__double__std_less_int___min_allocator_std_pair_const_int__double___ > = Rc::new(RefCell::new(std_multimap_int__double__std_less_int___min_allocator_std_pair_const_int__double___ :: new_1 ( { ( ar .as_pointer()  as Ptr< (Value<i32  >, Value<f64  >)  >   )  } , { ( ar .as_pointer()  as Ptr< (Value<i32  >, Value<f64  >)  >   ) . offset ( ( ( ( 144usize as usize ) ) . wrapping_div ( ( 16usize as usize ) ) ) as isize )  } , )  )) ;
            let r: Value<BidirectionalIterator<(Value<i32>, Value<f64>)>> = Rc::new(RefCell::new(
                ({
                    let ___x: Value<i32> = Rc::new(RefCell::new(4));
                    (*m.borrow()).upper_bound_74(___x.as_pointer())
                }),
            ));
            assert!((*r.borrow()) == ({ (*m.borrow()).begin_20() }));
            (*r.borrow_mut()) = ({
                let ___x: Value<i32> = Rc::new(RefCell::new(5));
                (*m.borrow()).upper_bound_74(___x.as_pointer())
            })
            .clone();
            assert!(
                (*r.borrow())
                    == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin_20() }) }), 3_isize)
            );
            (*r.borrow_mut()) = ({
                let ___x: Value<i32> = Rc::new(RefCell::new(6));
                (*m.borrow()).upper_bound_74(___x.as_pointer())
            })
            .clone();
            assert!(
                (*r.borrow())
                    == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin_20() }) }), 3_isize)
            );
            (*r.borrow_mut()) = ({
                let ___x: Value<i32> = Rc::new(RefCell::new(7));
                (*m.borrow()).upper_bound_74(___x.as_pointer())
            })
            .clone();
            assert!(
                (*r.borrow())
                    == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin_20() }) }), 6_isize)
            );
            (*r.borrow_mut()) = ({
                let ___x: Value<i32> = Rc::new(RefCell::new(8));
                (*m.borrow()).upper_bound_74(___x.as_pointer())
            })
            .clone();
            assert!(
                (*r.borrow())
                    == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin_20() }) }), 6_isize)
            );
            (*r.borrow_mut()) = ({
                let ___x: Value<i32> = Rc::new(RefCell::new(9));
                (*m.borrow()).upper_bound_74(___x.as_pointer())
            })
            .clone();
            assert!(
                (*r.borrow())
                    == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin_20() }) }), 9_isize)
            );
            (*r.borrow_mut()) = ({
                let ___x: Value<i32> = Rc::new(RefCell::new(10));
                (*m.borrow()).upper_bound_74(___x.as_pointer())
            })
            .clone();
            assert!((*r.borrow()) == ({ (*m.borrow()).end_22() }));
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
            let m : Value<std_multimap_int__double__std_less_int___min_allocator_std_pair_const_int__double___ > = Rc::new(RefCell::new(std_multimap_int__double__std_less_int___min_allocator_std_pair_const_int__double___ :: new_1 ( { ( ar .as_pointer()  as Ptr< (Value<i32  >, Value<f64  >)  >   )  } , { ( ar .as_pointer()  as Ptr< (Value<i32  >, Value<f64  >)  >   ) . offset ( ( ( ( 144usize as usize ) ) . wrapping_div ( ( 16usize as usize ) ) ) as isize )  } , )  )) ;
            let r: Value<BidirectionalIterator<(Value<i32>, Value<f64>)>> = Rc::new(RefCell::new(
                ({
                    let ___x: Value<i32> = Rc::new(RefCell::new(4));
                    (*m.borrow()).upper_bound_76(___x.as_pointer())
                }),
            ));
            assert!((*r.borrow()) == ({ (*m.borrow()).begin_21() }));
            (*r.borrow_mut()) = ({
                let ___x: Value<i32> = Rc::new(RefCell::new(5));
                (*m.borrow()).upper_bound_76(___x.as_pointer())
            })
            .clone();
            assert!(
                (*r.borrow())
                    == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin_21() }) }), 3_isize)
            );
            (*r.borrow_mut()) = ({
                let ___x: Value<i32> = Rc::new(RefCell::new(6));
                (*m.borrow()).upper_bound_76(___x.as_pointer())
            })
            .clone();
            assert!(
                (*r.borrow())
                    == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin_21() }) }), 3_isize)
            );
            (*r.borrow_mut()) = ({
                let ___x: Value<i32> = Rc::new(RefCell::new(7));
                (*m.borrow()).upper_bound_76(___x.as_pointer())
            })
            .clone();
            assert!(
                (*r.borrow())
                    == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin_21() }) }), 6_isize)
            );
            (*r.borrow_mut()) = ({
                let ___x: Value<i32> = Rc::new(RefCell::new(8));
                (*m.borrow()).upper_bound_76(___x.as_pointer())
            })
            .clone();
            assert!(
                (*r.borrow())
                    == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin_21() }) }), 6_isize)
            );
            (*r.borrow_mut()) = ({
                let ___x: Value<i32> = Rc::new(RefCell::new(9));
                (*m.borrow()).upper_bound_76(___x.as_pointer())
            })
            .clone();
            assert!(
                (*r.borrow())
                    == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin_21() }) }), 9_isize)
            );
            (*r.borrow_mut()) = ({
                let ___x: Value<i32> = Rc::new(RefCell::new(10));
                (*m.borrow()).upper_bound_76(___x.as_pointer())
            })
            .clone();
            assert!((*r.borrow()) == ({ (*m.borrow()).end_23() }));
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
        let m : Value<std_multimap_int__double__std_less_void___std_allocator_std_pair_const_int__double___ > = Rc::new(RefCell::new(std_multimap_int__double__std_less_void___std_allocator_std_pair_const_int__double___ :: new_1 ( { ( ar .as_pointer()  as Ptr< (Value<i32  >, Value<f64  >)  >   )  } , { ( ar .as_pointer()  as Ptr< (Value<i32  >, Value<f64  >)  >   ) . offset ( ( ( ( 144usize as usize ) ) . wrapping_div ( ( 16usize as usize ) ) ) as isize )  } , )  )) ;
        let r: Value<BidirectionalIterator<(Value<i32>, Value<f64>)>> = Rc::new(RefCell::new(
            ({
                let ___x: Value<i32> = Rc::new(RefCell::new(4));
                (*m.borrow()).upper_bound_74(___x.as_pointer())
            }),
        ));
        assert!((*r.borrow()) == ({ (*m.borrow()).begin_20() }));
        (*r.borrow_mut()) = ({
            let ___x: Value<i32> = Rc::new(RefCell::new(5));
            (*m.borrow()).upper_bound_74(___x.as_pointer())
        });
        assert!(
            (*r.borrow()) == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin_20() }) }), 3_isize)
        );
        (*r.borrow_mut()) = ({
            let ___x: Value<i32> = Rc::new(RefCell::new(6));
            (*m.borrow()).upper_bound_74(___x.as_pointer())
        });
        assert!(
            (*r.borrow()) == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin_20() }) }), 3_isize)
        );
        (*r.borrow_mut()) = ({
            let ___x: Value<i32> = Rc::new(RefCell::new(7));
            (*m.borrow()).upper_bound_74(___x.as_pointer())
        });
        assert!(
            (*r.borrow()) == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin_20() }) }), 6_isize)
        );
        (*r.borrow_mut()) = ({
            let ___x: Value<i32> = Rc::new(RefCell::new(8));
            (*m.borrow()).upper_bound_74(___x.as_pointer())
        });
        assert!(
            (*r.borrow()) == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin_20() }) }), 6_isize)
        );
        (*r.borrow_mut()) = ({
            let ___x: Value<i32> = Rc::new(RefCell::new(9));
            (*m.borrow()).upper_bound_74(___x.as_pointer())
        });
        assert!(
            (*r.borrow()) == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin_20() }) }), 9_isize)
        );
        (*r.borrow_mut()) = ({
            let ___x: Value<i32> = Rc::new(RefCell::new(10));
            (*m.borrow()).upper_bound_74(___x.as_pointer())
        });
        assert!((*r.borrow()) == ({ (*m.borrow()).end_22() }));
        (*r.borrow_mut()) = ({
            let ___x: Value<C2Int> = Rc::new(RefCell::new(C2Int::new_1({ 4 })));
            (*m.borrow()).upper_bound_75_struct_C2Int(___x.as_pointer())
        });
        assert!((*r.borrow()) == ({ (*m.borrow()).begin_20() }));
        (*r.borrow_mut()) = ({
            let ___x: Value<C2Int> = Rc::new(RefCell::new(C2Int::new_1({ 5 })));
            (*m.borrow()).upper_bound_75_struct_C2Int(___x.as_pointer())
        });
        assert!(
            (*r.borrow()) == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin_20() }) }), 3_isize)
        );
        (*r.borrow_mut()) = ({
            let ___x: Value<C2Int> = Rc::new(RefCell::new(C2Int::new_1({ 6 })));
            (*m.borrow()).upper_bound_75_struct_C2Int(___x.as_pointer())
        });
        assert!(
            (*r.borrow()) == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin_20() }) }), 3_isize)
        );
        (*r.borrow_mut()) = ({
            let ___x: Value<C2Int> = Rc::new(RefCell::new(C2Int::new_1({ 7 })));
            (*m.borrow()).upper_bound_75_struct_C2Int(___x.as_pointer())
        });
        assert!(
            (*r.borrow()) == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin_20() }) }), 6_isize)
        );
        (*r.borrow_mut()) = ({
            let ___x: Value<C2Int> = Rc::new(RefCell::new(C2Int::new_1({ 8 })));
            (*m.borrow()).upper_bound_75_struct_C2Int(___x.as_pointer())
        });
        assert!(
            (*r.borrow()) == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin_20() }) }), 6_isize)
        );
        (*r.borrow_mut()) = ({
            let ___x: Value<C2Int> = Rc::new(RefCell::new(C2Int::new_1({ 9 })));
            (*m.borrow()).upper_bound_75_struct_C2Int(___x.as_pointer())
        });
        assert!(
            (*r.borrow()) == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin_20() }) }), 9_isize)
        );
        (*r.borrow_mut()) = ({
            let ___x: Value<C2Int> = Rc::new(RefCell::new(C2Int::new_1({ 10 })));
            (*m.borrow()).upper_bound_75_struct_C2Int(___x.as_pointer())
        });
    }
    {
        let m : Value<std_multimap_PrivateConstructor__double__std_less_void___std_allocator_std_pair_const_PrivateConstructor__double___ > = Rc::new(RefCell::new(<std_multimap_PrivateConstructor__double__std_less_void___std_allocator_std_pair_const_PrivateConstructor__double___  >::default() )) ;
        ({
            let ___x: Value<(Value<PrivateConstructor>, Value<f64>)> = Rc::new(RefCell::new(
                ({
                    let ___x: Value<PrivateConstructor> =
                        Rc::new(RefCell::new(({ PrivateConstructor::make(5) })));
                    let ___y: Value<f64> = Rc::new(RefCell::new(1_f64));
                    libcc2rs::make_pair_refcount(___x.as_pointer(), ___y.as_pointer())
                }),
            ));
            (*m.borrow())
                .insert_39_struct_std_pair_struct_PrivateConstructor__double_(___x.as_pointer())
        });
        ({
            let ___x: Value<(Value<PrivateConstructor>, Value<f64>)> = Rc::new(RefCell::new(
                ({
                    let ___x: Value<PrivateConstructor> =
                        Rc::new(RefCell::new(({ PrivateConstructor::make(5) })));
                    let ___y: Value<f64> = Rc::new(RefCell::new(2_f64));
                    libcc2rs::make_pair_refcount(___x.as_pointer(), ___y.as_pointer())
                }),
            ));
            (*m.borrow())
                .insert_39_struct_std_pair_struct_PrivateConstructor__double_(___x.as_pointer())
        });
        ({
            let ___x: Value<(Value<PrivateConstructor>, Value<f64>)> = Rc::new(RefCell::new(
                ({
                    let ___x: Value<PrivateConstructor> =
                        Rc::new(RefCell::new(({ PrivateConstructor::make(5) })));
                    let ___y: Value<f64> = Rc::new(RefCell::new(3_f64));
                    libcc2rs::make_pair_refcount(___x.as_pointer(), ___y.as_pointer())
                }),
            ));
            (*m.borrow())
                .insert_39_struct_std_pair_struct_PrivateConstructor__double_(___x.as_pointer())
        });
        ({
            let ___x: Value<(Value<PrivateConstructor>, Value<f64>)> = Rc::new(RefCell::new(
                ({
                    let ___x: Value<PrivateConstructor> =
                        Rc::new(RefCell::new(({ PrivateConstructor::make(7) })));
                    let ___y: Value<f64> = Rc::new(RefCell::new(1_f64));
                    libcc2rs::make_pair_refcount(___x.as_pointer(), ___y.as_pointer())
                }),
            ));
            (*m.borrow())
                .insert_39_struct_std_pair_struct_PrivateConstructor__double_(___x.as_pointer())
        });
        ({
            let ___x: Value<(Value<PrivateConstructor>, Value<f64>)> = Rc::new(RefCell::new(
                ({
                    let ___x: Value<PrivateConstructor> =
                        Rc::new(RefCell::new(({ PrivateConstructor::make(7) })));
                    let ___y: Value<f64> = Rc::new(RefCell::new(2_f64));
                    libcc2rs::make_pair_refcount(___x.as_pointer(), ___y.as_pointer())
                }),
            ));
            (*m.borrow())
                .insert_39_struct_std_pair_struct_PrivateConstructor__double_(___x.as_pointer())
        });
        ({
            let ___x: Value<(Value<PrivateConstructor>, Value<f64>)> = Rc::new(RefCell::new(
                ({
                    let ___x: Value<PrivateConstructor> =
                        Rc::new(RefCell::new(({ PrivateConstructor::make(7) })));
                    let ___y: Value<f64> = Rc::new(RefCell::new(3_f64));
                    libcc2rs::make_pair_refcount(___x.as_pointer(), ___y.as_pointer())
                }),
            ));
            (*m.borrow())
                .insert_39_struct_std_pair_struct_PrivateConstructor__double_(___x.as_pointer())
        });
        ({
            let ___x: Value<(Value<PrivateConstructor>, Value<f64>)> = Rc::new(RefCell::new(
                ({
                    let ___x: Value<PrivateConstructor> =
                        Rc::new(RefCell::new(({ PrivateConstructor::make(9) })));
                    let ___y: Value<f64> = Rc::new(RefCell::new(1_f64));
                    libcc2rs::make_pair_refcount(___x.as_pointer(), ___y.as_pointer())
                }),
            ));
            (*m.borrow())
                .insert_39_struct_std_pair_struct_PrivateConstructor__double_(___x.as_pointer())
        });
        ({
            let ___x: Value<(Value<PrivateConstructor>, Value<f64>)> = Rc::new(RefCell::new(
                ({
                    let ___x: Value<PrivateConstructor> =
                        Rc::new(RefCell::new(({ PrivateConstructor::make(9) })));
                    let ___y: Value<f64> = Rc::new(RefCell::new(2_f64));
                    libcc2rs::make_pair_refcount(___x.as_pointer(), ___y.as_pointer())
                }),
            ));
            (*m.borrow())
                .insert_39_struct_std_pair_struct_PrivateConstructor__double_(___x.as_pointer())
        });
        ({
            let ___x: Value<(Value<PrivateConstructor>, Value<f64>)> = Rc::new(RefCell::new(
                ({
                    let ___x: Value<PrivateConstructor> =
                        Rc::new(RefCell::new(({ PrivateConstructor::make(9) })));
                    let ___y: Value<f64> = Rc::new(RefCell::new(3_f64));
                    libcc2rs::make_pair_refcount(___x.as_pointer(), ___y.as_pointer())
                }),
            ));
            (*m.borrow())
                .insert_39_struct_std_pair_struct_PrivateConstructor__double_(___x.as_pointer())
        });
        let r: Value<BidirectionalIterator<(Value<PrivateConstructor>, Value<f64>)>> =
            Rc::new(RefCell::new(
                ({
                    let ___x: Value<i32> = Rc::new(RefCell::new(4));
                    (*m.borrow()).upper_bound_75_int(___x.as_pointer())
                }),
            ));
        assert!((*r.borrow()) == ({ (*m.borrow()).begin_20() }));
        (*r.borrow_mut()) = ({
            let ___x: Value<i32> = Rc::new(RefCell::new(5));
            (*m.borrow()).upper_bound_75_int(___x.as_pointer())
        });
        assert!(
            (*r.borrow()) == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin_20() }) }), 3_isize)
        );
        (*r.borrow_mut()) = ({
            let ___x: Value<i32> = Rc::new(RefCell::new(6));
            (*m.borrow()).upper_bound_75_int(___x.as_pointer())
        });
        assert!(
            (*r.borrow()) == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin_20() }) }), 3_isize)
        );
        (*r.borrow_mut()) = ({
            let ___x: Value<i32> = Rc::new(RefCell::new(7));
            (*m.borrow()).upper_bound_75_int(___x.as_pointer())
        });
        assert!(
            (*r.borrow()) == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin_20() }) }), 6_isize)
        );
        (*r.borrow_mut()) = ({
            let ___x: Value<i32> = Rc::new(RefCell::new(8));
            (*m.borrow()).upper_bound_75_int(___x.as_pointer())
        });
        assert!(
            (*r.borrow()) == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin_20() }) }), 6_isize)
        );
        (*r.borrow_mut()) = ({
            let ___x: Value<i32> = Rc::new(RefCell::new(9));
            (*m.borrow()).upper_bound_75_int(___x.as_pointer())
        });
        assert!(
            (*r.borrow()) == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin_20() }) }), 9_isize)
        );
        (*r.borrow_mut()) = ({
            let ___x: Value<i32> = Rc::new(RefCell::new(10));
            (*m.borrow()).upper_bound_75_int(___x.as_pointer())
        });
        assert!((*r.borrow()) == ({ (*m.borrow()).end_22() }));
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
