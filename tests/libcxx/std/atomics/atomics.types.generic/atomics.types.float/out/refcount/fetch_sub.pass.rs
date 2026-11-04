extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_seq_cst_0(
    store_op: FnPtr<fn(Ptr<std_atomic_float_>, f32, f32)>,
    load_op: FnPtr<Generic>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_float_>, f32, f32)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<Generic>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_1(
    store_op: FnPtr<fn(Ptr<std_atomic_float_>, f32, f32)>,
    load_op: FnPtr<Generic>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_float_>, f32, f32)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<Generic>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_2(
    store_op: FnPtr<fn(Ptr<std_atomic_float_>, f32, f32)>,
    load_op: FnPtr<Generic>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_float_>, f32, f32)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<Generic>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_3(
    store_op: FnPtr<fn(Ptr<std_atomic_float_>, f32, f32)>,
    load_op: FnPtr<Generic>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_float_>, f32, f32)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<Generic>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_4(
    store_op: FnPtr<fn(Ptr<std_atomic_double_>, f64, f64)>,
    load_op: FnPtr<Generic>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_double_>, f64, f64)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<Generic>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_5(
    store_op: FnPtr<fn(Ptr<std_atomic_double_>, f64, f64)>,
    load_op: FnPtr<Generic>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_double_>, f64, f64)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<Generic>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_6(
    store_op: FnPtr<fn(Ptr<std_atomic_double_>, f64, f64)>,
    load_op: FnPtr<Generic>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_double_>, f64, f64)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<Generic>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_7(
    store_op: FnPtr<fn(Ptr<std_atomic_double_>, f64, f64)>,
    load_op: FnPtr<Generic>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_double_>, f64, f64)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<Generic>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_8(
    store_op: FnPtr<fn(Ptr<std_atomic_float_>, f32, f32)>,
    load_op: FnPtr<fn(Ptr<std_atomic_float_>) -> f32>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_float_>, f32, f32)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_float_>) -> f32>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_9(
    store_op: FnPtr<fn(Ptr<std_atomic_float_>, f32, f32)>,
    load_op: FnPtr<fn(Ptr<std_atomic_float_>) -> f32>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_float_>, f32, f32)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_float_>) -> f32>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_10(
    store_op: FnPtr<fn(Ptr<std_atomic_double_>, f64, f64)>,
    load_op: FnPtr<fn(Ptr<std_atomic_double_>) -> f64>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_double_>, f64, f64)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_double_>) -> f64>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_11(
    store_op: FnPtr<fn(Ptr<std_atomic_double_>, f64, f64)>,
    load_op: FnPtr<fn(Ptr<std_atomic_double_>) -> f64>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_double_>, f64, f64)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_double_>) -> f64>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_impl_12() {
    const _: () = assert!(
        true,
        "HasVolatileFetchSub<T> == std::atomic<T>::is_always_lock_free"
    );;
    const _: () = assert!(
        true,
        "noexcept(std::declval<MaybeVolatile<std::atomic<T>>&>().fetch_sub(T(0)))"
    );;
    {
        let a: Value<std_atomic_float_> =
            Rc::new(RefCell::new(std_atomic_float_::new_1({ (3.1_f64 as f32) })));
        let mut r: f32 = ({
            ((*a.borrow()) as std___atomic_float_float_)
                .fetch_sub_31((1.2_f64 as f32), Some(std_memory_order_relaxed))
        });
        assert!((r == (3.1_f64 as f32)));
        assert!(
            ({
                (({ ((*a.borrow()) as std___atomic_float_float_).load_13(None) })
                    == ((3.1_f64 as f32) - (1.2_f64 as f32)))
            })
        );
    }
    {
        let store: Value<FnPtr<fn(Ptr<std_atomic_float_>, f32, f32)>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<std_atomic_float_>, f32, f32)>::new(
                |x: Ptr<std_atomic_float_>, mut old_val: f32, mut new_val: f32| {
                    ({
                        ((*x.upgrade().deref()) as std___atomic_float_float_)
                            .fetch_sub_31((old_val - new_val), Some(std_memory_order_release))
                    });
                },
            ),
        ));
        let load: Value<FnPtr<fn(Ptr<std_atomic_float_>) -> f32>> = Rc::new(RefCell::new(FnPtr::<
            fn(Ptr<std_atomic_float_>) -> f32,
        >::new(
            |x: Ptr<std_atomic_float_>| -> f32 {
                {
                    return ({
                        ((*x.upgrade().deref()) as std___atomic_float_float_)
                            .load_13(Some(std_memory_order_acquire))
                    });
                }
            },
        )));
        ({ test_acquire_release_8((*store.borrow()).copy_from(), (*load.borrow()).copy_from()) });
    }
    {
        let fetch_sub: Value<FnPtr<fn(Ptr<std_atomic_float_>, f32, f32)>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<std_atomic_float_>, f32, f32)>::new(
                |x: Ptr<std_atomic_float_>, mut old_value: f32, mut new_val: f32| {
                    ({
                        ((*x.upgrade().deref()) as std___atomic_float_float_)
                            .fetch_sub_31((old_value - new_val), None)
                    });
                },
            ),
        ));
        let fetch_sub_with_order: Value<FnPtr<fn(Ptr<std_atomic_float_>, f32, f32)>> = Rc::new(
            RefCell::new(FnPtr::<fn(Ptr<std_atomic_float_>, f32, f32)>::new(
                |x: Ptr<std_atomic_float_>, mut old_value: f32, mut new_val: f32| {
                    ({
                        ((*x.upgrade().deref()) as std___atomic_float_float_)
                            .fetch_sub_31((old_value - new_val), Some(std_memory_order_seq_cst))
                    });
                },
            )),
        );
        let load: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(Generic, {})));
        ({
            test_seq_cst_0(
                (*fetch_sub.borrow()).copy_from(),
                (*load.borrow()).copy_from(),
            )
        });
        ({
            test_seq_cst_1(
                (*fetch_sub_with_order.borrow()).copy_from(),
                (*load.borrow()).copy_from(),
            )
        });
    }
}
pub fn test_impl_13() {
    const _: () = assert!(
        true,
        "HasVolatileFetchSub<T> == std::atomic<T>::is_always_lock_free"
    );;
    const _: () = assert!(
        true,
        "noexcept(std::declval<MaybeVolatile<std::atomic<T>>&>().fetch_sub(T(0)))"
    );;
    {
        let a: Value<std_atomic_float_> =
            Rc::new(RefCell::new(std_atomic_float_::new_1({ (3.1_f64 as f32) })));
        let mut r: f32 = ({
            ((*a.borrow()) as std___atomic_float_float_)
                .fetch_sub_32((1.2_f64 as f32), Some(std_memory_order_relaxed))
        });
        assert!((r == (3.1_f64 as f32)));
        assert!(
            ({
                (({ ((*a.borrow()) as std___atomic_float_float_).load_12(None) })
                    == ((3.1_f64 as f32) - (1.2_f64 as f32)))
            })
        );
    }
    {
        let store: Value<FnPtr<fn(Ptr<std_atomic_float_>, f32, f32)>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<std_atomic_float_>, f32, f32)>::new(
                |x: Ptr<std_atomic_float_>, mut old_val: f32, mut new_val: f32| {
                    ({
                        ((*x.upgrade().deref()) as std___atomic_float_float_)
                            .fetch_sub_32((old_val - new_val), Some(std_memory_order_release))
                    });
                },
            ),
        ));
        let load: Value<FnPtr<fn(Ptr<std_atomic_float_>) -> f32>> = Rc::new(RefCell::new(FnPtr::<
            fn(Ptr<std_atomic_float_>) -> f32,
        >::new(
            |x: Ptr<std_atomic_float_>| -> f32 {
                {
                    return ({
                        ((*x.upgrade().deref()) as std___atomic_float_float_)
                            .load_12(Some(std_memory_order_acquire))
                    });
                }
            },
        )));
        ({ test_acquire_release_9((*store.borrow()).copy_from(), (*load.borrow()).copy_from()) });
    }
    {
        let fetch_sub: Value<FnPtr<fn(Ptr<std_atomic_float_>, f32, f32)>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<std_atomic_float_>, f32, f32)>::new(
                |x: Ptr<std_atomic_float_>, mut old_value: f32, mut new_val: f32| {
                    ({
                        ((*x.upgrade().deref()) as std___atomic_float_float_)
                            .fetch_sub_32((old_value - new_val), None)
                    });
                },
            ),
        ));
        let fetch_sub_with_order: Value<FnPtr<fn(Ptr<std_atomic_float_>, f32, f32)>> = Rc::new(
            RefCell::new(FnPtr::<fn(Ptr<std_atomic_float_>, f32, f32)>::new(
                |x: Ptr<std_atomic_float_>, mut old_value: f32, mut new_val: f32| {
                    ({
                        ((*x.upgrade().deref()) as std___atomic_float_float_)
                            .fetch_sub_32((old_value - new_val), Some(std_memory_order_seq_cst))
                    });
                },
            )),
        );
        let load: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(Generic, {})));
        ({
            test_seq_cst_2(
                (*fetch_sub.borrow()).copy_from(),
                (*load.borrow()).copy_from(),
            )
        });
        ({
            test_seq_cst_3(
                (*fetch_sub_with_order.borrow()).copy_from(),
                (*load.borrow()).copy_from(),
            )
        });
    }
}
pub fn test_impl_14() {
    const _: () = assert!(
        true,
        "HasVolatileFetchSub<T> == std::atomic<T>::is_always_lock_free"
    );;
    const _: () = assert!(
        true,
        "noexcept(std::declval<MaybeVolatile<std::atomic<T>>&>().fetch_sub(T(0)))"
    );;
    {
        let a: Value<std_atomic_double_> =
            Rc::new(RefCell::new(std_atomic_double_::new_1({ 3.1_f64 })));
        let mut r: f64 = ({
            ((*a.borrow()) as std___atomic_float_double_)
                .fetch_sub_31(1.2_f64, Some(std_memory_order_relaxed))
        });
        assert!((r == 3.1_f64));
        assert!(
            ({
                (({ ((*a.borrow()) as std___atomic_float_double_).load_13(None) })
                    == (3.1_f64 - 1.2_f64))
            })
        );
    }
    {
        let store: Value<FnPtr<fn(Ptr<std_atomic_double_>, f64, f64)>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<std_atomic_double_>, f64, f64)>::new(
                |x: Ptr<std_atomic_double_>, mut old_val: f64, mut new_val: f64| {
                    ({
                        ((*x.upgrade().deref()) as std___atomic_float_double_)
                            .fetch_sub_31((old_val - new_val), Some(std_memory_order_release))
                    });
                },
            ),
        ));
        let load: Value<FnPtr<fn(Ptr<std_atomic_double_>) -> f64>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<std_atomic_double_>) -> f64>::new(|x: Ptr<std_atomic_double_>| -> f64 {
                {
                    return ({
                        ((*x.upgrade().deref()) as std___atomic_float_double_)
                            .load_13(Some(std_memory_order_acquire))
                    });
                }
            }),
        ));
        ({ test_acquire_release_10((*store.borrow()).copy_from(), (*load.borrow()).copy_from()) });
    }
    {
        let fetch_sub: Value<FnPtr<fn(Ptr<std_atomic_double_>, f64, f64)>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<std_atomic_double_>, f64, f64)>::new(
                |x: Ptr<std_atomic_double_>, mut old_value: f64, mut new_val: f64| {
                    ({
                        ((*x.upgrade().deref()) as std___atomic_float_double_)
                            .fetch_sub_31((old_value - new_val), None)
                    });
                },
            ),
        ));
        let fetch_sub_with_order: Value<FnPtr<fn(Ptr<std_atomic_double_>, f64, f64)>> = Rc::new(
            RefCell::new(FnPtr::<fn(Ptr<std_atomic_double_>, f64, f64)>::new(
                |x: Ptr<std_atomic_double_>, mut old_value: f64, mut new_val: f64| {
                    ({
                        ((*x.upgrade().deref()) as std___atomic_float_double_)
                            .fetch_sub_31((old_value - new_val), Some(std_memory_order_seq_cst))
                    });
                },
            )),
        );
        let load: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(Generic, {})));
        ({
            test_seq_cst_4(
                (*fetch_sub.borrow()).copy_from(),
                (*load.borrow()).copy_from(),
            )
        });
        ({
            test_seq_cst_5(
                (*fetch_sub_with_order.borrow()).copy_from(),
                (*load.borrow()).copy_from(),
            )
        });
    }
}
pub fn test_impl_15() {
    const _: () = assert!(
        true,
        "HasVolatileFetchSub<T> == std::atomic<T>::is_always_lock_free"
    );;
    const _: () = assert!(
        true,
        "noexcept(std::declval<MaybeVolatile<std::atomic<T>>&>().fetch_sub(T(0)))"
    );;
    {
        let a: Value<std_atomic_double_> =
            Rc::new(RefCell::new(std_atomic_double_::new_1({ 3.1_f64 })));
        let mut r: f64 = ({
            ((*a.borrow()) as std___atomic_float_double_)
                .fetch_sub_32(1.2_f64, Some(std_memory_order_relaxed))
        });
        assert!((r == 3.1_f64));
        assert!(
            ({
                (({ ((*a.borrow()) as std___atomic_float_double_).load_12(None) })
                    == (3.1_f64 - 1.2_f64))
            })
        );
    }
    {
        let store: Value<FnPtr<fn(Ptr<std_atomic_double_>, f64, f64)>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<std_atomic_double_>, f64, f64)>::new(
                |x: Ptr<std_atomic_double_>, mut old_val: f64, mut new_val: f64| {
                    ({
                        ((*x.upgrade().deref()) as std___atomic_float_double_)
                            .fetch_sub_32((old_val - new_val), Some(std_memory_order_release))
                    });
                },
            ),
        ));
        let load: Value<FnPtr<fn(Ptr<std_atomic_double_>) -> f64>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<std_atomic_double_>) -> f64>::new(|x: Ptr<std_atomic_double_>| -> f64 {
                {
                    return ({
                        ((*x.upgrade().deref()) as std___atomic_float_double_)
                            .load_12(Some(std_memory_order_acquire))
                    });
                }
            }),
        ));
        ({ test_acquire_release_11((*store.borrow()).copy_from(), (*load.borrow()).copy_from()) });
    }
    {
        let fetch_sub: Value<FnPtr<fn(Ptr<std_atomic_double_>, f64, f64)>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<std_atomic_double_>, f64, f64)>::new(
                |x: Ptr<std_atomic_double_>, mut old_value: f64, mut new_val: f64| {
                    ({
                        ((*x.upgrade().deref()) as std___atomic_float_double_)
                            .fetch_sub_32((old_value - new_val), None)
                    });
                },
            ),
        ));
        let fetch_sub_with_order: Value<FnPtr<fn(Ptr<std_atomic_double_>, f64, f64)>> = Rc::new(
            RefCell::new(FnPtr::<fn(Ptr<std_atomic_double_>, f64, f64)>::new(
                |x: Ptr<std_atomic_double_>, mut old_value: f64, mut new_val: f64| {
                    ({
                        ((*x.upgrade().deref()) as std___atomic_float_double_)
                            .fetch_sub_32((old_value - new_val), Some(std_memory_order_seq_cst))
                    });
                },
            )),
        );
        let load: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(Generic, {})));
        ({
            test_seq_cst_6(
                (*fetch_sub.borrow()).copy_from(),
                (*load.borrow()).copy_from(),
            )
        });
        ({
            test_seq_cst_7(
                (*fetch_sub_with_order.borrow()).copy_from(),
                (*load.borrow()).copy_from(),
            )
        });
    }
}
pub fn test_16() {
    ({ test_impl_12() });
    {
        ({ test_impl_13() });
    }
}
pub fn test_17() {
    ({ test_impl_14() });
    {
        ({ test_impl_15() });
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
    ({ test_16() });
    ({ test_17() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
