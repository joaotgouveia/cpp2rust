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
pub fn test_4() -> bool {
    {
        let ar: Value<Box<[(Value<i32>, Value<f64>)]>> = Rc::new(RefCell::new(Box::new([
            (Rc::new(RefCell::new(1)), Rc::new(RefCell::new((1 as f64)))),
            (Rc::new(RefCell::new(1)), Rc::new(RefCell::new(1.5_f64))),
            (Rc::new(RefCell::new(1)), Rc::new(RefCell::new((2 as f64)))),
            (Rc::new(RefCell::new(2)), Rc::new(RefCell::new((1 as f64)))),
            (Rc::new(RefCell::new(2)), Rc::new(RefCell::new(1.5_f64))),
            (Rc::new(RefCell::new(2)), Rc::new(RefCell::new((2 as f64)))),
            (Rc::new(RefCell::new(3)), Rc::new(RefCell::new((1 as f64)))),
            (Rc::new(RefCell::new(3)), Rc::new(RefCell::new(1.5_f64))),
            (Rc::new(RefCell::new(3)), Rc::new(RefCell::new((2 as f64)))),
            (Rc::new(RefCell::new(4)), Rc::new(RefCell::new((1 as f64)))),
            (Rc::new(RefCell::new(4)), Rc::new(RefCell::new(1.5_f64))),
            (Rc::new(RefCell::new(4)), Rc::new(RefCell::new((2 as f64)))),
            (Rc::new(RefCell::new(5)), Rc::new(RefCell::new((1 as f64)))),
            (Rc::new(RefCell::new(5)), Rc::new(RefCell::new(1.5_f64))),
            (Rc::new(RefCell::new(5)), Rc::new(RefCell::new((2 as f64)))),
            (Rc::new(RefCell::new(6)), Rc::new(RefCell::new((1 as f64)))),
            (Rc::new(RefCell::new(6)), Rc::new(RefCell::new(1.5_f64))),
            (Rc::new(RefCell::new(6)), Rc::new(RefCell::new((2 as f64)))),
            (Rc::new(RefCell::new(7)), Rc::new(RefCell::new((1 as f64)))),
            (Rc::new(RefCell::new(7)), Rc::new(RefCell::new(1.5_f64))),
            (Rc::new(RefCell::new(7)), Rc::new(RefCell::new((2 as f64)))),
            (Rc::new(RefCell::new(8)), Rc::new(RefCell::new((1 as f64)))),
            (Rc::new(RefCell::new(8)), Rc::new(RefCell::new(1.5_f64))),
            (Rc::new(RefCell::new(8)), Rc::new(RefCell::new((2 as f64)))),
        ])));
        let m: Value<Map<i32, (Value<i32>, Value<f64>)>> = Rc::new(RefCell::new(
            std_map_int__double__std_less_int___std_allocator_std_pair_const_int__double___::new_1(
                { (ar.as_pointer() as Ptr<(Value<i32>, Value<f64>)>) },
                {
                    (ar.as_pointer() as Ptr<(Value<i32>, Value<f64>)>)
                        .offset(((384usize as usize).wrapping_div((16usize as usize))) as isize)
                },
            ),
        ));
        assert!(
            ({
                ((({
                    let ___first: BidirectionalIterator<(Value<i32>, Value<f64>)> = m
                        .as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin());
                    let ___last: BidirectionalIterator<(Value<i32>, Value<f64>)> = m
                        .as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.end());
                    distance_5(___first, ___last)
                }) as usize)
                    == (*m.borrow()).len())
            })
        );
        assert!(
            ({
                ((({
                    let ___first: BidirectionalIterator<(Value<i32>, Value<f64>)> =
                        ({ (*m.borrow()).rbegin_24() });
                    let ___last: BidirectionalIterator<(Value<i32>, Value<f64>)> =
                        ({ (*m.borrow()).rend_26() });
                    distance_6(___first, ___last)
                }) as usize)
                    == (*m.borrow()).len())
            })
        );
        let i: Value<BidirectionalIterator<(Value<i32>, Value<f64>)>> =
            Rc::new(RefCell::new(BidirectionalIterator::default()));
        {
            let __a1 = m
                .as_pointer()
                .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin());
            i.as_pointer().with_mut(|__it| *__it = __a1);
            i.as_pointer()
        };
        let k: Value<BidirectionalIterator<(Value<i32>, Value<f64>)>> = Rc::new(RefCell::new(
            <BidirectionalIterator<(Value<i32>, Value<f64>)>>::from(({ (*i.borrow()).clone() })),
        ));
        assert!(
            <BidirectionalIterator<(Value<i32>, Value<f64>)>>::from(({ (*i.borrow()).clone() }))
                == (*k.borrow())
        );
        let mut j: i32 = 1;
        'loop_: while ((j as usize) <= (*m.borrow()).len()) {
            assert!(({ ((*(*(*i.borrow()).iter_deref().upgrade().deref()).0.borrow()) == j) }));
            assert!(({ ((*(*(*i.borrow()).iter_deref().upgrade().deref()).1.borrow()) == 1_f64) }));
            (*(*(*i.borrow()).iter_deref().upgrade().deref())
                .1
                .borrow_mut()) = 2.5_f64;
            assert!(
                ({ ((*(*(*i.borrow()).iter_deref().upgrade().deref()).1.borrow()) == 2.5_f64) })
            );
            {
                j.prefix_inc();
                {
                    i.as_pointer().with_mut(|__it| {
                        __it.iter_inc();
                    });
                    i.as_pointer()
                }
            };
        }
        assert!(
            (*i.borrow())
                == m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.end())
        );
        let mut j: i32 = ((*m.borrow()).len() as i32);
        'loop_: while (j >= 1) {
            {
                i.as_pointer().with_mut(|__it| {
                    __it.iter_dec();
                });
                i.as_pointer()
            };
            assert!(({ ((*(*(*i.borrow()).iter_deref().upgrade().deref()).0.borrow()) == j) }));
            assert!(
                ({ ((*(*(*i.borrow()).iter_deref().upgrade().deref()).1.borrow()) == 2.5_f64) })
            );
            (*(*(*i.borrow()).iter_deref().upgrade().deref())
                .1
                .borrow_mut()) = 1_f64;
            assert!(({ ((*(*(*i.borrow()).iter_deref().upgrade().deref()).1.borrow()) == 1_f64) }));
            j.prefix_dec();
        }
        assert!(
            (*i.borrow())
                == m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin())
        );
    }
    {
        let ar: Value<Box<[(Value<i32>, Value<f64>)]>> = Rc::new(RefCell::new(Box::new([
            (Rc::new(RefCell::new(1)), Rc::new(RefCell::new((1 as f64)))),
            (Rc::new(RefCell::new(1)), Rc::new(RefCell::new(1.5_f64))),
            (Rc::new(RefCell::new(1)), Rc::new(RefCell::new((2 as f64)))),
            (Rc::new(RefCell::new(2)), Rc::new(RefCell::new((1 as f64)))),
            (Rc::new(RefCell::new(2)), Rc::new(RefCell::new(1.5_f64))),
            (Rc::new(RefCell::new(2)), Rc::new(RefCell::new((2 as f64)))),
            (Rc::new(RefCell::new(3)), Rc::new(RefCell::new((1 as f64)))),
            (Rc::new(RefCell::new(3)), Rc::new(RefCell::new(1.5_f64))),
            (Rc::new(RefCell::new(3)), Rc::new(RefCell::new((2 as f64)))),
            (Rc::new(RefCell::new(4)), Rc::new(RefCell::new((1 as f64)))),
            (Rc::new(RefCell::new(4)), Rc::new(RefCell::new(1.5_f64))),
            (Rc::new(RefCell::new(4)), Rc::new(RefCell::new((2 as f64)))),
            (Rc::new(RefCell::new(5)), Rc::new(RefCell::new((1 as f64)))),
            (Rc::new(RefCell::new(5)), Rc::new(RefCell::new(1.5_f64))),
            (Rc::new(RefCell::new(5)), Rc::new(RefCell::new((2 as f64)))),
            (Rc::new(RefCell::new(6)), Rc::new(RefCell::new((1 as f64)))),
            (Rc::new(RefCell::new(6)), Rc::new(RefCell::new(1.5_f64))),
            (Rc::new(RefCell::new(6)), Rc::new(RefCell::new((2 as f64)))),
            (Rc::new(RefCell::new(7)), Rc::new(RefCell::new((1 as f64)))),
            (Rc::new(RefCell::new(7)), Rc::new(RefCell::new(1.5_f64))),
            (Rc::new(RefCell::new(7)), Rc::new(RefCell::new((2 as f64)))),
            (Rc::new(RefCell::new(8)), Rc::new(RefCell::new((1 as f64)))),
            (Rc::new(RefCell::new(8)), Rc::new(RefCell::new(1.5_f64))),
            (Rc::new(RefCell::new(8)), Rc::new(RefCell::new((2 as f64)))),
        ])));
        let m: Value<Map<i32, (Value<i32>, Value<f64>)>> = Rc::new(RefCell::new(
            std_map_int__double__std_less_int___std_allocator_std_pair_const_int__double___::new_1(
                { (ar.as_pointer() as Ptr<(Value<i32>, Value<f64>)>) },
                {
                    (ar.as_pointer() as Ptr<(Value<i32>, Value<f64>)>)
                        .offset(((384usize as usize).wrapping_div((16usize as usize))) as isize)
                },
            ),
        ));
        assert!(
            ({
                ((({
                    let ___first: BidirectionalIterator<(Value<i32>, Value<f64>)> = m
                        .as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin());
                    let ___last: BidirectionalIterator<(Value<i32>, Value<f64>)> = m
                        .as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.end());
                    distance_7(___first, ___last)
                }) as usize)
                    == (*m.borrow()).len())
            })
        );
        assert!(
            ({
                ((({
                    let ___first: BidirectionalIterator<(Value<i32>, Value<f64>)> =
                        ({ (*m.borrow()).cbegin() });
                    let ___last: BidirectionalIterator<(Value<i32>, Value<f64>)> =
                        ({ (*m.borrow()).cend() });
                    distance_7(___first, ___last)
                }) as usize)
                    == (*m.borrow()).len())
            })
        );
        assert!(
            ({
                ((({
                    let ___first: BidirectionalIterator<(Value<i32>, Value<f64>)> =
                        ({ (*m.borrow()).rbegin_25() });
                    let ___last: BidirectionalIterator<(Value<i32>, Value<f64>)> =
                        ({ (*m.borrow()).rend_27() });
                    distance_8(___first, ___last)
                }) as usize)
                    == (*m.borrow()).len())
            })
        );
        assert!(
            ({
                ((({
                    let ___first: BidirectionalIterator<(Value<i32>, Value<f64>)> =
                        ({ (*m.borrow()).crbegin() });
                    let ___last: BidirectionalIterator<(Value<i32>, Value<f64>)> =
                        ({ (*m.borrow()).crend() });
                    distance_8(___first, ___last)
                }) as usize)
                    == (*m.borrow()).len())
            })
        );
        let i: Value<BidirectionalIterator<(Value<i32>, Value<f64>)>> =
            Rc::new(RefCell::new(BidirectionalIterator::default()));
        {
            let __a1 = m
                .as_pointer()
                .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin());
            i.as_pointer().with_mut(|__it| *__it = __a1);
            i.as_pointer()
        };
        let mut j: i32 = 1;
        'loop_: while ((j as usize) <= (*m.borrow()).len()) {
            assert!(({ ((*(*(*i.borrow()).iter_deref().upgrade().deref()).0.borrow()) == j) }));
            assert!(({ ((*(*(*i.borrow()).iter_deref().upgrade().deref()).1.borrow()) == 1_f64) }));
            {
                j.prefix_inc();
                {
                    i.as_pointer().with_mut(|__it| {
                        __it.iter_inc();
                    });
                    i.as_pointer()
                }
            };
        }
        assert!(
            (*i.borrow())
                == m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.end())
        );
        let mut j: i32 = ((*m.borrow()).len() as i32);
        'loop_: while (j >= 1) {
            {
                i.as_pointer().with_mut(|__it| {
                    __it.iter_dec();
                });
                i.as_pointer()
            };
            assert!(({ ((*(*(*i.borrow()).iter_deref().upgrade().deref()).0.borrow()) == j) }));
            assert!(({ ((*(*(*i.borrow()).iter_deref().upgrade().deref()).1.borrow()) == 1_f64) }));
            j.prefix_dec();
        }
        assert!(
            (*i.borrow())
                == m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin())
        );
    }
    {
        let ar: Value<Box<[(Value<i32>, Value<f64>)]>> = Rc::new(RefCell::new(Box::new([
            (Rc::new(RefCell::new(1)), Rc::new(RefCell::new((1 as f64)))),
            (Rc::new(RefCell::new(1)), Rc::new(RefCell::new(1.5_f64))),
            (Rc::new(RefCell::new(1)), Rc::new(RefCell::new((2 as f64)))),
            (Rc::new(RefCell::new(2)), Rc::new(RefCell::new((1 as f64)))),
            (Rc::new(RefCell::new(2)), Rc::new(RefCell::new(1.5_f64))),
            (Rc::new(RefCell::new(2)), Rc::new(RefCell::new((2 as f64)))),
            (Rc::new(RefCell::new(3)), Rc::new(RefCell::new((1 as f64)))),
            (Rc::new(RefCell::new(3)), Rc::new(RefCell::new(1.5_f64))),
            (Rc::new(RefCell::new(3)), Rc::new(RefCell::new((2 as f64)))),
            (Rc::new(RefCell::new(4)), Rc::new(RefCell::new((1 as f64)))),
            (Rc::new(RefCell::new(4)), Rc::new(RefCell::new(1.5_f64))),
            (Rc::new(RefCell::new(4)), Rc::new(RefCell::new((2 as f64)))),
            (Rc::new(RefCell::new(5)), Rc::new(RefCell::new((1 as f64)))),
            (Rc::new(RefCell::new(5)), Rc::new(RefCell::new(1.5_f64))),
            (Rc::new(RefCell::new(5)), Rc::new(RefCell::new((2 as f64)))),
            (Rc::new(RefCell::new(6)), Rc::new(RefCell::new((1 as f64)))),
            (Rc::new(RefCell::new(6)), Rc::new(RefCell::new(1.5_f64))),
            (Rc::new(RefCell::new(6)), Rc::new(RefCell::new((2 as f64)))),
            (Rc::new(RefCell::new(7)), Rc::new(RefCell::new((1 as f64)))),
            (Rc::new(RefCell::new(7)), Rc::new(RefCell::new(1.5_f64))),
            (Rc::new(RefCell::new(7)), Rc::new(RefCell::new((2 as f64)))),
            (Rc::new(RefCell::new(8)), Rc::new(RefCell::new((1 as f64)))),
            (Rc::new(RefCell::new(8)), Rc::new(RefCell::new(1.5_f64))),
            (Rc::new(RefCell::new(8)), Rc::new(RefCell::new((2 as f64)))),
        ])));
        let m: Value<Map<i32, (Value<i32>, Value<f64>)>> = Rc::new(RefCell::new(
            std_map_int__double__std_less_int___min_allocator_std_pair_const_int__double___::new_1(
                { (ar.as_pointer() as Ptr<(Value<i32>, Value<f64>)>) },
                {
                    (ar.as_pointer() as Ptr<(Value<i32>, Value<f64>)>)
                        .offset(((384usize as usize).wrapping_div((16usize as usize))) as isize)
                },
            ),
        ));
        assert!(
            ({
                ((({
                    let ___first: BidirectionalIterator<(Value<i32>, Value<f64>)> = m
                        .as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin());
                    let ___last: BidirectionalIterator<(Value<i32>, Value<f64>)> = m
                        .as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.end());
                    distance_9(___first, ___last)
                }) as usize)
                    == (*m.borrow()).len())
            })
        );
        assert!(
            ({
                ((({
                    let ___first: BidirectionalIterator<(Value<i32>, Value<f64>)> =
                        ({ (*m.borrow()).rbegin_24() });
                    let ___last: BidirectionalIterator<(Value<i32>, Value<f64>)> =
                        ({ (*m.borrow()).rend_26() });
                    distance_10(___first, ___last)
                }) as usize)
                    == (*m.borrow()).len())
            })
        );
        let i: Value<BidirectionalIterator<(Value<i32>, Value<f64>)>> =
            Rc::new(RefCell::new(BidirectionalIterator::default()));
        {
            let __a1 = m
                .as_pointer()
                .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin());
            i.as_pointer().with_mut(|__it| *__it = __a1.clone());
            i.as_pointer()
        };
        let k: Value<BidirectionalIterator<(Value<i32>, Value<f64>)>> = Rc::new(RefCell::new(
            <BidirectionalIterator<(Value<i32>, Value<f64>)>>::from(({ (*i.borrow()).clone() })),
        ));
        assert!(
            <BidirectionalIterator<(Value<i32>, Value<f64>)>>::from(({ (*i.borrow()).clone() }))
                == (*k.borrow())
        );
        let mut j: i32 = 1;
        'loop_: while ((j as usize) <= (*m.borrow()).len()) {
            assert!(({ ((*(*(*i.borrow()).iter_deref().upgrade().deref()).0.borrow()) == j) }));
            assert!(({ ((*(*(*i.borrow()).iter_deref().upgrade().deref()).1.borrow()) == 1_f64) }));
            (*(*(*i.borrow()).iter_deref().upgrade().deref())
                .1
                .borrow_mut()) = 2.5_f64;
            assert!(
                ({ ((*(*(*i.borrow()).iter_deref().upgrade().deref()).1.borrow()) == 2.5_f64) })
            );
            {
                j.prefix_inc();
                {
                    i.as_pointer().with_mut(|__it| {
                        __it.iter_inc();
                    });
                    i.as_pointer()
                }
            };
        }
        assert!(
            (*i.borrow())
                == m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.end())
        );
        let mut j: i32 = ((*m.borrow()).len() as i32);
        'loop_: while (j >= 1) {
            {
                i.as_pointer().with_mut(|__it| {
                    __it.iter_dec();
                });
                i.as_pointer()
            };
            assert!(({ ((*(*(*i.borrow()).iter_deref().upgrade().deref()).0.borrow()) == j) }));
            assert!(
                ({ ((*(*(*i.borrow()).iter_deref().upgrade().deref()).1.borrow()) == 2.5_f64) })
            );
            (*(*(*i.borrow()).iter_deref().upgrade().deref())
                .1
                .borrow_mut()) = 1_f64;
            assert!(({ ((*(*(*i.borrow()).iter_deref().upgrade().deref()).1.borrow()) == 1_f64) }));
            j.prefix_dec();
        }
        assert!(
            (*i.borrow())
                == m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin())
        );
    }
    {
        let ar: Value<Box<[(Value<i32>, Value<f64>)]>> = Rc::new(RefCell::new(Box::new([
            (Rc::new(RefCell::new(1)), Rc::new(RefCell::new((1 as f64)))),
            (Rc::new(RefCell::new(1)), Rc::new(RefCell::new(1.5_f64))),
            (Rc::new(RefCell::new(1)), Rc::new(RefCell::new((2 as f64)))),
            (Rc::new(RefCell::new(2)), Rc::new(RefCell::new((1 as f64)))),
            (Rc::new(RefCell::new(2)), Rc::new(RefCell::new(1.5_f64))),
            (Rc::new(RefCell::new(2)), Rc::new(RefCell::new((2 as f64)))),
            (Rc::new(RefCell::new(3)), Rc::new(RefCell::new((1 as f64)))),
            (Rc::new(RefCell::new(3)), Rc::new(RefCell::new(1.5_f64))),
            (Rc::new(RefCell::new(3)), Rc::new(RefCell::new((2 as f64)))),
            (Rc::new(RefCell::new(4)), Rc::new(RefCell::new((1 as f64)))),
            (Rc::new(RefCell::new(4)), Rc::new(RefCell::new(1.5_f64))),
            (Rc::new(RefCell::new(4)), Rc::new(RefCell::new((2 as f64)))),
            (Rc::new(RefCell::new(5)), Rc::new(RefCell::new((1 as f64)))),
            (Rc::new(RefCell::new(5)), Rc::new(RefCell::new(1.5_f64))),
            (Rc::new(RefCell::new(5)), Rc::new(RefCell::new((2 as f64)))),
            (Rc::new(RefCell::new(6)), Rc::new(RefCell::new((1 as f64)))),
            (Rc::new(RefCell::new(6)), Rc::new(RefCell::new(1.5_f64))),
            (Rc::new(RefCell::new(6)), Rc::new(RefCell::new((2 as f64)))),
            (Rc::new(RefCell::new(7)), Rc::new(RefCell::new((1 as f64)))),
            (Rc::new(RefCell::new(7)), Rc::new(RefCell::new(1.5_f64))),
            (Rc::new(RefCell::new(7)), Rc::new(RefCell::new((2 as f64)))),
            (Rc::new(RefCell::new(8)), Rc::new(RefCell::new((1 as f64)))),
            (Rc::new(RefCell::new(8)), Rc::new(RefCell::new(1.5_f64))),
            (Rc::new(RefCell::new(8)), Rc::new(RefCell::new((2 as f64)))),
        ])));
        let m: Value<Map<i32, (Value<i32>, Value<f64>)>> = Rc::new(RefCell::new(
            std_map_int__double__std_less_int___min_allocator_std_pair_const_int__double___::new_1(
                { (ar.as_pointer() as Ptr<(Value<i32>, Value<f64>)>) },
                {
                    (ar.as_pointer() as Ptr<(Value<i32>, Value<f64>)>)
                        .offset(((384usize as usize).wrapping_div((16usize as usize))) as isize)
                },
            ),
        ));
        assert!(
            ({
                ((({
                    let ___first: BidirectionalIterator<(Value<i32>, Value<f64>)> = m
                        .as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin());
                    let ___last: BidirectionalIterator<(Value<i32>, Value<f64>)> = m
                        .as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.end());
                    distance_11(___first, ___last)
                }) as usize)
                    == (*m.borrow()).len())
            })
        );
        assert!(
            ({
                ((({
                    let ___first: BidirectionalIterator<(Value<i32>, Value<f64>)> =
                        ({ (*m.borrow()).cbegin() });
                    let ___last: BidirectionalIterator<(Value<i32>, Value<f64>)> =
                        ({ (*m.borrow()).cend() });
                    distance_11(___first, ___last)
                }) as usize)
                    == (*m.borrow()).len())
            })
        );
        assert!(
            ({
                ((({
                    let ___first: BidirectionalIterator<(Value<i32>, Value<f64>)> =
                        ({ (*m.borrow()).rbegin_25() });
                    let ___last: BidirectionalIterator<(Value<i32>, Value<f64>)> =
                        ({ (*m.borrow()).rend_27() });
                    distance_12(___first, ___last)
                }) as usize)
                    == (*m.borrow()).len())
            })
        );
        assert!(
            ({
                ((({
                    let ___first: BidirectionalIterator<(Value<i32>, Value<f64>)> =
                        ({ (*m.borrow()).crbegin() });
                    let ___last: BidirectionalIterator<(Value<i32>, Value<f64>)> =
                        ({ (*m.borrow()).crend() });
                    distance_12(___first, ___last)
                }) as usize)
                    == (*m.borrow()).len())
            })
        );
        let i: Value<BidirectionalIterator<(Value<i32>, Value<f64>)>> =
            Rc::new(RefCell::new(BidirectionalIterator::default()));
        {
            let __a1 = m
                .as_pointer()
                .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin());
            i.as_pointer().with_mut(|__it| *__it = __a1.clone());
            i.as_pointer()
        };
        let mut j: i32 = 1;
        'loop_: while ((j as usize) <= (*m.borrow()).len()) {
            assert!(({ ((*(*(*i.borrow()).iter_deref().upgrade().deref()).0.borrow()) == j) }));
            assert!(({ ((*(*(*i.borrow()).iter_deref().upgrade().deref()).1.borrow()) == 1_f64) }));
            {
                j.prefix_inc();
                {
                    i.as_pointer().with_mut(|__it| {
                        __it.iter_inc();
                    });
                    i.as_pointer()
                }
            };
        }
        assert!(
            (*i.borrow())
                == m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.end())
        );
        let mut j: i32 = ((*m.borrow()).len() as i32);
        'loop_: while (j >= 1) {
            {
                i.as_pointer().with_mut(|__it| {
                    __it.iter_dec();
                });
                i.as_pointer()
            };
            assert!(({ ((*(*(*i.borrow()).iter_deref().upgrade().deref()).0.borrow()) == j) }));
            assert!(({ ((*(*(*i.borrow()).iter_deref().upgrade().deref()).1.borrow()) == 1_f64) }));
            j.prefix_dec();
        }
        assert!(
            (*i.borrow())
                == m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin())
        );
    }
    {
        let ii1: Value<BidirectionalIterator<(Value<i32>, Value<f64>)>> =
            Rc::new(RefCell::new(BidirectionalIterator::default()));
        let ii2: Value<BidirectionalIterator<(Value<i32>, Value<f64>)>> =
            Rc::new(RefCell::new(BidirectionalIterator::default()));
        let ii4: Value<BidirectionalIterator<(Value<i32>, Value<f64>)>> =
            Rc::new(RefCell::new((*ii1.borrow()).clone()));
        let cii: Value<BidirectionalIterator<(Value<i32>, Value<f64>)>> =
            Rc::new(RefCell::new(BidirectionalIterator::default()));
        assert!(({ (*ii1.borrow()) == (*ii2.borrow()) }));
        assert!(({ (*ii1.borrow()) == (*ii4.borrow()) }));
        assert!(({ !((*ii1.borrow()) != (*ii2.borrow())) }));
        assert!(
            (<BidirectionalIterator<(Value<i32>, Value<f64>)>>::from(
                ({ (*ii1.borrow()).clone() })
            ) == (*cii.borrow()))
        );
        assert!(
            ((*cii.borrow())
                == <BidirectionalIterator<(Value<i32>, Value<f64>)>>::from(
                    ({ (*ii1.borrow()).clone() })
                ))
        );
        assert!(
            !(<BidirectionalIterator<(Value<i32>, Value<f64>)>>::from(
                ({ (*ii1.borrow()).clone() })
            ) != (*cii.borrow()))
        );
        assert!(
            !((*cii.borrow())
                != <BidirectionalIterator<(Value<i32>, Value<f64>)>>::from(
                    ({ (*ii1.borrow()).clone() })
                ))
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
    ({ test_4() });
    return 0;
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__;
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
