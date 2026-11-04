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
            (Rc::new(RefCell::new(1)), Rc::new(RefCell::new(1.5_f64))),
            (Rc::new(RefCell::new(2)), Rc::new(RefCell::new(2.5_f64))),
            (Rc::new(RefCell::new(3)), Rc::new(RefCell::new(3.5_f64))),
            (Rc::new(RefCell::new(4)), Rc::new(RefCell::new(4.5_f64))),
            (Rc::new(RefCell::new(5)), Rc::new(RefCell::new(5.5_f64))),
            (Rc::new(RefCell::new(6)), Rc::new(RefCell::new(6.5_f64))),
            (Rc::new(RefCell::new(7)), Rc::new(RefCell::new(7.5_f64))),
            (Rc::new(RefCell::new(8)), Rc::new(RefCell::new(8.5_f64))),
        ])));
        let m: Value<Map<i32, (Value<i32>, Value<f64>)>> = Rc::new(RefCell::new(
            std_map_int__double__std_less_int___std_allocator_std_pair_const_int__double___::new_1(
                { (ar.as_pointer() as Ptr<(Value<i32>, Value<f64>)>) },
                {
                    (ar.as_pointer() as Ptr<(Value<i32>, Value<f64>)>)
                        .offset(((128usize as usize).wrapping_div((16usize as usize))) as isize)
                },
            ),
        ));
        assert!(({ ((*m.borrow()).len() == 8_usize) }));
        let i: Value<BidirectionalIterator<(Value<i32>, Value<f64>)>> = Rc::new(RefCell::new(
            ({
                let ___first: BidirectionalIterator<(Value<i32>, Value<f64>)> =
                    ({ (*m.borrow()).cbegin() });
                let ___last: BidirectionalIterator<(Value<i32>, Value<f64>)> =
                    ({ (*m.borrow()).cbegin() });
                (*m.borrow()).erase_69(___first, ___last)
            }),
        ));
        assert!(({ ((*m.borrow()).len() == 8_usize) }));
        assert!(
            (*i.borrow())
                == m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin())
        );
        assert!(
            ((*(*m
                .as_pointer()
                .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin())
                .iter_deref()
                .upgrade()
                .deref())
            .0
            .borrow())
                == 1)
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
                == 1.5_f64)
        );
        assert!(
            ((*(*libcc2rs::next_refcount(
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                1_isize
            )
            .iter_deref()
            .upgrade()
            .deref())
            .0
            .borrow())
                == 2)
        );
        assert!(
            ((*(*libcc2rs::next_refcount(
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                1_isize
            )
            .iter_deref()
            .upgrade()
            .deref())
            .1
            .borrow())
                == 2.5_f64)
        );
        assert!(
            ((*(*libcc2rs::next_refcount(
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                2_isize
            )
            .iter_deref()
            .upgrade()
            .deref())
            .0
            .borrow())
                == 3)
        );
        assert!(
            ((*(*libcc2rs::next_refcount(
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                2_isize
            )
            .iter_deref()
            .upgrade()
            .deref())
            .1
            .borrow())
                == 3.5_f64)
        );
        assert!(
            ((*(*libcc2rs::next_refcount(
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                3_isize
            )
            .iter_deref()
            .upgrade()
            .deref())
            .0
            .borrow())
                == 4)
        );
        assert!(
            ((*(*libcc2rs::next_refcount(
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                3_isize
            )
            .iter_deref()
            .upgrade()
            .deref())
            .1
            .borrow())
                == 4.5_f64)
        );
        assert!(
            ((*(*libcc2rs::next_refcount(
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                4_isize
            )
            .iter_deref()
            .upgrade()
            .deref())
            .0
            .borrow())
                == 5)
        );
        assert!(
            ((*(*libcc2rs::next_refcount(
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                4_isize
            )
            .iter_deref()
            .upgrade()
            .deref())
            .1
            .borrow())
                == 5.5_f64)
        );
        assert!(
            ((*(*libcc2rs::next_refcount(
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                5_isize
            )
            .iter_deref()
            .upgrade()
            .deref())
            .0
            .borrow())
                == 6)
        );
        assert!(
            ((*(*libcc2rs::next_refcount(
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                5_isize
            )
            .iter_deref()
            .upgrade()
            .deref())
            .1
            .borrow())
                == 6.5_f64)
        );
        assert!(
            ((*(*libcc2rs::next_refcount(
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                6_isize
            )
            .iter_deref()
            .upgrade()
            .deref())
            .0
            .borrow())
                == 7)
        );
        assert!(
            ((*(*libcc2rs::next_refcount(
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                6_isize
            )
            .iter_deref()
            .upgrade()
            .deref())
            .1
            .borrow())
                == 7.5_f64)
        );
        assert!(
            ((*(*libcc2rs::next_refcount(
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                7_isize
            )
            .iter_deref()
            .upgrade()
            .deref())
            .0
            .borrow())
                == 8)
        );
        assert!(
            ((*(*libcc2rs::next_refcount(
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                7_isize
            )
            .iter_deref()
            .upgrade()
            .deref())
            .1
            .borrow())
                == 8.5_f64)
        );
        (*i.borrow_mut()) = ({
            let ___first: BidirectionalIterator<(Value<i32>, Value<f64>)> =
                ({ (*m.borrow()).cbegin() });
            let ___last: BidirectionalIterator<(Value<i32>, Value<f64>)> =
                libcc2rs::next_refcount(({ ({ (*m.borrow()).cbegin() }) }), 2_isize);
            (*m.borrow()).erase_69(___first, ___last)
        });
        assert!(({ ((*m.borrow()).len() == 6_usize) }));
        assert!(
            (*i.borrow())
                == m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin())
        );
        assert!(
            ((*(*libcc2rs::next_refcount(
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                0_isize
            )
            .iter_deref()
            .upgrade()
            .deref())
            .0
            .borrow())
                == 3)
        );
        assert!(
            ((*(*libcc2rs::next_refcount(
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                0_isize
            )
            .iter_deref()
            .upgrade()
            .deref())
            .1
            .borrow())
                == 3.5_f64)
        );
        assert!(
            ((*(*libcc2rs::next_refcount(
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                1_isize
            )
            .iter_deref()
            .upgrade()
            .deref())
            .0
            .borrow())
                == 4)
        );
        assert!(
            ((*(*libcc2rs::next_refcount(
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                1_isize
            )
            .iter_deref()
            .upgrade()
            .deref())
            .1
            .borrow())
                == 4.5_f64)
        );
        assert!(
            ((*(*libcc2rs::next_refcount(
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                2_isize
            )
            .iter_deref()
            .upgrade()
            .deref())
            .0
            .borrow())
                == 5)
        );
        assert!(
            ((*(*libcc2rs::next_refcount(
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                2_isize
            )
            .iter_deref()
            .upgrade()
            .deref())
            .1
            .borrow())
                == 5.5_f64)
        );
        assert!(
            ((*(*libcc2rs::next_refcount(
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                3_isize
            )
            .iter_deref()
            .upgrade()
            .deref())
            .0
            .borrow())
                == 6)
        );
        assert!(
            ((*(*libcc2rs::next_refcount(
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                3_isize
            )
            .iter_deref()
            .upgrade()
            .deref())
            .1
            .borrow())
                == 6.5_f64)
        );
        assert!(
            ((*(*libcc2rs::next_refcount(
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                4_isize
            )
            .iter_deref()
            .upgrade()
            .deref())
            .0
            .borrow())
                == 7)
        );
        assert!(
            ((*(*libcc2rs::next_refcount(
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                4_isize
            )
            .iter_deref()
            .upgrade()
            .deref())
            .1
            .borrow())
                == 7.5_f64)
        );
        assert!(
            ((*(*libcc2rs::next_refcount(
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                5_isize
            )
            .iter_deref()
            .upgrade()
            .deref())
            .0
            .borrow())
                == 8)
        );
        assert!(
            ((*(*libcc2rs::next_refcount(
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                5_isize
            )
            .iter_deref()
            .upgrade()
            .deref())
            .1
            .borrow())
                == 8.5_f64)
        );
        (*i.borrow_mut()) = ({
            let ___first: BidirectionalIterator<(Value<i32>, Value<f64>)> =
                libcc2rs::next_refcount(({ ({ (*m.borrow()).cbegin() }) }), 2_isize);
            let ___last: BidirectionalIterator<(Value<i32>, Value<f64>)> =
                libcc2rs::next_refcount(({ ({ (*m.borrow()).cbegin() }) }), 6_isize);
            (*m.borrow()).erase_69(___first, ___last)
        });
        assert!(({ ((*m.borrow()).len() == 2_usize) }));
        assert!(
            (*i.borrow())
                == libcc2rs::next_refcount(
                    m.as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                    2_isize
                )
        );
        assert!(
            ((*(*libcc2rs::next_refcount(
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                0_isize
            )
            .iter_deref()
            .upgrade()
            .deref())
            .0
            .borrow())
                == 3)
        );
        assert!(
            ((*(*libcc2rs::next_refcount(
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                0_isize
            )
            .iter_deref()
            .upgrade()
            .deref())
            .1
            .borrow())
                == 3.5_f64)
        );
        assert!(
            ((*(*libcc2rs::next_refcount(
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                1_isize
            )
            .iter_deref()
            .upgrade()
            .deref())
            .0
            .borrow())
                == 4)
        );
        assert!(
            ((*(*libcc2rs::next_refcount(
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                1_isize
            )
            .iter_deref()
            .upgrade()
            .deref())
            .1
            .borrow())
                == 4.5_f64)
        );
        (*i.borrow_mut()) = ({
            let ___first: BidirectionalIterator<(Value<i32>, Value<f64>)> =
                ({ (*m.borrow()).cbegin() });
            let ___last: BidirectionalIterator<(Value<i32>, Value<f64>)> =
                ({ (*m.borrow()).cend() });
            (*m.borrow()).erase_69(___first, ___last)
        });
        assert!(({ ((*m.borrow()).len() == 0_usize) }));
        assert!(
            (*i.borrow())
                == m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin())
        );
        assert!(
            (*i.borrow())
                == m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.end())
        );
    }
    {
        let ar: Value<Box<[(Value<i32>, Value<f64>)]>> = Rc::new(RefCell::new(Box::new([
            (Rc::new(RefCell::new(1)), Rc::new(RefCell::new(1.5_f64))),
            (Rc::new(RefCell::new(2)), Rc::new(RefCell::new(2.5_f64))),
            (Rc::new(RefCell::new(3)), Rc::new(RefCell::new(3.5_f64))),
            (Rc::new(RefCell::new(4)), Rc::new(RefCell::new(4.5_f64))),
            (Rc::new(RefCell::new(5)), Rc::new(RefCell::new(5.5_f64))),
            (Rc::new(RefCell::new(6)), Rc::new(RefCell::new(6.5_f64))),
            (Rc::new(RefCell::new(7)), Rc::new(RefCell::new(7.5_f64))),
            (Rc::new(RefCell::new(8)), Rc::new(RefCell::new(8.5_f64))),
        ])));
        let m: Value<Map<i32, (Value<i32>, Value<f64>)>> = Rc::new(RefCell::new(
            std_map_int__double__std_less_int___min_allocator_std_pair_const_int__double___::new_1(
                { (ar.as_pointer() as Ptr<(Value<i32>, Value<f64>)>) },
                {
                    (ar.as_pointer() as Ptr<(Value<i32>, Value<f64>)>)
                        .offset(((128usize as usize).wrapping_div((16usize as usize))) as isize)
                },
            ),
        ));
        assert!(({ ((*m.borrow()).len() == 8_usize) }));
        let i: Value<BidirectionalIterator<(Value<i32>, Value<f64>)>> = Rc::new(RefCell::new(
            ({
                let ___first: BidirectionalIterator<(Value<i32>, Value<f64>)> =
                    ({ (*m.borrow()).cbegin() });
                let ___last: BidirectionalIterator<(Value<i32>, Value<f64>)> =
                    ({ (*m.borrow()).cbegin() });
                (*m.borrow()).erase_69(___first, ___last)
            }),
        ));
        assert!(({ ((*m.borrow()).len() == 8_usize) }));
        assert!(
            (*i.borrow())
                == m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin())
        );
        assert!(
            ((*(*m
                .as_pointer()
                .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin())
                .iter_deref()
                .upgrade()
                .deref())
            .0
            .borrow())
                == 1)
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
                == 1.5_f64)
        );
        assert!(
            ((*(*libcc2rs::next_refcount(
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                1_isize
            )
            .iter_deref()
            .upgrade()
            .deref())
            .0
            .borrow())
                == 2)
        );
        assert!(
            ((*(*libcc2rs::next_refcount(
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                1_isize
            )
            .iter_deref()
            .upgrade()
            .deref())
            .1
            .borrow())
                == 2.5_f64)
        );
        assert!(
            ((*(*libcc2rs::next_refcount(
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                2_isize
            )
            .iter_deref()
            .upgrade()
            .deref())
            .0
            .borrow())
                == 3)
        );
        assert!(
            ((*(*libcc2rs::next_refcount(
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                2_isize
            )
            .iter_deref()
            .upgrade()
            .deref())
            .1
            .borrow())
                == 3.5_f64)
        );
        assert!(
            ((*(*libcc2rs::next_refcount(
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                3_isize
            )
            .iter_deref()
            .upgrade()
            .deref())
            .0
            .borrow())
                == 4)
        );
        assert!(
            ((*(*libcc2rs::next_refcount(
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                3_isize
            )
            .iter_deref()
            .upgrade()
            .deref())
            .1
            .borrow())
                == 4.5_f64)
        );
        assert!(
            ((*(*libcc2rs::next_refcount(
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                4_isize
            )
            .iter_deref()
            .upgrade()
            .deref())
            .0
            .borrow())
                == 5)
        );
        assert!(
            ((*(*libcc2rs::next_refcount(
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                4_isize
            )
            .iter_deref()
            .upgrade()
            .deref())
            .1
            .borrow())
                == 5.5_f64)
        );
        assert!(
            ((*(*libcc2rs::next_refcount(
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                5_isize
            )
            .iter_deref()
            .upgrade()
            .deref())
            .0
            .borrow())
                == 6)
        );
        assert!(
            ((*(*libcc2rs::next_refcount(
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                5_isize
            )
            .iter_deref()
            .upgrade()
            .deref())
            .1
            .borrow())
                == 6.5_f64)
        );
        assert!(
            ((*(*libcc2rs::next_refcount(
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                6_isize
            )
            .iter_deref()
            .upgrade()
            .deref())
            .0
            .borrow())
                == 7)
        );
        assert!(
            ((*(*libcc2rs::next_refcount(
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                6_isize
            )
            .iter_deref()
            .upgrade()
            .deref())
            .1
            .borrow())
                == 7.5_f64)
        );
        assert!(
            ((*(*libcc2rs::next_refcount(
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                7_isize
            )
            .iter_deref()
            .upgrade()
            .deref())
            .0
            .borrow())
                == 8)
        );
        assert!(
            ((*(*libcc2rs::next_refcount(
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                7_isize
            )
            .iter_deref()
            .upgrade()
            .deref())
            .1
            .borrow())
                == 8.5_f64)
        );
        (*i.borrow_mut()) = ({
            let ___first: BidirectionalIterator<(Value<i32>, Value<f64>)> =
                ({ (*m.borrow()).cbegin() });
            let ___last: BidirectionalIterator<(Value<i32>, Value<f64>)> =
                libcc2rs::next_refcount(({ ({ (*m.borrow()).cbegin() }) }), 2_isize);
            (*m.borrow()).erase_69(___first, ___last)
        })
        .clone();
        assert!(({ ((*m.borrow()).len() == 6_usize) }));
        assert!(
            (*i.borrow())
                == m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin())
        );
        assert!(
            ((*(*libcc2rs::next_refcount(
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                0_isize
            )
            .iter_deref()
            .upgrade()
            .deref())
            .0
            .borrow())
                == 3)
        );
        assert!(
            ((*(*libcc2rs::next_refcount(
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                0_isize
            )
            .iter_deref()
            .upgrade()
            .deref())
            .1
            .borrow())
                == 3.5_f64)
        );
        assert!(
            ((*(*libcc2rs::next_refcount(
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                1_isize
            )
            .iter_deref()
            .upgrade()
            .deref())
            .0
            .borrow())
                == 4)
        );
        assert!(
            ((*(*libcc2rs::next_refcount(
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                1_isize
            )
            .iter_deref()
            .upgrade()
            .deref())
            .1
            .borrow())
                == 4.5_f64)
        );
        assert!(
            ((*(*libcc2rs::next_refcount(
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                2_isize
            )
            .iter_deref()
            .upgrade()
            .deref())
            .0
            .borrow())
                == 5)
        );
        assert!(
            ((*(*libcc2rs::next_refcount(
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                2_isize
            )
            .iter_deref()
            .upgrade()
            .deref())
            .1
            .borrow())
                == 5.5_f64)
        );
        assert!(
            ((*(*libcc2rs::next_refcount(
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                3_isize
            )
            .iter_deref()
            .upgrade()
            .deref())
            .0
            .borrow())
                == 6)
        );
        assert!(
            ((*(*libcc2rs::next_refcount(
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                3_isize
            )
            .iter_deref()
            .upgrade()
            .deref())
            .1
            .borrow())
                == 6.5_f64)
        );
        assert!(
            ((*(*libcc2rs::next_refcount(
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                4_isize
            )
            .iter_deref()
            .upgrade()
            .deref())
            .0
            .borrow())
                == 7)
        );
        assert!(
            ((*(*libcc2rs::next_refcount(
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                4_isize
            )
            .iter_deref()
            .upgrade()
            .deref())
            .1
            .borrow())
                == 7.5_f64)
        );
        assert!(
            ((*(*libcc2rs::next_refcount(
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                5_isize
            )
            .iter_deref()
            .upgrade()
            .deref())
            .0
            .borrow())
                == 8)
        );
        assert!(
            ((*(*libcc2rs::next_refcount(
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                5_isize
            )
            .iter_deref()
            .upgrade()
            .deref())
            .1
            .borrow())
                == 8.5_f64)
        );
        (*i.borrow_mut()) = ({
            let ___first: BidirectionalIterator<(Value<i32>, Value<f64>)> =
                libcc2rs::next_refcount(({ ({ (*m.borrow()).cbegin() }) }), 2_isize);
            let ___last: BidirectionalIterator<(Value<i32>, Value<f64>)> =
                libcc2rs::next_refcount(({ ({ (*m.borrow()).cbegin() }) }), 6_isize);
            (*m.borrow()).erase_69(___first, ___last)
        })
        .clone();
        assert!(({ ((*m.borrow()).len() == 2_usize) }));
        assert!(
            (*i.borrow())
                == libcc2rs::next_refcount(
                    m.as_pointer()
                        .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                    2_isize
                )
        );
        assert!(
            ((*(*libcc2rs::next_refcount(
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                0_isize
            )
            .iter_deref()
            .upgrade()
            .deref())
            .0
            .borrow())
                == 3)
        );
        assert!(
            ((*(*libcc2rs::next_refcount(
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                0_isize
            )
            .iter_deref()
            .upgrade()
            .deref())
            .1
            .borrow())
                == 3.5_f64)
        );
        assert!(
            ((*(*libcc2rs::next_refcount(
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                1_isize
            )
            .iter_deref()
            .upgrade()
            .deref())
            .0
            .borrow())
                == 4)
        );
        assert!(
            ((*(*libcc2rs::next_refcount(
                m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin()),
                1_isize
            )
            .iter_deref()
            .upgrade()
            .deref())
            .1
            .borrow())
                == 4.5_f64)
        );
        (*i.borrow_mut()) = ({
            let ___first: BidirectionalIterator<(Value<i32>, Value<f64>)> =
                ({ (*m.borrow()).cbegin() });
            let ___last: BidirectionalIterator<(Value<i32>, Value<f64>)> =
                ({ (*m.borrow()).cend() });
            (*m.borrow()).erase_69(___first, ___last)
        })
        .clone();
        assert!(({ ((*m.borrow()).len() == 0_usize) }));
        assert!(
            (*i.borrow())
                == m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.begin())
        );
        assert!(
            (*i.borrow())
                == m.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<f64>)>| __m.end())
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
