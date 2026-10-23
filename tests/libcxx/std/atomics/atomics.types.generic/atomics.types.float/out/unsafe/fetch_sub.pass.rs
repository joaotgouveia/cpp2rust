extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_seq_cst_0(
    mut store_op: FnPtr<fn(*mut std_atomic_float_, (), ())>,
    mut load_op: FnPtr<Generic>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_1(
    mut store_op: FnPtr<fn(*mut std_atomic_float_, (), ())>,
    mut load_op: FnPtr<Generic>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_2(
    mut store_op: FnPtr<fn(*mut (), (), ())>,
    mut load_op: FnPtr<Generic>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_3(
    mut store_op: FnPtr<fn(*mut (), (), ())>,
    mut load_op: FnPtr<Generic>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_4(
    mut store_op: FnPtr<fn(*mut std_atomic_double_, (), ())>,
    mut load_op: FnPtr<Generic>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_5(
    mut store_op: FnPtr<fn(*mut std_atomic_double_, (), ())>,
    mut load_op: FnPtr<Generic>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_6(
    mut store_op: FnPtr<fn(*mut (), (), ())>,
    mut load_op: FnPtr<Generic>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_7(
    mut store_op: FnPtr<fn(*mut (), (), ())>,
    mut load_op: FnPtr<Generic>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_acquire_release_8(
    mut store_op: FnPtr<fn(*mut std_atomic_float_, (), ())>,
    mut load_op: FnPtr<fn(*mut std_atomic_float_) -> ()>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_acquire_release_9(
    mut store_op: FnPtr<fn(*mut (), (), ())>,
    mut load_op: FnPtr<fn(*mut ()) -> ()>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_acquire_release_10(
    mut store_op: FnPtr<fn(*mut std_atomic_double_, (), ())>,
    mut load_op: FnPtr<fn(*mut std_atomic_double_) -> ()>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_acquire_release_11(
    mut store_op: FnPtr<fn(*mut (), (), ())>,
    mut load_op: FnPtr<fn(*mut ()) -> ()>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_impl_12() {
    const _: () = assert!(
        true,
        "HasVolatileFetchSub<T> == std::atomic<T>::is_always_lock_free"
    );;
    const _: () = assert!(
        true,
        "noexcept(std::declval<MaybeVolatile<std::atomic<T>>&>().fetch_sub(T(0)))"
    );;
    {
        let mut a: std_atomic_float_ = unimplemented!();
        let mut r: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut store: FnPtr<fn(*mut std_atomic_float_, (), ())> =
            FnPtr::<fn(*mut std_atomic_float_, (), ())>::new(
                |x: *mut std_atomic_float_, mut old_val: (), mut new_val: ()| unsafe {
                    unimplemented!();
                },
            );
        let mut load: FnPtr<fn(*mut std_atomic_float_) -> ()> =
            FnPtr::<fn(*mut std_atomic_float_) -> ()>::new(|x: *mut std_atomic_float_| -> () {
                unsafe {
                    return (unsafe {
                        ((*x) as std___atomic_base_float__false_).load_6(Some(unimplemented!()))
                    });
                }
            });
        (unsafe { test_acquire_release_8(store.copy_from(), load.copy_from()) });
    }
    {
        let mut fetch_sub: FnPtr<fn(*mut std_atomic_float_, (), ())> =
            FnPtr::<fn(*mut std_atomic_float_, (), ())>::new(
                |x: *mut std_atomic_float_, mut old_value: (), mut new_val: ()| unsafe {
                    unimplemented!();
                },
            );
        let mut fetch_sub_with_order: FnPtr<fn(*mut std_atomic_float_, (), ())> =
            FnPtr::<fn(*mut std_atomic_float_, (), ())>::new(
                |x: *mut std_atomic_float_, mut old_value: (), mut new_val: ()| unsafe {
                    unimplemented!();
                },
            );
        let mut load: FnPtr<Generic> = lambda_unsafe!(Generic, {});
        (unsafe { test_seq_cst_0(fetch_sub.copy_from(), load.copy_from()) });
        (unsafe { test_seq_cst_1(fetch_sub_with_order.copy_from(), load.copy_from()) });
    }
}
pub unsafe fn test_impl_13() {
    const _: () = assert!(
        true,
        "HasVolatileFetchSub<T> == std::atomic<T>::is_always_lock_free"
    );;
    const _: () = assert!(
        true,
        "noexcept(std::declval<MaybeVolatile<std::atomic<T>>&>().fetch_sub(T(0)))"
    );;
    {
        let mut a: () = unimplemented!();
        let mut r: () = (unsafe { a.fetch_sub_13(1.2_f64, Some(unimplemented!())) });
        unimplemented!();
        unimplemented!();
    }
    {
        let mut store: FnPtr<fn(*mut (), (), ())> = FnPtr::<fn(*mut (), (), ())>::new(
            |x: *mut (), mut old_val: (), mut new_val: ()| unsafe {
                (unsafe {
                    let ___op: () = ((old_val) - (new_val));
                    (*x).fetch_sub_13(___op, Some(unimplemented!()))
                });
            },
        );
        let mut load: FnPtr<fn(*mut ()) -> ()> =
            FnPtr::<fn(*mut ()) -> ()>::new(|x: *mut ()| -> () {
                unsafe {
                    return (unsafe {
                        ((*x) as std___atomic_base_float__false_).load_5(Some(unimplemented!()))
                    });
                }
            });
        (unsafe { test_acquire_release_9(store.copy_from(), load.copy_from()) });
    }
    {
        let mut fetch_sub: FnPtr<fn(*mut (), (), ())> = FnPtr::<fn(*mut (), (), ())>::new(
            |x: *mut (), mut old_value: (), mut new_val: ()| unsafe {
                (unsafe {
                    let ___op: () = ((old_value) - (new_val));
                    (*x).fetch_sub_13(___op, None)
                });
            },
        );
        let mut fetch_sub_with_order: FnPtr<fn(*mut (), (), ())> =
            FnPtr::<fn(*mut (), (), ())>::new(
                |x: *mut (), mut old_value: (), mut new_val: ()| unsafe {
                    (unsafe {
                        let ___op: () = ((old_value) - (new_val));
                        (*x).fetch_sub_13(___op, Some(unimplemented!()))
                    });
                },
            );
        let mut load: FnPtr<Generic> = lambda_unsafe!(Generic, {});
        (unsafe { test_seq_cst_2(fetch_sub.copy_from(), load.copy_from()) });
        (unsafe { test_seq_cst_3(fetch_sub_with_order.copy_from(), load.copy_from()) });
    }
}
pub unsafe fn test_impl_14() {
    const _: () = assert!(
        true,
        "HasVolatileFetchSub<T> == std::atomic<T>::is_always_lock_free"
    );;
    const _: () = assert!(
        true,
        "noexcept(std::declval<MaybeVolatile<std::atomic<T>>&>().fetch_sub(T(0)))"
    );;
    {
        let mut a: std_atomic_double_ = unimplemented!();
        let mut r: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut store: FnPtr<fn(*mut std_atomic_double_, (), ())> =
            FnPtr::<fn(*mut std_atomic_double_, (), ())>::new(
                |x: *mut std_atomic_double_, mut old_val: (), mut new_val: ()| unsafe {
                    unimplemented!();
                },
            );
        let mut load: FnPtr<fn(*mut std_atomic_double_) -> ()> =
            FnPtr::<fn(*mut std_atomic_double_) -> ()>::new(|x: *mut std_atomic_double_| -> () {
                unsafe {
                    return (unsafe {
                        ((*x) as std___atomic_base_double__false_).load_6(Some(unimplemented!()))
                    });
                }
            });
        (unsafe { test_acquire_release_10(store.copy_from(), load.copy_from()) });
    }
    {
        let mut fetch_sub: FnPtr<fn(*mut std_atomic_double_, (), ())> =
            FnPtr::<fn(*mut std_atomic_double_, (), ())>::new(
                |x: *mut std_atomic_double_, mut old_value: (), mut new_val: ()| unsafe {
                    unimplemented!();
                },
            );
        let mut fetch_sub_with_order: FnPtr<fn(*mut std_atomic_double_, (), ())> =
            FnPtr::<fn(*mut std_atomic_double_, (), ())>::new(
                |x: *mut std_atomic_double_, mut old_value: (), mut new_val: ()| unsafe {
                    unimplemented!();
                },
            );
        let mut load: FnPtr<Generic> = lambda_unsafe!(Generic, {});
        (unsafe { test_seq_cst_4(fetch_sub.copy_from(), load.copy_from()) });
        (unsafe { test_seq_cst_5(fetch_sub_with_order.copy_from(), load.copy_from()) });
    }
}
pub unsafe fn test_impl_15() {
    const _: () = assert!(
        true,
        "HasVolatileFetchSub<T> == std::atomic<T>::is_always_lock_free"
    );;
    const _: () = assert!(
        true,
        "noexcept(std::declval<MaybeVolatile<std::atomic<T>>&>().fetch_sub(T(0)))"
    );;
    {
        let mut a: () = unimplemented!();
        let mut r: () = (unsafe { a.fetch_sub_13(1.2_f64, Some(unimplemented!())) });
        unimplemented!();
        unimplemented!();
    }
    {
        let mut store: FnPtr<fn(*mut (), (), ())> = FnPtr::<fn(*mut (), (), ())>::new(
            |x: *mut (), mut old_val: (), mut new_val: ()| unsafe {
                (unsafe {
                    let ___op: () = ((old_val) - (new_val));
                    (*x).fetch_sub_13(___op, Some(unimplemented!()))
                });
            },
        );
        let mut load: FnPtr<fn(*mut ()) -> ()> =
            FnPtr::<fn(*mut ()) -> ()>::new(|x: *mut ()| -> () {
                unsafe {
                    return (unsafe {
                        ((*x) as std___atomic_base_double__false_).load_5(Some(unimplemented!()))
                    });
                }
            });
        (unsafe { test_acquire_release_11(store.copy_from(), load.copy_from()) });
    }
    {
        let mut fetch_sub: FnPtr<fn(*mut (), (), ())> = FnPtr::<fn(*mut (), (), ())>::new(
            |x: *mut (), mut old_value: (), mut new_val: ()| unsafe {
                (unsafe {
                    let ___op: () = ((old_value) - (new_val));
                    (*x).fetch_sub_13(___op, None)
                });
            },
        );
        let mut fetch_sub_with_order: FnPtr<fn(*mut (), (), ())> =
            FnPtr::<fn(*mut (), (), ())>::new(
                |x: *mut (), mut old_value: (), mut new_val: ()| unsafe {
                    (unsafe {
                        let ___op: () = ((old_value) - (new_val));
                        (*x).fetch_sub_13(___op, Some(unimplemented!()))
                    });
                },
            );
        let mut load: FnPtr<Generic> = lambda_unsafe!(Generic, {});
        (unsafe { test_seq_cst_6(fetch_sub.copy_from(), load.copy_from()) });
        (unsafe { test_seq_cst_7(fetch_sub_with_order.copy_from(), load.copy_from()) });
    }
}
pub unsafe fn test_16() {
    (unsafe { test_impl_12() });
    {
        (unsafe { test_impl_13() });
    }
}
pub unsafe fn test_17() {
    (unsafe { test_impl_14() });
    {
        (unsafe { test_impl_15() });
    }
}
pub fn main() {
    let mut args: Vec<Vec<u8>> = std::env::args()
        .map(|arg| arg.as_bytes().to_vec())
        .collect();
    args.iter_mut().for_each(|v| v.push(0));
    let mut argv: Vec<*mut libc::c_char> = args
        .iter()
        .map(|arg| arg.as_ptr() as *mut libc::c_char)
        .collect();
    argv.push(::std::ptr::null_mut());
    unsafe {
        __cpp2rust_init_globals();
        ::std::process::exit(main_0((argv.len() - 1) as i32, argv.as_mut_ptr()) as i32)
    }
}
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    (unsafe { test_16() });
    (unsafe { test_17() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
