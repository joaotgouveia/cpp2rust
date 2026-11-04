extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_node_container_0(conv: FnPtr<fn(i32) -> (Value<i32>, Value<i32>)>) {
    let conv: Value<FnPtr<fn(i32) -> (Value<i32>, Value<i32>)>> = Rc::new(RefCell::new(conv));
    let c: Value<Map<i32, (Value<i32>, Value<i32>)>> = Rc::new(RefCell::new(Map::new()));
    let mut i: i32 = 0;
    'loop_: while (i != 10) {
        ({
            let ___x: Value<(Value<i32>, Value<i32>)> =
                Rc::new(RefCell::new(({ (*conv.borrow()).call(i) })));
            (*c.borrow()).insert_55_struct_std_pair_int__int_(___x.as_pointer())
        });
        i.prefix_inc();
    }
    {
        let mut i: i32 = 0;
        'loop_: while (i != 10) {
            let invoke_count: Value<i32> = Rc::new(RefCell::new(i));
            ({
                let ___first: BidirectionalIterator<(Value<i32>, Value<i32>)> =
                    libcc2rs::next_refcount(
                        c.as_pointer()
                            .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.begin()),
                        (i as isize) as isize,
                    );
                let ___last: BidirectionalIterator<(Value<i32>, Value<i32>)> = c
                    .as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.end());
                let ___f: FnPtr<fn(Ptr<(Value<i32>, Value<i32>)>)> = lambda!(
                    {
                        let c: Ptr<Map<i32, (Value<i32>, Value<i32>)>> = c.as_pointer();
                        let invoke_count: Ptr<i32> = invoke_count.as_pointer();
                    },
                    |val: Ptr<(Value<i32>, Value<i32>)>| {
                        assert!(
                            ({ (val).clone() } == {
                                (libcc2rs::next_refcount(
                                    c.with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.begin()),
                                    (invoke_count.with_mut(|__v| __v.postfix_inc()) as isize)
                                        as isize,
                                )
                                .iter_deref())
                            })
                        );
                    }
                );
                for_each_1(___first, ___last, ___f)
            });
            assert!(({ ((*invoke_count.borrow()) == 10) }));
            i.prefix_inc();
        }
    }
    {
        let mut i: i32 = 0;
        'loop_: while (i != 10) {
            let invoke_count: Value<i32> = Rc::new(RefCell::new(0));
            ({
                let ___first: BidirectionalIterator<(Value<i32>, Value<i32>)> = c
                    .as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.begin());
                let ___last: BidirectionalIterator<(Value<i32>, Value<i32>)> =
                    libcc2rs::prev_refcount(
                        c.as_pointer()
                            .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.end()),
                        (i as isize) as isize,
                    );
                let ___f: FnPtr<fn(Ptr<(Value<i32>, Value<i32>)>)> = lambda!(
                    {
                        let c: Ptr<Map<i32, (Value<i32>, Value<i32>)>> = c.as_pointer();
                        let invoke_count: Ptr<i32> = invoke_count.as_pointer();
                    },
                    |val: Ptr<(Value<i32>, Value<i32>)>| {
                        assert!(
                            ({ (val).clone() } == {
                                (libcc2rs::next_refcount(
                                    c.with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.begin()),
                                    (invoke_count.with_mut(|__v| __v.postfix_inc()) as isize)
                                        as isize,
                                )
                                .iter_deref())
                            })
                        );
                    }
                );
                for_each_2(___first, ___last, ___f)
            });
            assert!(({ ((*invoke_count.borrow()) == (10 - i)) }));
            i.prefix_inc();
        }
    }
    {
        {
            let invoke_count: Value<i32> = Rc::new(RefCell::new(0));
            ({
                let ___first: BidirectionalIterator<(Value<i32>, Value<i32>)> = c
                    .as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.begin());
                let ___last: BidirectionalIterator<(Value<i32>, Value<i32>)> = c
                    .as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.begin());
                let ___f: FnPtr<fn(Ptr<(Value<i32>, Value<i32>)>)> = lambda!(
                    {
                        let c: Ptr<Map<i32, (Value<i32>, Value<i32>)>> = c.as_pointer();
                        let invoke_count: Ptr<i32> = invoke_count.as_pointer();
                    },
                    |i: Ptr<(Value<i32>, Value<i32>)>| {
                        assert!(
                            ({ (i).clone() } == {
                                (libcc2rs::next_refcount(
                                    c.with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.begin()),
                                    (invoke_count.with_mut(|__v| __v.postfix_inc()) as isize)
                                        as isize,
                                )
                                .iter_deref())
                            })
                        );
                    }
                );
                for_each_3(___first, ___last, ___f)
            });
            assert!(({ ((*invoke_count.borrow()) == 0) }));
        }
        {
            let invoke_count: Value<i32> = Rc::new(RefCell::new(0));
            ({
                let ___first: BidirectionalIterator<(Value<i32>, Value<i32>)> = c
                    .as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.end());
                let ___last: BidirectionalIterator<(Value<i32>, Value<i32>)> = c
                    .as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.end());
                let ___f: FnPtr<fn(Ptr<(Value<i32>, Value<i32>)>)> = lambda!(
                    {
                        let c: Ptr<Map<i32, (Value<i32>, Value<i32>)>> = c.as_pointer();
                        let invoke_count: Ptr<i32> = invoke_count.as_pointer();
                    },
                    |i: Ptr<(Value<i32>, Value<i32>)>| {
                        assert!(
                            ({ (i).clone() } == {
                                (libcc2rs::next_refcount(
                                    c.with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.begin()),
                                    (invoke_count.with_mut(|__v| __v.postfix_inc()) as isize)
                                        as isize,
                                )
                                .iter_deref())
                            })
                        );
                    }
                );
                for_each_4(___first, ___last, ___f)
            });
            assert!(({ ((*invoke_count.borrow()) == 0) }));
        }
    }
    {
        let invoke_count: Value<i32> = Rc::new(RefCell::new(0));
        ({
            let ___first: BidirectionalIterator<(Value<i32>, Value<i32>)> = c
                .as_pointer()
                .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.begin());
            let ___last: BidirectionalIterator<(Value<i32>, Value<i32>)> = libcc2rs::next_refcount(
                c.as_pointer()
                    .with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.begin()),
                1_isize as isize,
            );
            let ___f: FnPtr<fn(Ptr<(Value<i32>, Value<i32>)>)> = lambda!(
                {
                    let c: Ptr<Map<i32, (Value<i32>, Value<i32>)>> = c.as_pointer();
                    let invoke_count: Ptr<i32> = invoke_count.as_pointer();
                },
                |i: Ptr<(Value<i32>, Value<i32>)>| {
                    assert!(
                        ({ (i).clone() } == {
                            (libcc2rs::next_refcount(
                                c.with(|__m: &Map<i32, (Value<i32>, Value<i32>)>| __m.begin()),
                                (invoke_count.with_mut(|__v| __v.postfix_inc()) as isize) as isize,
                            )
                            .iter_deref())
                        })
                    );
                }
            );
            for_each_5(___first, ___last, ___f)
        });
        assert!(({ ((*invoke_count.borrow()) == 1) }));
    }
}
pub fn test_6() -> bool {
    ({
        test_node_container_7(FnPtr::<fn(i32) -> i32>::new(|i: i32| -> i32 {
            {
                let i: Value<i32> = Rc::new(RefCell::new(i));
                return (*i.borrow());
            }
        }))
    });
    ({
        test_node_container_8(FnPtr::<fn(i32) -> i32>::new(|i: i32| -> i32 {
            {
                let i: Value<i32> = Rc::new(RefCell::new(i));
                return (*i.borrow());
            }
        }))
    });
    ({
        test_node_container_0(FnPtr::<fn(i32) -> (Value<i32>, Value<i32>)>::new(
            |mut i: i32| -> (Value<i32>, Value<i32>) {
                {
                    return (Rc::new(RefCell::new(i)), Rc::new(RefCell::new(i)));
                }
            },
        ))
    });
    ({
        test_node_container_9(FnPtr::<fn(i32) -> (Value<i32>, Value<i32>)>::new(
            |mut i: i32| -> (Value<i32>, Value<i32>) {
                {
                    return (Rc::new(RefCell::new(i)), Rc::new(RefCell::new(i)));
                }
            },
        ))
    });
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
    ({ test_6() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
