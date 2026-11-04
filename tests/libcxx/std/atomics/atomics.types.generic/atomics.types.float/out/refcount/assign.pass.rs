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
    load_op: FnPtr<fn(Ptr<std_atomic_float_>) -> f32>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_float_>, f32, f32)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_float_>) -> f32>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_1(
    store_op: FnPtr<fn(Ptr<std_atomic_float_>, f32, f32)>,
    load_op: FnPtr<fn(Ptr<std_atomic_float_>) -> f32>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_float_>, f32, f32)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_float_>) -> f32>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_2(
    store_op: FnPtr<fn(Ptr<std_atomic_double_>, f64, f64)>,
    load_op: FnPtr<fn(Ptr<std_atomic_double_>) -> f64>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_double_>, f64, f64)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_double_>) -> f64>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_seq_cst_3(
    store_op: FnPtr<fn(Ptr<std_atomic_double_>, f64, f64)>,
    load_op: FnPtr<fn(Ptr<std_atomic_double_>) -> f64>,
) {
    let store_op: Value<FnPtr<fn(Ptr<std_atomic_double_>, f64, f64)>> =
        Rc::new(RefCell::new(store_op));
    let load_op: Value<FnPtr<fn(Ptr<std_atomic_double_>) -> f64>> = Rc::new(RefCell::new(load_op));
    &(*store_op.borrow_mut());
    &(*load_op.borrow_mut());
}
pub fn test_impl_4() {
    const _: () = assert!(
        true,
        "HasVolatileAssign<T> == std::atomic<T>::is_always_lock_free"
    );;
    const _: () = assert!(
        true,
        "noexcept(std::declval<MaybeVolatile<std::atomic<T>>&>() = (T(0)))"
    );;
    {
        let a: Value<std_atomic_float_> =
            Rc::new(RefCell::new(std_atomic_float_::new_1({ (3.1_f64 as f32) })));
        let mut r: f32 = ({
            ((*a.borrow_mut()) as std___atomic_float_float_) = (1.2_f64 as f32);
            ((*a.borrow()) as std___atomic_float_float_)
        });
        assert!(
            ({
                (({ ((*a.borrow()) as std___atomic_float_float_).load_13(None) })
                    == (1.2_f64 as f32))
            })
        );
        assert!((r == (1.2_f64 as f32)));
    }
    {
        let assign: Value<FnPtr<fn(Ptr<std_atomic_float_>, f32, f32)>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<std_atomic_float_>, f32, f32)>::new(
                |x: Ptr<std_atomic_float_>, mut _a1: f32, mut new_val: f32| {
                    x.write(new_val);
                },
            ),
        ));
        let load: Value<FnPtr<fn(Ptr<std_atomic_float_>) -> f32>> = Rc::new(RefCell::new(FnPtr::<
            fn(Ptr<std_atomic_float_>) -> f32,
        >::new(
            |x: Ptr<std_atomic_float_>| -> f32 {
                {
                    return ({
                        ((*x.upgrade().deref()) as std___atomic_float_float_).load_13(None)
                    });
                }
            },
        )));
        ({ test_seq_cst_0((*assign.borrow()).copy_from(), (*load.borrow()).copy_from()) });
    }
}
pub fn test_impl_5() {
    const _: () = assert!(
        true,
        "HasVolatileAssign<T> == std::atomic<T>::is_always_lock_free"
    );;
    const _: () = assert!(
        true,
        "noexcept(std::declval<MaybeVolatile<std::atomic<T>>&>() = (T(0)))"
    );;
    {
        let a: Value<std_atomic_float_> =
            Rc::new(RefCell::new(std_atomic_float_::new_1({ (3.1_f64 as f32) })));
        let mut r: f32 = ({
            ((*a.borrow_mut()) as std___atomic_float_float_) = (1.2_f64 as f32);
            ((*a.borrow()) as std___atomic_float_float_)
        });
        assert!(
            ({
                (({ ((*a.borrow()) as std___atomic_float_float_).load_12(None) })
                    == (1.2_f64 as f32))
            })
        );
        assert!((r == (1.2_f64 as f32)));
    }
    {
        let assign: Value<FnPtr<fn(Ptr<std_atomic_float_>, f32, f32)>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<std_atomic_float_>, f32, f32)>::new(
                |x: Ptr<std_atomic_float_>, mut _a1: f32, mut new_val: f32| {
                    x.write(new_val);
                },
            ),
        ));
        let load: Value<FnPtr<fn(Ptr<std_atomic_float_>) -> f32>> = Rc::new(RefCell::new(FnPtr::<
            fn(Ptr<std_atomic_float_>) -> f32,
        >::new(
            |x: Ptr<std_atomic_float_>| -> f32 {
                {
                    return ({
                        ((*x.upgrade().deref()) as std___atomic_float_float_).load_12(None)
                    });
                }
            },
        )));
        ({ test_seq_cst_1((*assign.borrow()).copy_from(), (*load.borrow()).copy_from()) });
    }
}
pub fn test_impl_6() {
    const _: () = assert!(
        true,
        "HasVolatileAssign<T> == std::atomic<T>::is_always_lock_free"
    );;
    const _: () = assert!(
        true,
        "noexcept(std::declval<MaybeVolatile<std::atomic<T>>&>() = (T(0)))"
    );;
    {
        let a: Value<std_atomic_double_> =
            Rc::new(RefCell::new(std_atomic_double_::new_1({ 3.1_f64 })));
        let mut r: f64 = ({
            ((*a.borrow_mut()) as std___atomic_float_double_) = 1.2_f64;
            ((*a.borrow()) as std___atomic_float_double_)
        });
        assert!(
            ({ (({ ((*a.borrow()) as std___atomic_float_double_).load_13(None,) }) == 1.2_f64) })
        );
        assert!((r == 1.2_f64));
    }
    {
        let assign: Value<FnPtr<fn(Ptr<std_atomic_double_>, f64, f64)>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<std_atomic_double_>, f64, f64)>::new(
                |x: Ptr<std_atomic_double_>, mut _a1: f64, mut new_val: f64| {
                    x.write(new_val);
                },
            ),
        ));
        let load: Value<FnPtr<fn(Ptr<std_atomic_double_>) -> f64>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<std_atomic_double_>) -> f64>::new(|x: Ptr<std_atomic_double_>| -> f64 {
                {
                    return ({
                        ((*x.upgrade().deref()) as std___atomic_float_double_).load_13(None)
                    });
                }
            }),
        ));
        ({ test_seq_cst_2((*assign.borrow()).copy_from(), (*load.borrow()).copy_from()) });
    }
}
pub fn test_impl_7() {
    const _: () = assert!(
        true,
        "HasVolatileAssign<T> == std::atomic<T>::is_always_lock_free"
    );;
    const _: () = assert!(
        true,
        "noexcept(std::declval<MaybeVolatile<std::atomic<T>>&>() = (T(0)))"
    );;
    {
        let a: Value<std_atomic_double_> =
            Rc::new(RefCell::new(std_atomic_double_::new_1({ 3.1_f64 })));
        let mut r: f64 = ({
            ((*a.borrow_mut()) as std___atomic_float_double_) = 1.2_f64;
            ((*a.borrow()) as std___atomic_float_double_)
        });
        assert!(
            ({ (({ ((*a.borrow()) as std___atomic_float_double_).load_12(None,) }) == 1.2_f64) })
        );
        assert!((r == 1.2_f64));
    }
    {
        let assign: Value<FnPtr<fn(Ptr<std_atomic_double_>, f64, f64)>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<std_atomic_double_>, f64, f64)>::new(
                |x: Ptr<std_atomic_double_>, mut _a1: f64, mut new_val: f64| {
                    x.write(new_val);
                },
            ),
        ));
        let load: Value<FnPtr<fn(Ptr<std_atomic_double_>) -> f64>> = Rc::new(RefCell::new(
            FnPtr::<fn(Ptr<std_atomic_double_>) -> f64>::new(|x: Ptr<std_atomic_double_>| -> f64 {
                {
                    return ({
                        ((*x.upgrade().deref()) as std___atomic_float_double_).load_12(None)
                    });
                }
            }),
        ));
        ({ test_seq_cst_3((*assign.borrow()).copy_from(), (*load.borrow()).copy_from()) });
    }
}
pub fn test_8() {
    ({ test_impl_4() });
    {
        ({ test_impl_5() });
    }
}
pub fn test_9() {
    ({ test_impl_6() });
    {
        ({ test_impl_7() });
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
    ({ test_8() });
    ({ test_9() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
