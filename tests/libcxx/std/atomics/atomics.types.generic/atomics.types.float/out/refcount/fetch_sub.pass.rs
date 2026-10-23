extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_seq_cst_0(store_op: FnPtr<fn(Ptr<()>, (), ())>, load_op: FnPtr<Generic>) {
    let store_op: Value<FnPtr<fn(Ptr<()>, (), ())>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<Generic>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_1(store_op: FnPtr<fn(Ptr<()>, (), ())>, load_op: FnPtr<Generic>) {
    let store_op: Value<FnPtr<fn(Ptr<()>, (), ())>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<Generic>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_2(store_op: FnPtr<fn(Ptr<()>, (), ())>, load_op: FnPtr<Generic>) {
    let store_op: Value<FnPtr<fn(Ptr<()>, (), ())>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<Generic>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_3(store_op: FnPtr<fn(Ptr<()>, (), ())>, load_op: FnPtr<Generic>) {
    let store_op: Value<FnPtr<fn(Ptr<()>, (), ())>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<Generic>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_4(store_op: FnPtr<fn(Ptr<()>, (), ())>, load_op: FnPtr<Generic>) {
    let store_op: Value<FnPtr<fn(Ptr<()>, (), ())>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<Generic>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_5(store_op: FnPtr<fn(Ptr<()>, (), ())>, load_op: FnPtr<Generic>) {
    let store_op: Value<FnPtr<fn(Ptr<()>, (), ())>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<Generic>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_6(store_op: FnPtr<fn(Ptr<()>, (), ())>, load_op: FnPtr<Generic>) {
    let store_op: Value<FnPtr<fn(Ptr<()>, (), ())>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<Generic>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_7(store_op: FnPtr<fn(Ptr<()>, (), ())>, load_op: FnPtr<Generic>) {
    let store_op: Value<FnPtr<fn(Ptr<()>, (), ())>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<Generic>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_8(
    store_op: FnPtr<fn(Ptr<()>, (), ())>,
    load_op: FnPtr<fn(Ptr<()>) -> ()>,
) {
    let store_op: Value<FnPtr<fn(Ptr<()>, (), ())>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> ()>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_9(
    store_op: FnPtr<fn(Ptr<()>, (), ())>,
    load_op: FnPtr<fn(Ptr<()>) -> ()>,
) {
    let store_op: Value<FnPtr<fn(Ptr<()>, (), ())>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> ()>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_10(
    store_op: FnPtr<fn(Ptr<()>, (), ())>,
    load_op: FnPtr<fn(Ptr<()>) -> ()>,
) {
    let store_op: Value<FnPtr<fn(Ptr<()>, (), ())>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> ()>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_acquire_release_11(
    store_op: FnPtr<fn(Ptr<()>, (), ())>,
    load_op: FnPtr<fn(Ptr<()>) -> ()>,
) {
    let store_op: Value<FnPtr<fn(Ptr<()>, (), ())>> = Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<()>) -> ()>> = Rc::new(RefCell::new(load_op));
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
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r: () = ({ (*a.borrow()).fetch_sub_14(1.2_f64, Some(unimplemented!())) });
        unimplemented!();
        unimplemented!();
    }
    {
        let store: Value<FnPtr<fn(Ptr<()>, (), ())>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, (), ())>::new(
                |x: Ptr<()>, mut old_val: (), mut new_val: ()| {
                    ({
                        let ___op: () = (old_val - new_val);
                        (*x.upgrade().deref()).fetch_sub_14(___op, Some(unimplemented!()))
                    });
                },
            )));
        let load: Value<FnPtr<fn(Ptr<()>) -> ()>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<()>) -> ()>::new(|x: Ptr<()>| -> () {
                {
                    return ({
                        ((*x.upgrade().deref()) as std___atomic_base_float__false_)
                            .load_6(Some(unimplemented!()))
                    });
                }
            }),
        ));
        ({ test_acquire_release_8((*store.borrow()).copy_from(), (*load.borrow()).copy_from()) });
    }
    {
        let fetch_sub: Value<FnPtr<fn(Ptr<()>, (), ())>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, (), ())>::new(
                |x: Ptr<()>, mut old_value: (), mut new_val: ()| {
                    ({
                        let ___op: () = (old_value - new_val);
                        (*x.upgrade().deref()).fetch_sub_14(___op, None)
                    });
                },
            )));
        let fetch_sub_with_order: Value<FnPtr<fn(Ptr<()>, (), ())>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<()>, (), ())>::new(|x: Ptr<()>, mut old_value: (), mut new_val: ()| {
                ({
                    let ___op: () = (old_value - new_val);
                    (*x.upgrade().deref()).fetch_sub_14(___op, Some(unimplemented!()))
                });
            }),
        ));
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
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r: () = ({ (*a.borrow()).fetch_sub_13(1.2_f64, Some(unimplemented!())) });
        unimplemented!();
        unimplemented!();
    }
    {
        let store: Value<FnPtr<fn(Ptr<()>, (), ())>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, (), ())>::new(
                |x: Ptr<()>, mut old_val: (), mut new_val: ()| {
                    ({
                        let ___op: () = (old_val - new_val);
                        (*x.upgrade().deref()).fetch_sub_13(___op, Some(unimplemented!()))
                    });
                },
            )));
        let load: Value<FnPtr<fn(Ptr<()>) -> ()>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<()>) -> ()>::new(|x: Ptr<()>| -> () {
                {
                    return ({
                        ((*x.upgrade().deref()) as std___atomic_base_float__false_)
                            .load_5(Some(unimplemented!()))
                    });
                }
            }),
        ));
        ({ test_acquire_release_9((*store.borrow()).copy_from(), (*load.borrow()).copy_from()) });
    }
    {
        let fetch_sub: Value<FnPtr<fn(Ptr<()>, (), ())>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, (), ())>::new(
                |x: Ptr<()>, mut old_value: (), mut new_val: ()| {
                    ({
                        let ___op: () = (old_value - new_val);
                        (*x.upgrade().deref()).fetch_sub_13(___op, None)
                    });
                },
            )));
        let fetch_sub_with_order: Value<FnPtr<fn(Ptr<()>, (), ())>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<()>, (), ())>::new(|x: Ptr<()>, mut old_value: (), mut new_val: ()| {
                ({
                    let ___op: () = (old_value - new_val);
                    (*x.upgrade().deref()).fetch_sub_13(___op, Some(unimplemented!()))
                });
            }),
        ));
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
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r: () = ({ (*a.borrow()).fetch_sub_14(1.2_f64, Some(unimplemented!())) });
        unimplemented!();
        unimplemented!();
    }
    {
        let store: Value<FnPtr<fn(Ptr<()>, (), ())>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, (), ())>::new(
                |x: Ptr<()>, mut old_val: (), mut new_val: ()| {
                    ({
                        let ___op: () = (old_val - new_val);
                        (*x.upgrade().deref()).fetch_sub_14(___op, Some(unimplemented!()))
                    });
                },
            )));
        let load: Value<FnPtr<fn(Ptr<()>) -> ()>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<()>) -> ()>::new(|x: Ptr<()>| -> () {
                {
                    return ({
                        ((*x.upgrade().deref()) as std___atomic_base_double__false_)
                            .load_6(Some(unimplemented!()))
                    });
                }
            }),
        ));
        ({ test_acquire_release_10((*store.borrow()).copy_from(), (*load.borrow()).copy_from()) });
    }
    {
        let fetch_sub: Value<FnPtr<fn(Ptr<()>, (), ())>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, (), ())>::new(
                |x: Ptr<()>, mut old_value: (), mut new_val: ()| {
                    ({
                        let ___op: () = (old_value - new_val);
                        (*x.upgrade().deref()).fetch_sub_14(___op, None)
                    });
                },
            )));
        let fetch_sub_with_order: Value<FnPtr<fn(Ptr<()>, (), ())>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<()>, (), ())>::new(|x: Ptr<()>, mut old_value: (), mut new_val: ()| {
                ({
                    let ___op: () = (old_value - new_val);
                    (*x.upgrade().deref()).fetch_sub_14(___op, Some(unimplemented!()))
                });
            }),
        ));
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
        let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut r: () = ({ (*a.borrow()).fetch_sub_13(1.2_f64, Some(unimplemented!())) });
        unimplemented!();
        unimplemented!();
    }
    {
        let store: Value<FnPtr<fn(Ptr<()>, (), ())>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, (), ())>::new(
                |x: Ptr<()>, mut old_val: (), mut new_val: ()| {
                    ({
                        let ___op: () = (old_val - new_val);
                        (*x.upgrade().deref()).fetch_sub_13(___op, Some(unimplemented!()))
                    });
                },
            )));
        let load: Value<FnPtr<fn(Ptr<()>) -> ()>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<()>) -> ()>::new(|x: Ptr<()>| -> () {
                {
                    return ({
                        ((*x.upgrade().deref()) as std___atomic_base_double__false_)
                            .load_5(Some(unimplemented!()))
                    });
                }
            }),
        ));
        ({ test_acquire_release_11((*store.borrow()).copy_from(), (*load.borrow()).copy_from()) });
    }
    {
        let fetch_sub: Value<FnPtr<fn(Ptr<()>, (), ())>> =
            Rc::new(RefCell::new(FnPtr::<fn(Ptr<()>, (), ())>::new(
                |x: Ptr<()>, mut old_value: (), mut new_val: ()| {
                    ({
                        let ___op: () = (old_value - new_val);
                        (*x.upgrade().deref()).fetch_sub_13(___op, None)
                    });
                },
            )));
        let fetch_sub_with_order: Value<FnPtr<fn(Ptr<()>, (), ())>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<()>, (), ())>::new(|x: Ptr<()>, mut old_value: (), mut new_val: ()| {
                ({
                    let ___op: () = (old_value - new_val);
                    (*x.upgrade().deref()).fetch_sub_13(___op, Some(unimplemented!()))
                });
            }),
        ));
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    ({ test_16() });
    ({ test_17() });
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
