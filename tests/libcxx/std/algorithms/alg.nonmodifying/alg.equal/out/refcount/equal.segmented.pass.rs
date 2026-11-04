extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_containers_0() {
    {
        let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([1, 2, 3, 4])));
        let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(
            std_deque_int__std_allocator_int__::new_1({ Vec::from([1, 2, 3, 4]) }, None),
        ));
        let mut ret: bool = ({
            let ___first1: RandomAccessIterator<i32> = CppRange::begin(&lhs.decay());
            let ___last1: RandomAccessIterator<i32> = CppRange::end(&lhs.decay());
            equal_1(___first1, ___last1, ({ (*rhs.borrow_mut()).begin_24() }))
        });
        assert!((ret as bool));
    }
    {
        let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([1, 2, 3, 4])));
        let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(
            std_deque_int__std_allocator_int__::new_1({ Vec::from([2, 2, 3, 4]) }, None),
        ));
        let mut ret: bool = ({
            let ___first1: RandomAccessIterator<i32> = CppRange::begin(&lhs.decay());
            let ___last1: RandomAccessIterator<i32> = CppRange::end(&lhs.decay());
            equal_1(___first1, ___last1, ({ (*rhs.borrow_mut()).begin_24() }))
        });
        assert!(!(ret));
    }
    {
        let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
        ({
            generate_n_2(
                ({ back_inserter_3(lhs.as_pointer()) }),
                4095,
                lambda!(
                    {
                        let i: Value<i32> = Rc::new(RefCell::new(0));
                    },
                    || -> i32 {
                        return (*i.borrow_mut()).postfix_inc();
                    }
                ),
            )
        });
        let rhs: Value<Vec<i32>> =
            Rc::new(RefCell::new(std_deque_int__std_allocator_int__::new_2(
                { CppRange::begin(&lhs.decay()) },
                { CppRange::end(&lhs.decay()) },
                None,
            )));
        assert!(
            ({
                ({
                    let ___first1: RandomAccessIterator<i32> = CppRange::begin(&lhs.decay());
                    let ___last1: RandomAccessIterator<i32> = CppRange::end(&lhs.decay());
                    equal_1(___first1, ___last1, ({ (*rhs.borrow_mut()).begin_24() }))
                })
            })
        );
    }
    {
        let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
        ({
            generate_n_4(
                ({ back_inserter_3(lhs.as_pointer()) }),
                4095,
                lambda!(
                    {
                        let i: Value<i32> = Rc::new(RefCell::new(0));
                    },
                    || -> i32 {
                        return (*i.borrow_mut()).postfix_inc();
                    }
                ),
            )
        });
        let rhs: Value<Vec<i32>> =
            Rc::new(RefCell::new(std_deque_int__std_allocator_int__::new_2(
                { CppRange::begin(&lhs.decay()) },
                { CppRange::end(&lhs.decay()) },
                None,
            )));
        rhs.decay().to_last().write(-1_i32);
        assert!(
            ({
                !({
                    let ___first1: RandomAccessIterator<i32> = CppRange::begin(&lhs.decay());
                    let ___last1: RandomAccessIterator<i32> = CppRange::end(&lhs.decay());
                    equal_1(___first1, ___last1, ({ (*rhs.borrow_mut()).begin_24() }))
                })
            })
        );
    }
    {
        let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
        ({
            generate_n_5(
                ({ back_inserter_3(lhs.as_pointer()) }),
                4095,
                lambda!(
                    {
                        let i: Value<i32> = Rc::new(RefCell::new(0));
                    },
                    || -> i32 {
                        return (*i.borrow_mut()).postfix_inc();
                    }
                ),
            )
        });
        let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
        ({ (*rhs.borrow_mut()).resize_38(10_usize) });
        ({
            let ___position: std__Deque_iterator_int__const_int_ref__const_int_ptr_ =
                <std__Deque_iterator_int__const_int_ref__const_int_ptr_>::from(
                    ({ (*rhs.borrow_mut()).end_26() }),
                );
            let ___first: RandomAccessIterator<i32> = CppRange::begin(&lhs.decay());
            let ___last: RandomAccessIterator<i32> = CppRange::end(&lhs.decay());
            (*rhs.borrow_mut())
                .insert_64_class___gnu_cxx___normal_iterator_int_ptr__class_std_vector_int____void(
                    ___position,
                    ___first,
                    ___last,
                )
        });
        ({
            let ___first: std__Deque_iterator_int__const_int_ref__const_int_ptr_ =
                <std__Deque_iterator_int__const_int_ref__const_int_ptr_>::from(
                    ({ (*rhs.borrow_mut()).begin_24() }),
                );
            let ___last: std__Deque_iterator_int__const_int_ref__const_int_ptr_ =
                <std__Deque_iterator_int__const_int_ref__const_int_ptr_>::from(
                    ({ (*rhs.borrow_mut()).begin_24() }).iter_add(10_isize),
                );
            (*rhs.borrow_mut()).erase_69(___first, ___last)
        });
        assert!(
            ({
                ({
                    let ___first1: RandomAccessIterator<i32> = CppRange::begin(&lhs.decay());
                    let ___last1: RandomAccessIterator<i32> = CppRange::end(&lhs.decay());
                    equal_1(___first1, ___last1, ({ (*rhs.borrow_mut()).begin_24() }))
                })
            })
        );
    }
    {
        let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
        ({
            generate_n_6(
                ({ back_inserter_3(lhs.as_pointer()) }),
                4095,
                lambda!(
                    {
                        let i: Value<i32> = Rc::new(RefCell::new(0));
                    },
                    || -> i32 {
                        return (*i.borrow_mut()).postfix_inc();
                    }
                ),
            )
        });
        let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
        ({ (*rhs.borrow_mut()).resize_38(10_usize) });
        ({
            let ___position: std__Deque_iterator_int__const_int_ref__const_int_ptr_ =
                <std__Deque_iterator_int__const_int_ref__const_int_ptr_>::from(
                    ({ (*rhs.borrow_mut()).end_26() }),
                );
            let ___first: RandomAccessIterator<i32> = CppRange::begin(&lhs.decay());
            let ___last: RandomAccessIterator<i32> = CppRange::end(&lhs.decay());
            (*rhs.borrow_mut())
                .insert_64_class___gnu_cxx___normal_iterator_int_ptr__class_std_vector_int____void(
                    ___position,
                    ___first,
                    ___last,
                )
        });
        ({
            let ___first: std__Deque_iterator_int__const_int_ref__const_int_ptr_ =
                <std__Deque_iterator_int__const_int_ref__const_int_ptr_>::from(
                    ({ (*rhs.borrow_mut()).begin_24() }),
                );
            let ___last: std__Deque_iterator_int__const_int_ref__const_int_ptr_ =
                <std__Deque_iterator_int__const_int_ref__const_int_ptr_>::from(
                    ({ (*rhs.borrow_mut()).begin_24() }).iter_add(10_isize),
                );
            (*rhs.borrow_mut()).erase_69(___first, ___last)
        });
        rhs.decay().to_last().write(-1_i32);
        assert!(
            ({
                !({
                    let ___first1: RandomAccessIterator<i32> = CppRange::begin(&lhs.decay());
                    let ___last1: RandomAccessIterator<i32> = CppRange::end(&lhs.decay());
                    equal_1(___first1, ___last1, ({ (*rhs.borrow_mut()).begin_24() }))
                })
            })
        );
    }
}
pub fn test_containers_7() {
    {
        let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(
            std_deque_int__std_allocator_int__::new_1({ Vec::from([1, 2, 3, 4]) }, None),
        ));
        let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([1, 2, 3, 4])));
        let mut ret: bool = ({
            let ___first1: std__Deque_iterator_int__int_ref__int_ptr_ =
                ({ (*lhs.borrow_mut()).begin_24() });
            let ___last1: std__Deque_iterator_int__int_ref__int_ptr_ =
                ({ (*lhs.borrow_mut()).end_26() });
            equal_8(___first1, ___last1, CppRange::begin(&rhs.decay()))
        });
        assert!((ret as bool));
    }
    {
        let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(
            std_deque_int__std_allocator_int__::new_1({ Vec::from([1, 2, 3, 4]) }, None),
        ));
        let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::from([2, 2, 3, 4])));
        let mut ret: bool = ({
            let ___first1: std__Deque_iterator_int__int_ref__int_ptr_ =
                ({ (*lhs.borrow_mut()).begin_24() });
            let ___last1: std__Deque_iterator_int__int_ref__int_ptr_ =
                ({ (*lhs.borrow_mut()).end_26() });
            equal_8(___first1, ___last1, CppRange::begin(&rhs.decay()))
        });
        assert!(!(ret));
    }
    {
        let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
        ({
            generate_n_9(
                ({ back_inserter_10(lhs.as_pointer()) }),
                4095,
                lambda!(
                    {
                        let i: Value<i32> = Rc::new(RefCell::new(0));
                    },
                    || -> i32 {
                        return (*i.borrow_mut()).postfix_inc();
                    }
                ),
            )
        });
        let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(
            match ({ ({ (*lhs.borrow_mut()).begin_24() }) }, {
                ({ (*lhs.borrow_mut()).end_26() })
            }) {
                (mut __tmp_0, mut __tmp_1) => {
                    let mut __v = Vec::new();
                    let mut __it = (__tmp_0).clone();
                    while __it != __tmp_1 {
                        __v.push((__it.iter_deref().read()));
                        __it.iter_inc();
                    }
                    __v
                }
            },
        ));
        assert!(
            ({
                ({
                    let ___first1: std__Deque_iterator_int__int_ref__int_ptr_ =
                        ({ (*lhs.borrow_mut()).begin_24() });
                    let ___last1: std__Deque_iterator_int__int_ref__int_ptr_ =
                        ({ (*lhs.borrow_mut()).end_26() });
                    equal_8(___first1, ___last1, CppRange::begin(&rhs.decay()))
                })
            })
        );
    }
    {
        let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
        ({
            generate_n_11(
                ({ back_inserter_10(lhs.as_pointer()) }),
                4095,
                lambda!(
                    {
                        let i: Value<i32> = Rc::new(RefCell::new(0));
                    },
                    || -> i32 {
                        return (*i.borrow_mut()).postfix_inc();
                    }
                ),
            )
        });
        let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(
            match ({ ({ (*lhs.borrow_mut()).begin_24() }) }, {
                ({ (*lhs.borrow_mut()).end_26() })
            }) {
                (mut __tmp_2, mut __tmp_3) => {
                    let mut __v = Vec::new();
                    let mut __it = (__tmp_2).clone();
                    while __it != __tmp_3 {
                        __v.push((__it.iter_deref().read()));
                        __it.iter_inc();
                    }
                    __v
                }
            },
        ));
        rhs.decay().to_last().write(-1_i32);
        assert!(
            ({
                !({
                    let ___first1: std__Deque_iterator_int__int_ref__int_ptr_ =
                        ({ (*lhs.borrow_mut()).begin_24() });
                    let ___last1: std__Deque_iterator_int__int_ref__int_ptr_ =
                        ({ (*lhs.borrow_mut()).end_26() });
                    equal_8(___first1, ___last1, CppRange::begin(&rhs.decay()))
                })
            })
        );
    }
    {
        let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
        ({
            generate_n_12(
                ({ back_inserter_10(lhs.as_pointer()) }),
                4095,
                lambda!(
                    {
                        let i: Value<i32> = Rc::new(RefCell::new(0));
                    },
                    || -> i32 {
                        return (*i.borrow_mut()).postfix_inc();
                    }
                ),
            )
        });
        let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
        {
            let __a0 = 10_usize as usize;
            (*rhs.borrow_mut()).resize_with(__a0, || 0_i32)
        };
        match (
            { <RandomAccessIterator<i32>>::from(CppRange::end(&rhs.decay())) },
            { ({ (*lhs.borrow_mut()).begin_24() }) },
            { ({ (*lhs.borrow_mut()).end_26() }) },
        ) {
            (mut __tmp_4, mut __tmp_5, mut __tmp_6) => {
                let __start = __tmp_4.index();
                let mut __tmp: Vec<i32> = Vec::new();
                let mut __it = (__tmp_5).clone();
                while __it != __tmp_6 {
                    __tmp.push((__it.iter_deref().read()));
                    __it.iter_inc();
                }
                rhs.as_pointer().with_mut(|__v: &mut Vec<i32>| {
                    __v.splice(__start..__start, __tmp);
                });
                (__tmp_4).clone()
            }
        };
        ({
            let ___first: RandomAccessIterator<i32> =
                <RandomAccessIterator<i32>>::from(CppRange::begin(&rhs.decay()));
            let ___last: RandomAccessIterator<i32> =
                <RandomAccessIterator<i32>>::from(CppRange::begin(&rhs.decay()).iter_add(10_isize));
            (*rhs.borrow_mut()).erase_72(___first, ___last)
        });
        assert!(
            ({
                ({
                    let ___first1: std__Deque_iterator_int__int_ref__int_ptr_ =
                        ({ (*lhs.borrow_mut()).begin_24() });
                    let ___last1: std__Deque_iterator_int__int_ref__int_ptr_ =
                        ({ (*lhs.borrow_mut()).end_26() });
                    equal_8(___first1, ___last1, CppRange::begin(&rhs.decay()))
                })
            })
        );
    }
    {
        let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
        ({
            generate_n_13(
                ({ back_inserter_10(lhs.as_pointer()) }),
                4095,
                lambda!(
                    {
                        let i: Value<i32> = Rc::new(RefCell::new(0));
                    },
                    || -> i32 {
                        return (*i.borrow_mut()).postfix_inc();
                    }
                ),
            )
        });
        let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
        {
            let __a0 = 10_usize as usize;
            (*rhs.borrow_mut()).resize_with(__a0, || 0_i32)
        };
        match (
            { <RandomAccessIterator<i32>>::from(CppRange::end(&rhs.decay())) },
            { ({ (*lhs.borrow_mut()).begin_24() }) },
            { ({ (*lhs.borrow_mut()).end_26() }) },
        ) {
            (mut __tmp_7, mut __tmp_8, mut __tmp_9) => {
                let __start = __tmp_7.index();
                let mut __tmp: Vec<i32> = Vec::new();
                let mut __it = (__tmp_8).clone();
                while __it != __tmp_9 {
                    __tmp.push((__it.iter_deref().read()));
                    __it.iter_inc();
                }
                rhs.as_pointer().with_mut(|__v: &mut Vec<i32>| {
                    __v.splice(__start..__start, __tmp);
                });
                (__tmp_7).clone()
            }
        };
        ({
            let ___first: RandomAccessIterator<i32> =
                <RandomAccessIterator<i32>>::from(CppRange::begin(&rhs.decay()));
            let ___last: RandomAccessIterator<i32> =
                <RandomAccessIterator<i32>>::from(CppRange::begin(&rhs.decay()).iter_add(10_isize));
            (*rhs.borrow_mut()).erase_72(___first, ___last)
        });
        rhs.decay().to_last().write(-1_i32);
        assert!(
            ({
                !({
                    let ___first1: std__Deque_iterator_int__int_ref__int_ptr_ =
                        ({ (*lhs.borrow_mut()).begin_24() });
                    let ___last1: std__Deque_iterator_int__int_ref__int_ptr_ =
                        ({ (*lhs.borrow_mut()).end_26() });
                    equal_8(___first1, ___last1, CppRange::begin(&rhs.decay()))
                })
            })
        );
    }
}
pub fn test_containers_14() {
    {
        let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(
            std_deque_int__std_allocator_int__::new_1({ Vec::from([1, 2, 3, 4]) }, None),
        ));
        let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(
            std_deque_int__std_allocator_int__::new_1({ Vec::from([1, 2, 3, 4]) }, None),
        ));
        let mut ret: bool = ({
            let ___first1: std__Deque_iterator_int__int_ref__int_ptr_ =
                ({ (*lhs.borrow_mut()).begin_24() });
            let ___last1: std__Deque_iterator_int__int_ref__int_ptr_ =
                ({ (*lhs.borrow_mut()).end_26() });
            let ___first2: std__Deque_iterator_int__int_ref__int_ptr_ =
                ({ (*rhs.borrow_mut()).begin_24() });
            equal_15(___first1, ___last1, ___first2)
        });
        assert!((ret as bool));
    }
    {
        let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(
            std_deque_int__std_allocator_int__::new_1({ Vec::from([1, 2, 3, 4]) }, None),
        ));
        let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(
            std_deque_int__std_allocator_int__::new_1({ Vec::from([2, 2, 3, 4]) }, None),
        ));
        let mut ret: bool = ({
            let ___first1: std__Deque_iterator_int__int_ref__int_ptr_ =
                ({ (*lhs.borrow_mut()).begin_24() });
            let ___last1: std__Deque_iterator_int__int_ref__int_ptr_ =
                ({ (*lhs.borrow_mut()).end_26() });
            let ___first2: std__Deque_iterator_int__int_ref__int_ptr_ =
                ({ (*rhs.borrow_mut()).begin_24() });
            equal_15(___first1, ___last1, ___first2)
        });
        assert!(!(ret));
    }
    {
        let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
        ({
            generate_n_16(
                ({ back_inserter_10(lhs.as_pointer()) }),
                4095,
                lambda!(
                    {
                        let i: Value<i32> = Rc::new(RefCell::new(0));
                    },
                    || -> i32 {
                        return (*i.borrow_mut()).postfix_inc();
                    }
                ),
            )
        });
        let rhs: Value<Vec<i32>> =
            Rc::new(RefCell::new(std_deque_int__std_allocator_int__::new_3(
                { ({ (*lhs.borrow_mut()).begin_24() }) },
                { ({ (*lhs.borrow_mut()).end_26() }) },
                None,
            )));
        assert!(
            ({
                ({
                    let ___first1: std__Deque_iterator_int__int_ref__int_ptr_ =
                        ({ (*lhs.borrow_mut()).begin_24() });
                    let ___last1: std__Deque_iterator_int__int_ref__int_ptr_ =
                        ({ (*lhs.borrow_mut()).end_26() });
                    let ___first2: std__Deque_iterator_int__int_ref__int_ptr_ =
                        ({ (*rhs.borrow_mut()).begin_24() });
                    equal_15(___first1, ___last1, ___first2)
                })
            })
        );
    }
    {
        let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
        ({
            generate_n_17(
                ({ back_inserter_10(lhs.as_pointer()) }),
                4095,
                lambda!(
                    {
                        let i: Value<i32> = Rc::new(RefCell::new(0));
                    },
                    || -> i32 {
                        return (*i.borrow_mut()).postfix_inc();
                    }
                ),
            )
        });
        let rhs: Value<Vec<i32>> =
            Rc::new(RefCell::new(std_deque_int__std_allocator_int__::new_3(
                { ({ (*lhs.borrow_mut()).begin_24() }) },
                { ({ (*lhs.borrow_mut()).end_26() }) },
                None,
            )));
        rhs.decay().to_last().write(-1_i32);
        assert!(
            ({
                !({
                    let ___first1: std__Deque_iterator_int__int_ref__int_ptr_ =
                        ({ (*lhs.borrow_mut()).begin_24() });
                    let ___last1: std__Deque_iterator_int__int_ref__int_ptr_ =
                        ({ (*lhs.borrow_mut()).end_26() });
                    let ___first2: std__Deque_iterator_int__int_ref__int_ptr_ =
                        ({ (*rhs.borrow_mut()).begin_24() });
                    equal_15(___first1, ___last1, ___first2)
                })
            })
        );
    }
    {
        let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
        ({
            generate_n_18(
                ({ back_inserter_10(lhs.as_pointer()) }),
                4095,
                lambda!(
                    {
                        let i: Value<i32> = Rc::new(RefCell::new(0));
                    },
                    || -> i32 {
                        return (*i.borrow_mut()).postfix_inc();
                    }
                ),
            )
        });
        let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
        ({ (*rhs.borrow_mut()).resize_38(10_usize) });
        ({
            let ___position: std__Deque_iterator_int__const_int_ref__const_int_ptr_ =
                <std__Deque_iterator_int__const_int_ref__const_int_ptr_>::from(
                    ({ (*rhs.borrow_mut()).end_26() }),
                );
            let ___first: std__Deque_iterator_int__int_ref__int_ptr_ =
                ({ (*lhs.borrow_mut()).begin_24() });
            let ___last: std__Deque_iterator_int__int_ref__int_ptr_ =
                ({ (*lhs.borrow_mut()).end_26() });
            (*rhs.borrow_mut()).insert_64_struct_std__Deque_iterator_int__int_ref__int_ptr__void(
                ___position,
                ___first,
                ___last,
            )
        });
        ({
            let ___first: std__Deque_iterator_int__const_int_ref__const_int_ptr_ =
                <std__Deque_iterator_int__const_int_ref__const_int_ptr_>::from(
                    ({ (*rhs.borrow_mut()).begin_24() }),
                );
            let ___last: std__Deque_iterator_int__const_int_ref__const_int_ptr_ =
                <std__Deque_iterator_int__const_int_ref__const_int_ptr_>::from(
                    ({ (*rhs.borrow_mut()).begin_24() }).iter_add(10_isize),
                );
            (*rhs.borrow_mut()).erase_69(___first, ___last)
        });
        assert!(
            ({
                ({
                    let ___first1: std__Deque_iterator_int__int_ref__int_ptr_ =
                        ({ (*lhs.borrow_mut()).begin_24() });
                    let ___last1: std__Deque_iterator_int__int_ref__int_ptr_ =
                        ({ (*lhs.borrow_mut()).end_26() });
                    let ___first2: std__Deque_iterator_int__int_ref__int_ptr_ =
                        ({ (*rhs.borrow_mut()).begin_24() });
                    equal_15(___first1, ___last1, ___first2)
                })
            })
        );
    }
    {
        let lhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
        ({
            generate_n_19(
                ({ back_inserter_10(lhs.as_pointer()) }),
                4095,
                lambda!(
                    {
                        let i: Value<i32> = Rc::new(RefCell::new(0));
                    },
                    || -> i32 {
                        return (*i.borrow_mut()).postfix_inc();
                    }
                ),
            )
        });
        let rhs: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
        ({ (*rhs.borrow_mut()).resize_38(10_usize) });
        ({
            let ___position: std__Deque_iterator_int__const_int_ref__const_int_ptr_ =
                <std__Deque_iterator_int__const_int_ref__const_int_ptr_>::from(
                    ({ (*rhs.borrow_mut()).end_26() }),
                );
            let ___first: std__Deque_iterator_int__int_ref__int_ptr_ =
                ({ (*lhs.borrow_mut()).begin_24() });
            let ___last: std__Deque_iterator_int__int_ref__int_ptr_ =
                ({ (*lhs.borrow_mut()).end_26() });
            (*rhs.borrow_mut()).insert_64_struct_std__Deque_iterator_int__int_ref__int_ptr__void(
                ___position,
                ___first,
                ___last,
            )
        });
        ({
            let ___first: std__Deque_iterator_int__const_int_ref__const_int_ptr_ =
                <std__Deque_iterator_int__const_int_ref__const_int_ptr_>::from(
                    ({ (*rhs.borrow_mut()).begin_24() }),
                );
            let ___last: std__Deque_iterator_int__const_int_ref__const_int_ptr_ =
                <std__Deque_iterator_int__const_int_ref__const_int_ptr_>::from(
                    ({ (*rhs.borrow_mut()).begin_24() }).iter_add(10_isize),
                );
            (*rhs.borrow_mut()).erase_69(___first, ___last)
        });
        rhs.decay().to_last().write(-1_i32);
        assert!(
            ({
                !({
                    let ___first1: std__Deque_iterator_int__int_ref__int_ptr_ =
                        ({ (*lhs.borrow_mut()).begin_24() });
                    let ___last1: std__Deque_iterator_int__int_ref__int_ptr_ =
                        ({ (*lhs.borrow_mut()).end_26() });
                    let ___first2: std__Deque_iterator_int__int_ref__int_ptr_ =
                        ({ (*rhs.borrow_mut()).begin_24() });
                    equal_15(___first1, ___last1, ___first2)
                })
            })
        );
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
    ({ test_containers_0() });
    ({ test_containers_7() });
    ({ test_containers_14() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
