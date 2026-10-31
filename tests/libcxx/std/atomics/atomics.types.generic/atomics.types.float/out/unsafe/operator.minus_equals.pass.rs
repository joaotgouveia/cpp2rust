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
    mut store_op: FnPtr<fn(*mut (), f64, f64)>,
    mut load_op: FnPtr<fn(*mut ()) -> f64>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_seq_cst_3(
    mut store_op: FnPtr<fn(*mut (), f64, f64)>,
    mut load_op: FnPtr<fn(*mut ()) -> f64>,
) {
    &(store_op);
    &(load_op);
}
pub unsafe fn test_impl_4() {
    const _: () = assert!(
        true,
        "HasVolatileMinusEquals<T> == std::atomic<T>::is_always_lock_free"
    );;
    const _: () = assert!(
        true,
        "noexcept(std::declval<MaybeVolatile<std::atomic<T>>&>() -= T(0))"
    );;
    {
        let mut a: () = unimplemented!();
        let mut r: f32 = unimplemented!();
        assert!(((r) == ((3.1_f64 as f32) - (1.2_f64 as f32))));
        assert!(((unimplemented!()) == ((3.1_f64 as f32) - (1.2_f64 as f32))));
    }
    {
        let mut minus_equals: FnPtr<fn(*mut (), f32, f32)> = FnPtr::<fn(*mut (), f32, f32)>::new(
            |x: *mut (), mut old_value: f32, mut new_val: f32| unsafe {
                unimplemented!();
            },
        );
        let mut load: FnPtr<fn(*mut ()) -> f32> =
            FnPtr::<fn(*mut ()) -> f32>::new(|x: *mut ()| -> f32 {
                unsafe {
                    return unimplemented!();
                }
            });
        (unsafe { test_seq_cst_0(minus_equals.copy_from(), load.copy_from()) });
    }
}
pub unsafe fn test_impl_5() {
    const _: () = assert!(
        true,
        "HasVolatileMinusEquals<T> == std::atomic<T>::is_always_lock_free"
    );;
    const _: () = assert!(
        true,
        "noexcept(std::declval<MaybeVolatile<std::atomic<T>>&>() -= T(0))"
    );;
    {
        let mut a: () = unimplemented!();
        let mut r: f32 = unimplemented!();
        assert!(((r) == ((3.1_f64 as f32) - (1.2_f64 as f32))));
        assert!(((unimplemented!()) == ((3.1_f64 as f32) - (1.2_f64 as f32))));
    }
    {
        let mut minus_equals: FnPtr<fn(*mut (), f32, f32)> = FnPtr::<fn(*mut (), f32, f32)>::new(
            |x: *mut (), mut old_value: f32, mut new_val: f32| unsafe {
                unimplemented!();
            },
        );
        let mut load: FnPtr<fn(*mut ()) -> f32> =
            FnPtr::<fn(*mut ()) -> f32>::new(|x: *mut ()| -> f32 {
                unsafe {
                    return unimplemented!();
                }
            });
        (unsafe { test_seq_cst_1(minus_equals.copy_from(), load.copy_from()) });
    }
}
pub unsafe fn test_impl_6() {
    const _: () = assert!(
        true,
        "HasVolatileMinusEquals<T> == std::atomic<T>::is_always_lock_free"
    );;
    const _: () = assert!(
        true,
        "noexcept(std::declval<MaybeVolatile<std::atomic<T>>&>() -= T(0))"
    );;
    {
        let mut a: () = unimplemented!();
        let mut r: f64 = unimplemented!();
        assert!(((r) == ((3.1_f64) - (1.2_f64))));
        assert!(((unimplemented!()) == ((3.1_f64) - (1.2_f64))));
    }
    {
        let mut minus_equals: FnPtr<fn(*mut (), f64, f64)> = FnPtr::<fn(*mut (), f64, f64)>::new(
            |x: *mut (), mut old_value: f64, mut new_val: f64| unsafe {
                unimplemented!();
            },
        );
        let mut load: FnPtr<fn(*mut ()) -> f64> =
            FnPtr::<fn(*mut ()) -> f64>::new(|x: *mut ()| -> f64 {
                unsafe {
                    return unimplemented!();
                }
            });
        (unsafe { test_seq_cst_2(minus_equals.copy_from(), load.copy_from()) });
    }
}
pub unsafe fn test_impl_7() {
    const _: () = assert!(
        true,
        "HasVolatileMinusEquals<T> == std::atomic<T>::is_always_lock_free"
    );;
    const _: () = assert!(
        true,
        "noexcept(std::declval<MaybeVolatile<std::atomic<T>>&>() -= T(0))"
    );;
    {
        let mut a: () = unimplemented!();
        let mut r: f64 = unimplemented!();
        assert!(((r) == ((3.1_f64) - (1.2_f64))));
        assert!(((unimplemented!()) == ((3.1_f64) - (1.2_f64))));
    }
    {
        let mut minus_equals: FnPtr<fn(*mut (), f64, f64)> = FnPtr::<fn(*mut (), f64, f64)>::new(
            |x: *mut (), mut old_value: f64, mut new_val: f64| unsafe {
                unimplemented!();
            },
        );
        let mut load: FnPtr<fn(*mut ()) -> f64> =
            FnPtr::<fn(*mut ()) -> f64>::new(|x: *mut ()| -> f64 {
                unsafe {
                    return unimplemented!();
                }
            });
        (unsafe { test_seq_cst_3(minus_equals.copy_from(), load.copy_from()) });
    }
}
pub unsafe fn test_8() {
    (unsafe { test_impl_4() });
    {
        (unsafe { test_impl_5() });
    }
}
pub unsafe fn test_9() {
    (unsafe { test_impl_6() });
    {
        (unsafe { test_impl_7() });
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
    (unsafe { test_8() });
    (unsafe { test_9() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
