extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_seq_cst_0(
    mut store_op: FnPtr<fn(*mut (), (), ())>,
    mut load_op: FnPtr<fn(*mut ()) -> ()>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_1(
    mut store_op: FnPtr<fn(*mut (), (), ())>,
    mut load_op: FnPtr<fn(*mut ()) -> ()>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_2(
    mut store_op: FnPtr<fn(*mut (), (), ())>,
    mut load_op: FnPtr<fn(*mut ()) -> ()>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_3(
    mut store_op: FnPtr<fn(*mut (), (), ())>,
    mut load_op: FnPtr<fn(*mut ()) -> ()>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_4(
    mut store_op: FnPtr<fn(*mut (), (), ())>,
    mut load_op: FnPtr<fn(*mut ()) -> ()>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_5(
    mut store_op: FnPtr<fn(*mut (), (), ())>,
    mut load_op: FnPtr<fn(*mut ()) -> ()>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_6(
    mut store_op: FnPtr<fn(*mut (), (), ())>,
    mut load_op: FnPtr<fn(*mut ()) -> ()>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_7(
    mut store_op: FnPtr<fn(*mut (), (), ())>,
    mut load_op: FnPtr<fn(*mut ()) -> ()>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_acquire_release_8(
    mut store_op: FnPtr<fn(*mut (), (), ())>,
    mut load_op: FnPtr<fn(*mut ()) -> ()>,
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
    mut store_op: FnPtr<fn(*mut (), (), ())>,
    mut load_op: FnPtr<fn(*mut ()) -> ()>,
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
        "HasVolatileFetchAdd<T> == std::atomic<T>::is_always_lock_free"
    );;
    const _: () = assert!(
        true,
        "noexcept(std::declval<MaybeVolatile<std::atomic<T>>&>().fetch_add(T(0)))"
    );;
    {
        let mut a: () = unimplemented!();
        let mut r: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut store: FnPtr<fn(*mut (), (), ())> = FnPtr::<fn(*mut (), (), ())>::new(
            |x: *mut (), mut old_val: (), mut new_val: ()| unsafe {
                unimplemented!();
            },
        );
        let mut load: FnPtr<fn(*mut ()) -> ()> =
            FnPtr::<fn(*mut ()) -> ()>::new(|x: *mut ()| -> () {
                unsafe {
                    return unimplemented!();
                }
            });
        (unsafe { test_acquire_release_8(store.copy_from(), load.copy_from()) });
    }
    {
        let mut fetch_add: FnPtr<fn(*mut (), (), ())> = FnPtr::<fn(*mut (), (), ())>::new(
            |x: *mut (), mut old_value: (), mut new_val: ()| unsafe {
                unimplemented!();
            },
        );
        let mut fetch_add_with_order: FnPtr<fn(*mut (), (), ())> =
            FnPtr::<fn(*mut (), (), ())>::new(
                |x: *mut (), mut old_value: (), mut new_val: ()| unsafe {
                    unimplemented!();
                },
            );
        let mut load: FnPtr<fn(*mut ()) -> ()> =
            FnPtr::<fn(*mut ()) -> ()>::new(|x: *mut ()| -> () {
                unsafe {
                    return unimplemented!();
                }
            });
        (unsafe { test_seq_cst_0(fetch_add.copy_from(), load.copy_from()) });
        (unsafe { test_seq_cst_1(fetch_add_with_order.copy_from(), load.copy_from()) });
    }
}
pub unsafe fn test_impl_13() {
    const _: () = assert!(
        true,
        "HasVolatileFetchAdd<T> == std::atomic<T>::is_always_lock_free"
    );;
    const _: () = assert!(
        true,
        "noexcept(std::declval<MaybeVolatile<std::atomic<T>>&>().fetch_add(T(0)))"
    );;
    {
        let mut a: () = unimplemented!();
        let mut r: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut store: FnPtr<fn(*mut (), (), ())> = FnPtr::<fn(*mut (), (), ())>::new(
            |x: *mut (), mut old_val: (), mut new_val: ()| unsafe {
                unimplemented!();
            },
        );
        let mut load: FnPtr<fn(*mut ()) -> ()> =
            FnPtr::<fn(*mut ()) -> ()>::new(|x: *mut ()| -> () {
                unsafe {
                    return unimplemented!();
                }
            });
        (unsafe { test_acquire_release_9(store.copy_from(), load.copy_from()) });
    }
    {
        let mut fetch_add: FnPtr<fn(*mut (), (), ())> = FnPtr::<fn(*mut (), (), ())>::new(
            |x: *mut (), mut old_value: (), mut new_val: ()| unsafe {
                unimplemented!();
            },
        );
        let mut fetch_add_with_order: FnPtr<fn(*mut (), (), ())> =
            FnPtr::<fn(*mut (), (), ())>::new(
                |x: *mut (), mut old_value: (), mut new_val: ()| unsafe {
                    unimplemented!();
                },
            );
        let mut load: FnPtr<fn(*mut ()) -> ()> =
            FnPtr::<fn(*mut ()) -> ()>::new(|x: *mut ()| -> () {
                unsafe {
                    return unimplemented!();
                }
            });
        (unsafe { test_seq_cst_2(fetch_add.copy_from(), load.copy_from()) });
        (unsafe { test_seq_cst_3(fetch_add_with_order.copy_from(), load.copy_from()) });
    }
}
pub unsafe fn test_impl_14() {
    const _: () = assert!(
        true,
        "HasVolatileFetchAdd<T> == std::atomic<T>::is_always_lock_free"
    );;
    const _: () = assert!(
        true,
        "noexcept(std::declval<MaybeVolatile<std::atomic<T>>&>().fetch_add(T(0)))"
    );;
    {
        let mut a: () = unimplemented!();
        let mut r: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut store: FnPtr<fn(*mut (), (), ())> = FnPtr::<fn(*mut (), (), ())>::new(
            |x: *mut (), mut old_val: (), mut new_val: ()| unsafe {
                unimplemented!();
            },
        );
        let mut load: FnPtr<fn(*mut ()) -> ()> =
            FnPtr::<fn(*mut ()) -> ()>::new(|x: *mut ()| -> () {
                unsafe {
                    return unimplemented!();
                }
            });
        (unsafe { test_acquire_release_10(store.copy_from(), load.copy_from()) });
    }
    {
        let mut fetch_add: FnPtr<fn(*mut (), (), ())> = FnPtr::<fn(*mut (), (), ())>::new(
            |x: *mut (), mut old_value: (), mut new_val: ()| unsafe {
                unimplemented!();
            },
        );
        let mut fetch_add_with_order: FnPtr<fn(*mut (), (), ())> =
            FnPtr::<fn(*mut (), (), ())>::new(
                |x: *mut (), mut old_value: (), mut new_val: ()| unsafe {
                    unimplemented!();
                },
            );
        let mut load: FnPtr<fn(*mut ()) -> ()> =
            FnPtr::<fn(*mut ()) -> ()>::new(|x: *mut ()| -> () {
                unsafe {
                    return unimplemented!();
                }
            });
        (unsafe { test_seq_cst_4(fetch_add.copy_from(), load.copy_from()) });
        (unsafe { test_seq_cst_5(fetch_add_with_order.copy_from(), load.copy_from()) });
    }
}
pub unsafe fn test_impl_15() {
    const _: () = assert!(
        true,
        "HasVolatileFetchAdd<T> == std::atomic<T>::is_always_lock_free"
    );;
    const _: () = assert!(
        true,
        "noexcept(std::declval<MaybeVolatile<std::atomic<T>>&>().fetch_add(T(0)))"
    );;
    {
        let mut a: () = unimplemented!();
        let mut r: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut store: FnPtr<fn(*mut (), (), ())> = FnPtr::<fn(*mut (), (), ())>::new(
            |x: *mut (), mut old_val: (), mut new_val: ()| unsafe {
                unimplemented!();
            },
        );
        let mut load: FnPtr<fn(*mut ()) -> ()> =
            FnPtr::<fn(*mut ()) -> ()>::new(|x: *mut ()| -> () {
                unsafe {
                    return unimplemented!();
                }
            });
        (unsafe { test_acquire_release_11(store.copy_from(), load.copy_from()) });
    }
    {
        let mut fetch_add: FnPtr<fn(*mut (), (), ())> = FnPtr::<fn(*mut (), (), ())>::new(
            |x: *mut (), mut old_value: (), mut new_val: ()| unsafe {
                unimplemented!();
            },
        );
        let mut fetch_add_with_order: FnPtr<fn(*mut (), (), ())> =
            FnPtr::<fn(*mut (), (), ())>::new(
                |x: *mut (), mut old_value: (), mut new_val: ()| unsafe {
                    unimplemented!();
                },
            );
        let mut load: FnPtr<fn(*mut ()) -> ()> =
            FnPtr::<fn(*mut ()) -> ()>::new(|x: *mut ()| -> () {
                unsafe {
                    return unimplemented!();
                }
            });
        (unsafe { test_seq_cst_6(fetch_add.copy_from(), load.copy_from()) });
        (unsafe { test_seq_cst_7(fetch_add_with_order.copy_from(), load.copy_from()) });
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
