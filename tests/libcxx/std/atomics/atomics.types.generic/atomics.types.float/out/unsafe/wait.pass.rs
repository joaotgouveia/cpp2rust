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
    mut load_op: FnPtr<fn(*mut std_atomic_float_) -> ()>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_1(
    mut store_op: FnPtr<fn(*mut std_atomic_float_, (), ())>,
    mut load_op: FnPtr<fn(*mut std_atomic_float_) -> ()>,
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
    mut store_op: FnPtr<fn(*mut std_atomic_double_, (), ())>,
    mut load_op: FnPtr<fn(*mut std_atomic_double_) -> ()>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_5(
    mut store_op: FnPtr<fn(*mut std_atomic_double_, (), ())>,
    mut load_op: FnPtr<fn(*mut std_atomic_double_) -> ()>,
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
        "noexcept(std::declval<MaybeVolatile<std::atomic<T>>&>().wait(T()))"
    );;
    {
        let mut a: std_atomic_float_ = unimplemented!();
        (unsafe {
            (a as std___atomic_base_float__false_).wait_20(1.1_f64, Some(unimplemented!()))
        });
    }
    {
        let mut store: FnPtr<fn(*mut std_atomic_float_, (), ())> =
            FnPtr::<fn(*mut std_atomic_float_, (), ())>::new(
                |x: *mut std_atomic_float_, mut _a1: (), mut new_val: ()| unsafe {
                    (unsafe {
                        ((*x) as std___atomic_base_float__false_)
                            .store_4(new_val, Some(unimplemented!()))
                    });
                },
            );
        let mut load: FnPtr<fn(*mut std_atomic_float_) -> ()> =
            FnPtr::<fn(*mut std_atomic_float_) -> ()>::new(|x: *mut std_atomic_float_| -> () {
                unsafe {
                    let mut result: () = (unsafe {
                        ((*x) as std___atomic_base_float__false_).load_6(unimplemented!())
                    });
                    (unsafe {
                        ((*x) as std___atomic_base_float__false_)
                            .wait_20(9999.999_f64, Some(unimplemented!()))
                    });
                    return result;
                }
            });
        (unsafe { test_acquire_release_8(store.copy_from(), load.copy_from()) });
    }
    {
        let mut store: FnPtr<fn(*mut std_atomic_float_, (), ())> =
            FnPtr::<fn(*mut std_atomic_float_, (), ())>::new(
                |x: *mut std_atomic_float_, mut _a1: (), mut new_val: ()| unsafe {
                    (unsafe { ((*x) as std___atomic_base_float__false_).store_4(new_val, None) });
                },
            );
        let mut load_no_arg: FnPtr<fn(*mut std_atomic_float_) -> ()> =
            FnPtr::<fn(*mut std_atomic_float_) -> ()>::new(|x: *mut std_atomic_float_| -> () {
                unsafe {
                    let mut result: () = (unsafe {
                        ((*x) as std___atomic_base_float__false_).load_6(unimplemented!())
                    });
                    (unsafe {
                        ((*x) as std___atomic_base_float__false_).wait_20(9999.999_f64, None)
                    });
                    return result;
                }
            });
        let mut load_with_order: FnPtr<fn(*mut std_atomic_float_) -> ()> =
            FnPtr::<fn(*mut std_atomic_float_) -> ()>::new(|x: *mut std_atomic_float_| -> () {
                unsafe {
                    let mut result: () = (unsafe {
                        ((*x) as std___atomic_base_float__false_).load_6(unimplemented!())
                    });
                    (unsafe {
                        ((*x) as std___atomic_base_float__false_)
                            .wait_20(9999.999_f64, Some(unimplemented!()))
                    });
                    return result;
                }
            });
        (unsafe { test_seq_cst_0(store.copy_from(), load_no_arg.copy_from()) });
        (unsafe { test_seq_cst_1(store.copy_from(), load_with_order.copy_from()) });
    }
}
pub unsafe fn test_impl_13() {
    const _: () = assert!(
        true,
        "noexcept(std::declval<MaybeVolatile<std::atomic<T>>&>().wait(T()))"
    );;
    {
        let mut a: () = unimplemented!();
        (unsafe {
            (a as std___atomic_base_float__false_).wait_19(1.1_f64, Some(unimplemented!()))
        });
    }
    {
        let mut store: FnPtr<fn(*mut (), (), ())> =
            FnPtr::<fn(*mut (), (), ())>::new(|x: *mut (), mut _a1: (), mut new_val: ()| unsafe {
                (unsafe {
                    ((*x) as std___atomic_base_float__false_)
                        .store_3(new_val, Some(unimplemented!()))
                });
            });
        let mut load: FnPtr<fn(*mut ()) -> ()> =
            FnPtr::<fn(*mut ()) -> ()>::new(|x: *mut ()| -> () {
                unsafe {
                    let mut result: () = (unsafe {
                        ((*x) as std___atomic_base_float__false_).load_5(unimplemented!())
                    });
                    (unsafe {
                        ((*x) as std___atomic_base_float__false_)
                            .wait_19(9999.999_f64, Some(unimplemented!()))
                    });
                    return result;
                }
            });
        (unsafe { test_acquire_release_9(store.copy_from(), load.copy_from()) });
    }
    {
        let mut store: FnPtr<fn(*mut (), (), ())> =
            FnPtr::<fn(*mut (), (), ())>::new(|x: *mut (), mut _a1: (), mut new_val: ()| unsafe {
                (unsafe { ((*x) as std___atomic_base_float__false_).store_3(new_val, None) });
            });
        let mut load_no_arg: FnPtr<fn(*mut ()) -> ()> =
            FnPtr::<fn(*mut ()) -> ()>::new(|x: *mut ()| -> () {
                unsafe {
                    let mut result: () = (unsafe {
                        ((*x) as std___atomic_base_float__false_).load_5(unimplemented!())
                    });
                    (unsafe {
                        ((*x) as std___atomic_base_float__false_).wait_19(9999.999_f64, None)
                    });
                    return result;
                }
            });
        let mut load_with_order: FnPtr<fn(*mut ()) -> ()> =
            FnPtr::<fn(*mut ()) -> ()>::new(|x: *mut ()| -> () {
                unsafe {
                    let mut result: () = (unsafe {
                        ((*x) as std___atomic_base_float__false_).load_5(unimplemented!())
                    });
                    (unsafe {
                        ((*x) as std___atomic_base_float__false_)
                            .wait_19(9999.999_f64, Some(unimplemented!()))
                    });
                    return result;
                }
            });
        (unsafe { test_seq_cst_2(store.copy_from(), load_no_arg.copy_from()) });
        (unsafe { test_seq_cst_3(store.copy_from(), load_with_order.copy_from()) });
    }
}
pub unsafe fn test_impl_14() {
    const _: () = assert!(
        true,
        "noexcept(std::declval<MaybeVolatile<std::atomic<T>>&>().wait(T()))"
    );;
    {
        let mut a: std_atomic_double_ = unimplemented!();
        (unsafe {
            (a as std___atomic_base_double__false_).wait_20(1.1_f64, Some(unimplemented!()))
        });
    }
    {
        let mut store: FnPtr<fn(*mut std_atomic_double_, (), ())> =
            FnPtr::<fn(*mut std_atomic_double_, (), ())>::new(
                |x: *mut std_atomic_double_, mut _a1: (), mut new_val: ()| unsafe {
                    (unsafe {
                        ((*x) as std___atomic_base_double__false_)
                            .store_4(new_val, Some(unimplemented!()))
                    });
                },
            );
        let mut load: FnPtr<fn(*mut std_atomic_double_) -> ()> =
            FnPtr::<fn(*mut std_atomic_double_) -> ()>::new(|x: *mut std_atomic_double_| -> () {
                unsafe {
                    let mut result: () = (unsafe {
                        ((*x) as std___atomic_base_double__false_).load_6(unimplemented!())
                    });
                    (unsafe {
                        ((*x) as std___atomic_base_double__false_)
                            .wait_20(9999.999_f64, Some(unimplemented!()))
                    });
                    return result;
                }
            });
        (unsafe { test_acquire_release_10(store.copy_from(), load.copy_from()) });
    }
    {
        let mut store: FnPtr<fn(*mut std_atomic_double_, (), ())> =
            FnPtr::<fn(*mut std_atomic_double_, (), ())>::new(
                |x: *mut std_atomic_double_, mut _a1: (), mut new_val: ()| unsafe {
                    (unsafe { ((*x) as std___atomic_base_double__false_).store_4(new_val, None) });
                },
            );
        let mut load_no_arg: FnPtr<fn(*mut std_atomic_double_) -> ()> =
            FnPtr::<fn(*mut std_atomic_double_) -> ()>::new(|x: *mut std_atomic_double_| -> () {
                unsafe {
                    let mut result: () = (unsafe {
                        ((*x) as std___atomic_base_double__false_).load_6(unimplemented!())
                    });
                    (unsafe {
                        ((*x) as std___atomic_base_double__false_).wait_20(9999.999_f64, None)
                    });
                    return result;
                }
            });
        let mut load_with_order: FnPtr<fn(*mut std_atomic_double_) -> ()> =
            FnPtr::<fn(*mut std_atomic_double_) -> ()>::new(|x: *mut std_atomic_double_| -> () {
                unsafe {
                    let mut result: () = (unsafe {
                        ((*x) as std___atomic_base_double__false_).load_6(unimplemented!())
                    });
                    (unsafe {
                        ((*x) as std___atomic_base_double__false_)
                            .wait_20(9999.999_f64, Some(unimplemented!()))
                    });
                    return result;
                }
            });
        (unsafe { test_seq_cst_4(store.copy_from(), load_no_arg.copy_from()) });
        (unsafe { test_seq_cst_5(store.copy_from(), load_with_order.copy_from()) });
    }
}
pub unsafe fn test_impl_15() {
    const _: () = assert!(
        true,
        "noexcept(std::declval<MaybeVolatile<std::atomic<T>>&>().wait(T()))"
    );;
    {
        let mut a: () = unimplemented!();
        (unsafe {
            (a as std___atomic_base_double__false_).wait_19(1.1_f64, Some(unimplemented!()))
        });
    }
    {
        let mut store: FnPtr<fn(*mut (), (), ())> =
            FnPtr::<fn(*mut (), (), ())>::new(|x: *mut (), mut _a1: (), mut new_val: ()| unsafe {
                (unsafe {
                    ((*x) as std___atomic_base_double__false_)
                        .store_3(new_val, Some(unimplemented!()))
                });
            });
        let mut load: FnPtr<fn(*mut ()) -> ()> =
            FnPtr::<fn(*mut ()) -> ()>::new(|x: *mut ()| -> () {
                unsafe {
                    let mut result: () = (unsafe {
                        ((*x) as std___atomic_base_double__false_).load_5(unimplemented!())
                    });
                    (unsafe {
                        ((*x) as std___atomic_base_double__false_)
                            .wait_19(9999.999_f64, Some(unimplemented!()))
                    });
                    return result;
                }
            });
        (unsafe { test_acquire_release_11(store.copy_from(), load.copy_from()) });
    }
    {
        let mut store: FnPtr<fn(*mut (), (), ())> =
            FnPtr::<fn(*mut (), (), ())>::new(|x: *mut (), mut _a1: (), mut new_val: ()| unsafe {
                (unsafe { ((*x) as std___atomic_base_double__false_).store_3(new_val, None) });
            });
        let mut load_no_arg: FnPtr<fn(*mut ()) -> ()> =
            FnPtr::<fn(*mut ()) -> ()>::new(|x: *mut ()| -> () {
                unsafe {
                    let mut result: () = (unsafe {
                        ((*x) as std___atomic_base_double__false_).load_5(unimplemented!())
                    });
                    (unsafe {
                        ((*x) as std___atomic_base_double__false_).wait_19(9999.999_f64, None)
                    });
                    return result;
                }
            });
        let mut load_with_order: FnPtr<fn(*mut ()) -> ()> =
            FnPtr::<fn(*mut ()) -> ()>::new(|x: *mut ()| -> () {
                unsafe {
                    let mut result: () = (unsafe {
                        ((*x) as std___atomic_base_double__false_).load_5(unimplemented!())
                    });
                    (unsafe {
                        ((*x) as std___atomic_base_double__false_)
                            .wait_19(9999.999_f64, Some(unimplemented!()))
                    });
                    return result;
                }
            });
        (unsafe { test_seq_cst_6(store.copy_from(), load_no_arg.copy_from()) });
        (unsafe { test_seq_cst_7(store.copy_from(), load_with_order.copy_from()) });
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
