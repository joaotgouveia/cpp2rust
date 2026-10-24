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
        "noexcept(std::declval<MaybeVolatile<std::atomic<T>>&>() = (T(0)))"
    );;
    {
        let mut a: () = unimplemented!();
        let mut r: () = (unsafe {
            (a as std___atomic_base_float__false_).exchange_10(1.2_f64, Some(unimplemented!()))
        });
        unimplemented!();
        unimplemented!();
    }
    {
        let mut exchange: FnPtr<fn(*mut (), (), ())> =
            FnPtr::<fn(*mut (), (), ())>::new(|x: *mut (), mut _a1: (), mut new_val: ()| unsafe {
                (unsafe {
                    ((*x) as std___atomic_base_float__false_)
                        .exchange_10(new_val, Some(unimplemented!()))
                });
            });
        let mut load: FnPtr<fn(*mut ()) -> ()> =
            FnPtr::<fn(*mut ()) -> ()>::new(|x: *mut ()| -> () {
                unsafe {
                    return (unsafe {
                        ((*x) as std___atomic_base_float__false_).load_6(Some(unimplemented!()))
                    });
                }
            });
        (unsafe { test_acquire_release_8(exchange.copy_from(), load.copy_from()) });
    }
    {
        let mut exchange_no_arg: FnPtr<fn(*mut (), (), ())> =
            FnPtr::<fn(*mut (), (), ())>::new(|x: *mut (), mut _a1: (), mut new_val: ()| unsafe {
                (unsafe { ((*x) as std___atomic_base_float__false_).exchange_10(new_val, None) });
            });
        let mut exchange_with_order: FnPtr<fn(*mut (), (), ())> =
            FnPtr::<fn(*mut (), (), ())>::new(|x: *mut (), mut _a1: (), mut new_val: ()| unsafe {
                (unsafe {
                    ((*x) as std___atomic_base_float__false_)
                        .exchange_10(new_val, Some(unimplemented!()))
                });
            });
        let mut load: FnPtr<fn(*mut ()) -> ()> =
            FnPtr::<fn(*mut ()) -> ()>::new(|x: *mut ()| -> () {
                unsafe {
                    return (unsafe { ((*x) as std___atomic_base_float__false_).load_6(None) });
                }
            });
        (unsafe { test_seq_cst_0(exchange_no_arg.copy_from(), load.copy_from()) });
        (unsafe { test_seq_cst_1(exchange_with_order.copy_from(), load.copy_from()) });
    }
}
pub unsafe fn test_impl_13() {
    const _: () = assert!(
        true,
        "noexcept(std::declval<MaybeVolatile<std::atomic<T>>&>() = (T(0)))"
    );;
    {
        let mut a: () = unimplemented!();
        let mut r: () = (unsafe {
            (a as std___atomic_base_float__false_).exchange_9(1.2_f64, Some(unimplemented!()))
        });
        unimplemented!();
        unimplemented!();
    }
    {
        let mut exchange: FnPtr<fn(*mut (), (), ())> =
            FnPtr::<fn(*mut (), (), ())>::new(|x: *mut (), mut _a1: (), mut new_val: ()| unsafe {
                (unsafe {
                    ((*x) as std___atomic_base_float__false_)
                        .exchange_9(new_val, Some(unimplemented!()))
                });
            });
        let mut load: FnPtr<fn(*mut ()) -> ()> =
            FnPtr::<fn(*mut ()) -> ()>::new(|x: *mut ()| -> () {
                unsafe {
                    return (unsafe {
                        ((*x) as std___atomic_base_float__false_).load_5(Some(unimplemented!()))
                    });
                }
            });
        (unsafe { test_acquire_release_9(exchange.copy_from(), load.copy_from()) });
    }
    {
        let mut exchange_no_arg: FnPtr<fn(*mut (), (), ())> =
            FnPtr::<fn(*mut (), (), ())>::new(|x: *mut (), mut _a1: (), mut new_val: ()| unsafe {
                (unsafe { ((*x) as std___atomic_base_float__false_).exchange_9(new_val, None) });
            });
        let mut exchange_with_order: FnPtr<fn(*mut (), (), ())> =
            FnPtr::<fn(*mut (), (), ())>::new(|x: *mut (), mut _a1: (), mut new_val: ()| unsafe {
                (unsafe {
                    ((*x) as std___atomic_base_float__false_)
                        .exchange_9(new_val, Some(unimplemented!()))
                });
            });
        let mut load: FnPtr<fn(*mut ()) -> ()> =
            FnPtr::<fn(*mut ()) -> ()>::new(|x: *mut ()| -> () {
                unsafe {
                    return (unsafe { ((*x) as std___atomic_base_float__false_).load_5(None) });
                }
            });
        (unsafe { test_seq_cst_2(exchange_no_arg.copy_from(), load.copy_from()) });
        (unsafe { test_seq_cst_3(exchange_with_order.copy_from(), load.copy_from()) });
    }
}
pub unsafe fn test_impl_14() {
    const _: () = assert!(
        true,
        "noexcept(std::declval<MaybeVolatile<std::atomic<T>>&>() = (T(0)))"
    );;
    {
        let mut a: () = unimplemented!();
        let mut r: () = (unsafe {
            (a as std___atomic_base_double__false_).exchange_10(1.2_f64, Some(unimplemented!()))
        });
        unimplemented!();
        unimplemented!();
    }
    {
        let mut exchange: FnPtr<fn(*mut (), (), ())> =
            FnPtr::<fn(*mut (), (), ())>::new(|x: *mut (), mut _a1: (), mut new_val: ()| unsafe {
                (unsafe {
                    ((*x) as std___atomic_base_double__false_)
                        .exchange_10(new_val, Some(unimplemented!()))
                });
            });
        let mut load: FnPtr<fn(*mut ()) -> ()> =
            FnPtr::<fn(*mut ()) -> ()>::new(|x: *mut ()| -> () {
                unsafe {
                    return (unsafe {
                        ((*x) as std___atomic_base_double__false_).load_6(Some(unimplemented!()))
                    });
                }
            });
        (unsafe { test_acquire_release_10(exchange.copy_from(), load.copy_from()) });
    }
    {
        let mut exchange_no_arg: FnPtr<fn(*mut (), (), ())> =
            FnPtr::<fn(*mut (), (), ())>::new(|x: *mut (), mut _a1: (), mut new_val: ()| unsafe {
                (unsafe { ((*x) as std___atomic_base_double__false_).exchange_10(new_val, None) });
            });
        let mut exchange_with_order: FnPtr<fn(*mut (), (), ())> =
            FnPtr::<fn(*mut (), (), ())>::new(|x: *mut (), mut _a1: (), mut new_val: ()| unsafe {
                (unsafe {
                    ((*x) as std___atomic_base_double__false_)
                        .exchange_10(new_val, Some(unimplemented!()))
                });
            });
        let mut load: FnPtr<fn(*mut ()) -> ()> =
            FnPtr::<fn(*mut ()) -> ()>::new(|x: *mut ()| -> () {
                unsafe {
                    return (unsafe { ((*x) as std___atomic_base_double__false_).load_6(None) });
                }
            });
        (unsafe { test_seq_cst_4(exchange_no_arg.copy_from(), load.copy_from()) });
        (unsafe { test_seq_cst_5(exchange_with_order.copy_from(), load.copy_from()) });
    }
}
pub unsafe fn test_impl_15() {
    const _: () = assert!(
        true,
        "noexcept(std::declval<MaybeVolatile<std::atomic<T>>&>() = (T(0)))"
    );;
    {
        let mut a: () = unimplemented!();
        let mut r: () = (unsafe {
            (a as std___atomic_base_double__false_).exchange_9(1.2_f64, Some(unimplemented!()))
        });
        unimplemented!();
        unimplemented!();
    }
    {
        let mut exchange: FnPtr<fn(*mut (), (), ())> =
            FnPtr::<fn(*mut (), (), ())>::new(|x: *mut (), mut _a1: (), mut new_val: ()| unsafe {
                (unsafe {
                    ((*x) as std___atomic_base_double__false_)
                        .exchange_9(new_val, Some(unimplemented!()))
                });
            });
        let mut load: FnPtr<fn(*mut ()) -> ()> =
            FnPtr::<fn(*mut ()) -> ()>::new(|x: *mut ()| -> () {
                unsafe {
                    return (unsafe {
                        ((*x) as std___atomic_base_double__false_).load_5(Some(unimplemented!()))
                    });
                }
            });
        (unsafe { test_acquire_release_11(exchange.copy_from(), load.copy_from()) });
    }
    {
        let mut exchange_no_arg: FnPtr<fn(*mut (), (), ())> =
            FnPtr::<fn(*mut (), (), ())>::new(|x: *mut (), mut _a1: (), mut new_val: ()| unsafe {
                (unsafe { ((*x) as std___atomic_base_double__false_).exchange_9(new_val, None) });
            });
        let mut exchange_with_order: FnPtr<fn(*mut (), (), ())> =
            FnPtr::<fn(*mut (), (), ())>::new(|x: *mut (), mut _a1: (), mut new_val: ()| unsafe {
                (unsafe {
                    ((*x) as std___atomic_base_double__false_)
                        .exchange_9(new_val, Some(unimplemented!()))
                });
            });
        let mut load: FnPtr<fn(*mut ()) -> ()> =
            FnPtr::<fn(*mut ()) -> ()>::new(|x: *mut ()| -> () {
                unsafe {
                    return (unsafe { ((*x) as std___atomic_base_double__false_).load_5(None) });
                }
            });
        (unsafe { test_seq_cst_6(exchange_no_arg.copy_from(), load.copy_from()) });
        (unsafe { test_seq_cst_7(exchange_with_order.copy_from(), load.copy_from()) });
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
