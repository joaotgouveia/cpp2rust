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
pub struct min_allocator_int_ {}
impl CppMove for min_allocator_int_ {
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
pub fn test_7() -> bool {
    {
        {
            let ar: Value<Box<[i32]>> =
                Rc::new(RefCell::new(Box::new([5, 6, 7, 8, 9, 10, 11, 12])));
            let m: Value<std_set_int__std_less_int___std_allocator_int__> = Rc::new(RefCell::new(
                std_set_int__std_less_int___std_allocator_int__::new_1(
                    { (ar.as_pointer() as Ptr<i32>) },
                    {
                        (ar.as_pointer() as Ptr<i32>).offset(
                            ((::std::mem::size_of::<[i32; 8]>() as usize)
                                .wrapping_div((::std::mem::size_of::<i32>() as usize)))
                                as isize,
                        )
                    },
                ),
            ));
            let r: Value<BidirectionalIterator<i32>> = Rc::new(RefCell::new(
                ({
                    let ___x: Value<i32> = Rc::new(RefCell::new(5));
                    (*m.borrow()).find_59(___x.as_pointer())
                }),
            ));
            assert!((*r.borrow()) == ({ (*m.borrow()).begin() }));
            {
                let __a1 = ({
                    let ___x: Value<i32> = Rc::new(RefCell::new(6));
                    (*m.borrow()).find_59(___x.as_pointer())
                });
                r.as_pointer().with_mut(|__it| *__it = __a1);
                r.as_pointer()
            };
            assert!(
                (*r.borrow())
                    == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin() }) }), 1_isize as isize)
            );
            {
                let __a1 = ({
                    let ___x: Value<i32> = Rc::new(RefCell::new(7));
                    (*m.borrow()).find_59(___x.as_pointer())
                });
                r.as_pointer().with_mut(|__it| *__it = __a1);
                r.as_pointer()
            };
            assert!(
                (*r.borrow())
                    == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin() }) }), 2_isize as isize)
            );
            {
                let __a1 = ({
                    let ___x: Value<i32> = Rc::new(RefCell::new(8));
                    (*m.borrow()).find_59(___x.as_pointer())
                });
                r.as_pointer().with_mut(|__it| *__it = __a1);
                r.as_pointer()
            };
            assert!(
                (*r.borrow())
                    == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin() }) }), 3_isize as isize)
            );
            {
                let __a1 = ({
                    let ___x: Value<i32> = Rc::new(RefCell::new(9));
                    (*m.borrow()).find_59(___x.as_pointer())
                });
                r.as_pointer().with_mut(|__it| *__it = __a1);
                r.as_pointer()
            };
            assert!(
                (*r.borrow())
                    == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin() }) }), 4_isize as isize)
            );
            {
                let __a1 = ({
                    let ___x: Value<i32> = Rc::new(RefCell::new(10));
                    (*m.borrow()).find_59(___x.as_pointer())
                });
                r.as_pointer().with_mut(|__it| *__it = __a1);
                r.as_pointer()
            };
            assert!(
                (*r.borrow())
                    == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin() }) }), 5_isize as isize)
            );
            {
                let __a1 = ({
                    let ___x: Value<i32> = Rc::new(RefCell::new(11));
                    (*m.borrow()).find_59(___x.as_pointer())
                });
                r.as_pointer().with_mut(|__it| *__it = __a1);
                r.as_pointer()
            };
            assert!(
                (*r.borrow())
                    == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin() }) }), 6_isize as isize)
            );
            {
                let __a1 = ({
                    let ___x: Value<i32> = Rc::new(RefCell::new(12));
                    (*m.borrow()).find_59(___x.as_pointer())
                });
                r.as_pointer().with_mut(|__it| *__it = __a1);
                r.as_pointer()
            };
            assert!(
                (*r.borrow())
                    == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin() }) }), 7_isize as isize)
            );
            {
                let __a1 = ({
                    let ___x: Value<i32> = Rc::new(RefCell::new(4));
                    (*m.borrow()).find_59(___x.as_pointer())
                });
                r.as_pointer().with_mut(|__it| *__it = __a1);
                r.as_pointer()
            };
            assert!(
                (*r.borrow())
                    == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin() }) }), 8_isize as isize)
            );
        }
        {
            let ar: Value<Box<[i32]>> =
                Rc::new(RefCell::new(Box::new([5, 6, 7, 8, 9, 10, 11, 12])));
            let m: Value<std_set_int__std_less_int___std_allocator_int__> = Rc::new(RefCell::new(
                std_set_int__std_less_int___std_allocator_int__::new_1(
                    { (ar.as_pointer() as Ptr<i32>) },
                    {
                        (ar.as_pointer() as Ptr<i32>).offset(
                            ((::std::mem::size_of::<[i32; 8]>() as usize)
                                .wrapping_div((::std::mem::size_of::<i32>() as usize)))
                                as isize,
                        )
                    },
                ),
            ));
            let r: Value<BidirectionalIterator<i32>> = Rc::new(RefCell::new(
                ({
                    let ___x: Value<i32> = Rc::new(RefCell::new(5));
                    (*m.borrow()).find_60(___x.as_pointer())
                }),
            ));
            assert!((*r.borrow()) == ({ (*m.borrow()).begin() }));
            {
                let __a1 = ({
                    let ___x: Value<i32> = Rc::new(RefCell::new(6));
                    (*m.borrow()).find_60(___x.as_pointer())
                });
                r.as_pointer().with_mut(|__it| *__it = __a1);
                r.as_pointer()
            };
            assert!(
                (*r.borrow())
                    == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin() }) }), 1_isize as isize)
            );
            {
                let __a1 = ({
                    let ___x: Value<i32> = Rc::new(RefCell::new(7));
                    (*m.borrow()).find_60(___x.as_pointer())
                });
                r.as_pointer().with_mut(|__it| *__it = __a1);
                r.as_pointer()
            };
            assert!(
                (*r.borrow())
                    == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin() }) }), 2_isize as isize)
            );
            {
                let __a1 = ({
                    let ___x: Value<i32> = Rc::new(RefCell::new(8));
                    (*m.borrow()).find_60(___x.as_pointer())
                });
                r.as_pointer().with_mut(|__it| *__it = __a1);
                r.as_pointer()
            };
            assert!(
                (*r.borrow())
                    == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin() }) }), 3_isize as isize)
            );
            {
                let __a1 = ({
                    let ___x: Value<i32> = Rc::new(RefCell::new(9));
                    (*m.borrow()).find_60(___x.as_pointer())
                });
                r.as_pointer().with_mut(|__it| *__it = __a1);
                r.as_pointer()
            };
            assert!(
                (*r.borrow())
                    == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin() }) }), 4_isize as isize)
            );
            {
                let __a1 = ({
                    let ___x: Value<i32> = Rc::new(RefCell::new(10));
                    (*m.borrow()).find_60(___x.as_pointer())
                });
                r.as_pointer().with_mut(|__it| *__it = __a1);
                r.as_pointer()
            };
            assert!(
                (*r.borrow())
                    == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin() }) }), 5_isize as isize)
            );
            {
                let __a1 = ({
                    let ___x: Value<i32> = Rc::new(RefCell::new(11));
                    (*m.borrow()).find_60(___x.as_pointer())
                });
                r.as_pointer().with_mut(|__it| *__it = __a1);
                r.as_pointer()
            };
            assert!(
                (*r.borrow())
                    == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin() }) }), 6_isize as isize)
            );
            {
                let __a1 = ({
                    let ___x: Value<i32> = Rc::new(RefCell::new(12));
                    (*m.borrow()).find_60(___x.as_pointer())
                });
                r.as_pointer().with_mut(|__it| *__it = __a1);
                r.as_pointer()
            };
            assert!(
                (*r.borrow())
                    == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin() }) }), 7_isize as isize)
            );
            {
                let __a1 = ({
                    let ___x: Value<i32> = Rc::new(RefCell::new(4));
                    (*m.borrow()).find_60(___x.as_pointer())
                });
                r.as_pointer().with_mut(|__it| *__it = __a1);
                r.as_pointer()
            };
            assert!(
                (*r.borrow())
                    == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin() }) }), 8_isize as isize)
            );
        }
    }
    {
        let ar: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([5, 6, 7, 8, 9, 10, 11, 12])));
        let m: Value<std_set_int__std_greater_int___std_allocator_int__> = Rc::new(RefCell::new(
            std_set_int__std_greater_int___std_allocator_int__::new_1(
                { (ar.as_pointer() as Ptr<i32>) },
                {
                    (ar.as_pointer() as Ptr<i32>).offset(
                        ((::std::mem::size_of::<[i32; 8]>() as usize)
                            .wrapping_div((::std::mem::size_of::<i32>() as usize)))
                            as isize,
                    )
                },
            ),
        ));
        assert!(
            ({
                let ___x: Value<i32> = Rc::new(RefCell::new(12));
                (*m.borrow()).find_59(___x.as_pointer())
            }) == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin() }) }), 0_isize as isize)
        );
        assert!(
            ({
                let ___x: Value<i32> = Rc::new(RefCell::new(11));
                (*m.borrow()).find_59(___x.as_pointer())
            }) == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin() }) }), 1_isize as isize)
        );
        assert!(
            ({
                let ___x: Value<i32> = Rc::new(RefCell::new(10));
                (*m.borrow()).find_59(___x.as_pointer())
            }) == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin() }) }), 2_isize as isize)
        );
        assert!(
            ({
                let ___x: Value<i32> = Rc::new(RefCell::new(9));
                (*m.borrow()).find_59(___x.as_pointer())
            }) == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin() }) }), 3_isize as isize)
        );
        assert!(
            ({
                let ___x: Value<i32> = Rc::new(RefCell::new(8));
                (*m.borrow()).find_59(___x.as_pointer())
            }) == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin() }) }), 4_isize as isize)
        );
        assert!(
            ({
                let ___x: Value<i32> = Rc::new(RefCell::new(7));
                (*m.borrow()).find_59(___x.as_pointer())
            }) == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin() }) }), 5_isize as isize)
        );
        assert!(
            ({
                let ___x: Value<i32> = Rc::new(RefCell::new(6));
                (*m.borrow()).find_59(___x.as_pointer())
            }) == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin() }) }), 6_isize as isize)
        );
        assert!(
            ({
                let ___x: Value<i32> = Rc::new(RefCell::new(5));
                (*m.borrow()).find_59(___x.as_pointer())
            }) == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin() }) }), 7_isize as isize)
        );
        assert!(
            ({
                let ___x: Value<i32> = Rc::new(RefCell::new(4));
                (*m.borrow()).find_59(___x.as_pointer())
            }) == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin() }) }), 8_isize as isize)
        );
        assert!(
            libcc2rs::next_refcount(({ ({ (*m.borrow()).begin() }) }), 8_isize as isize)
                == ({ (*m.borrow()).end() })
        );
    }
    {
        {
            let ar: Value<Box<[i32]>> =
                Rc::new(RefCell::new(Box::new([5, 6, 7, 8, 9, 10, 11, 12])));
            let m: Value<std_set_int__std_less_int___min_allocator_int__> = Rc::new(RefCell::new(
                std_set_int__std_less_int___min_allocator_int__::new_1(
                    { (ar.as_pointer() as Ptr<i32>) },
                    {
                        (ar.as_pointer() as Ptr<i32>).offset(
                            ((::std::mem::size_of::<[i32; 8]>() as usize)
                                .wrapping_div((::std::mem::size_of::<i32>() as usize)))
                                as isize,
                        )
                    },
                ),
            ));
            let r: Value<BidirectionalIterator<i32>> = Rc::new(RefCell::new(
                ({
                    let ___x: Value<i32> = Rc::new(RefCell::new(5));
                    (*m.borrow()).find_59(___x.as_pointer())
                }),
            ));
            assert!((*r.borrow()) == ({ (*m.borrow()).begin() }));
            {
                let __a1 = ({
                    let ___x: Value<i32> = Rc::new(RefCell::new(6));
                    (*m.borrow()).find_59(___x.as_pointer())
                });
                r.as_pointer().with_mut(|__it| *__it = __a1.clone());
                r.as_pointer()
            };
            assert!(
                (*r.borrow())
                    == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin() }) }), 1_isize as isize)
            );
            {
                let __a1 = ({
                    let ___x: Value<i32> = Rc::new(RefCell::new(7));
                    (*m.borrow()).find_59(___x.as_pointer())
                });
                r.as_pointer().with_mut(|__it| *__it = __a1.clone());
                r.as_pointer()
            };
            assert!(
                (*r.borrow())
                    == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin() }) }), 2_isize as isize)
            );
            {
                let __a1 = ({
                    let ___x: Value<i32> = Rc::new(RefCell::new(8));
                    (*m.borrow()).find_59(___x.as_pointer())
                });
                r.as_pointer().with_mut(|__it| *__it = __a1.clone());
                r.as_pointer()
            };
            assert!(
                (*r.borrow())
                    == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin() }) }), 3_isize as isize)
            );
            {
                let __a1 = ({
                    let ___x: Value<i32> = Rc::new(RefCell::new(9));
                    (*m.borrow()).find_59(___x.as_pointer())
                });
                r.as_pointer().with_mut(|__it| *__it = __a1.clone());
                r.as_pointer()
            };
            assert!(
                (*r.borrow())
                    == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin() }) }), 4_isize as isize)
            );
            {
                let __a1 = ({
                    let ___x: Value<i32> = Rc::new(RefCell::new(10));
                    (*m.borrow()).find_59(___x.as_pointer())
                });
                r.as_pointer().with_mut(|__it| *__it = __a1.clone());
                r.as_pointer()
            };
            assert!(
                (*r.borrow())
                    == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin() }) }), 5_isize as isize)
            );
            {
                let __a1 = ({
                    let ___x: Value<i32> = Rc::new(RefCell::new(11));
                    (*m.borrow()).find_59(___x.as_pointer())
                });
                r.as_pointer().with_mut(|__it| *__it = __a1.clone());
                r.as_pointer()
            };
            assert!(
                (*r.borrow())
                    == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin() }) }), 6_isize as isize)
            );
            {
                let __a1 = ({
                    let ___x: Value<i32> = Rc::new(RefCell::new(12));
                    (*m.borrow()).find_59(___x.as_pointer())
                });
                r.as_pointer().with_mut(|__it| *__it = __a1.clone());
                r.as_pointer()
            };
            assert!(
                (*r.borrow())
                    == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin() }) }), 7_isize as isize)
            );
            {
                let __a1 = ({
                    let ___x: Value<i32> = Rc::new(RefCell::new(4));
                    (*m.borrow()).find_59(___x.as_pointer())
                });
                r.as_pointer().with_mut(|__it| *__it = __a1.clone());
                r.as_pointer()
            };
            assert!(
                (*r.borrow())
                    == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin() }) }), 8_isize as isize)
            );
        }
        {
            let ar: Value<Box<[i32]>> =
                Rc::new(RefCell::new(Box::new([5, 6, 7, 8, 9, 10, 11, 12])));
            let m: Value<std_set_int__std_less_int___min_allocator_int__> = Rc::new(RefCell::new(
                std_set_int__std_less_int___min_allocator_int__::new_1(
                    { (ar.as_pointer() as Ptr<i32>) },
                    {
                        (ar.as_pointer() as Ptr<i32>).offset(
                            ((::std::mem::size_of::<[i32; 8]>() as usize)
                                .wrapping_div((::std::mem::size_of::<i32>() as usize)))
                                as isize,
                        )
                    },
                ),
            ));
            let r: Value<BidirectionalIterator<i32>> = Rc::new(RefCell::new(
                ({
                    let ___x: Value<i32> = Rc::new(RefCell::new(5));
                    (*m.borrow()).find_60(___x.as_pointer())
                }),
            ));
            assert!((*r.borrow()) == ({ (*m.borrow()).begin() }));
            {
                let __a1 = ({
                    let ___x: Value<i32> = Rc::new(RefCell::new(6));
                    (*m.borrow()).find_60(___x.as_pointer())
                });
                r.as_pointer().with_mut(|__it| *__it = __a1.clone());
                r.as_pointer()
            };
            assert!(
                (*r.borrow())
                    == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin() }) }), 1_isize as isize)
            );
            {
                let __a1 = ({
                    let ___x: Value<i32> = Rc::new(RefCell::new(7));
                    (*m.borrow()).find_60(___x.as_pointer())
                });
                r.as_pointer().with_mut(|__it| *__it = __a1.clone());
                r.as_pointer()
            };
            assert!(
                (*r.borrow())
                    == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin() }) }), 2_isize as isize)
            );
            {
                let __a1 = ({
                    let ___x: Value<i32> = Rc::new(RefCell::new(8));
                    (*m.borrow()).find_60(___x.as_pointer())
                });
                r.as_pointer().with_mut(|__it| *__it = __a1.clone());
                r.as_pointer()
            };
            assert!(
                (*r.borrow())
                    == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin() }) }), 3_isize as isize)
            );
            {
                let __a1 = ({
                    let ___x: Value<i32> = Rc::new(RefCell::new(9));
                    (*m.borrow()).find_60(___x.as_pointer())
                });
                r.as_pointer().with_mut(|__it| *__it = __a1.clone());
                r.as_pointer()
            };
            assert!(
                (*r.borrow())
                    == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin() }) }), 4_isize as isize)
            );
            {
                let __a1 = ({
                    let ___x: Value<i32> = Rc::new(RefCell::new(10));
                    (*m.borrow()).find_60(___x.as_pointer())
                });
                r.as_pointer().with_mut(|__it| *__it = __a1.clone());
                r.as_pointer()
            };
            assert!(
                (*r.borrow())
                    == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin() }) }), 5_isize as isize)
            );
            {
                let __a1 = ({
                    let ___x: Value<i32> = Rc::new(RefCell::new(11));
                    (*m.borrow()).find_60(___x.as_pointer())
                });
                r.as_pointer().with_mut(|__it| *__it = __a1.clone());
                r.as_pointer()
            };
            assert!(
                (*r.borrow())
                    == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin() }) }), 6_isize as isize)
            );
            {
                let __a1 = ({
                    let ___x: Value<i32> = Rc::new(RefCell::new(12));
                    (*m.borrow()).find_60(___x.as_pointer())
                });
                r.as_pointer().with_mut(|__it| *__it = __a1.clone());
                r.as_pointer()
            };
            assert!(
                (*r.borrow())
                    == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin() }) }), 7_isize as isize)
            );
            {
                let __a1 = ({
                    let ___x: Value<i32> = Rc::new(RefCell::new(4));
                    (*m.borrow()).find_60(___x.as_pointer())
                });
                r.as_pointer().with_mut(|__it| *__it = __a1.clone());
                r.as_pointer()
            };
            assert!(
                (*r.borrow())
                    == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin() }) }), 8_isize as isize)
            );
        }
    }
    {
        let ar: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([5, 6, 7, 8, 9, 10, 11, 12])));
        let m: Value<std_set_int__std_less_void___std_allocator_int__> = Rc::new(RefCell::new(
            std_set_int__std_less_void___std_allocator_int__::new_1(
                { (ar.as_pointer() as Ptr<i32>) },
                {
                    (ar.as_pointer() as Ptr<i32>).offset(
                        ((::std::mem::size_of::<[i32; 8]>() as usize)
                            .wrapping_div((::std::mem::size_of::<i32>() as usize)))
                            as isize,
                    )
                },
            ),
        ));
        let r: Value<BidirectionalIterator<i32>> = Rc::new(RefCell::new(
            ({
                let ___x: Value<i32> = Rc::new(RefCell::new(5));
                (*m.borrow()).find_59(___x.as_pointer())
            }),
        ));
        assert!((*r.borrow()) == ({ (*m.borrow()).begin() }));
        {
            let __a1 = ({
                let ___x: Value<i32> = Rc::new(RefCell::new(6));
                (*m.borrow()).find_59(___x.as_pointer())
            });
            r.as_pointer().with_mut(|__it| *__it = __a1);
            r.as_pointer()
        };
        assert!(
            (*r.borrow())
                == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin() }) }), 1_isize as isize)
        );
        {
            let __a1 = ({
                let ___x: Value<i32> = Rc::new(RefCell::new(7));
                (*m.borrow()).find_59(___x.as_pointer())
            });
            r.as_pointer().with_mut(|__it| *__it = __a1);
            r.as_pointer()
        };
        assert!(
            (*r.borrow())
                == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin() }) }), 2_isize as isize)
        );
        {
            let __a1 = ({
                let ___x: Value<i32> = Rc::new(RefCell::new(8));
                (*m.borrow()).find_59(___x.as_pointer())
            });
            r.as_pointer().with_mut(|__it| *__it = __a1);
            r.as_pointer()
        };
        assert!(
            (*r.borrow())
                == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin() }) }), 3_isize as isize)
        );
        {
            let __a1 = ({
                let ___x: Value<i32> = Rc::new(RefCell::new(9));
                (*m.borrow()).find_59(___x.as_pointer())
            });
            r.as_pointer().with_mut(|__it| *__it = __a1);
            r.as_pointer()
        };
        assert!(
            (*r.borrow())
                == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin() }) }), 4_isize as isize)
        );
        {
            let __a1 = ({
                let ___x: Value<i32> = Rc::new(RefCell::new(10));
                (*m.borrow()).find_59(___x.as_pointer())
            });
            r.as_pointer().with_mut(|__it| *__it = __a1);
            r.as_pointer()
        };
        assert!(
            (*r.borrow())
                == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin() }) }), 5_isize as isize)
        );
        {
            let __a1 = ({
                let ___x: Value<i32> = Rc::new(RefCell::new(11));
                (*m.borrow()).find_59(___x.as_pointer())
            });
            r.as_pointer().with_mut(|__it| *__it = __a1);
            r.as_pointer()
        };
        assert!(
            (*r.borrow())
                == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin() }) }), 6_isize as isize)
        );
        {
            let __a1 = ({
                let ___x: Value<i32> = Rc::new(RefCell::new(12));
                (*m.borrow()).find_59(___x.as_pointer())
            });
            r.as_pointer().with_mut(|__it| *__it = __a1);
            r.as_pointer()
        };
        assert!(
            (*r.borrow())
                == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin() }) }), 7_isize as isize)
        );
        {
            let __a1 = ({
                let ___x: Value<i32> = Rc::new(RefCell::new(4));
                (*m.borrow()).find_59(___x.as_pointer())
            });
            r.as_pointer().with_mut(|__it| *__it = __a1);
            r.as_pointer()
        };
        assert!(
            (*r.borrow())
                == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin() }) }), 8_isize as isize)
        );
    }
    {
        let m : Value<std_set_PrivateConstructor__std_less_void___std_allocator_PrivateConstructor__ > = Rc::new(RefCell::new(<std_set_PrivateConstructor__std_less_void___std_allocator_PrivateConstructor__  >::default() )) ;
        ({
            let ___x: Value<PrivateConstructor> =
                Rc::new(RefCell::new(({ PrivateConstructor::make(5) })));
            (*m.borrow()).insert_37(___x.as_pointer())
        });
        ({
            let ___x: Value<PrivateConstructor> =
                Rc::new(RefCell::new(({ PrivateConstructor::make(6) })));
            (*m.borrow()).insert_37(___x.as_pointer())
        });
        ({
            let ___x: Value<PrivateConstructor> =
                Rc::new(RefCell::new(({ PrivateConstructor::make(7) })));
            (*m.borrow()).insert_37(___x.as_pointer())
        });
        ({
            let ___x: Value<PrivateConstructor> =
                Rc::new(RefCell::new(({ PrivateConstructor::make(8) })));
            (*m.borrow()).insert_37(___x.as_pointer())
        });
        ({
            let ___x: Value<PrivateConstructor> =
                Rc::new(RefCell::new(({ PrivateConstructor::make(9) })));
            (*m.borrow()).insert_37(___x.as_pointer())
        });
        ({
            let ___x: Value<PrivateConstructor> =
                Rc::new(RefCell::new(({ PrivateConstructor::make(10) })));
            (*m.borrow()).insert_37(___x.as_pointer())
        });
        ({
            let ___x: Value<PrivateConstructor> =
                Rc::new(RefCell::new(({ PrivateConstructor::make(11) })));
            (*m.borrow()).insert_37(___x.as_pointer())
        });
        ({
            let ___x: Value<PrivateConstructor> =
                Rc::new(RefCell::new(({ PrivateConstructor::make(12) })));
            (*m.borrow()).insert_37(___x.as_pointer())
        });
        let r: Value<BidirectionalIterator<PrivateConstructor>> = Rc::new(RefCell::new(
            ({
                let ___x: Value<i32> = Rc::new(RefCell::new(5));
                (*m.borrow()).find_61_int(___x.as_pointer())
            }),
        ));
        assert!((*r.borrow()) == ({ (*m.borrow()).begin() }));
        {
            let __a1 = ({
                let ___x: Value<i32> = Rc::new(RefCell::new(6));
                (*m.borrow()).find_61_int(___x.as_pointer())
            });
            r.as_pointer().with_mut(|__it| *__it = __a1);
            r.as_pointer()
        };
        assert!(
            (*r.borrow())
                == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin() }) }), 1_isize as isize)
        );
        {
            let __a1 = ({
                let ___x: Value<i32> = Rc::new(RefCell::new(7));
                (*m.borrow()).find_61_int(___x.as_pointer())
            });
            r.as_pointer().with_mut(|__it| *__it = __a1);
            r.as_pointer()
        };
        assert!(
            (*r.borrow())
                == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin() }) }), 2_isize as isize)
        );
        {
            let __a1 = ({
                let ___x: Value<i32> = Rc::new(RefCell::new(8));
                (*m.borrow()).find_61_int(___x.as_pointer())
            });
            r.as_pointer().with_mut(|__it| *__it = __a1);
            r.as_pointer()
        };
        assert!(
            (*r.borrow())
                == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin() }) }), 3_isize as isize)
        );
        {
            let __a1 = ({
                let ___x: Value<i32> = Rc::new(RefCell::new(9));
                (*m.borrow()).find_61_int(___x.as_pointer())
            });
            r.as_pointer().with_mut(|__it| *__it = __a1);
            r.as_pointer()
        };
        assert!(
            (*r.borrow())
                == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin() }) }), 4_isize as isize)
        );
        {
            let __a1 = ({
                let ___x: Value<i32> = Rc::new(RefCell::new(10));
                (*m.borrow()).find_61_int(___x.as_pointer())
            });
            r.as_pointer().with_mut(|__it| *__it = __a1);
            r.as_pointer()
        };
        assert!(
            (*r.borrow())
                == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin() }) }), 5_isize as isize)
        );
        {
            let __a1 = ({
                let ___x: Value<i32> = Rc::new(RefCell::new(11));
                (*m.borrow()).find_61_int(___x.as_pointer())
            });
            r.as_pointer().with_mut(|__it| *__it = __a1);
            r.as_pointer()
        };
        assert!(
            (*r.borrow())
                == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin() }) }), 6_isize as isize)
        );
        {
            let __a1 = ({
                let ___x: Value<i32> = Rc::new(RefCell::new(12));
                (*m.borrow()).find_61_int(___x.as_pointer())
            });
            r.as_pointer().with_mut(|__it| *__it = __a1);
            r.as_pointer()
        };
        assert!(
            (*r.borrow())
                == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin() }) }), 7_isize as isize)
        );
        {
            let __a1 = ({
                let ___x: Value<i32> = Rc::new(RefCell::new(4));
                (*m.borrow()).find_61_int(___x.as_pointer())
            });
            r.as_pointer().with_mut(|__it| *__it = __a1);
            r.as_pointer()
        };
        assert!(
            (*r.borrow())
                == libcc2rs::next_refcount(({ ({ (*m.borrow()).begin() }) }), 8_isize as isize)
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
pub struct min_pointer_int__std_integral_constant_unsigned_long__0__;
pub trait PrivateConstructorImpl {
    fn get(&self) -> i32;
}
impl PrivateConstructorImpl for Ptr<PrivateConstructor> {
    fn get(&self) -> i32 {
        return (*self).with(|__s| __s.val);
    }
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
pub fn __cpp2rust_init_globals() {
    let _ = outstanding_bytes_0.with(|_| ());
    let _ = alloc_count_1.with(|_| ());
    let _ = dealloc_count_2.with(|_| ());
    let _ = disable_default_constructor_3.with(|_| ());
}
