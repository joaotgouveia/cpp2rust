extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_seq_cst_0(
    mut store_op: FnPtr<fn(*mut (), f32, f32)>,
    mut load_op: FnPtr<fn(*mut ()) -> f32>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_1(
    mut store_op: FnPtr<fn(*mut (), f32, f32)>,
    mut load_op: FnPtr<fn(*mut ()) -> f32>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_2(
    mut store_op: FnPtr<fn(*mut (), f32, f32)>,
    mut load_op: FnPtr<fn(*mut ()) -> f32>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_3(
    mut store_op: FnPtr<fn(*mut (), f32, f32)>,
    mut load_op: FnPtr<fn(*mut ()) -> f32>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_4(
    mut store_op: FnPtr<fn(*mut (), f64, f64)>,
    mut load_op: FnPtr<fn(*mut ()) -> f64>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_5(
    mut store_op: FnPtr<fn(*mut (), f64, f64)>,
    mut load_op: FnPtr<fn(*mut ()) -> f64>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_6(
    mut store_op: FnPtr<fn(*mut (), f64, f64)>,
    mut load_op: FnPtr<fn(*mut ()) -> f64>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_7(
    mut store_op: FnPtr<fn(*mut (), f64, f64)>,
    mut load_op: FnPtr<fn(*mut ()) -> f64>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_acquire_release_8(
    mut store_op: FnPtr<fn(*mut (), f32, f32)>,
    mut load_op: FnPtr<fn(*mut ()) -> f32>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_acquire_release_9(
    mut store_op: FnPtr<fn(*mut (), f32, f32)>,
    mut load_op: FnPtr<fn(*mut ()) -> f32>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_acquire_release_10(
    mut store_op: FnPtr<fn(*mut (), f64, f64)>,
    mut load_op: FnPtr<fn(*mut ()) -> f64>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_acquire_release_11(
    mut store_op: FnPtr<fn(*mut (), f64, f64)>,
    mut load_op: FnPtr<fn(*mut ()) -> f64>,
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
        let mut r: f32 = unimplemented!();
        assert!(((unimplemented!()) == (1.2_f64 as f32)));
        assert!(((r) == (3.1_f64 as f32)));
    }
    {
        let mut exchange: FnPtr<fn(*mut (), f32, f32)> = FnPtr::<fn(*mut (), f32, f32)>::new(
            |x: *mut (), mut _a1: f32, mut new_val: f32| unsafe {
                unimplemented!();
            },
        );
        let mut load: FnPtr<fn(*mut ()) -> f32> =
            FnPtr::<fn(*mut ()) -> f32>::new(|x: *mut ()| -> f32 {
                unsafe {
                    return unimplemented!();
                }
            });
        (unsafe { test_acquire_release_8(exchange.copy_from(), load.copy_from()) });
    }
    {
        let mut exchange_no_arg: FnPtr<fn(*mut (), f32, f32)> = FnPtr::<fn(*mut (), f32, f32)>::new(
            |x: *mut (), mut _a1: f32, mut new_val: f32| unsafe {
                unimplemented!();
            },
        );
        let mut exchange_with_order: FnPtr<fn(*mut (), f32, f32)> =
            FnPtr::<fn(*mut (), f32, f32)>::new(
                |x: *mut (), mut _a1: f32, mut new_val: f32| unsafe {
                    unimplemented!();
                },
            );
        let mut load: FnPtr<fn(*mut ()) -> f32> =
            FnPtr::<fn(*mut ()) -> f32>::new(|x: *mut ()| -> f32 {
                unsafe {
                    return unimplemented!();
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
        let mut r: f32 = unimplemented!();
        assert!(((unimplemented!()) == (1.2_f64 as f32)));
        assert!(((r) == (3.1_f64 as f32)));
    }
    {
        let mut exchange: FnPtr<fn(*mut (), f32, f32)> = FnPtr::<fn(*mut (), f32, f32)>::new(
            |x: *mut (), mut _a1: f32, mut new_val: f32| unsafe {
                unimplemented!();
            },
        );
        let mut load: FnPtr<fn(*mut ()) -> f32> =
            FnPtr::<fn(*mut ()) -> f32>::new(|x: *mut ()| -> f32 {
                unsafe {
                    return unimplemented!();
                }
            });
        (unsafe { test_acquire_release_9(exchange.copy_from(), load.copy_from()) });
    }
    {
        let mut exchange_no_arg: FnPtr<fn(*mut (), f32, f32)> = FnPtr::<fn(*mut (), f32, f32)>::new(
            |x: *mut (), mut _a1: f32, mut new_val: f32| unsafe {
                unimplemented!();
            },
        );
        let mut exchange_with_order: FnPtr<fn(*mut (), f32, f32)> =
            FnPtr::<fn(*mut (), f32, f32)>::new(
                |x: *mut (), mut _a1: f32, mut new_val: f32| unsafe {
                    unimplemented!();
                },
            );
        let mut load: FnPtr<fn(*mut ()) -> f32> =
            FnPtr::<fn(*mut ()) -> f32>::new(|x: *mut ()| -> f32 {
                unsafe {
                    return unimplemented!();
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
        let mut r: f64 = unimplemented!();
        assert!(((unimplemented!()) == (1.2_f64)));
        assert!(((r) == (3.1_f64)));
    }
    {
        let mut exchange: FnPtr<fn(*mut (), f64, f64)> = FnPtr::<fn(*mut (), f64, f64)>::new(
            |x: *mut (), mut _a1: f64, mut new_val: f64| unsafe {
                unimplemented!();
            },
        );
        let mut load: FnPtr<fn(*mut ()) -> f64> =
            FnPtr::<fn(*mut ()) -> f64>::new(|x: *mut ()| -> f64 {
                unsafe {
                    return unimplemented!();
                }
            });
        (unsafe { test_acquire_release_10(exchange.copy_from(), load.copy_from()) });
    }
    {
        let mut exchange_no_arg: FnPtr<fn(*mut (), f64, f64)> = FnPtr::<fn(*mut (), f64, f64)>::new(
            |x: *mut (), mut _a1: f64, mut new_val: f64| unsafe {
                unimplemented!();
            },
        );
        let mut exchange_with_order: FnPtr<fn(*mut (), f64, f64)> =
            FnPtr::<fn(*mut (), f64, f64)>::new(
                |x: *mut (), mut _a1: f64, mut new_val: f64| unsafe {
                    unimplemented!();
                },
            );
        let mut load: FnPtr<fn(*mut ()) -> f64> =
            FnPtr::<fn(*mut ()) -> f64>::new(|x: *mut ()| -> f64 {
                unsafe {
                    return unimplemented!();
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
        let mut r: f64 = unimplemented!();
        assert!(((unimplemented!()) == (1.2_f64)));
        assert!(((r) == (3.1_f64)));
    }
    {
        let mut exchange: FnPtr<fn(*mut (), f64, f64)> = FnPtr::<fn(*mut (), f64, f64)>::new(
            |x: *mut (), mut _a1: f64, mut new_val: f64| unsafe {
                unimplemented!();
            },
        );
        let mut load: FnPtr<fn(*mut ()) -> f64> =
            FnPtr::<fn(*mut ()) -> f64>::new(|x: *mut ()| -> f64 {
                unsafe {
                    return unimplemented!();
                }
            });
        (unsafe { test_acquire_release_11(exchange.copy_from(), load.copy_from()) });
    }
    {
        let mut exchange_no_arg: FnPtr<fn(*mut (), f64, f64)> = FnPtr::<fn(*mut (), f64, f64)>::new(
            |x: *mut (), mut _a1: f64, mut new_val: f64| unsafe {
                unimplemented!();
            },
        );
        let mut exchange_with_order: FnPtr<fn(*mut (), f64, f64)> =
            FnPtr::<fn(*mut (), f64, f64)>::new(
                |x: *mut (), mut _a1: f64, mut new_val: f64| unsafe {
                    unimplemented!();
                },
            );
        let mut load: FnPtr<fn(*mut ()) -> f64> =
            FnPtr::<fn(*mut ()) -> f64>::new(|x: *mut ()| -> f64 {
                unsafe {
                    return unimplemented!();
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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    (unsafe { test_16() });
    (unsafe { test_17() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
